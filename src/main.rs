use clap::Parser;

mod msgs;

mod handle_query;
use handle_query::QueryOptions;
use handle_query::handle_query;

mod handle_json;

#[derive(Parser, Debug)]
#[command(
    version,
    disable_help_flag = true,
    long_about = None)]
struct Cli {
    #[arg(short, long)]
    help: bool,

    queries: Vec<String>,

    #[arg(short, long)]
    verbose: bool,

    #[arg(short, long)]
    informative: bool,

    #[arg(short, long)]
    common: bool,
}

//   -i, --informative <query>     Display all available information for each shown result

fn help() {
    println!("{}", msgs::HELP_CMD);
    std::process::exit(0);
}

fn version_command() {
    println!("{}", msgs::VERSION_CMD);
    std::process::exit(0);
}

fn main() {
    let args = Cli::parse();
    if args.queries.is_empty() && !args.help {
        println!("\nPlease provide a query, or run suzuri --help for instructions on usage.");
        std::process::exit(0);
    }

    if args.help {
        help();
    }
    if args.version {
        version_command();
    }
    let query_args = QueryOptions::new(args.verbose, args.common);

    for q in &args.queries {
        handle_query(q, &query_args);
    }

    // println!("{}", serde_json::to_string_pretty(&json).unwrap());
}
