#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::golem_ai::input_analyzer_exports::golem_ai_input_analyzer_api::*;
use crate::bindings::golem_ai::entry_categorizer_client::golem_ai_entry_categorizer_client::Entry as CategorizedEntry;
use crate::bindings::golem_ai::entry_categorizer_client::golem_ai_entry_categorizer_client::GolemAiEntryCategorizerApi;
use crate::bindings::golem_ai::entry_categorizer_client::golem_ai_entry_categorizer_client::RawEntry as CategorizerRawEntry;
use crate::bindings::golem_ai::entry_categorizer_exports::golem_ai_entry_categorizer_api::EntryType as CategorizerEntryType;
use common_lib::{ask_openai, get_openai_api_key, OpenAIRequest};
use std::cell::RefCell;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
struct RawEntry {
    category: String,
    data: String,
}

thread_local! {
    static INPUT: RefCell<String> = RefCell::new("".to_string())
}
thread_local! {
    static CONTEXT: RefCell<Vec<Entry>> = RefCell::new(vec![])
}

struct InputAnalyzer;

fn get_categories_from_json_markdown(json: String) -> Result<Vec<RawEntry>, String> {
    let s = json
        .strip_prefix("```json")
        .and_then(|s| s.strip_suffix("```"))
        .unwrap_or(&json)
        .trim();
    let entries: Vec<RawEntry> = serde_json::from_str(s).map_err(|e| e.to_string())?;
    Ok(entries)
}

fn categorize_entries_par(entries: Vec<RawEntry>) -> Vec<Result<CategorizedEntry, String>> {
    println!("CATEGORIZE ENTRIES: {}", entries.len());

    let mut futures = vec![];
    let mut subs = vec![];
    for entry in entries {
        let api = GolemAiEntryCategorizerApi::new();
        let request: CategorizerRawEntry = entry.into();
        let response = api.categorize(&request);
        let sub = response.subscribe();
        futures.push(response);
        subs.push(sub);
    }

    let n = futures.len();

    println!("CATEGORIZE ENTRIES INVOKED: {}", n);
    // https://learn.golem.cloud/common-language-guide/rpc#writing-non-blocking-remote-calls

    let mut values: Vec<Result<CategorizedEntry, String>> = vec![Err("Not ready".to_string()); n];
    let mut mapping: Vec<usize> = (0..n).collect();
    let mut remaining = subs.iter().collect::<Vec<_>>();

    // Repeatedly poll the futures until all of them are ready
    while !remaining.is_empty() {
        let poll_result = golem_rust::wasm_rpc::wasi::io::poll::poll(remaining.as_slice());

        // poll_result is a list of indexes of the futures that are ready
        for idx in &poll_result {
            let counter_idx = mapping[*idx as usize];
            let future = &futures[counter_idx];
            let value = future
                .get()
                .expect("future did not return a value because after marked as completed");
            values[counter_idx] = value;
        }

        // Removing the completed futures from the list
        remaining = remaining
            .into_iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                if poll_result.contains(&(idx as u32)) {
                    None
                } else {
                    Some(item)
                }
            })
            .collect();

        // Updating the index mapping
        mapping = mapping
            .into_iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                if poll_result.contains(&(idx as u32)) {
                    None
                } else {
                    Some(item)
                }
            })
            .collect();
    }

    values
}

fn verify_entries_par(
    entries: Vec<(RawEntry, CategorizedEntry)>,
) -> Vec<Result<CategorizedEntry, String>> {
    println!("VERIFY ENTRIES: {}", entries.len());

    let mut futures = vec![];
    let mut subs = vec![];
    for (raw_entry, entry) in entries {
        let api = GolemAiEntryCategorizerApi::new();
        let response = api.verify(&raw_entry.into(), &entry);
        let sub = response.subscribe();
        futures.push(response);
        subs.push(sub);
    }

    let n = futures.len();

    println!("VERIFY ENTRIES INVOKED: {}", n);
    // https://learn.golem.cloud/common-language-guide/rpc#writing-non-blocking-remote-calls

    let mut values: Vec<Result<CategorizedEntry, String>> = vec![Err("Not ready".to_string()); n];
    let mut mapping: Vec<usize> = (0..n).collect();
    let mut remaining = subs.iter().collect::<Vec<_>>();

    // Repeatedly poll the futures until all of them are ready
    while !remaining.is_empty() {
        let poll_result = golem_rust::wasm_rpc::wasi::io::poll::poll(remaining.as_slice());

        // poll_result is a list of indexes of the futures that are ready
        for idx in &poll_result {
            let counter_idx = mapping[*idx as usize];
            let future = &futures[counter_idx];
            let value = future
                .get()
                .expect("future did not return a value because after marked as completed");
            values[counter_idx] = value;
        }

        // Removing the completed futures from the list
        remaining = remaining
            .into_iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                if poll_result.contains(&(idx as u32)) {
                    None
                } else {
                    Some(item)
                }
            })
            .collect();

        // Updating the index mapping
        mapping = mapping
            .into_iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                if poll_result.contains(&(idx as u32)) {
                    None
                } else {
                    Some(item)
                }
            })
            .collect();
    }

    values
}

impl From<RawEntry> for CategorizerRawEntry {
    fn from(value: RawEntry) -> Self {
        CategorizerRawEntry {
            category: value.category,
            data: value.data,
        }
    }
}

impl From<CategorizedEntry> for Entry {
    fn from(value: CategorizedEntry) -> Self {
        Entry {
            category: value.category,
            title: value.title,
            description: value.description,
            links: value.links,
            entry_type: match value.entry_type {
                CategorizerEntryType::Feature => EntryType::Feature,
                CategorizerEntryType::Bug => EntryType::Bug,
            },
        }
    }
}

pub fn context() -> String {
    "You are a helpful assistant. Your role is to read and extract all the entries present in a Notion document written in markdown. The entries represent potential features or bugfixes or an application. They are categorized in specific sections, that represent the importance of each entry. Some of them are already done, others are in todo or in progress. Collect all the entries. Represent each entry as a JSON object containing, 2 keys: 'category' (string), representing the importance of the entry and 'data' (string), representing all the raw description of the feature or bug, including the nested elements of the entry with any link or code snippet. All those JSON object can be placed in a JSON list. The JSON can be compact, with no extra characters added. It's very important that you return me only a valid JSON structure, don't return any markdown prefix and don't anny any extra space or newline characters, because I will have to parse your response in JSON directly so it should be very clean and compact.".to_string()
}

impl Guest for InputAnalyzer {
    fn analyze(input: String) -> Result<(), String> {
        INPUT.with(|i| {
            *i.borrow_mut() = input.clone();
        });

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
                let values = get_categories_from_json_markdown(response.clone());
                match values {
                    Ok(values) => {
                        println!("ANALYZE RESPONSE: {}", response.clone());
                        let categorized = categorize_entries_par(values);
                        let successfully_categorized: Vec<Entry> = categorized
                            .clone()
                            .into_iter()
                            .filter_map(|x| x.ok())
                            .map(|x| x.into())
                            .collect();
                        CONTEXT.with(|ctx| {
                            *ctx.borrow_mut() = successfully_categorized;
                        });
                        println!("CATEGORIZE RESPONSE: {:?}", categorized);
                    }
                    Err(e) => {
                        println!("ANALYZE RESPONSE PARSING ERROR: {}", e);
                        return Err(e);
                    }
                }

                Ok(())
            }
            Err(e) => {
                println!("ANALYZE ERROR: {}", e);
                Err(e)
            }
        }
    }

    fn get_results() -> Vec<Entry> {
        CONTEXT.with(|ctx| ctx.borrow().clone())
    }
}

bindings::export!(InputAnalyzer with_types_in bindings);
