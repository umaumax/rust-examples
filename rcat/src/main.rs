use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Stdin;
use std::ops::Fn;
use std::str::FromStr;

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
impl FromStr for ColorWhen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(ColorWhen::Always),
            "never" => Ok(ColorWhen::Never),
            "auto" => Ok(ColorWhen::Auto),
            _ => Err(String::from(format!(
                "ColorWhen string is 'always|never|auto', not allowed '{}'",
                s
            ))),
        }
    }
}

fn main() {
    let matches = build_app().get_matches();
    let color_when = ColorWhen::from_str(matches.value_of("color").unwrap());
    let base_line = matches
        .value_of("line")
        .unwrap_or("0")
        .parse::<i32>()
        .expect("parse line option");
    let line_context = matches
        .value_of("context")
        .unwrap_or("3")
        .parse::<i32>()
        .expect("parse context option");

    let isatty: bool = atty::is(atty::Stream::Stdout);
    let color_flag: bool = match color_when {
        Ok(ColorWhen::Always) => true,
        Ok(ColorWhen::Never) => false,
        Ok(ColorWhen::Auto) => isatty,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let files: Vec<_> = matches.values_of("files").unwrap().collect();

    if files.len() == 0 {
        let stdin: Stdin = std::io::stdin();
        let mut buf_file = BufReader::new(stdin);
    // TODO
    // let _ = write_lines(&mut buf_file, f).expect("write all failed");
    } else {
        files
            .iter()
            .for_each(|filename| match File::open(filename) {
                Ok(file) => {
                    let mut buf_file = BufReader::new(file);
                    // TODO
                    // let _ = write_lines(&mut buf_file);
                    // TODO: このクロージャを変数に代入するようにしたいが，エラーがでてしまう
                    let _ = write_lines(&mut buf_file, |nr, s| {
                        if base_line == nr {
                            // NOTE: green
                            if color_flag {
                                print!("\x1b[32m{:>6}  {}\x1b[m", nr, s);
                            } else {
                                print!("{:>6}  {}", nr, s);
                            }
                        } else if base_line <= 0
                            || base_line - line_context <= nr && nr <= base_line + line_context
                        {
                            print!("{:>6}  {}", nr, s);
                        }
                        if base_line > 0 && nr == base_line + line_context {
                            return false;
                        }
                        true
                    });
                }
                Err(e) => eprintln!(
                    "{}: '{}' at {}",
                    e,
                    filename,
                    env::current_dir().unwrap().to_string_lossy()
                ),
            })
    }
}

fn write_all(istream: &mut dyn BufRead) -> Result<(), io::Error> {
    let mut s = String::new();
    istream.read_to_string(&mut s)?;
    println!("{}", s);
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
            Ok(0) => break, // EOR
            Ok(_) => {
                let ret = f(nr, &s);
                s.clear();
                if !ret {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
        nr += 1;
    }
    Ok(())
}
