pub fn string_to_method<T>(method: T) -> reqwest::Method
where
  T: AsRef<str>,
{
  match method.as_ref().to_uppercase().as_str() {
    "GET" => reqwest::Method::GET,
    "POST" => reqwest::Method::POST,
    "PUT" => reqwest::Method::PUT,
    "DELETE" => reqwest::Method::DELETE,
    "HEAD" => reqwest::Method::HEAD,
    "OPTIONS" => reqwest::Method::OPTIONS,
    "CONNECT" => reqwest::Method::CONNECT,
    "PATCH" => reqwest::Method::PATCH,
    "TRACE" => reqwest::Method::TRACE,
    _ => reqwest::Method::GET,
  }
}
