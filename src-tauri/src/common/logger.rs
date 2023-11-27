use env_logger::Env;

/// IPC通信のログを出力するマクロ
#[macro_export]
macro_rules! log_ipc {
    ($obj:expr, $method:ident, $req:expr) => {{
        log::info!("Request: {:?}", $req);
        let result = $obj.$method($req).await;
        match &result {
            Ok(res) => log::info!("Response: {:?}", res),
            Err(err) => log::error!("Error: {}", err),
        }
        result
    }};
}

pub fn init_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}
