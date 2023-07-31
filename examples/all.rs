use termux_api::{battery, brightness, camera_info, camera_photo};

fn main() {
    dbg!(battery::status().unwrap());
    dbg!(brightness::brightness(&brightness::Value::Integer(255)).unwrap());
    dbg!(camera_info::camera_info().unwrap());
    dbg!(camera_photo("./termux_photo_api.jpg", None).unwrap());
}
