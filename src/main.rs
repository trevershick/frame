use clap::{App, Arg};
use frame::ToFrame;
use std::process::exit;

fn main() {
    let matches = App::new("frame")
        .version("1.0.0")
        .about("Convert a duration specifier to a frame")
        .arg(
            Arg::with_name("duration")
                .required(true)
                .index(1)
                .help("A duration specifier in the format <number>s|m|h|d|w|y', ex. 37h or 3d'"),
        )
        .get_matches();

    let duration: &str = matches.value_of("duration").unwrap();

    match duration.to_frame() {
        Ok(frame) => {
            println!("{} {} {}", frame[0], frame[1], frame[2]);
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
