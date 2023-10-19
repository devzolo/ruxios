#[macro_export]
macro_rules! fetch {
    // Sem headers e sem método especificado
    ($url:expr) => {
        fetch!($url, { method: "GET" })
    };

    // Com método, mas sem headers especificados
    ($url:expr, { method: $method:expr }) => {
        fetch!($url, { method: $method, headers: {} })
    };

    // Com método e headers especificados como pares chave-valor
    ($url:expr, { method: $method:expr, headers: { $($header_key:expr => $header_val:expr),* } }) => {
        fetch!($url, { method: $method, headers: { $($header_key => $header_val),* }, body: Value::Null })
    };

    // Com método e headers especificados como pares chave-valor e body
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

    // Com método e headers especificados como HashMap
    ($url:expr, { method: $method:expr, body: $body:expr }) => {
        fetch!($url, { method: $method, headers: {}, body: $body })
    };

    // Com método e headers especificados como HashMap e body
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
