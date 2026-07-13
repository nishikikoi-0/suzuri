use crate::db;
use crate::handle_json::{JishoResponse, JsonParsed, ParseOptions};
use colored::Colorize;
use rusqlite::Connection;

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

pub fn handle_query(q: &str, args: &QueryOptions, conn: &Connection) {
    if !args.verbose {
        if let Ok(Some(entry)) = db::get_override(conn, q) {
            if entry.override_type == "query" {
                let definition = entry.value.replace('@', "\n");
                if entry.reading.len() == 0 {
                    println!("\n{}\n", definition);
                } else if entry.kanji.len() > 0 {
                    let (first, rest) = definition.split_once('\n').unwrap();
                    println!(
                        "\n{} {}{}{}\n\n{}\n{}",
                        entry.kanji.truecolor(0x00, 0xE0, 0xBA),
                        "【".truecolor(0xFF, 0xCF, 0x00),
                        entry.reading.truecolor(0x91, 0x00, 0x8D),
                        "】".truecolor(0xFF, 0xCF, 0x00),
                        first.truecolor(0xFF, 0x34, 0x83),
                        rest
                    );
                } else {
                    let (first, rest) = definition.split_once('\n').unwrap();
                    println!(
                        "\n{}\n\n{}\n{}",
                        entry.reading.truecolor(0x00, 0xE0, 0xBA),
                        first.truecolor(0xFF, 0x34, 0x83),
                        rest
                    );
                };
                if entry.replace == 1 {
                    return;
                }
            }
        }
    }

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
        parsed.print_definition_verbose(conn);
    } else {
        parsed.print_definition(0, conn)
    }
}
