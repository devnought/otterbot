// TODO: Maybe go back to single hook?

extern crate hipchat;
extern crate otterbot;
#[macro_use]
extern crate warp;

use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use warp::Filter;

fn main() {
    let config = otterbot::Config::load("config.json").expect("Could not load config");

    let data = Arc::new(RwLock::new(otterbot::DataStore::new()));
    let data_ref = warp::any().map(move || data.clone());

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
        .and(data_ref.clone())
        .map(|req, datastore| warp::reply::json(&otterbot::fact(req, datastore)));

    let fact_add = path!("otterbot" / "fact" / "add")
        .and(warp::path::index())
        .and(warp::body::json())
        .and(data_ref.clone())
        .map(|req, datastore| warp::reply::json(&otterbot::fact_add(req, datastore)));

    let routes = warp::get(descriptor)
        .or(warp::post(fact))
        .or(warp::post(fact_add));

    println!("Starting server on {}", &addr);
    warp::serve(routes).run(addr);
}
