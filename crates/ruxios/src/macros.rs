#[macro_export]
macro_rules! fetch {
    // Case: No headers and no specific method
    ($url:expr) => {
        fetch!($url, { method: "GET" })
    };

    // Case: With method, but no specified headers
    ($url:expr, { method: $method:expr }) => {
        fetch!($url, { method: $method, headers: {} })
    };

    // Case: With method and headers specified as key-value pairs
    ($url:expr, { method: $method:expr, headers: { $($header_key:expr => $header_val:expr),* } }) => {
        fetch!($url, { method: $method, headers: { $($header_key => $header_val),* }, body: Value::Null })
    };

    // Case: With method, headers specified as key-value pairs, and body
    ($url:expr, { method: $method:expr, headers: { $($header_key:expr => $header_val:expr),* }, body: $body:expr }) => {
        async {
            use std::collections::HashMap;

            let api = Ruxios::from(RuxiosConfig {
                base_url: String::from(""),
                ..Default::default()
            });

            let headers_map: HashMap<String, String> = {
                #[allow(unused_mut)]
                let mut h = HashMap::new();
                $( h.insert(String::from($header_key), String::from($header_val)); )*
                h
            };

            api
                .request::<Value, Value, Value>(
                    RuxiosRequestConfig {
                        url: String::from($url),
                        method: Some(String::from($method)),
                        headers: Some(headers_map),
                        ..Default::default()
                    },
                    Some(&$body),
                )
                .await
        }
    };

    // Case: With method and headers specified as a HashMap
    ($url:expr, { method: $method:expr, body: $body:expr }) => {
        fetch!($url, { method: $method, headers: {}, body: $body })
    };

    // Case: With method, headers specified as a HashMap, and body
    ($url:expr, { method: $method:expr, headers: $headers_map:expr, body: $body:expr }) => {
        async {
            let api = Ruxios::from(RuxiosConfig {
                base_url: String::from(""),
                ..Default::default()
            });

            let converted_headers: std::collections::HashMap<String, String> =
            $headers_map.into_iter().map(|(k, v)| (k.into(), v.into())).collect();

            api
                .request::<Value, Value, Value>(
                    RuxiosRequestConfig {
                        url: String::from($url),
                        method: Some(String::from($method)),
                        headers: Some(converted_headers),
                        ..Default::default()
                    },
                    Some(&$body),
                )
                .await
        }
    };
}

#[macro_export]
macro_rules! json_strigify {
  ($json:expr) => {
    serde_json::to_string(&$json)
  };
}
