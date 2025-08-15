use crate::dtos::response::base::BaseResponseDTO;

pub trait IService<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    async fn run(&self) -> Result<BaseResponseDTO, Error>;
}