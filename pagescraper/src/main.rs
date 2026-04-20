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
    let area_codes =  document.select(&area_codes_selector);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_path)
        .expect("Unable to open file");

    for element in area_codes {
        let mut area_code = element
            .text()
            .collect::<String>();
        
        // println!("{:#?}", area_code);
        
        // file.write_all(area_code.as_bytes()).expect("Unable to append data");
        writeln!(file, "{}", area_code);
    }

    println!("File generated!");

    Ok(())
}
