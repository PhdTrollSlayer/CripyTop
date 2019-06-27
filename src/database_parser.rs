use csv::*;
use std::fs::File;
use std::process::Command;

pub fn gen_table() -> Vec<String>{
    let reader = Reader::from_reader(File::open("history.csv").expect("Could not open history.csv"));
    let mut iter = reader.into_records();

    let mut records = Vec::<StringRecord>::new();

    loop {
        if let Some(result) = iter.next() {
            let o = result.unwrap();
            records.push(o);
        } else {
            break;
        }
    }

    let u = records.get((records.len() - 7)..).unwrap();

    let mut d: Vec<String> = Vec::new();

    for x in u {
        if !x.get(0 as usize).unwrap().is_empty() {
            d.push(format!("<td>{}</td>
                           <td>{}</td>
                           <td>{}</td>
                           <td>{}</td>", x.get(2).unwrap().trim(), x.get(3).unwrap().trim(),x.get(4).unwrap().trim(),x.get(5).unwrap().trim()))

        }
    }
    d
}
