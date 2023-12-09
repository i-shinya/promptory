use once_cell::sync::OnceCell;

use crate::usecase::chat::Chat;
use crate::{log_ipc, usecase};

pub struct Controller<T>
where
    T: Chat + ?Sized + 'static,
{
    chat: T,
}

impl<T> Controller<T>
where
    T: Chat + 'static,
{
    /// controllerの初期化
    pub fn init(usecase: T) {
        let _ = CONTROLLER.set(Box::new(Controller { chat: usecase }));
    }
}

/// usecaseを保持するためのstruct
/// tauriコマンドはtraitやstruct内に定義できないようなのでこのようにしています
/// staticはコンパイル時に型を決定するので、traitを使うときはBoxで囲む必要があります
static CONTROLLER: OnceCell<Box<Controller<dyn Chat>>> = OnceCell::new();

fn get_controller() -> &'static Box<Controller<dyn Chat>> {
    CONTROLLER.get().expect("Controller is not initialized")
}

/// チャットを実行する
#[tauri::command]
pub async fn post_chat(request: usecase::chat::ChatRequest) -> Result<String, String> {
    let res = log_ipc!(get_controller().chat, run_chat, request);
    match res {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}
