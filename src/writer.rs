use csv::*;
use std::fs::OpenOptions;
use std::process::Command;

use parser::*;

pub fn record(r: &Crypto) {
    let _gen = Command::new("sh")
        .arg("-c")
        .arg("touch history.csv")
        .output()
        .expect("Error when running R to build the graph");

    let mut writer = WriterBuilder::new()
    .has_headers(false)
    .from_writer(OpenOptions::new().append(true).open("history.csv").expect("Could not open to write history.csv"));

    writer.write_record(&["","","","","","",""]).unwrap();
    writer.serialize(r).unwrap();

}
