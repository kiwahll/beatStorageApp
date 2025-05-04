use std::io::{self, stdout};
use crossterm::{cursor::MoveTo, execute, terminal::{Clear, ClearType}};
use dialoguer::Select;
use serde_json::Value;
use thiserror::Error;

#[tokio::main]
async fn main() {
    let json: Value = fetch_data().await.unwrap();

    let mut index: usize = 0;
    loop {
        index = menu(&json, index.clone()).into();
        let item: &&Value = &json.as_array().unwrap().get(index).unwrap();

        for key in item.as_object().unwrap().keys() {
            println!("{}: {}", key, item.get(key).unwrap());
        }

        // PAUSE
        _ = io::stdin().read_line(&mut String::new());
        // Clear
        execute!(
            stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0)
        ).unwrap()
    }
}

fn menu(json: &Value, index: usize) -> usize {
    let mut selection_items: Vec<&str> = Vec::new();
    if let Some(array) = json.as_array() {
        for item in array {
            selection_items.push(item.get("title").unwrap().as_str().unwrap());
        }
    }

    let selection = Select::new()
        .default(index)
        .items(&selection_items)
        .interact()
        .unwrap();

    selection
}

async fn fetch_data() -> Result<Value, MyError> {
    let url: &str = "https://api.sampleapis.com/coffee/hot";
    let response = reqwest::get(url).await?.text().await?;
    let json: Value = serde_json::from_str(&response)?;
    Ok(json)
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Network error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error)
}