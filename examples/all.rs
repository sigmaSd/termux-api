use termux_api::battery;

fn main() {
    dbg!(battery::status().unwrap());
}
