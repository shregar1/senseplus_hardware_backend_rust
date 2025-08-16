use alloc::string::{String, ToString};

#[derive(Debug, Clone)]
pub struct Config {
    pub device_urn: String,
    pub location_urn: String,
    pub wifi_ssid: String,
    pub wifi_password: String,
    pub server_base_url: String,
}

impl Config {

    pub fn new() -> Self {
        Self {
            device_urn: option_env!("DEVICE_URN").expect("DEVICE_URN must be set").to_string(),
            location_urn: option_env!("LOCATION_URN").expect("LOCATION_URN must be set").to_string(),
            wifi_ssid: option_env!("WIFI_SSID").expect("WIFI_SSID must be set").to_string(),
            wifi_password: option_env!("WIFI_PASSWORD").expect("WIFI_PASSWORD must be set").to_string(),
            server_base_url: option_env!("SEVER_BASE_URL").expect("SEVER_BASE_URL must be set").to_string()
        }
    }
}