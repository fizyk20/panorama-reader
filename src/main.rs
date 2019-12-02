mod data;

use clap::{App, Arg};
use data::AllData;
use libflate::gzip::Decoder;
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate serde_derive;

fn main() {
    let matches = App::new("Panorama Reader")
        .version("0.1")
        .arg(
            Arg::with_name("input")
                .help("Path to the input file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches
        .value_of("input")
        .expect("please provide an input file");

    let mut file = File::open(filename).expect("couldn't open the input file");
    let mut zipped_data = vec![];
    let _ = file
        .read_to_end(&mut zipped_data)
        .expect("couldn't read the file");

    let mut decoder = Decoder::new(&zipped_data[..]).expect("couldn't create the decoder");
    let mut data = vec![];
    let _ = decoder
        .read_to_end(&mut data)
        .expect("couldn't inflate the data");

    let data: AllData = bincode::deserialize(&data[..]).expect("couldn't deserialize the data");

    println!(
        "{}, {}",
        data.params.output.width, data.params.output.height
    );
}
