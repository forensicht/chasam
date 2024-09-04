use std::io;
use std::io::prelude::*;

#[allow(unused)]
pub struct Lines<R> {
    reader: io::BufReader<R>,
    buf: String,
}

impl<R: Read> Lines<R> {
    pub fn new(r: R) -> Lines<R> {
        Lines {
            reader: io::BufReader::new(r),
            buf: String::new(),
        }
    }

    pub fn next(&mut self) -> Option<io::Result<&str>> {
        self.buf.clear();

        match self.reader.read_line(&mut self.buf) {
            Ok(nbytes) => {
                if nbytes == 0 {
                    None // no more lines!
                } else {
                    let line = self.buf.trim_end();
                    Some(Ok(line))
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

// The above code is equivalent to this code.
// This method reduces String allocation amount.
//
// let mut reader = io::BufReader::new(file);
// let mut buf = String::new();
// while reader.read_line(&mut buf)? > 0 {
//     {
//         let line = buf.trim_end();
//         println!("{}", line);
//     }
//     buf.clear();
// }
