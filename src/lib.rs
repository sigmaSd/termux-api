// NOTE: maybe its better to move wasi to its own crate.
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
mod wasi;

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct BatteryStatus {
    pub health: Health,
    pub percentage: usize,
    // pub plugged: String,
    // pub status: String,
    pub temperature: f64,
    pub current: isize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Health {
    COLD,
    DEAD,
    GOOD,
    OVERHEAT,
    #[serde(rename = "OVER_VOLTAGE")]
    OverVoltage,
    UNKNOWN,
    #[serde(rename = "UNSPECIFIED_FAILURE")]
    UnspecifiedFailure,
    // TODO: handle Integer.toString(health)
}

pub fn battery_status() -> Result<BatteryStatus> {
    Ok(serde_json::from_slice(
        &std::process::Command::new("termux-battery-status")
            .output()?
            .stdout,
    )?)
}
