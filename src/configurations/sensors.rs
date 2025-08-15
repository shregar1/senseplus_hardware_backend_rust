use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

use crate::dtos::configurations::sensors::SensorsConfigDTO;

const CONFIG_PATH: &str = "src/configs/sensors/config.json";
static CONFIG: OnceCell<SensorsConfigDTO> = OnceCell::new();

pub fn get_config() -> &'static SensorsConfigDTO {
    OnceCell.get_or_init(|| {
        let file: File = File::open(CONFIG_PATH).expect(&format!("Failed to open {} config file", CONFIG_PATH));
        let reader: BufReader<File> = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .expect(&format!("Failed to parse {} config JSON", CONFIG_PATH));
        config
    })
}