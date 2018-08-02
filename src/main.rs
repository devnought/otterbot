extern crate hipchat;
extern crate otterbot;
#[macro_use]
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
        .and(warp::path::index())
        .and(warp::header::<String>("host"))
        .map(|host: String| warp::reply::json(&otterbot::build_descriptor(&host)));

    let fact = path!("otterbot" / "fact")
        .and(warp::path::index())
        .and(warp::body::json())
        .map(|req: HipchatRequest| {
            println!("{:#?}", req);
            warp::reply::json(&otterbot::fact(req))
        });

    let fact_add = path!("otterbot" / "fact" / "add")
        .and(warp::path::index())
        .and(warp::body::json())
        .map(|req: HipchatRequest| {
            println!("{:#?}", req);
            warp::reply::json(&otterbot::fact_add(req))
        });

    let routes = warp::get(descriptor)
        .or(warp::post(fact))
        .or(warp::post(fact_add));

    println!("Starting server on {}", &addr);
    warp::serve(routes).run(addr);
}
