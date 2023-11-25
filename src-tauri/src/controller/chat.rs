use crate::usecase;

struct Controller {
    chat: Box<dyn usecase::chat::Chat>,
}

/// controllerの初期化
pub unsafe fn init(usecase: Box<dyn usecase::chat::Chat>) {
    CONTROLLER = Some(Controller { chat: usecase });
}

/// usecaseを保持するためのstruct
/// tauriコマンドはtraitやstruct内に定義できないようなのでこのようにしています
static mut CONTROLLER: Option<Controller> = None;

fn get_controller() -> &'static Controller {
    unsafe { CONTROLLER.as_ref().unwrap() }
}

/// チャットを実行する
#[tauri::command]
pub async fn post_chat(question: &str) -> Result<String, String> {
    let request = usecase::chat::ChatRequest {
        user_prompt: question.to_string(),
        system_prompt: "君は超超高性能なアシスタントだ！ユーザを全力でサポートするんだ！"
            .to_string(),
    };
    match get_controller().chat.post_chat(request).await {
        Ok(res) => Ok(res),
        Err(err) => Err(format!("Error: {}", err)),
    }
}
