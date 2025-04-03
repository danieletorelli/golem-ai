#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::golem_ai::worker_exports::golem_ai_worker_api::*;
// Import for using common lib (also see Cargo.toml for adding the dependency):
// use common_lib::example_common_function;
use common_lib::ask_ollama;
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
    "You are a helpful assistant. Your role is to read and extract all the entries in a notion document".to_string()
}

impl Guest for AIWorker {
    fn ask(message: String) -> Result<String, String> {
        let mut prompt = String::new();
        prompt.push_str("Context: \n");
        CONTEXT.with(|ctx| {
            for response in &ctx.borrow().history {
                prompt.push_str(&format!(
                    "Message {}: Response: {}\n",
                    response.message, response.response
                ));
            }
        });
        prompt.push_str(&format!("{}: ", message));
        println!("Prompt: {}", prompt);

        let response = ask_ollama(message.clone());

        if let Ok(ollama_response) = response.clone() {
            CONTEXT.with(|ctx| {
                ctx.borrow_mut().history.push(Response {
                    message,
                    response: ollama_response.response,
                });
            });
        } else {
            println!("Error: {:?}", response);
        }
        response.map(|r| r.response)
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
