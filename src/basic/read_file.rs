// use std::{env, error};
use std::error::Error;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::io::{Write, BufReader, BufRead};

pub fn run() {
    // let cwd = env::current_dir().unwrap();
    // let path = cwd.join("src/basic/files/lines.txt");
    let path = Path::new("src/basic/files/lines.txt");

    let display = path.display();

    let mut file = match File::open(&path) {
       // The `description` method of `io::Error` returns a string that
       // describes the error
       Err(why) => panic!("couldn't open {}: {}", display,
                                                  why.description()),
       Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
       Err(why) => panic!("couldn't read {}: {}", display,
                                                  why.description()),
       Ok(_) => print!("{} contains:\n{}", display, s),
    }



}


pub fn run2() -> Result<(), io::Error> {
    let path = Path::new("src/basic/files/lines2.txt");

    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
