#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::golem_ai::worker_exports::golem_ai_worker_api::*;
use common_lib::*;

use std::cell::RefCell;

struct Context {
    history: Vec<Response>,
}
impl Default for Context {
    fn default() -> Self {
        Self {
            history: Vec::new(),
        }
    }
}

struct Response {
    message: String,
    response: String,
}

thread_local! {
    static CONTEXT: RefCell<Context> = RefCell::new(Context::default());
}

struct AIWorker;

pub fn context() -> String {
    "You are a helpful assistant. Your role is to read and extract all the entries present in a notion document. Some of them are already done".to_string()
}

impl Guest for AIWorker {
    fn ask(message: String) -> Result<String, String> {
        let request = OpenAIRequest::new_system_and_user(
            "gpt-3.5-turbo".to_string(),
            context(),
            message.clone(),
            false,
            0.7,
        );

        match ask_openai(request, get_openai_api_key()).and_then(|r| r.get_message_or_err()) {
            Ok(response) => {
                CONTEXT.with(|ctx| {
                    ctx.borrow_mut().history.push(Response {
                        message,
                        response: response.clone(),
                    });
                });
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }

    fn history() -> Vec<HistoryEntry> {
        CONTEXT.with(|ctx| {
            ctx.borrow()
                .history
                .iter()
                .map(|r| HistoryEntry {
                    message: r.message.clone(),
                    response: r.response.clone(),
                })
                .collect()
        })
    }
}

bindings::export!(AIWorker with_types_in bindings);
