// NOTE: maybe its better to move wasi to its own crate.
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
mod wasi;

use serde::{Deserialize, Serialize};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod battery {
    //https://github.com/termux/termux-api/blob/master/app/src/main/java/com/termux/api/apis/BatteryStatusAPI.java
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Status {
        pub health: Health,
        pub percentage: usize,
        pub plugged: Plugged,
        pub status: StatusString,
        pub temperature: f64,
        pub current: isize,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum StatusString {
        CHARGING,
        DISCHARGING,
        FULL,
        #[serde(rename = "NOT_CHARGING")]
        NotCharging,
        UNKNOWN,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub enum Plugged {
        UNPLUGGED,
        #[serde(rename = "PLUGGED_AC")]
        PluggedAc,
        #[serde(rename = "PLUGGED_USB")]
        PluggedUsb,
        #[serde(rename = "PLUGGED_WIRELESS")]
        PluggedWireless,
        Integer(String),
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
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
        #[serde(untagged)]
        Integer(String),
    }

    pub fn status() -> Result<Status> {
        Ok(serde_json::from_slice(
            &std::process::Command::new("termux-battery-status")
                .output()?
                .stdout,
        )?)
    }
}
