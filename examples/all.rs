use termux_api::battery_status;

fn main() {
    dbg!(battery_status().unwrap());
}
