use std::fs::OpenOptions;
use std::io::{Write};
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize}; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://www.country-codes.org/us-area-codes";
    let output_path = "area-codes.csv";

    let html = reqwest::get(url)
        .await?
        .text()
        .await?;
    
    // println!("{html:#?}");

    let document = Html::parse_document(&html);
    let area_codes_selector = Selector::parse(".area-code-badge strong").unwrap();
    let area_code_elements =  document.select(&area_codes_selector);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_path)
        .expect("Unable to open file");

    let mut area_codes = Vec::new();

    for element in area_code_elements {
        let area_code = element
            .text()
            .collect::<String>();        
        area_codes.push(area_code)
    }

    area_codes.sort_by(|a, b| a.cmp(&b));

    // println!("{:#?}", area_codes);

    let serialized_area_codes = serde_json::to_string(&area_codes).unwrap();
    let area_codes_bytes = serialized_area_codes.as_bytes();
        
    file.write_all(area_codes_bytes).expect("Unable to append data");

    println!("File generated!");

    Ok(())
}
