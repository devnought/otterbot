extern crate hipchat;
extern crate otterbot;
extern crate warp;

use std::net::SocketAddr;
use warp::Filter;

fn main() {
    let data = otterbot::open_db("data.json");
    let data_ref = warp::any().map(move || data.clone());

    let addr = format!("0.0.0.0:{}", 8001)
        .parse::<SocketAddr>()
        .expect("Could not parse address");

    let descriptor = warp::get2()
        .and(warp::path("otterbot"))
        .and(warp::path::index())
        .and(warp::header::<String>("host"))
        .map(|host: String| warp::reply::json(&otterbot::build_descriptor(&host)));

    let dispatcher = warp::post2()
        .and(warp::path("otterbot"))
        .and(warp::path::index())
        .and(warp::body::json())
        .and(data_ref.clone())
        .map(|req, datastore| warp::reply::json(&otterbot::dispatcher(req, datastore)));

    let routes = descriptor.or(dispatcher);

    println!("Starting server on {}", &addr);
    warp::serve(routes).run(addr);
}
