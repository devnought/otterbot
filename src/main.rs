extern crate hipchat;
extern crate otterbot;
#[macro_use]
extern crate rouille;
extern crate serde_json;

fn main() {
    let config = otterbot::Config::load("config.json").expect("Could not load config");
    let addr = format!("0.0.0.0:{}", config.port());

    println!("Starting server on {}", &addr);

    rouille::start_server(addr, move |request| {
        router!(request,
            (GET) (/otterbot) => {
                let descriptor = otterbot::build_descriptor(&config);
                rouille::Response::json(&descriptor)
            },

            (POST) (/otterbot) => {
                let stream = request.data().expect("No data!");
                let cmd = serde_json::from_reader(stream).expect("Could not deserialize body");

                match otterbot::post(cmd, &config) {
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
