mod data;
mod interface;
mod rendering;

use clap::{App, Arg};
pub use data::{AllData, Params, PixelColor};
use libflate::gzip::Decoder;
pub use rendering::create_surface;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use gio::prelude::*;
use gio::ApplicationFlags;
use gtk::Application;

use interface::build_ui;

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

    let app = Application::new(None, ApplicationFlags::FLAGS_NONE)
        .expect("Couldn't create a GTK application!");

    let data = Rc::new(data);
    app.connect_activate(move |app| build_ui(app, data.clone()));

    app.run(&[]);
}
