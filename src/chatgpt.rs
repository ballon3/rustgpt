use serde_json::Result;
use reqwest::Client;

pub struct ChatGPT {
    pub client: Client,
    pub auth_token: String,
    pub model: String,
}

impl ChatGPT {
    pub fn new(auth_token: String, model: String) -> Self {
        ChatGPT {
            client: Client::new(),
            auth_token,
            model,
        }
    }

    pub async fn complete(&self, prompt: String, temperature: f32, max_tokens: i32, top_p: f32, frequency_penalty: f32, presence_penalty: f32) -> Result<String> {
        let json_body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "temperature": temperature,
            "max_tokens": max_tokens,
            "top_p": top_p as f32,
            "frequency_penalty": frequency_penalty as f32,
            "presence_penalty": presence_penalty as f32
        });

        let body_str = serde_json::to_string(&json_body)?;
        let resp = self.client
            .post("https://api.openai.com/v1/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .body(body_str)
            .send()
            .await
            .expect("Failed to send request");
        let body: serde_json::Value = match resp.json().await {
            Ok(body) => body,
            Err(e) => {
                eprintln!("Error getting json response: {}", e);
                std::process::exit(1);
            }
        };
        //println!("Raw json response: {:?}", body);

        let result = body["choices"][0]["text"].as_str().unwrap();
        Ok(result.to_string())
    }
}
