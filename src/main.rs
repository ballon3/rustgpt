use structopt::StructOpt;
mod chatgpt;
use chatgpt::ChatGPT;

#[derive(StructOpt, Debug)]
#[structopt(name = "rgpt", about = "Rust OpenAI GPT-3 CLI Tool")]
struct Opt {
    #[structopt(short = "a", long = "auth", required = false, default_value = "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")]
    auth: String,

    #[structopt(short = "m", long = "model", required = false, default_value = "davinci")]
    model: String,

    #[structopt(short = "p", long = "prompt", required = false, default_value = "hey davinci")]
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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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