use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use dialoguer::Select;
use dotenv::dotenv;
use serde_json::Value;
use strum::VariantNames;
use std::{
    io::{self, stdout},
    process::exit, usize,
};
use std::process::Command;
#[macro_use]
extern crate dotenv_codegen;
use strum_macros::{EnumIter, VariantNames};

#[derive(EnumIter, VariantNames, Debug)]
enum BeatOption {
    Back,
    Open,
    Download
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    clear();
    let json: Value = fetch_data().await.unwrap();

    let mut index: usize = 0;
    loop {
        index = beat_selection(&json, index.clone()).unwrap_or_else(|e| {
            eprintln!("{}", e);
            pause();
            exit(1);
        });
        let item: &&Value = &json.as_array().unwrap().get(index).unwrap();

        for key in item.as_object().unwrap().keys() {
            println!("{}: {}", key, item.get(key).unwrap());
        }
        match beat_options() {
            1 => {
                _ = open::that(item.get("url").unwrap().as_str().unwrap());
            },
            2 => {
                download(item);
                println!("Done!");
            },
            _ => {}
        }
        clear();
    }
}

fn clear() {
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
}

fn pause() {
    _ = io::stdin().read_line(&mut String::new());
}

fn download(item: &Value) {
    let status = Command::new("yt-dlp")
        .args([
            "-f",
            "bestaudio",
            "-o",
            item.get("title").unwrap().as_str().unwrap(),
            "--extract-audio",
            "--audio-format",
            "flac",
            item.get("url").unwrap().as_str().unwrap(),
        ])
        .current_dir(dotenv!("DOWNLOAD_PATH"))
        .status()
        .expect("failed to execute yt-dlp");

    println!("Status: {}", status);
}

fn beat_selection(json: &Value, index: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let mut selection_items: Vec<&str> = Vec::new();
    if let Some(array) = json.as_array() {
        for item in array {
            selection_items.push(item.get("title").unwrap().as_str().unwrap());
        }
    }

    if *&selection_items.is_empty() {
        return Err("No items found in JSON".into());
    }

    let selection = Select::new()
        .default(index)
        .items(&selection_items)
        .interact()
        .unwrap();

    Ok(selection)
}

fn beat_options() -> usize {
    Select::new().default(0).items(BeatOption::VARIANTS).interact().unwrap()
}

async fn fetch_data() -> Result<Value, Box<dyn std::error::Error>> {
    let response = reqwest::get(dotenv!("URL"))
        .await
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            pause();
            exit(1);
        })
        .text()
        .await
        .unwrap();
    let json: Value = serde_json::from_str(&response).unwrap();

    Ok(json)
}
