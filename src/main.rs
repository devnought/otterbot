extern crate hipchat;
extern crate otterbot;
extern crate warp;

use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use warp::Filter;

fn main() {
    //let config = otterbot::Config::load("config.json").expect("Could not load config");

    let data = Arc::new(RwLock::new(otterbot::DataStore::new()));
    let data_ref = warp::any().map(move || data.clone());

    let addr = format!("0.0.0.0:{}", 8001)
        .parse::<SocketAddr>()
        .expect("Could not parse address");

    let descriptor = warp::path("otterbot")
        .and(warp::path::index())
        .and(warp::header::<String>("host"))
        .map(|host: String| warp::reply::json(&otterbot::build_descriptor(&host)));

    let dispatcher = warp::path("otterbot")
        .and(warp::path::index())
        .and(warp::body::json())
        .and(data_ref.clone())
        .map(|req, datastore| warp::reply::json(&otterbot::dispatcher(req, datastore)));

    let routes = warp::get(descriptor).or(warp::post(dispatcher));

    println!("Starting server on {}", &addr);
    warp::serve(routes).run(addr);
}
