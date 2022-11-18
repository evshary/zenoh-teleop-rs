use zenoh::prelude::r#async::*;
use serde_derive::Serialize;
use cdr::{CdrLe, Infinite};

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

async fn pub_twist(session: &Session, cmd_key: &String, linear: f64, angular: f64) {
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
    session.put(cmd_key, encoded).res().await.unwrap();
}

#[async_std::main]
async fn main() {
    env_logger::init();
    println!("Opening session...");
    let session = zenoh::open(Config::default()).res().await.unwrap();
    let key = String::from("rt/turtle1/cmd_vel");
    pub_twist(&session, &key, 1.0, 1.0).await;
}
