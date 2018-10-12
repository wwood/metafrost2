pub mod get_occurences;

extern crate bio;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate nix;

use std::io;
use std::str;
use self::get_occurences::Metafrost;
use std::cmp::min;
use std::collections;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;


pub fn read_metafrost(metafrost_file_path: &str) -> Metafrost {
    let mut metafrost = get_occurences::Metafrost::new();
    let mut metaf = File::open(metafrost_file_path);
    let mut file = BufReader::new(metaf.unwrap());
//    let mut record;
    for (i, line) in file.lines().enumerate() {
        if i != 0 {
            let line = line.unwrap();
            let record: Vec<&str> = line.split('\t').collect();
//            println!("metafrost line: {} \t {} \t {}", record[0], record[1], record[2]);
            metafrost.establish_line(record[0].to_string(), record[1].to_string(), record[2].to_string());
        }else{
            info!("Removing Headers");
        }

    }

    return metafrost;
}