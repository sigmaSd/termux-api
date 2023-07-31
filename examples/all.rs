use termux_api::{battery, brightness};

fn main() {
    dbg!(battery::status().unwrap());
    dbg!(brightness::brightness(&brightness::Value::Integer(120)).unwrap());
    dbg!(brightness::brightness(&brightness::Value::Auto).unwrap());
}
