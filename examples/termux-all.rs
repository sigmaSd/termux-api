use termux_api_rs::*;

fn main() {
    dbg!(battery_status().unwrap());
    dbg!(brightness(&brightness::Value::Integer(255)).unwrap());
    dbg!(camera_info().unwrap());
    dbg!(camera_photo("./termux_photo_api.jpg", None).unwrap());
    dbg!(clipboard_get().unwrap());
    dbg!(clipboard_set("hello").unwrap());
    dbg!(clipboard_get().unwrap());
}
