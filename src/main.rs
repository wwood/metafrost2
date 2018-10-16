extern crate metafrost2;
extern crate clap;
use clap::*;

extern crate log;

use std::str;

fn main() {
    let mut app = build_cli();
    let matches = app.clone().get_matches();

    match matches.subcommand_name() {
        Some("COF") => {
            let m = matches.subcommand_matches("COF").unwrap();
            let metafrost_file: &str = m.value_of("metafrost-file").unwrap();
            metafrost2::read_metafrost(&metafrost_file);
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
