pub mod get_occurences;

#[macro_use]
extern crate log;

use std::str::{self, FromStr};
use self::get_occurences::Combinations;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn read_metafrost(metafrost_file_path: &str) -> Combinations {
    let mut metafrost = get_occurences::Metafrost::new();
    let metaf = File::open(metafrost_file_path);
    let file = BufReader::new(metaf.unwrap());
    let mut combinations = get_occurences::Combinations{
        combos: HashMap::new(),
    };
    let mut pre_read = -1;
    let mut cur_read;
    let mut cnt = 0;
    for (i, line) in file.lines().enumerate() {
        if i != 0  {
            let line = line.unwrap();
            let record: Vec<&str> = line.split('\t').collect();
            cur_read = i32::from_str(record[0]).unwrap();
            if pre_read == -1{
                pre_read = i32::from_str(record[0]).unwrap();
//            println!("metafrost line: {} \t {} \t {}", record[0], record[1], record[2]);
                metafrost.establish_line(record[0].to_string(), record[1].to_string(), record[2].to_string());
            }else if cur_read==pre_read{
                pre_read = i32::from_str(record[0]).unwrap();
//            println!("metafrost line: {} \t {} \t {}", record[0], record[1], record[2]);
                metafrost.establish_line(record[0].to_string(), record[1].to_string(), record[2].to_string());
            }else{
                cnt += 1;
                println!("New read: {}", cnt);
                combinations = metafrost.clone().establish_cof(combinations);
                let mut metafrost = get_occurences::Metafrost::new();
                pre_read = i32::from_str(record[0]).unwrap();
//            println!("metafrost line: {} \t {} \t {}", record[0], record[1], record[2]);
                metafrost.establish_line(record[0].to_string(), record[1].to_string(), record[2].to_string());
            }
        }else{
            info!("Removing Headers");
        }

    }

    return combinations;
}