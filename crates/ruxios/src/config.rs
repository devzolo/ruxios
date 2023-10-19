use std::collections::HashMap;

pub struct RuxiosConfig {
  pub base_url: String,
  pub timeout: u32,
  pub headers: Vec<String>,
}

impl Default for RuxiosConfig {
  fn default() -> Self {
    Self {
      base_url: String::from(""),
      timeout: 10000,
      headers: Vec::new(),
    }
  }
}

impl RuxiosConfig {
  pub fn new() -> Self {
    Self::default()
  }
}

pub struct RuxiosRequestConfig {
  pub url: String,
  pub method: Option<String>,
  pub headers: Option<HashMap<String, String>>,
  pub _nop: (),
}

impl Default for RuxiosRequestConfig {
  fn default() -> Self {
    RuxiosRequestConfig {
      url: String::new(),
      method: Some(String::from("GET")),
      headers: None,
      _nop: (),
    }
  }
}
