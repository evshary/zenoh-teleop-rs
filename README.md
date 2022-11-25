# zenoh-teleop-rs

This is from https://github.com/atolab/zenoh-demo/tree/main/ROS2/zenoh-rust-teleop

# Usage

* build

```shell
cargo build 
```

* Run

```shell
# Run turtlesim
ros2 run turtlesim turtlesim_node
# Run zenoh-bridge-dds
./zenoh-bridge-dds
```

* You can run with keyboard / RESTful API

  - Keyboard

  ```shell
  # Run the teleop
  cargo run --bin zenoh-teleop
  # Control with keyboard
  ```

  -  RESTful API

  ```shell
  # Run with REST API
  cargo run --bin zenoh-restful-teleop
  # Control with curl
  curl http://127.0.0.1:8000/direction/forward
  curl http://127.0.0.1:8000/direction/backward
  curl http://127.0.0.1:8000/direction/right
  curl http://127.0.0.1:8000/direction/left
  ```
