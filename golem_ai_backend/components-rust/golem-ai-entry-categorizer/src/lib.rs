#[allow(static_mut_refs)]
mod bindings;

use crate::bindings::exports::golem_ai::entry_categorizer_exports::golem_ai_entry_categorizer_api::*;
use common_lib::{ask_openai, get_openai_api_key, OpenAIRequest};

struct Categorizer;

pub fn context() -> String {
    "You are a helpful assistant. Your role is to categorize an entry inferring more precise information from its description. Return me a JSON object containing keys: 'category' (string), which is the same as the 'category' key in input, 'entry_type' (enum: 'feature', 'bug'), which identifies if this is a feature or a bug; 'title' (string), which contains a short bug meaningful description of what is described; 'description' (string), which contains all data that you got, but well formatted in markdown and spell-checked; 'links' (list of strings), which should contain all links present in the data that you received. Pay attention to not change the meaning of what is describe. The JSON can be compact, with no extra characters added. It's very important that you return me only a valid JSON structure, don't return any markdown prefix and don't anny any extra space or newline characters, because I will have to parse your response in JSON directly so it should be very clean and compact.".to_string()
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
enum ParsedEntryType {
    #[serde(rename = "feature")]
    Feature,
    #[serde(rename = "bug")]
    Bug,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Eq, PartialEq)]
struct ParsedEntry {
    category: String,
    entry_type: ParsedEntryType,
    title: String,
    description: String,
    links: Vec<String>,
}

impl From<ParsedEntry> for Entry {
    fn from(parsed_entry: ParsedEntry) -> Self {
        Entry {
            category: parsed_entry.category,
            title: parsed_entry.title,
            description: parsed_entry.description,
            links: parsed_entry.links,
            entry_type: match parsed_entry.entry_type {
                ParsedEntryType::Feature => EntryType::Feature,
                ParsedEntryType::Bug => EntryType::Bug,
            },
        }
    }
}

fn parse_markdown(json: String) -> Result<ParsedEntry, String> {
    let s = json
        .strip_prefix("```json")
        .and_then(|s| s.strip_suffix("```"))
        .unwrap_or(&json)
        .trim();
    serde_json::from_str(s).map_err(|e| e.to_string())
}

impl Guest for Categorizer {
    fn categorize(entry: RawEntry) -> Result<Entry, String> {
        println!("ENTRY: {:?}", entry.clone());

        let request = OpenAIRequest::new_system_and_user(
            "gpt-3.5-turbo".to_string(),
            context(),
            format!("CATEGORY: {}\nDATA: {}", entry.category, entry.data),
            false,
            0.7,
        );

        match ask_openai(request, get_openai_api_key()).and_then(|r| r.get_message_or_err()) {
            Ok(response) => {
                println!("RESPONSE: {}", response.clone());
                let parsed = parse_markdown(response).map(|p| p.into());
                println!("PARSED: {:?}", parsed.clone());
                parsed
            }
            Err(e) => Err(e),
        }
    }
}

bindings::export!(Categorizer with_types_in bindings);

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn test_parser() {
        let json = r#"```json{"category":"Critical missing features","entry_type": "feature","title": "Missing feature: Construct worker URI for ephemeral workers using component name resolution APIs","description": "@vigoo@ziverge.com - It is extremely hard to construct worker URI for RPC for ephemeral workers using the new component name resolution APIs. We have everything needed to do for durable workers (component-id + name), but for ephemeral ones, we can resolve the component id by name, but then we cannot construct the urn:worker:<component-id> string. The reason is the WIT component-id type is a u64 pair (representing an uuid) and there is no way to easily convert that to a string (without knowing that it is a uuid and implementing a uuid to string conversion). In Rust the golem-rust crate at least has a conversion from the WIT Uuid to the 'real' uuid::Uuid. For now, we should add a variant of the worker_id host function that generates an ephemeral one by only taking a component-id. Post 1.2 we should not use URIs in the location parameter of RPC resources","links": ["https://github.com/golemcloud/golem/pull/1432"]}```"#;
        let parsed = parse_markdown(json.to_string()).unwrap();
        assert_eq!(parsed.category, "Critical missing features");
        assert_eq!(parsed.entry_type, ParsedEntryType::Feature);
        assert_eq!(parsed.title, "Missing feature: Construct worker URI for ephemeral workers using component name resolution APIs");
        assert_eq!(parsed.description, "@vigoo@ziverge.com - It is extremely hard to construct worker URI for RPC for ephemeral workers using the new component name resolution APIs. We have everything needed to do for durable workers (component-id + name), but for ephemeral ones, we can resolve the component id by name, but then we cannot construct the urn:worker:<component-id> string. The reason is the WIT component-id type is a u64 pair (representing an uuid) and there is no way to easily convert that to a string (without knowing that it is a uuid and implementing a uuid to string conversion). In Rust the golem-rust crate at least has a conversion from the WIT Uuid to the 'real' uuid::Uuid. For now, we should add a variant of the worker_id host function that generates an ephemeral one by only taking a component-id. Post 1.2 we should not use URIs in the location parameter of RPC resources");
        assert_eq!(
            parsed.links,
            vec!["https://github.com/golemcloud/golem/pull/1432"]
        );
    }
}
