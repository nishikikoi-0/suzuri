#[derive(Debug, serde::Deserialize, Clone)]
pub struct JishoResponse {
    data: Vec<Data>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Data {
    #[serde(alias = "slug")]
    word: String,
    is_common: bool,
    // tags: Vec<String>,
    // jlpt: Option<Vec<String>>,
    japanese: Vec<Japanese>,
    senses: Vec<Senses>,
}

pub struct ParseOptions {
    pub common: bool,
}

impl ParseOptions {
    pub fn new(c: bool) -> Self {
        Self { common: c }
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Senses {
    english_definitions: Vec<String>,
    parts_of_speech: Vec<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct Japanese {
    // word: Option<String>,
    reading: String,
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
                .filter(|x| x.is_common)
                .collect(),
        }
    }

    fn print_entry(entry: &Data) {
        println!("\n{} 【{}】\n", entry.word, entry.japanese[0].reading);
        for sense in &entry.senses {
            println!(
                "• {} ({})",
                sense.english_definitions.join(", "),
                sense.parts_of_speech[0]
            );
        }
    }
    pub fn print_definition(&self, i: usize) {
        Self::print_entry(&self.definitions[i]);
    }

    pub fn print_definition_verbose(&self) {
        for entry in &self.definitions {
            Self::print_entry(entry);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}
