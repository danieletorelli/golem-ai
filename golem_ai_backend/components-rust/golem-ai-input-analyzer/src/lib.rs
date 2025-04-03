#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::golem_ai::input_analyzer_exports::golem_ai_input_analyzer_api::*;
// Import for using common lib (also see Cargo.toml for adding the dependency):
// use common_lib::example_common_function;
use common_lib::{ask_openai, get_openai_api_key, OpenAIRequest};
use std::cell::RefCell;
use serde::{Deserialize, Serialize};

struct Input {
    input: String,
    entries: Vec<RawEntry>,
}

struct RawEntry {
    category: String,
    data: String,
}

thread_local! {
    static CONTEXT: RefCell<Input> = RefCell::new(Input {
        input: "".to_string(),
        entries: vec![],
    })
}

struct InputAnalyzer;


fn get_categories_from_json_markdown(json: String) -> Result<Vec<CategoryWithData>, String> {
    let s = json.strip_prefix("```json").and_then(|s| s.strip_suffix("```")).unwrap_or(&json).trim();
    let entries: Vec<CategoryWithData> = serde_json::from_str(s).map_err(|e| e.to_string())?;
    Ok(entries)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoryWithData {
    pub category: String,
    pub data: CategoryData,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CategoryData {
    pub description: String,
    pub link: Option<String>
}


pub fn context() -> String {
    "You are a helpful assistant. Your role is to read and extract all the entries present in a Notion document written in markdown. The entries represent potential features or bugfixes or an application. They are categorized in specific sections, that represent the importance of each entry. Some of them are already done, others are in todo or in progress. Collect all the entries. Represent each entry as a JSON object containing, 2 keys: 'category', representing the importance of the entry and 'data', representing all the raw description of the feature or bug, including the nested elements of the entry with any link or code snippet. All those JSON object can be placed in a JSON list. It's very important that you return me only the JSON structure, because I will have to parse your response in JSON.".to_string()
}

impl Guest for InputAnalyzer {
    fn analyze(input: String) -> Result<String, String> {
        println!("INPUT: {}", input.clone());

        let request = OpenAIRequest::new_system_and_user(
            "gpt-3.5-turbo".to_string(),
            context(),
            input.clone(),
            false,
            0.7,
        );

        match ask_openai(request, get_openai_api_key()).and_then(|r| r.get_message_or_err()) {
            Ok(response) => {
                println!("RESPONSE: {:?}", get_categories_from_json_markdown(response.clone()));
                println!("RESPONSE: {}", response.clone());
                // CONTEXT.with(|ctx| {
                //     ctx.borrow_mut().history.push(Response {
                //         message,
                //         response: response.clone(),
                //     });
                // });
                Ok(response)
            }
            Err(e) => Err(e),
        }
    }

    // fn history() -> Vec<HistoryEntry> {
    //     CONTEXT.with(|ctx| {
    //         ctx.borrow()
    //             .history
    //             .iter()
    //             .map(|r| HistoryEntry {
    //                 message: r.message.clone(),
    //                 response: r.response.clone(),
    //             })
    //             .collect()
    //     })
    // }
}

bindings::export!(InputAnalyzer with_types_in bindings);
