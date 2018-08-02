extern crate hipchat;
extern crate otterbot;
extern crate warp;

use hipchat::request::HipchatRequest;
use std::net::SocketAddr;
use warp::Filter;

fn main() {
    let config = otterbot::Config::load("config.json").expect("Could not load config");
    let addr = format!("0.0.0.0:{}", config.port())
        .parse::<SocketAddr>()
        .expect("Could not parse address");

    let descriptor = warp::path("otterbot")
        .and(warp::header::<String>("host"))
        .map(|host: String| {
            let descriptor = otterbot::build_descriptor(&host);
            warp::reply::json(&descriptor)
        });

    let ob = warp::path("otterbot")
        .and(warp::body::json())
        .map(move |req: HipchatRequest| warp::reply::json(&otterbot::post(req, &config).unwrap()));

    let routes = warp::get(descriptor).or(warp::post(ob));

    println!("Starting server on {}", &addr);
    warp::serve(routes).run(addr);
}
