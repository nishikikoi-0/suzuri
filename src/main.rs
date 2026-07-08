use clap::Parser;
use curl::easy::Easy;

#[derive(Parser, Debug)]
#[command(
    version,
    disable_help_flag = true,
    long_about = None)]
struct Cli {
    #[arg(short,long)]
    help: bool,

    query: Option<String>,

    #[arg(short,long)]
    verbose: bool,

    #[arg(short,long)]
    informative: bool,

}

#[derive(Debug, serde::Deserialize)]
struct JishoResponse {
    data: Vec<Data>,
}

#[derive(Debug, serde::Deserialize)]
struct Data {
    slug: String,
    // is_common: bool,
    // tags: Vec<String>,
    // jlpt: Option<Vec<String>>,
    japanese: Vec<Japanese>,
    senses: Vec<Senses>,
}

#[derive(Debug, serde::Deserialize)]
struct Senses {
    english_definitions: Vec<String>,
    parts_of_speech: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
struct Japanese {
    // word: Option<String>,
    reading: String,
}


//   -i, --informative <query>     Display all available information for each shown result

fn help() {

    println!("
Suzuri v0.1.0 - a CLI-based Japanese dictionary using the Jisho API
Made by nishikikoi

Usage:

    suzuri [OPTIONS] <query>

Arguments:

    <input>                 Input accepted in romaji, kana, 
                            kanji, and english.

Options:

    -v, --verbose <query>   Show all search results, rather 
                            than only the most relevant one
    -h, --help              Show this help message and exit
        --version           Show version information

Examples:

    suzuri bakuzen/ばくぜん/漠然
        All display the kanji, reading, and definition of 漠然。

    suzuri ocean
        Displays information for 海洋 【かいよう】
    
    suzuri -v disappointment
        Displays information for 失望, 残念, 失意, etc.");
    std::process::exit(0);
}


fn print_definition(d: &Vec<Data>, i: usize) {
    let entry = &d[i];
    println!("");
    println!("{} [{}]", entry.slug, entry.japanese[0].reading);
    println!("");
    for sense in &entry.senses {
        println!(
            "• {} ({})",
            sense.english_definitions.join(", "),
            sense.parts_of_speech[0]
        );
    }
}

fn print_definition_verbose(d: &Vec<Data>) {
    for entry in d {
    println!("");
    println!("{} 【{}】", entry.slug, entry.japanese[0].reading);
    println!("");
    for sense in &entry.senses {
        println!(
            "• {} ({})",
            sense.english_definitions.join(", "),
            sense.parts_of_speech[0]
        );
    }
    }
}


fn main() {
    let args = Cli::parse();

    if args.query.is_none() && !args.help {
        println!("No arguments.");
        help();
    }

    if args.help {
        help();
    }

    let query = args.query.unwrap();

    let url = format!(
        "https://jisho.org/api/v1/search/words?keyword={}",
        query
    );

    let mut suzuri = Easy::new();

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

    let definitions: Vec<Data> = jisho.data;

    // println!("{}", definitions[2].japanese[0].reading)
    if args.verbose {
        print_definition_verbose(&definitions);
    } else {
        print_definition(&definitions, 0)
    }

    // println!("{}", serde_json::to_string_pretty(&json).unwrap());

}