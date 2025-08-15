pub trait ISensor<T> {
    fn urn(&self) -> String;
    fn device_urn(&self) -> String;
    fn location_urn(&self) -> String;
    fn name(&self) -> String;
    pub async fn read(&self) -> Result<T, Error>;
}
