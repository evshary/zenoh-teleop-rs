use zenoh::prelude::sync::*;
use zenoh::config::Config;
use serde_derive::Serialize;
use cdr::{CdrLe, Infinite};

use rocket::State;
#[macro_use] extern crate rocket;

#[derive(Serialize, PartialEq)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, PartialEq)]
struct Twist {
    linear: Vector3,
    angular: Vector3,
}

fn pub_twist(session: &Session, cmd_key: &String, linear: f64, angular: f64) -> bool {
    let twist = Twist {
        linear: Vector3 {
            x: linear,
            y: 0.0,
            z: 0.0,
        },
        angular: Vector3 {
            x: 0.0,
            y: 0.0,
            z: angular,
        },
    };

    let encoded = cdr::serialize::<_, _, CdrLe>(&twist, Infinite).unwrap();
    session.put(cmd_key, encoded).res().unwrap();
    true
}

fn help() -> String {
    format!("forward:  /direction/forward\n\
             backward: /direction/backward\n\
             left:     /direction/left\n\
             right:    /direction/right"
           )
}

#[get("/")]
fn homepage() -> String {
    format!("This is simple teleop operation homepage\n\n{}", help())
}

#[get("/<direction>")]
fn direction(direction: &str, zc: &State<ZenohConnect>) -> String {
    let result = match direction {
        "forward" => pub_twist(&zc.session, &zc.key, 2.0, 0.0),
        "backward" => pub_twist(&zc.session, &zc.key, -2.0, 0.0),
        "left" => pub_twist(&zc.session, &zc.key, 0.0, 2.0),
        "right" => pub_twist(&zc.session, &zc.key, 0.0, -2.0),
        _ => false,
    };
    if result {
        format!("Direction is {}!", direction)
    } else {
        format!("Wrong direction\n\n{}", help())
    }
}

struct ZenohConnect {
    session: Session,
    key: String,
}

#[launch]
fn rocket() -> _ {
    let session = zenoh::open(Config::default()).res().unwrap();
    let key = String::from("rt/turtle1/cmd_vel");
    rocket::build()
        .manage(ZenohConnect{session, key})
        .mount("/", routes![homepage])
        .mount("/direction", routes![direction])
}
