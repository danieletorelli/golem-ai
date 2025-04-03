use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OllamaResponse {
    pub model: String,
    pub response: String,
    pub created_at: String,
    pub done: bool,
    pub done_reason: String,
}

pub fn ask_ollama(prompt: String) -> Result<OllamaResponse, String> {
    let request = OllamaRequest {
        model: "gemma3".to_string(),
        prompt,
        stream: false,
    };

    let client = Client::builder().build().expect("Failed to create client");

    client
        .post(&"http://ollama:11434/api/generate".to_string())
        .json(&request)
        .send()
        .and_then(|x| {
            println!("Response: {:?}", x);
            x.json::<OllamaResponse>()
        })
        .map_err(|e| e.to_string())
}

/*
{
   "model":"gemma3",
   "created_at":"2025-04-02T09:40:14.256108802Z",
   "response":"Hello there! How can I help you today? 😊 \n\nDo you want to:\n\n*   Chat about something?\n*   Ask me a question?\n*   Play a game?\n*   Something else entirely?",
   "done":true,
   "done_reason":"stop",
   "context":[
      105,
      2364,
      107,
      9259,
      106,
      107,
      105,
      4368,
      107,
      9259,
      993,
      236888,
      2088,
      740,
      564,
      1601,
      611,
      3124,
      236881,
      103453,
      236743,
      108,
      6294,
      611,
      1461,
      531,
      236787,
      108,
      236829,
      138,
      25380,
      1003,
      2613,
      236881,
      107,
      236829,
      138,
      29020,
      786,
      496,
      2934,
      236881,
      107,
      236829,
      138,
      9274,
      496,
      2290,
      236881,
      107,
      236829,
      138,
      33128,
      1663,
      11716,
      236881
   ],
   "total_duration":4461958710,
   "load_duration":49905333,
   "prompt_eval_count":10,
   "prompt_eval_duration":144714000,
   "eval_count":48,
   "eval_duration":4266850544
}
*/
