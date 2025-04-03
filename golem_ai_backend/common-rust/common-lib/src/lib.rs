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
        .and_then(|x| x.json::<OllamaResponse>())
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

/**

curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer API_KEY" \
  -d '{
    "model": "gpt-4",
    "messages": [
      {"role": "system", "content": "You are a poetic assistant."},
      {"role": "user", "content": "Write a short poem about AI."}
    ],
    "temperature": 0.8,
    "max_tokens": 100,
    "stream": false
  }'


curl https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer API_KEY" \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [
      {"role": "system", "content": "You are a helpful assistant."},
      {"role": "user", "content": "Explain quantum computing in simple terms."}
    ],
    "temperature": 0.7
  }'

{
  "id": "chatcmpl-BIAN9cIYZ4HlPhVdjCNTErEWSm5k6",
  "object": "chat.completion",
  "created": 1743669475,
  "model": "gpt-3.5-turbo-0125",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Sure! Quantum computing is a type of computing that uses quantum particles like atoms or photons to store and process information. These particles can exist in multiple states at once thanks to a property called superposition, which allows quantum computers to perform many calculations simultaneously. This gives quantum computers the potential to solve complex problems much faster than classical computers. Additionally, quantum computers leverage another property called entanglement, where particles can be linked together in a way that their states are dependent on each other even when separated by large distances. This enables quantum computers to perform tasks that are impossible for classical computers.",
        "refusal": null,
        "annotations": []
      },
      "logprobs": null,
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 25,
    "completion_tokens": 117,
    "total_tokens": 142,
    "prompt_tokens_details": {
      "cached_tokens": 0,
      "audio_tokens": 0
    },
    "completion_tokens_details": {
      "reasoning_tokens": 0,
      "audio_tokens": 0,
      "accepted_prediction_tokens": 0,
      "rejected_prediction_tokens": 0
    }
  },
  "service_tier": "default",
  "system_fingerprint": null
}

*/

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<OpenAIChatCompletionRequestMessage>,
    pub stream: bool,
    pub temperature: f32,
}

impl OpenAIRequest {
    pub fn new(
        model: String,
        messages: Vec<OpenAIChatCompletionRequestMessage>,
        stream: bool,
        temperature: f32,
    ) -> OpenAIRequest {
        OpenAIRequest {
            model,
            messages,
            stream,
            temperature,
        }
    }

    pub fn new_system_and_user(
        model: String,
        system_content: String,
        user_content: String,
        stream: bool,
        temperature: f32,
    ) -> OpenAIRequest {
        OpenAIRequest {
            model,
            messages: vec![
                OpenAIChatCompletionRequestMessage::new_system(system_content),
                OpenAIChatCompletionRequestMessage::new_user(user_content),
            ],
            stream,
            temperature,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionRequestMessage {
    pub role: String,
    pub content: String,
}

impl OpenAIChatCompletionRequestMessage {
    pub fn new(role: String, content: String) -> OpenAIChatCompletionRequestMessage {
        OpenAIChatCompletionRequestMessage { role, content }
    }

    pub fn new_user(content: String) -> OpenAIChatCompletionRequestMessage {
        OpenAIChatCompletionRequestMessage {
            role: "user".to_string(),
            content,
        }
    }

    pub fn new_system(content: String) -> OpenAIChatCompletionRequestMessage {
        OpenAIChatCompletionRequestMessage {
            role: "system".to_string(),
            content,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenAIResponse {
    pub id: String,
    pub model: String,
    pub object: String,
    pub choices: Vec<OpenAIChatCompletionResponseChoice>,
}

impl OpenAIResponse {
    pub fn get_message(&self) -> Option<String> {
        if self.choices.len() > 0 {
            Some(self.choices[0].message.content.clone())
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionResponseChoice {
    pub index: usize,
    pub message: OpenAIChatCompletionResponseMessage,
    pub finish_reason: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpenAIChatCompletionResponseMessage {
    pub role: String,
    pub content: String,
}

pub fn ask_openai(request: OpenAIRequest, api_key: String) -> Result<OpenAIResponse, String> {
    let client = Client::builder().build().expect("Failed to create client");

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .map_err(|e| e.to_string())?;

    response.json::<OpenAIResponse>().map_err(|e| e.to_string())
}

pub fn get_openai_api_key() -> String {
    std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")
}
