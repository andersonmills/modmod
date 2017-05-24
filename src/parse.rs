use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

pub fn update_puppetfile_mods(source: &str, version: &str) {
    let puppetfile_path = Path::new("Puppetfile");
    let puppetfile = match File::open(&puppetfile_path) {
        Err(why) => panic!("could not open your Puppetfile: {}",  why.description()),
        Ok(file) => file
    };

    let b = BufReader::new(puppetfile);

    for line in b.lines() {
        let l = line.unwrap();
        println!("{:?}", l);
    }
}
