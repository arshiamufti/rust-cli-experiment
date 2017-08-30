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

fn get_size(path: &Path) -> Result<InputType, io::Error> {
    let md_result = fs::symlink_metadata(path);
    let md = md_result?;
    if md.is_file() {
        Ok(InputType::File(md.len()))
    } else if md.is_dir() {
        let cwd = env::current_dir().unwrap();
        let name = PathBuf::from(path);
        let change_dir_res = env::set_current_dir(&name);
        if let Err(e) = change_dir_res {
            return Ok(InputType::Error(e));
        }
        let ls = fs::read_dir(".")?;
        let sizes = ls.map( |item| {
            let i = item.expect("couldn't unwrap a file/directory in $pwd");
            let res = get_size(i.path().as_path());
            //println!("name: {:?}", i.file_name());
            //println!("res: {:?}", res);
            match res {
                Ok(InputType::File(s)) => s,
                Ok(InputType::Directory(s)) => s,
                Ok(InputType::Error(_)) => 0,
                Err(_) => 0
            }
        });
        let total_size = sizes.sum();
        assert!(env::set_current_dir(cwd).is_ok());
        Ok(InputType::Directory(total_size))
    } else {
        Ok(InputType::Error(
            io::Error::new(io::ErrorKind::Other, "metadata is neither a file nor directory, ignoring...")))
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
        Ok(InputType::Error(e)) => println!("oh no! {}", e),
        Err(e) => println!("an error occured! {}", e)
    }
}
