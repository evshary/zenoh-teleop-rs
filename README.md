# zenoh-teleop-rs

This is from https://github.com/atolab/zenoh-demo/tree/main/ROS2/zenoh-rust-teleop

# Usage

* build

```shell
colcon
```

* Run

```shell
# Run turtlesim
ros2 run turtlesim turtlesim_node
# Run zenoh-bridge-dds
./zenoh-bridge-dds
# Run the teleop
cargo run
```
