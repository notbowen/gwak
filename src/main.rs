#![allow(dead_code)]
#![allow(unused_variables)]

use crate::adapters::ctfd::CtfdAdapter;
use crate::types::adapter::Adapter;
use clap::Parser;

mod adapters;
mod platforms;
mod types;
mod utils;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// CTF platform username (omit the token if you're using this)
    #[arg(short, long)]
    username: Option<String>,

    /// CTF platform password (omit the token if you're using this)
    #[arg(short, long)]
    password: Option<String>,

    /// CTF platform access key (omit the username & password if you're using this)
    #[arg(short, long)]
    token: Option<String>,

    /// The URL of the CTF platform
    #[arg()]
    website: String,

    /// The CTF backend the platform is using
    #[arg(value_enum)]
    platform: platforms::Platform,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse arguments from user & validate
    let args = Args::parse();

    let msg = match &args {
        Args {
            username: Some(_),
            password: None,
            ..
        } => "Missing password!",
        Args {
            username: None,
            password: Some(_),
            ..
        } => "Missing username!",
        Args {
            username: None,
            password: None,
            token: None,
            ..
        } => "Missing token or login credentials!",
        _ => "",
    };

    if msg != "" {
        error!(msg);
        return Ok(());
    }

    // Retrieve the challenges from the platform
    let adapter: Box<dyn Adapter> = match args.platform {
        platforms::Platform::Ctfd => Box::new(CtfdAdapter::new(
            args.website,
            args.username,
            args.password,
            args.token,
        )),
    };

    let challenges = adapter.get_challenges().await;
    info!(&format!("Obtained the following challenges: {:?}", challenges));

    Ok(())
}
