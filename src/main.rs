extern crate clap;
use clap::{App, Arg};
use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::io;
use std::env;

#[derive(Debug)]
pub enum InputType {
    File(u64),
    Directory(u64),
    Error(io::Error),
}

fn get_size(path: &Path) -> InputType {
    let md_result = fs::symlink_metadata(&path);
    if let Err(e) = md_result {
        return InputType::Error(e);
    }
    let md = md_result.unwrap();
    if md.is_file() {
        InputType::File(md.len())
    } else if md.is_dir() {
        let cwd = env::current_dir().unwrap();
        let name = PathBuf::from(path);
        let change_dir_res = env::set_current_dir(&name);
        if let Err(e) = change_dir_res {
            return InputType::Error(e);
        }
        let ls_res = fs::read_dir(".");
        if let Err(e) = ls_res {
            return InputType::Error(e);
        }
        let ls = ls_res.unwrap();
        let sizes = ls.map( |item| {
            let i = item.expect("couldn't unwrap a file/directory in $pwd");
            let res = get_size(i.path().as_path());
            //println!("name: {:?}", i.file_name());
            //println!("res: {:?}", res);
            match res {
                InputType::File(s) => s,
                InputType::Directory(s) => s,
                InputType::Error(_) => 0
            }
        });
        let total_size = sizes.sum();
        assert!(env::set_current_dir(cwd).is_ok());
        InputType::Directory(total_size)
    } else {
        InputType::Error(io::Error::new(io::ErrorKind::Other, "metadata is neither a file nor directory, ignoring..."))
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
        InputType::File(s) => println!("the input is a file of {} bytes", s),
        InputType::Directory(s) => println!("the input is a directory of {} bytes", s),
        InputType::Error(e) => println!("oh no! {}", e)
    }
}
