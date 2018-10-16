#[macro_use]
extern crate log;

use std::str::{self, FromStr};
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub fn read_metafrost(metafrost_file_path: &str) {
    let metaf = File::open(metafrost_file_path);
    let file = BufReader::new(metaf.unwrap());
    let mut combinations = std::collections::HashMap::new();

    let mut pre_read = -1;
    let mut cur_read;
    let mut pre_position = -1;
    let mut cur_position;
    let mut colours = vec!();

    let process_kmer = |colours: &mut Vec<String>,
            combinations: &mut std::collections::HashMap<String, u32>| {
                let key = colours.join(",").to_string();
                debug!("key: {}", key);
                let count = combinations.entry(key).or_insert(0);
                *count += 1;
    };

    for (i, line) in file.lines().enumerate() {
        if i != 0  {
            let line = line.unwrap();
            debug!("line: {}", line);
            let record: Vec<String> = line.split('\t').map(|s| s.to_string()).collect();
            cur_read = i32::from_str(&record[0]).unwrap();
            if record[1] == "-" {
                // bug in metafrost? - the position should be printed out.
                cur_position = pre_position + 1;
            } else {
                cur_position = i32::from_str(&record[1]).unwrap();
            }
            let colour = record[2].to_string();

            if pre_read == cur_read && pre_position == cur_position {
                // Another kmer / position
                colours.push(colour);
            } else {
                // Process previous chunk
                if cur_read != 0 || cur_position != 0 {
                    process_kmer(&mut colours, &mut combinations);
                }
                // Setup current position
                pre_read = cur_read;
                pre_position = cur_position;
                colours = vec!(colour);
            }
        }else{
            debug!("Removing Headers");
        }
    }
    // process last chunk
    process_kmer(&mut colours, &mut combinations);

    // print out counts
    for (colours, count) in &combinations {
        println!("{}\t{}", colours, count);
    }
}
