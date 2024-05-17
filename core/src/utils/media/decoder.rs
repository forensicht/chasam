use std::{collections::VecDeque, path::Path};

use anyhow::Context;
use bytes::Bytes;
use ffmpeg::{format, media::Type, software::scaling, util::frame};
use ffmpeg_next as ffmpeg;
use image::{imageops, DynamicImage, GenericImage, ImageBuffer, Rgba};

const FRAME_DIMENSION: u32 = 160;

#[derive(Debug, Default)]
struct VideoDump {
    width: u32,
    height: u32,
    frames: Vec<Bytes>,
}

pub fn make_thumbnail_to_vec<P>(media_path: P) -> anyhow::Result<(Vec<DynamicImage>, Vec<u8>)>
where
    P: AsRef<Path>,
{
    let dump = dump_video_frames(media_path)?;
    let frames = frames_to_image(&dump)?;
    let buf = concat_frames(&frames)?;

    Ok((frames, buf))
}

fn dump_video_frames<P: AsRef<Path>>(video_path: P) -> anyhow::Result<VideoDump> {
    ffmpeg::init()?;

    let mut options = ffmpeg::Dictionary::new();
    options.set("framerate", "1");

    let mut input_format_context = ffmpeg::format::input_with_dictionary(&video_path, options)?;

    // shows a dump of the video
    // let video_path = video_path.as_os_str().to_str().unwrap();
    // format::context::input::dump(&input_format_context, 0, Some(video_path));

    let (video_stream_index, frame_rate, mut decoder) = {
        let stream = input_format_context
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;

        let frame_rate = f64::from(stream.avg_frame_rate()).round() as i32;
        let stream_index = stream.index();
        let decode_context = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
        let decoder = decode_context.decoder().video()?;

        (stream_index, frame_rate, decoder)
    };

    let mut video_dump = VideoDump::default();
    video_dump.width = decoder.width();
    video_dump.height = decoder.height();

    let mut sws_context = scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        format::Pixel::RGBA,
        decoder.width(),
        decoder.height(),
        scaling::Flags::BILINEAR,
    )
    .with_context(|| "invalid swscontext parameter")?;

    let mut processed_frames = 0;

    let mut receive_and_process_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> anyhow::Result<(), ffmpeg::Error> {
            let mut decoded = frame::Video::empty();

            while decoder.receive_frame(&mut decoded).is_ok() {
                if processed_frames == 0 || processed_frames == frame_rate {
                    let mut rgb_frame = frame::Video::empty();
                    sws_context.run(&decoded, &mut rgb_frame)?;

                    let data = rgb_frame.data(0).to_owned();
                    video_dump.frames.push(Bytes::from(data));

                    processed_frames = 0;
                }

                processed_frames += 1;
            }

            Ok(())
        };

    for (stream, packet) in input_format_context.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            receive_and_process_frames(&mut decoder)?;
        }
    }

    decoder.send_eof()?;
    receive_and_process_frames(&mut decoder)?;

    Ok(video_dump)
}

fn concat_frames(frames: &[DynamicImage]) -> anyhow::Result<Vec<u8>> {
    let nframes = frames.len();
    let (cols, rows) = get_cols_and_rows(nframes);
    let nparts = cols * rows;
    let img_width_out: u32 = frames.iter().map(|img| img.width()).take(cols).sum();
    let img_height_out: u32 = frames.iter().map(|img| img.height()).take(rows).sum();

    // Initialize an image buffer with the appropriate size.
    let mut imgbuf = ImageBuffer::new(img_width_out, img_height_out);
    let mut accumulated_width = 0;
    let mut accumulated_height = 0;

    // Distributes frames using the residual distribution algorithm.
    let mut distribution_frames = distribute_frames(nframes, nparts)?;
    let mut step = 0;

    for img in frames.iter() {
        step += 1;

        if step == *distribution_frames.front().unwrap() {
            distribution_frames.pop_front();
            step = 0;

            if accumulated_width == img_width_out {
                accumulated_width = 0;
                accumulated_height += img.height();
            }

            imgbuf.copy_from(img, accumulated_width, accumulated_height)?;
            accumulated_width += img.width();
        }
    }

    let mut buf = Vec::new();
    imgbuf.write_to(
        &mut std::io::Cursor::new(&mut buf),
        image::ImageOutputFormat::Jpeg(75),
    )?;

    Ok(buf)
}

fn frames_to_image(dump: &VideoDump) -> anyhow::Result<Vec<DynamicImage>> {
    let width = dump.width;
    let height = dump.height;
    let frames = dump
        .frames
        .iter()
        .map(|frame| {
            let img_buf = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, frame.to_vec())
                .ok_or(anyhow::Error::msg("could not to create image buffer"))?;
            let img = DynamicImage::ImageRgba8(img_buf);
            let img = img.resize(
                FRAME_DIMENSION,
                FRAME_DIMENSION,
                imageops::FilterType::Lanczos3,
            );
            Ok(img)
        })
        .collect::<anyhow::Result<Vec<DynamicImage>>>();

    frames
}

#[inline]
fn get_cols_and_rows(nframes: usize) -> (usize, usize) {
    if nframes >= 100 {
        (10, 10)
    } else if nframes >= 64 {
        (8, 8)
    } else if nframes >= 49 {
        (7, 7)
    } else if nframes >= 36 {
        (6, 6)
    } else if nframes >= 25 {
        (5, 5)
    } else if nframes >= 16 {
        (4, 4)
    } else if nframes >= 9 {
        (3, 3)
    } else if nframes >= 4 {
        (2, 2)
    } else {
        (3, 1)
    }
}

fn distribute_frames(nframes: usize, nparts: usize) -> anyhow::Result<VecDeque<usize>> {
    if nparts == 0 {
        anyhow::bail!("nparts must be greater than 0");
    }

    let base_distribution = nframes / nparts;
    let remainder = nframes % nparts;

    let mut distribution = VecDeque::with_capacity(nparts);
    for _ in 0..nparts {
        distribution.push_back(base_distribution);
    }

    for i in 0..remainder {
        distribution[i] += 1;
    }

    Ok(distribution)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image;

    #[test]
    fn test_should_distribute_frames_in_balanced_parts() {
        let nframes = 15;
        let nparts = 9;
        let distribution = distribute_frames(nframes, nparts).unwrap();

        let want_distribution = VecDeque::from([2, 2, 2, 2, 2, 2, 1, 1, 1]);

        assert_eq!(distribution, want_distribution);
        assert_eq!(nframes, distribution.iter().sum());
    }

    #[test]
    fn test_should_dump_video_frames() {
        let filename = "D:\\video\\vid00.mp4";
        match dump_video_frames(filename) {
            Ok(dump) => {
                println!("Frames: {}", dump.frames.len());
                save_file_dump_frame(&dump).expect("error saving file dump frame");
                assert!(true);
            }
            Err(err) => assert!(false, "{err}"),
        }
    }

    #[test]
    fn test_should_concat_video_frames() {
        let filename = "D:\\video\\vid00.mp4";
        if let Ok(dump) = dump_video_frames(filename) {
            match frames_to_image(&dump) {
                Ok(frames) => match concat_frames(&frames) {
                    Ok(buf) => {
                        let len = buf.len();
                        println!("image bytes: {}", len);
                        assert_ne!(len, 0);
                    }
                    Err(err) => assert!(false, "{err}"),
                },
                Err(err) => assert!(false, "{err}"),
            }
        } else {
            assert!(false);
        }
    }

    fn save_file_dump_frame(dump: &VideoDump) -> anyhow::Result<()> {
        let width = dump.width;
        let height = dump.height;
        let frames = &dump.frames;

        for (index, frame) in frames.iter().enumerate() {
            let path = format!("D:\\video\\frames\\vid_{}.jpeg", index);
            image::save_buffer(
                path,
                &frame.slice(..),
                width,
                height,
                image::ColorType::Rgba8,
            )?;
        }

        Ok(())
    }
}
