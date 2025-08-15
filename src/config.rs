use dotenv::dotenv;

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
        dotenv().ok();
        Self {
            device_urn: env::var("DEVICE_URN").expect("DEVICE_URN must be set."),
            location_urn: env::var("LOCATION_URN").expect("LOCATION_URN must be set."),
            wifi_ssid: env::var("WIFI_SSID").expect("WIFI_SSID must be set."),
            wifi_password: env::var("WIFI_PASSWORD").expect("WIFI_PASSWORD must be set."),
            server_base_url: env::var("SEVER_BASE_URL").expect("SEVER_BASE_URL must be set.")
        }
    }
}