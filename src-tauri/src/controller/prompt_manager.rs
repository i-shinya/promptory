use once_cell::sync::OnceCell;

use crate::usecase::prompt_manager::PromptManager;
use crate::{log_ipc, usecase};

pub struct Controller<T>
where
    T: PromptManager + ?Sized + 'static,
{
    prompt_manager: T,
}

impl<T> Controller<T>
where
    T: PromptManager + 'static,
{
    /// controllerの初期化
    pub fn init(usecase: T) {
        let _ = CONTROLLER.set(Box::new(Controller {
            prompt_manager: usecase,
        }));
    }
}

/// usecaseを保持するためのstruct
/// tauriコマンドはtraitやstruct内に定義できないようなのでこのようにしています
/// staticはコンパイル時に型を決定するので、traitを使うときはBoxで囲む必要があります
static CONTROLLER: OnceCell<Box<Controller<dyn PromptManager>>> = OnceCell::new();

fn get_controller() -> &'static Box<Controller<dyn PromptManager>> {
    CONTROLLER.get().expect("Controller is not initialized")
}

/// プロンプトマネージャーを取得する
#[tauri::command]
pub async fn get_prompt_managers(
    request: usecase::prompt_manager::GetPromptManagerRequest,
) -> Result<String, String> {
    let res = log_ipc!(
        get_controller().prompt_manager,
        get_prompt_managers,
        request
    );
    match res {
        Ok(res) => serde_json::to_string(&res).map_err(|err| err.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

/// プロンプトマネージャーを保存する
#[tauri::command]
pub async fn create_prompt_manager(
    request: usecase::prompt_manager::CreatePromptManagerRequest,
) -> Result<String, String> {
    let res = log_ipc!(
        get_controller().prompt_manager,
        create_prompt_manager,
        request
    );
    match res {
        Ok(res) => serde_json::to_string(&res).map_err(|err| err.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

/// プロンプトマネージャーを削除する
#[tauri::command]
pub async fn logical_delete_prompt_manager(
    request: usecase::prompt_manager::DeletePromptManagerRequest,
) -> Result<(), String> {
    let res = log_ipc!(
        get_controller().prompt_manager,
        logical_delete_prompt_managers,
        request
    );
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
