// implements the same things using walkdir

extern crate clap;
extern crate walkdir;
use clap::{App, Arg};
use std::path::Path;
use walkdir::{WalkDir, DirEntry};
use std::fs;
use std::io;
use walkdir::WalkDirIterator;

#[derive(Debug)]
pub enum InputType {
    File(u64),
    Directory(u64),
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn get_size(path: &Path) -> Result<InputType, io::Error> {
    let md = fs::symlink_metadata(path).unwrap();
    if md.is_file() {
       Ok(InputType::File(md.len()))
    }
    else if md.is_dir() {
        let mut sum: u64 = 0;
        let walker = WalkDir::new(path).max_depth(1).min_depth(1).into_iter();
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
                      let entry = entry.unwrap();
                      let path = entry.path();
                      //println!("path: {:?}", path);
                      let size = match get_size(path) {
                          Ok(InputType::File(s)) => s,
                          Ok(InputType::Directory(s)) => s,
                          Err(_) => 0
                      };
                      sum = sum + size;
        }
        Ok(InputType::Directory(sum))
    }
    else {
        Err(io::Error::new(io::ErrorKind::Other, "the specified path is neither a file nor directory..."))
    }
}

fn main() {
    let matches = App::new("size-rs")
        .version("1.0")
        .author("arshia")
        .about("check the size of a file or directory")
        .arg(Arg::with_name("input")
             .short("i")
             .long("input")
             .value_name("INPUT")
             .help("Specify the file or directory")
             .takes_value(true)
             .required(true))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let p = Path::new(input);
    let res = get_size(&p);
    println!("Input: {}", input);
    match res {
        Ok(InputType::File(s)) => println!("the input is a file of {} bytes", s),
        Ok(InputType::Directory(s)) => println!("the input is a directory of {} bytes", s),
        Err(e) => println!("an error occured! {}", e)
    }
}
