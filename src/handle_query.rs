use crate::handle_json::{JishoResponse, JsonParsed, ParseOptions};

use curl::easy::Easy;

pub struct QueryOptions {
    verbose: bool,
    common: bool,
}

impl QueryOptions {
    pub fn new(v: bool, c: bool) -> Self {
        Self {
            verbose: v,
            common: c,
        }
    }
}

pub fn handle_query(q: &str, args: &QueryOptions) {
    let mut suzuri = Easy::new();

    let encoded_query = suzuri.url_encode(q.as_bytes());

    let url = format!(
        "https://jisho.org/api/v1/search/words?keyword={}",
        encoded_query
    );

    suzuri.url(&url).unwrap();

    let mut response = Vec::new();

    {
        let mut transfer = suzuri.transfer();

        transfer
            .write_function(|data| {
                response.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    let jisho: JishoResponse = serde_json::from_slice(&response).unwrap();

    let parse_args = ParseOptions::new(args.common);

    let parsed = JsonParsed::new(jisho, &parse_args);

    // println!("{}", d[2].japanese[0].reading)
    if parsed.is_empty() {
        println!("\nNo search results for \"{}\"", encoded_query);
    } else if args.verbose {
        parsed.print_definition_verbose();
    } else {
        parsed.print_definition(0)
    }
}
