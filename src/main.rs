extern crate hipchat;
#[macro_use]
extern crate rouille;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod message;

mod config;
use config::Config;

mod functionality;
use functionality::{build_descriptor, post_mb};

fn main() {
    let config = Config::load("config.json").expect("Could not load config");
    let addr = format!("0.0.0.0:{}", config.port());

    println!("Starting server on {}", &addr);

    rouille::start_server(addr, move |request| {
        router!(request,
            (GET) (/mb) => {
                let descriptor = build_descriptor(&config);
                rouille::Response::json(&descriptor)
            },

            (POST) (/mb) => {
                let stream = request.data().expect("No data!");
                let cmd = serde_json::from_reader(stream).expect("Could not deserialize body");

                match post_mb(cmd, &config) {
                    Some(res) => rouille::Response::json(&res),
                    None => rouille::Response {
                        status_code: 200,
                        headers: vec![],
                        data: rouille::ResponseBody::empty(),
                        upgrade: None,
                    }
                }
            },

            _ => rouille::Response::empty_404()
        )
    });
}
