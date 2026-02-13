use csv::Writer;
use scraper::{Html, Selector};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::thread;
use std::time;

#[tokio::main]
async fn main() {
    // Note: Last checked, Dictionary.com WOTD has 400 pages.
    let page_range = 1..51;
    let csv_path =
        Path::new("/Users/cristian.poll/ws/sandbox-rust/web_scraper/output/wotd_list.csv");

    // Create a csv writer and write the header. Loads the file for append if it already exists.
    let mut wtr = create_or_load_file(csv_path);

    // Loop through Dictionary.com pages, parse, and write them to the csv
    println!("Begin parsing");
    for i in page_range {
        println!("Parsing page {i}");
        let text: String = get_dictionary_com_page(i).await;
        let words = parse_dictionary_com_page(&text);

        for word in words {
            wtr.write_record(word).expect("csv should be writable");
        }
        wtr.flush().expect("file is writable"); // Flush after every page

        thread::sleep(time::Duration::from_secs(2));
    }
    println!("Done");
}

fn create_or_load_file(path: &Path) -> Writer<File> {
    let mut wtr: Writer<File>;
    if path.exists() {
        let csv_file = File::options()
            .append(true)
            .open(path)
            .expect("file to exist"); // TODO: Is it more idiomatic to try/error?
        wtr = Writer::from_writer(csv_file);
    } else {
        fs::create_dir_all(path.parent().unwrap())
            .expect("path parents should allow folder creation");
        let csv_file = File::create(path).expect("path should be writable");
        wtr = Writer::from_writer(csv_file);
        wtr.write_record([
            "Word",
            "Date",
            "Phonetics",
            "Part of Speech",
            "Definition",
            "Explanation",
            "Example",
        ])
        .expect("csv should be writable");
    }
    return wtr;
}

async fn get_dictionary_com_page(page: u32) -> String {
    let response: reqwest::Response = reqwest::get(format!(
        "https://www.dictionary.com/word-of-the-day?page={}",
        page
    ))
    .await
    .unwrap();

    // TODO: .error_for_status()?
    match response.status() {
        reqwest::StatusCode::OK => {
            // println!("OK");
        }
        _ => {
            panic!("Request failed");
        }
    }

    // TODO: Read more about Rust error propagation (?) and unwrap
    let text = response.text().await.unwrap();

    return text;
}

// TODO: Turn this into an iterator that puts out a String array for every word on the page
fn parse_dictionary_com_page(text: &str) -> Vec<Vec<String>> {
    let document = Html::parse_document(text);

    let wrapper_selector = Selector::parse(".wotd-entry-wrapper").unwrap();
    let wrappers = document.select(&wrapper_selector);
    let mut results: Vec<Vec<String>> = Vec::new();

    let selectors = [
        Selector::parse(".wotd-entry-headword").unwrap(),
        Selector::parse(".wotd-entry-date").unwrap(),
        Selector::parse(".wotd-entry-phonetics").unwrap(),
        Selector::parse(".wotd-entry-pos").unwrap(),
        Selector::parse(".wotd-entry-definition").unwrap(),
        Selector::parse(".wotd-entry-explanation-section p").unwrap(),
        Selector::parse(".wotd-entry-example").unwrap(),
    ];

    for wrapper in wrappers {
        let mut word_properties: Vec<String> = Vec::new();
        for selector in &selectors {
            let value = wrapper.select(&selector).next();

            if value == None {
                word_properties.push("".to_string());
            } else {
                word_properties.push(value.unwrap().inner_html().trim().to_string());
            }
        }
        results.push(word_properties);
    }

    return results;
}

// Debug functions to avoid hitting Dictionary.com when testing
// fn save_page_result(content: &str, path: &str) {
//     fs::write(path, content).expect("path should be writable");
// }

// fn load_page_result(path: String) -> String {
//     let data = fs::read_to_string(path).expect("path should exist");
//     return data;
// }
