use anyhow::Result;
use serde::{Deserialize, Serialize};
use spin_sdk::{
    http::{not_found, Request, Response},
    http_component,
};

const OPEN_API_KEY: &str = "OPEN_API_KEY";

#[derive(Debug, Deserialize)]
struct CompletionRequest {
    content: String,
}

#[derive(Debug, Serialize)]
struct CompletionResponse {
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Messages {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Messages>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Messages,
    finish_reason: String,
    index: u64,
}

#[http_component]
fn gpt3(req: Request) -> Result<Response> {
    let method = req.method();
    if method == "POST" {
        let open_api_key = std::env::var(OPEN_API_KEY)?;

        let completion_request: CompletionRequest =
            serde_json::from_slice(req.body().clone().unwrap().to_vec().as_slice()).unwrap();

        let mut messages = Vec::new();
        messages.push(Messages {
            role: "user".to_string(),
            content: completion_request.content,
        });

        let open_ai_request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: messages,
        };

        let uri = format!("https://api.openai.com/v1/chat/completions");
        let res = spin_sdk::outbound_http::send_request(
            http::Request::builder()
                .method("POST")
                .uri(uri)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", open_api_key))
                .body(Some(
                    serde_json::to_string(&open_ai_request).unwrap().into(),
                ))?,
        )?;

        let open_ai_response: OpenAIResponse =
            serde_json::from_slice(res.body().clone().unwrap().to_vec().as_slice()).unwrap();

        let content = &open_ai_response.choices[0].message.content;

        let mut completion_response = CompletionResponse {
            content: content.to_string(),
        };

        let json_response = serde_json::to_string(&completion_response)?;

        return Ok(http::Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(Some(json_response.into()))?);
    } else {
        return not_found();
    }
}
