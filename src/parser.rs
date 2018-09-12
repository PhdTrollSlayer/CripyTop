use serde_json::*;
use std::fs::File;
use std::process::Command;

#[allow(dead_code)]
pub struct List {
    entries: Vec<Value>
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct Crypto {
    name: String,
    symbol: String,
    usd_price: f64,
    ref_price: f64,
    qtn_stored: f64,
    usd_total:f64,
    timestamp: String,
}


impl List {
    pub fn new_config() -> List {
        let config: Value = from_reader(File::open("config.json").unwrap()).expect("Err in the api key parsing");

        List {
            entries: config["data"].as_array().unwrap().to_vec()
        }

    }

    pub fn new_request() -> (List, String) {
        let config: Value = from_reader(File::open("config.json").unwrap()).expect("Err in the api key parsing");

        let mut url_request: String = "curl https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?CMC_PRO_API_KEY=".to_string();
        url_request.push_str(&config["api_key"].to_string());


        let loader = Command::new("sh")
            .arg("-c")
            .arg(url_request)
            .output()
            .expect("Falha ao se conectar a API!");

        let output = String::from_utf8_lossy(&loader.stdout);
        let api_return: Value = from_str(&output).expect("Err in the api return parsing");

        (List {
            entries: api_return["data"].as_array().unwrap().to_vec()
        },
            api_return["status"]["timestamp"].to_string()
        )
    }

}

impl Crypto {
    pub fn get_needed(c: &List, r: &List, t: &String) -> Vec<Crypto> {
        let config: Value = from_reader(File::open("config.json").unwrap()).expect("Err in the api key parsing");
        let ref_slug = &config["ref_slug"];
        let mut ref_price = 0f64;

        for c in &r.entries {
            if &c["slug"] == ref_slug {
                println!("Using: {} as reference.", c["name"].to_string());
                ref_price = c["quote"]["USD"]["price"].as_f64().unwrap();
            }
        }

        let mut vec_final = Vec::<Crypto>::new();

        for entry in &c.entries {
            for item in &r.entries {
                if entry["website_slug"] == item["slug"] {
                    let mut s = item["symbol"].to_string();
                    s.retain(|c| c != '\"');
                    vec_final.push(Crypto{
                        name: item["name"].to_string(),
                        symbol: s,
                        ref_price: (item["quote"]["USD"]["price"].as_f64().unwrap() / ref_price),
                        qtn_stored: entry["wallet"].as_f64().unwrap(),
                        usd_price: item["quote"]["USD"]["price"].as_f64().unwrap(),
                        usd_total: (item["quote"]["USD"]["price"].as_f64().unwrap() * entry["wallet"].as_f64().unwrap()),
                        timestamp: t.to_string(),
                    })
                }
            }
        }

        vec_final
    }
}
