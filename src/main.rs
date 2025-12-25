use std::env;
use std::fs;
use std::path::Path;
use std::time::Duration;
use reqwest::Client;
//use crate::Client;

fn main() {
    // Read environment variables
    let gallery_id = env::var("GALLERY_ID")
    .unwrap_or_else(|_| {
        eprintln!("GALLERY_ID not set. Using default 3690850 for testing.");
        "3690850".to_string()
    });
    let pages: u32 = env::var("PAGES")
        .unwrap_or("10".into())
        .parse()
        .expect("Invalid PAGES");

    let output_dir = format!("downloads/{}", gallery_id);
    fs::create_dir_all(&output_dir).expect("Failed to create directory");

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create client");

    for page in 1..=pages {
        let url = format!(
            "https://i3.nhentai.net/galleries/{}/{}.jpg",
            gallery_id, page
        );

        let file_path = format!("{}/{}.jpg", output_dir, page);
        if Path::new(&file_path).exists() {
            println!("Skipping page {}", page);
            continue;
        }

        println!("Downloading page {}", page);

        match client.get(&url).send() {
            Ok(resp) if resp.status().is_success() => {
                let bytes = resp.bytes().expect("Failed to read bytes");
                fs::write(&file_path, bytes).expect("Failed to save file");
            }
            Ok(resp) => {
                eprintln!("Failed page {}: {}", page, resp.status());
            }
            Err(e) => {
                eprintln!("Error page {}: {}", page, e);
            }
        }
    }

    println!("✅ Download complete");
}
