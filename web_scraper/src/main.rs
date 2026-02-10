use scraper::Html;

#[tokio::main]
async fn main() {
    //    let body = get_dictionary_com_page(1).await;
    //    println!("{body}");
    // say_hello().await;

    // let response: reqwest::Response = reqwest::get("https://www.rust-lang.org").await.unwrap();

    // match response.status() {
    //     reqwest::StatusCode::OK => {
    //         println!("OK");
    //     }
    //     _ => {
    //         panic!("Request failed");
    //     }
    // }

    // // TODO: Read more about Rust error propagation (?) and unwrap
    // let text = response.text().await.unwrap();

    let text: String = get_dictionary_com_page(1).await;

    // TODO: Save this in a file so we don't hit dictionary.com too often while testing

    parse_dictionary_com_page(text);

    println!("{text}");
}

async fn say_hello() {
    println!("hello, world!");
}

fn parse_dictionary_com_page(&text: &str) {
    let document = Html::parse_document(text);
}

async fn get_dictionary_com_page(page: u32) -> String {
    // let body = reqwest::get("https://www.dictionary.com/word-of-the-day?page={page}")
    let response: reqwest::Response = reqwest::get(format!(
        "https://www.dictionary.com/word-of-the-day?page={}",
        page
    ))
    .await
    .unwrap();

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
