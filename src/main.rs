use structopt::StructOpt;
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

// ...

#[derive(structopt::StructOpt, Debug)]
#[structopt(name = "rgpt", about = "Rust OpenAI GPT-3 CLI Tool")]
struct Opt {
    #[structopt(
        short = "a",
        long = "auth",
        required = false,
        default_value = "",
        env = "OPENAI_API_KEY"
    )]
    auth: String,

    #[structopt(short = "m", long = "model", required = false, default_value = "text-davinci-003")]
    model: String,

    #[structopt(short = "p", long = "prompt", required = false, default_value = "What is Rust GPT CLI?")]
    prompt: String,
    
    #[structopt(short = "t", long = "temperature", required = false, default_value = "0.5")]
    temperature: f32,

    #[structopt(short = "x", long = "max_tokens", required = false, default_value = "512")]
    max_tokens: i32,

    #[structopt(short = "o", long = "top_p", required = false, default_value = "1")]
    top_p: f32,

    #[structopt(short = "f", long = "frequency_penalty", required = false, default_value = "0")]
    frequency_penalty: f32,

    #[structopt(short = "r", long = "presence_penalty", required = false, default_value = "0")]
    presence_penalty: f32,
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    // Activate debug mode
    // #[structopt(short, long)]
    // debug: bool,

    // // The number of occurrences of the `v/verbose` flag
    // /// Verbose mode (-v, -vv, -vvv, etc.)
    // #[structopt(short, long, parse(from_occurrences))]
    // verbose: u8,

    // /// admin_level to consider
    // #[structopt(short, long)]
    // level: Vec<String>,

    // /// Files to process
    // #[structopt(name = "FILE", parse(from_os_str))]
    // files: Vec<PathBuf>,
    
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    
    let auth_token = opt.auth;
    let model = opt.model;
    let prompt = opt.prompt;
    let temperature = opt.temperature;
    let max_tokens = opt.max_tokens;
    let top_p = opt.top_p as f32;
    let frequency_penalty = opt.frequency_penalty as f32;
    let presence_penalty = opt.presence_penalty as f32;

    let gpt = ChatGPT::new(auth_token, model);
    //println!("Got a repsonse from openai");

    let result = gpt.complete(prompt, temperature, max_tokens, top_p, frequency_penalty, presence_penalty).await;

    match result {
        Ok(text) => println!("{}", text),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}