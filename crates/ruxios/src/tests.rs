use serde_json::json;

use crate::{json_strigify, prelude::*};

#[derive(Debug, Serialize, Deserialize)]
struct RequestPayload {}

#[derive(Debug, Serialize, Deserialize)]
struct ResponsePayload {
  pub login: String,
}

#[tokio::test]
async fn test_ruxios_request_get() {
  let api = Ruxios::from(RuxiosConfig {
    base_url: String::from("https://api.github.com"),
    ..Default::default()
  });

  let res = api
    .request::<Value, Value, Value>(
      RuxiosRequestConfig {
        url: String::from("/users/devzolo"),
        method: Some(String::from("GET")),
        ..Default::default()
      },
      None,
    )
    .await;

  assert!(res.is_ok());

  let res = res.expect("Failed to fetch data");

  assert!(res.ok());

  let data: Value = res.json().expect("Failed to parse data");

  assert_eq!(data["login"], "devzolo");
}

#[tokio::test]
async fn test_ruxios_fetch_macro() {
  #[derive(Debug, Serialize, Deserialize)]
  struct User {
    login: String,
    id: u32,
  }

  let res = fetch!("https://api.github.com/users/devzolo")
    .await
    .expect("Failed to fetch data");

  assert!(res.ok());

  let data: User = res.json().expect("Failed to parse data");

  assert_eq!(data.login, "devzolo");
}

#[tokio::test]
async fn test_ruxios_fetch_macro_short() {
  #[derive(Debug, Serialize, Deserialize)]
  struct User {
    login: String,
    id: u32,
  }

  let res: User = fetch!("https://api.github.com/users/devzolo", {
    method: "GET",
    headers: {
      "Content-Type" => "application/json"
    }
  })
  .await
  .and_then(|res| Ok(res.json()?))
  .expect("Failed to fetch data");

  assert_eq!(res.login, "devzolo");
}

#[tokio::test]
async fn test_ruxios_fetch_call_rest_api() {
  let res = fetch!("https://jsonplaceholder.typicode.com/todos/1", {
    method: "GET",
    headers: {
      "Content-Type" => "application/json"
    }
  })
  .await
  .expect("Failed to fetch data");

  assert!(res.ok());

  let data: Value = res.json().expect("Failed to parse data");

  assert_eq!(data["userId"], 1);
}

#[tokio::test]
async fn test_ruxios_fetch_call_rest_api_with_body() {
  #[derive(Debug, Serialize, Deserialize)]
  struct ResponsePayload {
    title: String,
  }

  let res = fetch!("https://jsonplaceholder.typicode.com/posts", {
    method: "POST",
    headers: {
      "Content-Type" => "application/json"
    },
    body: json!({
      "title": "foo",
      "body": "bar",
      "userId": 1
    })
  })
  .await
  .expect("Failed to fetch data");

  assert!(res.ok());

  let data: ResponsePayload = res.json().expect("Failed to parse data");

  let str = json_strigify!(&data).expect("Failed to stringify json");

  assert_eq!(str, "{\"title\":\"foo\"}");

  assert_eq!(data.title, "foo");
}

#[tokio::test]
async fn test_ruxios_get_request() {
  let api = Ruxios::from(RuxiosConfig {
    base_url: String::from("https://jsonplaceholder.typicode.com"),
    ..Default::default()
  });

  let res = api
    .get::<Value, Value>("/users/1")
    .await
    .expect("Failed to fetch data");

  assert!(res.ok());

  let data: Value = res.json().expect("Failed to parse data");

  assert_eq!(data["id"], 1);
}
