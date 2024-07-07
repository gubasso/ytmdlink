use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, ValueEnum};
use reqwest::get;
use serde_json::Value;

#[derive(ValueEnum, Clone, Debug)]
enum LinkType {
    /// `[title](url)` format.
    Link,
    /// `[^X]: url "title"` format.
    Ref,
}

#[derive(Parser, Debug)]
#[command(name = "ytmdlink", about = "A program to fetch YouTube oEmbed data and generate markdown links")]
struct CliArgs {
    /// Youtube URL
    url: String,
    /// Type of the md link output
    #[arg(short, long, value_enum, default_value_t = LinkType::Link)]
    link_type: LinkType,
}

fn get_val(resp: &HashMap<String, Value>, key: &str) -> Result<String> {
    resp.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_owned())
        .ok_or_else(||anyhow!("Key '{}' not found or not a string", key))
        
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();
    let req_url = format!("https://www.youtube.com/oembed?url={}&format=json", args.url);
    let resp = get(req_url).await?
        .json::<HashMap<String, Value>>()
        .await?;
    let title = get_val(&resp, "title").context("Fetching title failed")?;
    let author = get_val(&resp, "author_name").context("Fetching author_name failed")?;
    let link_title = format!("{title} - {author}");
    match args.link_type {
        LinkType::Link => {
            print!("[{}]({})", link_title, args.url);
        },
        LinkType::Ref => {
            print!("[^X]: {} \"{}\"", args.url, link_title);
        }
    }
    Ok(())
}
