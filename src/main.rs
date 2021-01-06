mod cli;
mod xgl;

use cli::parse_opts;
use serde::Serialize;
use std::fs;
use std::io;
use xgl::parse;

#[derive(Serialize)]
struct Line<'a> {
  #[serde(rename(serialize = "Time"))]
  time: &'a String,
  #[serde(rename(serialize = "Id"))]
  id: &'a String,
  #[serde(rename(serialize = "Command"))]
  command: &'a String,
  #[serde(rename(serialize = "Argument"))]
  argument: &'a str,
}

fn main() {
  let opts = parse_opts();

  if opts.file == "-" {
    let reader = io::BufReader::new(io::stdin());
    parse(reader, print_line).unwrap();
  } else {
    let f = fs::File::open(opts.file).unwrap();
    let reader = io::BufReader::new(f);
    parse(reader, print_line).unwrap();
  }
}

fn print_line(header: &xgl::Header, arg: &str) {
  let line = Line {
    time: &header.time,
    id: &header.id,
    command: &header.command,
    argument: arg,
  };

  let json = serde_json::to_string(&line).unwrap();
  println!("{}", json);
}
