use crate::db;
use colored::Colorize;
use rusqlite::Connection;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct JishoResponse {
    data: Vec<Data>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Data {
    #[serde(alias = "slug")]
    word: String,
    is_common: Option<bool>,
    // tags: Vec<&str>,
    // jlpt: Option<Vec<&str>>,
    japanese: Vec<Japanese>,
    senses: Vec<Senses>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Senses {
    english_definitions: Vec<String>,
    parts_of_speech: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Japanese {
    // word: Option<&str>,
    reading: String,
}

pub struct ParseOptions {
    pub common: bool,
}

impl ParseOptions {
    pub fn new(c: bool) -> Self {
        Self { common: c }
    }
}

pub struct JsonParsed {
    definitions: Vec<Data>,
}

impl JsonParsed {
    pub fn new(j: JishoResponse, args: &ParseOptions) -> Self {
        let parsed = Self {
            definitions: j.data,
        };

        if args.common {
            parsed.common_filter()
        } else {
            parsed
        }
    }

    fn common_filter(&self) -> Self {
        Self {
            definitions: self
                .definitions
                .clone()
                .into_iter()
                .filter(|x| x.is_common.unwrap_or(false))
                .collect(),
        }
    }

    fn print_entry(entry: &Data, conn: &Connection) {
        println!(
            "\n{} {}{}{}\n",
            entry.word.truecolor(0x00, 0xE0, 0xBA),
            "【".truecolor(0xFF, 0xCF, 0x00),
            entry.japanese[0].reading.truecolor(0x91, 0x00, 0x8D),
            "】".truecolor(0xFF, 0xCF, 0x00)
        );
        let mut pos_prev: String = "".to_string();
        for sense in &entry.senses {
            'definition: {
                if let Ok(Some(entry)) =
                    db::get_override(conn, &sense.english_definitions.join(", "))
                {
                    if entry.override_type == "skip_definition" {
                        break 'definition;
                    }
                }

                let pos = sense
                    .parts_of_speech
                    .as_ref()
                    .and_then(|v| v.first())
                    .map(String::as_str)
                    .unwrap_or("other");
                if pos != "other" {
                    if pos != pos_prev {
                        println!(
                            "   {}\n     {}",
                            pos.to_string().truecolor(0xFF, 0x34, 0x83).bold(),
                            sense.english_definitions.join(", ")
                        );
                        pos_prev = pos.to_string();
                    } else {
                        println!("     {}", sense.english_definitions.join(", "));
                    }
                }
            }
        }
    }
    pub fn print_definition(&self, i: usize, conn: &Connection) {
        Self::print_entry(&self.definitions[i], conn);
    }

    pub fn print_definition_verbose(&self, conn: &Connection) {
        for entry in &self.definitions {
            Self::print_entry(entry, conn);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}
