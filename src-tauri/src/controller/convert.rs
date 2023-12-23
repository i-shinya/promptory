// tauriのResponseに変換するマクロ
#[macro_export]
macro_rules! convert_to_tauri_result {
    ($value:expr) => {{
        match $value {
            Ok(val) => serde_json::to_string(&val).map_err(|err| err.to_string()),
            Err(err) => Err(err.to_string()),
        }
    }};
}
