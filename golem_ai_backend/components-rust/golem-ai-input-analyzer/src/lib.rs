#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::golem_ai::input_analyzer_exports::golem_ai_input_analyzer_api::*;
// Import for using common lib (also see Cargo.toml for adding the dependency):
// use common_lib::example_common_function;
use common_lib::{ask_openai, get_openai_api_key, OpenAIRequest};
use std::cell::RefCell;

struct Context {
    input: String,
    entries: Vec<String>,
}

thread_local! {
    static CONTEXT: RefCell<Context> = RefCell::new();
}

struct InputAnalyzer;

pub fn context() -> String {
    "You are a helpful assistant. Your role is to read and extract all the entries present in a Notion document written in markdown. The entries represent potential features or bugfixes or an application. They are categorized in specific sections, that represent the importance of each entry. Some of them are already done, others are in todo or in progress. Collect all the entries. Represent each entry as a JSON object containing, 2 keys: 'category', representing the importance of the entry and 'data', representing all the raw description of the feature or bug, including the nested elements of the entry with any link or code snippet. All those JSON object can be placed in a JSON list. It's very important that you return me only the JSON structure, because I will have to parse your response in JSON.".to_string()
}

impl Guest for InputAnalyzer {
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

bindings::export!(InputAnalyzer with_types_in bindings);
