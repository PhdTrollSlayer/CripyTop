extern crate serde_json;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate argparse;

mod parser;
mod writer;
mod database_parser;

use parser::*;
use writer::*;
use database_parser::*;

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut slug = String::new();
    let mut get_api_records = false;

    {
       let mut ap = ArgumentParser::new();
       ap.set_description("Manipulate cryptocurrency market data locally.\n
       Developer: Rodrigo Mauricio(TrollSlayer) <rodrigomauricio@protonmail.com>\n
       Licenced under GNU's GPLv3.");
       ap.refer(&mut slug)
           .add_option(&["-G"], Store,
           "Generate Graph");
       ap.refer(&mut get_api_records)
            .add_option(&["-A"], StoreTrue,
            "Get ccryptocurrency data");
       ap.parse_args_or_exit();
    }

    if get_api_records {
        let config = List::new_config();
        let api_return = List::new_request();

        let mine = Crypto::get_needed(&config, &api_return.0, &api_return.1);

        for x in &mine {
            record(x);
        }
    }

    if !slug.is_empty() {
        generate_graph(&slug);
    }
    /*
    */


}
