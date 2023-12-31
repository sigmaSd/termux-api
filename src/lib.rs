#![allow(clippy::missing_errors_doc, clippy::wildcard_imports)]
// NOTE: maybe its better to move wasi to its own crate.
#[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
mod wasi;

use serde::{Deserialize, Serialize};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! spawn {
    ($api: expr) => {
        Ok(spawn!(@internal_spawn spawn!(@internal_new $api)))
    };
    ($api: expr, $($arg: expr),*) => {
        Ok(spawn!(@internal_spawn spawn!(@internal_new $api).args(&[$($arg),*])))
    };
    (@output $api: expr) => {
         Ok(String::from_utf8(
           spawn!(@internal_output spawn!(@internal_new $api))
         )?)
    };
    (@output @json $api: expr) => {
        Ok(serde_json::from_slice(
            &spawn!(@internal_output spawn!(@internal_new $api))
        )?)
    };
    // internal captures
    (@internal_new $api: expr) => {
        std::process::Command::new($api)
    };
    (@internal_spawn $cmd: expr) => {
        $cmd.spawn()?.wait().map(|_|{})?
    };
    (@internal_output $cmd: expr) => {
        $cmd.output()?.stdout
    }
}

pub mod battery_status {
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
}
pub fn battery_status() -> Result<battery_status::Status> {
    spawn!(@output @json "termux-battery-status")
}

pub mod brightness {
    //https://github.com/termux/termux-api/blob/master/app/src/main/java/com/termux/api/apis/BrightnessAPI.java
    pub enum Value {
        Auto,
        Integer(usize),
    }
    impl std::fmt::Display for Value {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Auto => f.write_str("auto"),
                Self::Integer(int) => f.write_str(&int.to_string()),
            }
        }
    }
}
pub fn brightness(value: &brightness::Value) -> Result<()> {
    spawn!("termux-brightness", value.to_string())
}

pub mod camera_info {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Info {
        id: String,
        facing: Facing,
        jpeg_output_sizes: Vec<JsonOutputSize>,
        focal_lengths: Vec<f32>,
        auto_exposure_modes: Vec<FlashModeValue>,
        physical_size: PhysicalSize,
        capabilities: Vec<Capability>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    enum Facing {
        #[serde(rename = "front")]
        Front,
        #[serde(rename = "back")]
        Back,
        #[serde(untagged)]
        Other(usize),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct JsonOutputSize {
        width: usize,
        height: usize,
    }

    #[derive(Serialize, Deserialize, Debug)]
    enum FlashModeValue {
        #[serde(rename = "CONTROL_AE_MODE_OFF")]
        ControlAeModeOff,
        #[serde(rename = "CONTROL_AE_MODE_ON")]
        ControlAeModeOn,
        #[serde(rename = "CONTROL_AE_MODE_ON_ALWAYS_FLASH")]
        ControlAeModeOnAlwaysFlash,
        #[serde(rename = "CONTROL_AE_MODE_ON_AUTO_FLASH")]
        ControlAeModeOnAutoFlash,
        #[serde(rename = "CONTROL_AE_MODE_ON_EXTERNAL_FLASH")]
        ControlAeModeOnExternalFlash,
        #[serde(untagged)]
        Integer(isize),
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct PhysicalSize {
        width: f32,
        height: f32,
    }
    #[derive(Serialize, Deserialize, Debug)]
    enum Capability {
        #[serde(rename = "backward_compatible")]
        BackwardCompatible,
        #[serde(rename = "burst_capture")]
        BurstCapture,
        #[serde(rename = "constrained_high_speed_video")]
        ConstrainedHighSpeedVideo,
        #[serde(rename = "depth_output")]
        DepthOutput,
        #[serde(rename = "logical_multi_camera")]
        LogicalMultiCamera,
        #[serde(rename = "manual_post_processing")]
        ManualPostProcessing,
        #[serde(rename = "manual_sensor")]
        ManualSensor,
        #[serde(rename = "monochrome")]
        Monochrome,
        #[serde(rename = "motion_tracking")]
        MotionTracking,
        #[serde(rename = "private_reprocessing")]
        PrivateReprocessing,
        #[serde(rename = "raw")]
        Raw,
        #[serde(rename = "read_sensor_settings")]
        ReadSensorSettings,
        #[serde(rename = "yuv_reprocessing")]
        YuvReprocessing,
        Integer(isize),
    }
}
pub fn camera_info() -> Result<Vec<camera_info::Info>> {
    spawn!(@output @json "termux-camera-info")
}

pub fn camera_photo(output_file: &str, camera_id: Option<usize>) -> Result<()> {
    spawn!(
        "termux-camera-photo",
        "-c",
        &camera_id.unwrap_or(0).to_string(),
        output_file
    )
}

pub fn clipboard_get() -> Result<String> {
    spawn!(@output "termux-clipboard-get")
}

pub fn clipboard_set(value: &str) -> Result<()> {
    spawn!("termux-clipboard-set", value)
}
