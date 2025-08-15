use crate::dtos::response::base::BaseResponseDTO;

pub trait IFactory<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    async fn get(&self, key: String) -> Result<T, Error>;
}