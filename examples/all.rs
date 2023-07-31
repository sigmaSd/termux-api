use rs_lib::battery_status;

fn main() {
    dbg!(battery_status().unwrap());
}
