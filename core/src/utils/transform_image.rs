use std::f64::consts::PI;

pub type PixelMatrix = Vec<Vec<f64>>;

pub fn image_to_pixel_matrix(img: &image::GrayImage) -> PixelMatrix {
    let (w, h) = img.dimensions();
    let mut output_matrix = new_canvas(h, w);

    for y in 0..h {
        for x in 0..w {
            output_matrix[y as usize][x as usize] = img.get_pixel(x, y).0[0] as f64;
        }
    }

    output_matrix
}

fn new_canvas(x: u32, y: u32) -> PixelMatrix {
    let canvas = vec![vec![0.0; y as usize]; x as usize];
    canvas
}

/// dct2d function returns a  result of DCT2D by using the seperable property.
pub fn dct2d<T>(input: &[T], w: usize, h: usize) -> Vec<Vec<f64>>
where
    T: AsRef<[f64]>,
{
    let mut output = vec![vec![0.0; w]; h];

    for i in 0..h {
        output[i] = dct1d(input[i].as_ref());
    }

    for i in 0..w {
        let mut aux = vec![0.0; h];

        for j in 0..h {
            aux[j] = output[j][i];
        }

        let rows = dct1d(&aux);

        for j in 0..rows.len() {
            output[j][i] = rows[j];
        }
    }

    output
}

/// dct1d function returns result of DCT-II.
/// dct type II, unscaled. Algorithm by Byeong Gi Lee, 1984.
fn dct1d(input: &[f64]) -> Vec<f64> {
    let mut input = input.to_owned();
    let len = input.len();
    let mut temp = vec![0.0; len];

    forward_transform(&mut input, &mut temp, len);
    input.to_owned()
}

fn forward_transform(input: &mut [f64], temp: &mut [f64], len: usize) {
    if len == 1 {
        return;
    }

    let half_len = len / 2;
    for i in 0..half_len {
        let (x, y) = (input[i], input[len - 1 - i]);
        temp[i] = x + y;
        temp[i + half_len] = (x - y) / (f64::cos((i as f64 + 0.5) * PI / len as f64) * 2.0);
    }

    forward_transform(temp, input, half_len);
    forward_transform(&mut temp[half_len..], input, half_len);

    for i in 0..half_len - 1 {
        input[i * 2 + 0] = temp[i];
        input[i * 2 + 1] = temp[i + half_len] + temp[i + half_len + 1];
    }

    input[len - 2] = temp[half_len - 1];
    input[len - 1] = temp[len - 1];
}
