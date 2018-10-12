extern crate metafrost2;
pub mod get_occurences;
extern crate bio;
extern crate clap;
use clap::*;
extern crate rust_htslib;
extern crate nix;
extern crate tempfile;
extern crate csv;

#[macro_use]
extern crate log;
use log::LogLevelFilter;
extern crate env_logger;
use env_logger::LogBuilder;

use std::io::{self, Write, Read};
use std::env;
use std::str;
use std::process::{self, Command};
use std::collections::HashSet;
use std::path::Path;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File};
use csv::{Writer, ReaderBuilder};

fn main() {
    let mut app = build_cli();
    let matches = app.clone().get_matches();

    match matches.subcommand_name() {
        Some("COF") => {
            let m = matches.subcommand_matches("COF").unwrap();
            let mut genomes_string = String::new();

            let metafrost_file: &str = m.value_of("metafrost-file").unwrap();
            let mut metafrost = metafrost2::read_metafrost(&metafrost_file);
            let mut cof = metafrost.establish_cof();
            for (key, value) in cof.combos.iter(){
                println!("{} \t {}", key, value);
            }
        }
        _ => {
            app.print_help().unwrap();
            println!();
        }
    }
}



fn build_cli() -> App<'static, 'static> {

    return App::new("kmer_indexer")
        .author("Rhys J. P. Newell <r.newell near uq.edu.au>")
        .about("Creation of kmer indexes from metagenomic path files using Kallisto or Bifrost")
        .help("
        usage: file parser <subcommand>

        modes:
        \tCOF \t Create Combination Occurrence File from Metafrost output

            ")
        .subcommand(
            SubCommand::with_name("COF")
                .about("Calculate unique kmers in a genome, \
                and compare kmer counts from multiple genomes")
                .arg(Arg::with_name("metafrost-file")
                    .short("m")
                    .long("metafrost-file")
                    .takes_value(true)
                    .required(true))
                .arg(Arg::with_name("verbose")
                    .short("v")
                    .long("verbose"))
                .arg(Arg::with_name("quiet")
                    .short("q")
                    .long("quiet")))
}