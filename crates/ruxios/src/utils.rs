use std::collections::HashMap;

pub fn convert_map<K, V>(input: HashMap<K, V>) -> HashMap<String, String>
where
  K: Into<String>,
  V: Into<String>,
{
  input
    .into_iter()
    .map(|(k, v)| (k.into(), v.into()))
    .collect()
}
