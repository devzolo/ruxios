pub trait TryConvert<T: serde::de::DeserializeOwned> {
  fn try_convert(self) -> Result<T, serde_json::Error>;
}
