#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::io::fs;
use std::io::{BufferedReader, File};

use std::vec::Vec;

use regex::Regex;


fn main() {
  let args = std::os::args();
  if args.len() < 2 {
    println!("too few args");
    return
  }

  let pattern = match Regex::new(args[1].as_slice()){
    Ok(re) => box re,
    Err(err) => fail!("{}", err)
  };

  if args.len() == 2 {
    println!("Reading from stdin");
    search_stdin(&pattern);
  } else {
    for file in args.slice_from(2).iter() {
      search(&pattern, Path::new(file.as_slice()));
    }
  }
}

fn search_stdin(pattern: &Box<Regex>) {
  for line in std::io::stdin().lines() {
    match line {
      Ok(text) => {
        if pattern.is_match(text.as_slice()) {
          println!("{}", text);
        }
      },
      Err(_) => {}
    }
  }
}

fn search(pattern: &Box<Regex>, path: Path){
  let file = Path::new(path);

  if file.is_file() {
    handle_file(file, pattern);
  } else {
    match fs::walk_dir(&file) {
      Ok(mut path_iter) => {
        for i in path_iter {
          if i.is_file() {
            handle_file(i, pattern);
          }
        }
      },
      Err(_) => {}
    }
  }
}

fn handle_file(path: Path, pattern: &Box<Regex>) {
  let file = File::open(&path);
  let mut reader = BufferedReader::new(file);
  let mut matches: Vec<String> = Vec::new();

  for line in reader.lines() {
    match line {
      Ok(text) => {
        if pattern.is_match(text.as_slice()) {
          matches.push(text);
        }
      },
      Err(_) => {}
    }
  }

  for line in matches.iter() {
    println!("{}:{}", path.display(), line);
  }
}
