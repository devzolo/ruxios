#[derive(Debug, Default)]
pub enum HttpMethod {
  #[default]
  GET,
  POST,
  PUT,
  DELETE,
  PATCH,
  HEAD,
  OPTIONS,
  CONNECT,
  TRACE,
}
