use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead, Read};

pub fn get_buf_reader(file: &str) -> BufReader<Box<dyn std::io::Read>> {
    let read: Box<dyn std::io::Read> = match file {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(file).expect(&(format!("Error opening {} file", file)))),
    };
    BufReader::new(read)
}

/// 非UTF-8文字を含むテキストに対してread_lineを可能とするstd::io::BufReadのラッパーの例
/// FYI: [polymorphism \- How to do polymorphic IO from either a File or stdin in Rust? \- Stack Overflow]( https://stackoverflow.com/questions/36088116/how-to-do-polymorphic-io-from-either-a-file-or-stdin-in-rust/49964042 )
struct LossyReader<'a> {
    source: Box<dyn std::io::BufRead + 'a>,
}
impl<'a> LossyReader<'a> {
    fn new(r: Box<dyn std::io::BufRead>) -> Box<dyn std::io::BufRead> {
        Box::new(LossyReader { source: r })
    }
}
impl<'a> Read for LossyReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf)
    }
}
// [std::io::BufRead \- Rust]( https://doc.rust-lang.org/std/io/trait.BufRead.html#required-methods )
impl<'a> BufRead for LossyReader<'a> {
    // Required Methods
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.source.consume(amt);
    }
    // Provided Methods
    // trait側でデフォルト実装が存在するので、実装は強制されない
    fn read_line(&mut self, buf: &mut String) -> std::io::Result<usize> {
        let mut byte_buf = vec![];
        let num_bytes = self.source.read_until(b'\n', &mut byte_buf)?;
        *buf = String::from_utf8_lossy(&byte_buf).into_owned();
        Ok(num_bytes)
    }
}

#[derive(Debug)]
struct ArgError {
    message: String,
}

impl std::error::Error for ArgError {}
impl std::fmt::Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArgError: {}", self.message)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return Err(Box::new(ArgError {
            message: "Invalid args".to_string(),
        }));
    }
    let filepath = &args[1];

    let buf_reader = get_buf_reader(&filepath);
    let r: &mut Box<dyn std::io::BufRead> = &mut LossyReader::new(Box::new(buf_reader));
    let mut s = String::new();
    loop {
        s.clear();
        match r.read_line(&mut s) {
            Ok(0) => break, // EOF
            Ok(_) => {
                print!("{}", s);
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }
    Ok(())
}
