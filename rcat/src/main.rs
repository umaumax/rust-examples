use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::ops::Fn;
use std::str::FromStr;

use anyhow::{Context, Result};

fn build_app() -> clap::App<'static, 'static> {
    let program = std::env::args()
        .nth(0)
        .and_then(|s| {
            std::path::PathBuf::from(s)
                .file_stem()
                .map(|s| s.to_string_lossy().into_owned())
        })
        .unwrap();

    clap::App::new(program)
        .about("original cat command by rust")
        .version("0.0.1")
        .setting(clap::AppSettings::VersionlessSubcommands)
        .arg(clap::Arg::from_usage(
            "--color=[WHEN] \
            'use markers to highlight the mathing strings; \
            WHEN is [always], [never], or [auto]'",
        ))
        .arg(clap::Arg::from_usage(
            "--line=[NUM] \
            'print taeget line of output context;",
        ))
        .arg(clap::Arg::from_usage(
            "-C --context=[NUM] \
            'print NUM lines of output context;",
        ))
        .arg(
            clap::Arg::with_name("files")
                .help("Sets the input file to use")
                .required(true)
                .multiple(true)
                .index(1),
        )
}

enum ColorWhen {
    Always,
    Never,
    Auto,
}

#[derive(Debug)]
struct ColorWhenError(String);
impl fmt::Display for ColorWhenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ColorWhen string is 'always|never|auto', not allowed '{}'",
            self
        )
    }
}
impl std::error::Error for ColorWhenError {}

impl FromStr for ColorWhen {
    type Err = ColorWhenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(ColorWhen::Always),
            "never" => Ok(ColorWhen::Never),
            "auto" => Ok(ColorWhen::Auto),
            _ => Err(ColorWhenError(s.to_string())),
        }
    }
}
impl ColorWhen {
    fn str_to_color_flag(&self, isatty: bool) -> bool {
        match self {
            ColorWhen::Always => true,
            ColorWhen::Never => false,
            ColorWhen::Auto => isatty,
        }
    }
}

pub fn get_buf_reader(file: &str) -> BufReader<Box<dyn std::io::Read>> {
    let read: Box<dyn std::io::Read> = match file {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(file).expect(&(format!("Error opening {} file", file)))),
    };
    BufReader::new(read)
}

pub fn get_buf_reader_safe(
    file: &str,
) -> Result<BufReader<Box<dyn std::io::Read>>, std::io::Error> {
    let reader: Box<dyn std::io::Read> = match file {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(file)?),
    };
    Ok(BufReader::new(reader))
}

fn main() -> Result<()> {
    let matches = build_app().get_matches();
    let base_line = matches
        .value_of("line")
        .unwrap_or("0")
        .parse::<i32>()
        .with_context(|| format!("failed parse --line option"))?;
    let line_context = matches
        .value_of("context")
        .unwrap_or("3")
        .parse::<i32>()
        .with_context(|| format!("failed parse -C, --context option"))?;
    let color_when = ColorWhen::from_str(matches.value_of("color").unwrap_or("auto"))
        .with_context(|| format!("failed parse --color option"))?;

    let isatty: bool = atty::is(atty::Stream::Stdout);
    let color_flag: bool = color_when.str_to_color_flag(isatty);

    let mut files: Vec<_> = matches.values_of("files").unwrap().collect();
    // NOTE: default input is stdin
    if files.len() == 0 {
        files.push("-");
    }
    files.iter().try_for_each(|filename| -> Result<()> {
        let mut reader = get_buf_reader_safe(filename).with_context(|| {
            format!(
                "while opening file '{}' at {}",
                filename,
                env::current_dir().unwrap().to_string_lossy()
            )
        })?;
        // TODO: このクロージャを変数に代入するようにしたいが，エラーがでてしまう
        write_lines(&mut reader, |nr, s| -> bool {
            let output_flag =
                base_line <= 0 || base_line - line_context <= nr && nr <= base_line + line_context;
            if output_flag {
                let mut prefix = "";
                let mut suffix = "";
                if base_line == nr && color_flag {
                    prefix = "\x1b[32m"; // NOTE: green
                    suffix = "\x1b[m";
                }
                print!("{}{:>6}  {}{}", prefix, nr, s, suffix);
            }
            // NOTE: skip rest of the file
            if base_line > 0 && nr == base_line + line_context {
                return false;
            }
            true
        })?;
        Ok(())
    })?;
    Ok(())
}

fn write_lines<R, F>(r: &mut R, f: F) -> Result<(), io::Error>
where
    R: std::io::BufRead,
    F: Fn(i32, &String) -> bool,
{
    let mut s = String::new();
    let mut nr = 1;
    loop {
        match r.read_line(&mut s) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let ret = f(nr, &s);
                s.clear();
                if !ret {
                    break;
                }
            }
            Err(err) => return Err(err),
        }
        nr += 1;
    }
    Ok(())
}
