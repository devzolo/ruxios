use serde_json::{json, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct RuxiosErrorValue<T> {
  pub status: u16,
  pub value: T,
}

#[derive(Error, Debug)]
pub enum RuxiosError<T> {
  #[error("Request failed: {0}")]
  ReqwestError(#[from] reqwest::Error),

  #[error("Deserialization failed: {0}")]
  SerdeError(#[from] serde_json::Error),

  #[error("Failed to get env: {0}")]
  EnvError(#[from] std::env::VarError),

  #[error("Method error: {0:?}")]
  MethodError(RuxiosErrorValue<T>),
}

// pub trait ErrorFromString<T: AsRef<str>> {
//   fn from_string(body: T) -> Self;
// }

// impl<T: AsRef<str>> ErrorFromString<T> for RuxiosError<String> {
//   fn from_string(body: T) -> Self {
//     Self::MethodError(RuxiosErrorValue {
//       status: 0,
//       value: body.as_ref().to_owned(),
//     })
//   }
// }

impl RuxiosError<Value> {
  pub fn from_string<T>(body: T) -> RuxiosError<Value>
  where
    T: AsRef<str>,
  {
    let body = json!({
      "message": body.as_ref(),
    });

    Self::MethodError(RuxiosErrorValue {
      status: 0,
      value: body,
    })
  }

  pub fn from_value<T>(body: T) -> RuxiosError<Value>
  where
    T: Into<Value>,
  {
    let body = body.into();

    Self::MethodError(RuxiosErrorValue {
      status: 0,
      value: body,
    })
  }
}
