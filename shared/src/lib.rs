//! 教程阶段二的共享协议。作为成员5 GUI 的编译依赖，交由成员1统一维护。
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Command {
    Takeoff,
    Land,
    Goto { lat: f64, lon: f64 },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DroneMsg {
    State {
        drone_id: String,
        lat: f64,
        lon: f64,
        alt: f64,
        battery: f64,
        status: String,
    },
    Ack {
        command: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GuiMsg {
    SendCommand { drone_id: String, command: Command },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMsg {
    Snapshot { drones: Vec<DroneState> },
    Log { text: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DroneState {
    pub drone_id: String,
    pub lat: f64,
    pub lon: f64,
    pub alt: f64,
    pub battery: f64,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "role", rename_all = "snake_case")]
pub enum Hello {
    Drone { id: String },
    Gui,
}

pub const LAT_MIN: f64 = 29.85;
pub const LAT_MAX: f64 = 29.89;
pub const LON_MIN: f64 = 121.52;
pub const LON_MAX: f64 = 121.58;

pub fn latlon_to_uv(lat: f64, lon: f64) -> (f32, f32) {
    let u = ((lon - LON_MIN) / (LON_MAX - LON_MIN)).clamp(0.0, 1.0) as f32;
    let v = ((LAT_MAX - lat) / (LAT_MAX - LAT_MIN)).clamp(0.0, 1.0) as f32;
    (u, v)
}

pub fn uv_to_latlon(u: f32, v: f32) -> (f64, f64) {
    let lon = LON_MIN + u.clamp(0.0, 1.0) as f64 * (LON_MAX - LON_MIN);
    let lat = LAT_MAX - v.clamp(0.0, 1.0) as f64 * (LAT_MAX - LAT_MIN);
    (lat, lon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_roundtrip() {
        let c = Command::Goto {
            lat: 29.87,
            lon: 121.55,
        };
        let s = serde_json::to_string(&c).unwrap();
        let back: Command = serde_json::from_str(&s).unwrap();
        assert!(matches!(back, Command::Goto { lat, lon } if lat == 29.87 && lon == 121.55));
    }

    #[test]
    fn hello_roundtrip() {
        let s = serde_json::to_string(&Hello::Drone { id: "d1".into() }).unwrap();
        assert!(s.contains("\"role\":\"drone\""));
        assert!(
            matches!(serde_json::from_str::<Hello>(&s).unwrap(), Hello::Drone { id } if id == "d1")
        );
    }

    #[test]
    fn uv_latlon_inverse() {
        for (lat, lon) in [
            (29.87, 121.55),
            (LAT_MIN, LON_MIN),
            (LAT_MAX, LON_MAX),
            (29.86321, 121.56789),
        ] {
            let (u, v) = latlon_to_uv(lat, lon);
            let (back_lat, back_lon) = uv_to_latlon(u, v);
            // 归一化坐标使用 f32，需允许其舍入误差。
            assert!((back_lat - lat).abs() < 5e-9);
            assert!((back_lon - lon).abs() < 5e-9);
        }
        assert_eq!(latlon_to_uv(LAT_MAX, LON_MIN), (0.0, 0.0));
        assert_eq!(latlon_to_uv(LAT_MIN, LON_MAX), (1.0, 1.0));
    }
}
