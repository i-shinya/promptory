use once_cell::sync::OnceCell;

use crate::usecase::comparing_prompt::ComparingPrompt;
use crate::{convert_to_tauri_result, log_ipc, usecase};

pub struct Controller<T>
where
    T: ComparingPrompt + ?Sized + 'static,
{
    comparing_prompt: T,
}

impl<T> Controller<T>
where
    T: ComparingPrompt + 'static,
{
    /// controllerの初期化
    pub fn init(usecase: T) {
        let _ = CONTROLLER.set(Box::new(Controller {
            comparing_prompt: usecase,
        }));
    }
}

/// usecaseを保持するためのstruct
/// tauriコマンドはtraitやstruct内に定義できないようなのでこのようにしています
/// staticはコンパイル時に型を決定するので、traitを使うときはBoxで囲む必要があります
static CONTROLLER: OnceCell<Box<Controller<dyn ComparingPrompt>>> = OnceCell::new();

fn get_controller() -> &'static Box<Controller<dyn ComparingPrompt>> {
    CONTROLLER.get().expect("Controller is not initialized")
}

/// プロンプト比較設定を追加する
#[tauri::command]
pub async fn add_comparing_prompt_setting(
    request: usecase::comparing_prompt::AddComparingPromptSettingRequest,
) -> Result<String, String> {
    let res = log_ipc!(
        get_controller().comparing_prompt,
        add_comparing_prompt_setting,
        request
    );
    convert_to_tauri_result!(res)
}

/// プロンプト比較設定を取得する
#[tauri::command]
pub async fn get_comparing_prompt_setting(
    request: usecase::comparing_prompt::GetComparingPromptSettingRequest,
) -> Result<String, String> {
    let res = log_ipc!(
        get_controller().comparing_prompt,
        get_comparing_prompt_setting,
        request
    );
    convert_to_tauri_result!(res)
}

/// プロンプト比較設定を全て取得する
#[tauri::command]
pub async fn get_all_comparing_prompt_settings(
    request: usecase::comparing_prompt::GetComparingPromptSettingsRequest,
) -> Result<String, String> {
    let res = log_ipc!(
        get_controller().comparing_prompt,
        get_all_comparing_prompt_settings,
        request
    );
    convert_to_tauri_result!(res)
}

/// プロンプト比較実行を保存する
#[tauri::command]
pub async fn save_comparing_prompt_run(
    request: usecase::comparing_prompt::SaveComparingPromptRunRequest,
) -> Result<String, String> {
    let res = log_ipc!(get_controller().comparing_prompt, save_run, request);
    convert_to_tauri_result!(res)
}

/// プロンプト比較を実行する
#[tauri::command]
pub async fn run_comparing_prompt(
    request: usecase::comparing_prompt::RunChatRequest,
) -> Result<String, String> {
    let res = log_ipc!(get_controller().comparing_prompt, run_chat, request);
    convert_to_tauri_result!(res)
}
