use zenoh::prelude::r#async::*;
use serde_derive::Serialize;
use cdr::{CdrLe, Infinite};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

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
    println!("Press arrow keys to control turtle...");
    crossterm::terminal::enable_raw_mode().unwrap();
    loop {
        match crossterm::event::read() {
            Ok(Event::Key(KeyEvent { code: KeyCode::Up, modifiers: _, kind: _, state: _ })) => {
                pub_twist(&session, &key, 2.0, 0.0).await
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Down, modifiers: _, kind: _, state: _ })) => {
                pub_twist(&session, &key, -2.0, 0.0).await
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Left, modifiers: _, kind: _, state: _ })) => {
                pub_twist(&session, &key, 0.0, 2.0).await
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Right, modifiers: _, kind: _, state: _ })) => {
                pub_twist(&session, &key, 0.0, -2.0).await
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Char(' '), modifiers: _, kind: _, state: _ })) => {
                pub_twist(&session, &key, 0.0, 0.0).await
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Esc, modifiers: _, kind: _, state: _ })) |
            Ok(Event::Key(KeyEvent { code: KeyCode::Char('q'), modifiers: _, kind: _, state: _ })) => {
                break
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers, kind: _, state: _})) => {
                if modifiers.contains(KeyModifiers::CONTROL) { break }
            },
            Ok(_) => (),
            Err(_) => {}
        }
    }
    // Stop the robot
    pub_twist(&session, &key, 0.0, 0.0).await;
    crossterm::terminal::disable_raw_mode().unwrap();
}
