use csv::*;
use std::fs::File;
use std::process::Command;

pub fn generate_graph(r: &String) {
    let reader = Reader::from_reader(File::open("history.csv").expect("Could not open history.csv"));
    let mut iter = reader.into_records();

    let mut records = Vec::<StringRecord>::new();

    let mut data = String::from("Rscript --vanilla graph.r ");

    loop {
        if let Some(result) = iter.next() {
            let o = result.unwrap();
            records.push(o);
        } else {
            break;
        }
    }

    for x in records {
        if x.get(1).unwrap() == r {
            data.push_str(&x.get(2).unwrap().to_string());
            data.push_str(",");
        }
    }
    data.pop();

    let mut fln = String::from("graph_");
    fln.push_str(&r);
    fln.push_str(".jpg");

    data.push_str(" ");
    data.push_str(&r);
    data.push_str(" ");
    data.push_str(&fln);

    let _gen = Command::new("sh")
        .arg("-c")
        .arg(data)
        .output()
        .expect("Error when running R to build the graph");
}
