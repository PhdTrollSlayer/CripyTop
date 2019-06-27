#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde_json;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate argparse;

#[macro_use] extern crate rocket;
use rocket::response::content;

mod parser;
mod writer;
mod database_parser;

use parser::*;
use writer::*;
use database_parser::*;


#[get("/")]
fn index() -> content::Html<String> {
    /*
    let config = List::new_config();
    let api_return = List::new_request();

    let mine = Crypto::get_needed(&config, &api_return.0, &api_return.1);
    */

    let mut data: Vec<String>;

    data = gen_table();

    dbg!(&data);

    content::Html(
        format!(include_str!("../site/index.html"), data[0], data[1], data[2], data[3]))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
