use csv::Writer;
use scraper::{Html, Selector};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::{error::Error, io, process};

const WORK_PATH: &str = "/Users/cristian.poll/ws/sandbox-rust/web_scraper/output";

#[tokio::main]
async fn main() {
    //let text: String = get_dictionary_com_page(1).await;
    let result_path: String = format!("{}/result.html", WORK_PATH);
    let text: String = load_page_result(result_path); // For testing without scraping repeatedly
    //save_page_result(&text, result_path); // For testing without scraping repeatedly

    // Create a csv writer and write the header. Will truncate existing file.
    // TODO: Don't truncate, so we can add on if our scrape ends early
    let csv_path: String = format!("{}/output.csv", WORK_PATH); // TODO: Use std::path::Path
    let wtr = create_or_load_file(&csv_path);

    println!("Parsing");
    let words = parse_dictionary_com_page(&text);

    // Create an empty csv file
    // Load a page
    // Parse the page and concatenate it to the csv file
    // Should I use Serde instead?
    // [{ word: "foo", date: "123" }]
}

fn create_or_load_file(path: &str) -> Writer<File> {
    let mut wtr: Writer<File>;
    if (Path::new(path).exists()) {
        let csv_file = File::options()
            .read(false)
            .write(true)
            .open(path)
            .expect("file to exist"); // TODO: Is it more idiomatic to try/error?
        wtr = Writer::from_writer(csv_file);
    } else {
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

fn save_page_result(content: &str, path: &str) {
    fs::write(path, content).expect("path should be writable");
}

fn load_page_result(path: String) -> String {
    let data = fs::read_to_string(path).expect("path should exist");
    return data;
}

// TODO: Turn this into an iterator that puts out a String array for every word on the page
fn parse_dictionary_com_page(text: &str) {
    let document = Html::parse_document(text);

    // See: https://www.scrapingbee.com/blog/web-scraping-rust/
    // See: https://docs.rs/scraper/latest/scraper/

    // For each <div class="wotd-entry-wrapper">:
    // Date: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(1) > div:nth-child(1) (.wotd-entry-date)
    // Name: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(1) > a:nth-child(2) (.wotd-entry-headword)
    // Phonetics: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(1) > div:nth-child(3) > p:nth-child(2) (.wotd-entry-phonetics)
    // Part of speech: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(2) > div:nth-child(1) (.wotd-entry-pos)
    // Definition: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(2) > p:nth-child(2) (.wotd-entry-definition)
    // Explanation: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(3) > p:nth-child(2) (.wotd-entry-explanation-section p) Note: <b> tags may appear
    // Example: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(4) > p:nth-child(2) (.wotd-entry-example)

    let mut results: [String; 7] = Default::default();

    let wrapper_selector = Selector::parse(".wotd-entry-wrapper").unwrap();
    let wrappers = document.select(&wrapper_selector);

    // let names = wrappers.filter_map(|wotd| wotd.select(&name_selector).next());

    // let name_list = names.map(|a| a.inner_html().trim().to_string());
    // println!("name list");
    // for n in name_list {
    //     println!("{n}");
    // }
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
        let mut i: usize = 0;
        for selector in &selectors {
            let value = wrapper
                .select(&selector)
                .next()
                .unwrap()
                .inner_html()
                .trim()
                .to_string();
            println!("{value}");
            results[i] = value;
            i += 1;
        }
    }
}

async fn get_dictionary_com_page(page: u32) -> String {
    // let body = reqwest::get("https://www.dictionary.com/word-of-the-day?page={page}")
    // Note: Last checked, Dictionary.com WOTD has 400 pages.
    let response: reqwest::Response = reqwest::get(format!(
        "https://www.dictionary.com/word-of-the-day?page={}",
        page
    ))
    .await
    .unwrap();

    // TODO: .error_for_status()?
    match response.status() {
        reqwest::StatusCode::OK => {
            println!("OK");
        }
        _ => {
            panic!("Request failed");
        }
    }

    // TODO: Read more about Rust error propagation (?) and unwrap
    let text = response.text().await.unwrap();

    return text;
}

// Pull data from https://www.dictionary.com/word-of-the-day?page=2

// Scrape out the goods:
// For each <div class="wotd-entry-wrapper">:
// Date: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(1) > div:nth-child(1) (.wotd-entry-date)
// Name: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(1) > a:nth-child(2) (.wotd-entry-headword)
// Phonetics: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(1) > div:nth-child(3) > p:nth-child(2) (.wotd-entry-phonetics)
// Part of speech: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(2) > div:nth-child(1) (.wotd-entry-pos)
// Definition: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(2) > p:nth-child(2) (.wotd-entry-definition)
// Explanation: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(3) > p:nth-child(2) (.wotd-entry-explanation-section p) Note: <b> tags may appear
// Example: div.wotd-entry-wrapper:nth-child(1) > div:nth-child(4) > p:nth-child(2) (.wotd-entry-example)

// <div class="wotd-entry-wrapper">
//       <div>
//         <div class="wotd-entry-date">February 10, 2026</div>
//         <a class="wotd-entry-headword" href="/browse/eudemonic">eudemonic</a>
//         <div class="wotd-entry-phonetics-section">
//         <button aria-label="play audio of eudemonic" class="common-btn-headword-audio js-play-word-audio" data-audiosrc="E03/E0336900/E0336900.mp3" data-audioorigin="https://nonprod-audio.dictionary.com"></button>
//         <p class="wotd-entry-phonetics">[yoo-di-<b>mon</b>-ik]</p>
//       </div>
//   <div>
//     <div class="wotd-entry-pos">adjective</div>
//     <p class="wotd-entry-definition">pertaining or conducive to happiness</p>
//   </div>
//   <div class="wotd-entry-explanation-section">
//     <div class="wotd-entry-explanation-pill">Explanation</div>
//     <p>A <b>eudemonic </b>lifestyle focuses on deeper fulfillment rather than fleeting pleasure. Rooted in philosophical traditions, the word points to choices and values that support well-being in a deeper, more enduring sense. From meaningful work to strong relationships, a <b>eudemonic </b>approach values happiness over time.</p>
//   </div>
//     <div>
//       <div class="wotd-entry-example-heading">Example</div>
//       <p class="wotd-entry-example">The artist was guided by <b>eudemonic </b>ideals, believing that creative fulfillment mattered more than wealth.</p>
//     </div>
// </div>
