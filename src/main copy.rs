#[macro_use]
extern crate serde;

use serde::Deserialize;
use std::collections::HashMap;
use structopt::StructOpt;
use serde_json::{Result, json};
use reqwest::{Client, Error};

pub struct ChatGPT {
    pub client: Client,
    pub auth_token: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct DALLEImage {
    pub url: String,
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
    
    pub async fn generate_dalle_image(&self, prompt: String, size: String) -> Result<DALLEImage> {
        let json_body = serde_json::json!({
            "model": "image-alpha-001",
            "prompt": prompt,
            "size": size
        });

        let body_str = serde_json::to_string(&json_body)?;
        let resp = self.client
            .post("https://api.openai.com/v1/images/generations")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .body(body_str)
            .send()
            .await?;

        let body: HashMap<String, DALLEImage> = resp.json().await?;
        let dalle_image = body.get("data").ok_or("Failed to parse DALL-E image response")?;

        Ok(dalle_image.clone())
    }

    pub async fn complete_code(&self, prompt: &str) -> Result<CodeCompletionResponse> {
        let json_body = serde_json::json!({
            "model": self.model,
            "prompt": prompt,
            "temperature": 0.5,
            "max_tokens": 2048,
            "n": 1,
            "stop": "\n"
        });

        let body_str = serde_json::to_string(&json_body)?;
        let resp = self.client
            .post("https://api.openai.com/v1/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.auth_token))
            .body(body_str)
            .send()
            .await?;

        let body: CodeCompletionResponse = resp.json().await?;

        Ok(body)
    }
}    

const OPENAI_API_URL: &str = "https://api.openai.com/v1";

#[derive(Deserialize)]
struct CompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<Logprobs>,
    pub finish_reason: String,
}

#[derive(Deserialize)]
struct Logprobs {
    pub tokens: Vec<String>,
    pub token_logprobs: Vec<f32>,
    pub top_logprobs: Vec<Vec<f32>>,
    pub text_offset: Vec<usize>,
}

#[derive(Deserialize)]
struct CodeCompletionResponse {
    pub choices: Vec<CodeCompletionChoice>,
}

#[derive(Deserialize)]
struct CodeCompletionChoice {
    pub text: String,
    pub index: usize,
    pub finish_reason: String,
}

fn get_client(auth_token: String) -> Result<Client, Error> {
    Client::builder()
        .user_agent("rgpt-cli")
        .default_headers(
            reqwest::header::HeaderMap::from_iter(
                vec![
                    ("Content-Type", "application/json"),
                    ("Authorization", format!("Bearer {}", auth_token)),
                ]
                .into_iter()
                .map(|(key, value)| (reqwest::header::HeaderName::from_bytes(key.as_bytes()).unwrap(), value.parse().unwrap())),
            ),
        )
        .build()
        .map_err(Error::from)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rgpt", about = "Rust OpenAI GPT-3 CLI Tool")]
struct Opt {
    #[structopt(short = "a", long = "auth", required = false, default_value = "sk-xxx")]
    auth: String,

    #[structopt(short = "m", long = "model", required = false, default_value = "davinci")]
    model: String,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "chat")]
    Chat {
        #[structopt(short = "p", long = "prompt", required = true)]
        prompt: String,

        #[structopt(short = "t", long = "temperature", required = false, default_value = "0.5")]
        temperature: f32,

        #[structopt(short = "x", long = "max_tokens", required = false, default_value = "512")]
        max_tokens: i32,

        #[structopt(short = "o", long = "top_p", required = false, default_value = "1")]
        top_p: i32,

        #[structopt(short = "f", long = "frequency_penalty", required = false, default_value = "1")]
        frequency_penalty: i32,

        #[structopt(short = "r", long = "presence_penalty", required = false, default_value = "1")]
        presence_penalty: i32,
    },

    #[structopt(name = "code")]
    Code {
        #[structopt(short = "p", long = "prompt", required = true)]
        prompt: String,
    },

    #[structopt(name = "image")]
    Image {
        #[structopt(short = "p", long = "prompt", required = true)]
        prompt: String,

        #[structopt(short = "s", long = "size", required = false, default_value = "1024x1024")]
        size: String,

        #[structopt(short = "n", long = "num_images", required = false, default_value = "1")]
        num_images: i32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
let opt = Opt::from_args();
let auth_token = opt.auth.clone();
let model = opt.model.clone();

match opt.cmd {
    Command::Chat {
        prompt,
        temperature,
        max_tokens,
        top_p,
        frequency_penalty,
        presence_penalty,
    } => {
        let client = get_client(auth_token)?;

        let body = json!({
            "model": model,
            "prompt": prompt,
            "temperature": temperature,
            "max_tokens": max_tokens,
            "top_p": top_p as f32,
            "frequency_penalty": frequency_penalty as f32,
            "presence_penalty": presence_penalty as f32,
        });

        let res = client
            .post(&format!("{}/completions", OPENAI_API_URL))
            .json(&body)
            .send()
            .await?
            .json::<CompletionResponse>()
            .await?;

        let response_text = res.choices[0].text.trim();

        println!("{}", response_text);
    }

    Command::Code { prompt } => {
        let client = get_client(auth_token)?;

        let body = json!({
            "model": model,
            "prompt": prompt,
            "temperature": 0.5,
            "max_tokens": 1024,
            "n": 1,
            "stop": "\n"
        });

        let res = client
            .post(&format!("{}/engines/codex/completions", OPENAI_API_URL))
            .json(&body)
            .send()
            .await?
            .json::<CodeCompletionResponse>()
            .await?;

        let response_text = res.choices[0].text.trim();

        println!("{}", response_text);
    }

    Command::Image {
        prompt,
        size,
        num_images,
    } => {
        let client = get_client(auth_token)?;

        let body = json!({
            "model": model,
            "prompt": prompt,
            "size": size,
            "num_images": num_images,
        });

        let res = client
            .post(&format!("{}/images/generations", OPENAI_API_URL))
            .json(&body)
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;

        for (i, url) in res.iter().enumerate() {
            println!("Image {}: {}", i + 1, url);
        }
    }
}

Ok(())
}