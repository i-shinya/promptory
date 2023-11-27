use once_cell::sync::OnceCell;

use crate::{log_ipc, usecase};

struct Controller {
    chat: Box<dyn usecase::chat::Chat>,
}

/// controllerの初期化
pub fn init(usecase: Box<dyn usecase::chat::Chat>) {
    let _ = CONTROLLER.set(Controller { chat: usecase });
}

/// usecaseを保持するためのstruct
/// tauriコマンドはtraitやstruct内に定義できないようなのでこのようにしています
static CONTROLLER: OnceCell<Controller> = OnceCell::new();

fn get_controller() -> &'static Controller {
    CONTROLLER.get().expect("Controller is not initialized")
}

/// チャットを実行する
#[tauri::command]
pub async fn post_chat(request: usecase::chat::ChatRequest) -> Result<String, String> {
    let res = log_ipc!(get_controller().chat, post_chat, request);
    match res {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}
