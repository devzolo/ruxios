use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

use crate::{
  config::{RuxiosConfig, RuxiosRequestConfig},
  error::{RuxiosError, RuxiosErrorValue},
  request::string_to_method,
  traits::TryConvert,
};

/// Primary structure of the Ruxios client.
///
/// Provides a set of methods to make HTTP requests.
pub struct Ruxios {
  client: Client,
  /// Configuration options for the Ruxios client.
  pub config: RuxiosConfig,
}

impl From<RuxiosConfig> for Ruxios {
  fn from(config: RuxiosConfig) -> Self {
    Self {
      config,
      ..Default::default()
    }
  }
}
/// Represents the response structure returned by the Ruxios client.
pub struct RuxiosResponse<T> {
  /// HTTP status code of the response.
  pub status: u16,
  /// Actual data content of the response.
  pub data: T,
}

impl<T> RuxiosResponse<T> {
  /// Checks if the response status is a success (2xx range).
  pub fn ok(&self) -> bool {
    self.status >= 200 && self.status < 300
  }
}

impl RuxiosResponse<Value> {
  /// Converts the response data from JSON to the given type.
  pub fn json<T>(&self) -> Result<T, serde_json::Error>
  where
    T: serde::de::DeserializeOwned,
  {
    serde_json::from_value(self.data.clone())
  }
}

impl<T> TryConvert<T> for RuxiosResponse<Value>
where
  T: serde::de::DeserializeOwned,
{
  fn try_convert(self) -> Result<T, serde_json::Error> {
    serde_json::from_value(self.data)
  }
}

impl Ruxios {
  /// Creates a new Ruxios client with default configuration.
  pub fn new() -> Self {
    Self::default()
  }

  /// Generalized method to perform an HTTP request.
  pub async fn request<TReq, TRes, TErr>(
    &self,
    cfg: RuxiosRequestConfig,
    data: Option<&TReq>,
  ) -> Result<RuxiosResponse<TRes>, RuxiosError<TErr>>
  where
    TReq: Serialize,
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    let full_url = format!("{}{}", self.config.base_url, cfg.url);

    let req_builder = self
      .client
      .request(
        string_to_method(cfg.method.unwrap_or("GET".to_owned())),
        full_url,
      )
      .header("User-Agent", "Ruxios");

    let req_builder = if let Some(headers) = cfg.headers {
      let mut req_builder = req_builder;
      for (key, value) in headers {
        req_builder = req_builder.header(key, value);
      }
      req_builder
    } else {
      req_builder
    };

    let req_builder = if let Some(data) = data {
      req_builder.body(serde_json::to_string(data)?)
    } else {
      req_builder
    };

    let res = req_builder.send().await?;

    let status = res.status();

    let body = res.text().await?;

    if status.is_success() {
      let response_data: TRes = serde_json::from_str(&body)?;
      Ok(RuxiosResponse {
        status: status.as_u16(),
        data: response_data,
      })
    } else {
      let error_response: TErr = serde_json::from_str(&body)?;
      Err(RuxiosError::MethodError(RuxiosErrorValue {
        status: status.as_u16(),
        value: error_response,
      }))
    }
  }

  /// Performs a GET request to the specified URL.
  pub async fn get<TRes, TErr>(&self, url: &str) -> Result<RuxiosResponse<TRes>, RuxiosError<TErr>>
  where
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    self
      .request::<Value, TRes, TErr>(
        RuxiosRequestConfig {
          url: String::from(url),
          method: Some(String::from("GET")),
          ..Default::default()
        },
        None,
      )
      .await
  }

  /// Performs a POST request to the specified URL.
  pub async fn post<TReq, TRes, TErr>(
    &self,
    url: &str,
    data: &TReq,
  ) -> Result<RuxiosResponse<TRes>, RuxiosError<TErr>>
  where
    TReq: Serialize,
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    self
      .request::<TReq, TRes, TErr>(
        RuxiosRequestConfig {
          url: String::from(url),
          method: Some(String::from("POST")),
          ..Default::default()
        },
        Some(data),
      )
      .await
  }

  /// Performs a PUT request to the specified URL.
  pub async fn put<TReq, TRes, TErr>(
    &self,
    url: &str,
    data: &TReq,
  ) -> Result<RuxiosResponse<TRes>, RuxiosError<TErr>>
  where
    TReq: Serialize,
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    self
      .request::<TReq, TRes, TErr>(
        RuxiosRequestConfig {
          url: String::from(url),
          method: Some(String::from("PUT")),
          ..Default::default()
        },
        Some(data),
      )
      .await
  }

  /// Performs a DELETE request to the specified URL.
  pub async fn delete<TRes, TErr>(
    &self,
    url: &str,
  ) -> Result<RuxiosResponse<TRes>, RuxiosError<TErr>>
  where
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    self
      .request::<Value, TRes, TErr>(
        RuxiosRequestConfig {
          url: String::from(url),
          method: Some(String::from("DELETE")),
          ..Default::default()
        },
        None,
      )
      .await
  }

  /// Performs a PATCH request to the specified URL.
  pub async fn patch<TReq, TRes, TErr>(
    &self,
    url: &str,
    data: &TReq,
  ) -> Result<RuxiosResponse<TRes>, RuxiosError<TErr>>
  where
    TReq: Serialize,
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    self
      .request::<TReq, TRes, TErr>(
        RuxiosRequestConfig {
          url: String::from(url),
          method: Some(String::from("PATCH")),
          ..Default::default()
        },
        Some(data),
      )
      .await
  }

  /// Performs a HEAD request to the specified URL.
  pub async fn head<TRes, TErr>(&self, url: &str) -> Result<RuxiosResponse<TRes>, RuxiosError<TErr>>
  where
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    self
      .request::<Value, TRes, TErr>(
        RuxiosRequestConfig {
          url: String::from(url),
          method: Some(String::from("HEAD")),
          ..Default::default()
        },
        None,
      )
      .await
  }

  /// Performs an OPTIONS request to the specified URL.
  pub async fn options<TRes, TErr>(
    &self,
    url: &str,
  ) -> Result<RuxiosResponse<TRes>, RuxiosError<TErr>>
  where
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    self
      .request::<Value, TRes, TErr>(
        RuxiosRequestConfig {
          url: String::from(url),
          method: Some(String::from("OPTIONS")),
          ..Default::default()
        },
        None,
      )
      .await
  }

  /// Performs a TRACE request to the specified URL.
  pub async fn trace<TRes, TErr>(
    &self,
    url: &str,
  ) -> Result<RuxiosResponse<TRes>, RuxiosError<TErr>>
  where
    TRes: serde::de::DeserializeOwned,
    TErr: serde::de::DeserializeOwned,
  {
    self
      .request::<Value, TRes, TErr>(
        RuxiosRequestConfig {
          url: String::from(url),
          method: Some(String::from("TRACE")),
          ..Default::default()
        },
        None,
      )
      .await
  }
}

impl Default for Ruxios {
  /// Implements a default configuration for a Ruxios client.
  fn default() -> Self {
    Self {
      client: Client::new(),
      config: RuxiosConfig::new(),
    }
  }
}
