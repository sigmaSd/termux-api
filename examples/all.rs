use termux_api::{battery, brightness, camera_info};

fn main() {
    dbg!(battery::status().unwrap());
    dbg!(brightness::brightness(&brightness::Value::Integer(120)).unwrap());
    dbg!(brightness::brightness(&brightness::Value::Auto).unwrap());
    dbg!(camera_info::camera_info().unwrap());
}
