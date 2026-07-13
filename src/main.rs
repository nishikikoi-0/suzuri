use clap::Parser;
mod db;
use db::DB_VERSION;
mod handle_query;
mod msgs;
use handle_query::QueryOptions;
use handle_query::handle_query;
use rusqlite::Connection;
mod handle_json;

#[derive(Parser, Debug)]
#[command(
    disable_version_flag = true,
    disable_help_flag = true,
    long_about = None)]
struct Cli {
    #[arg(short, long)]
    help: bool,

    #[arg(long)]
    version: bool,

    queries: Vec<String>,

    #[arg(short, long)]
    verbose: bool,

    //    #[arg(short, long)]
    //    informative: bool,
    #[arg(short, long)]
    common: bool,

    #[arg(long)]
    db_version: bool,

    #[arg(long)]
    update_db: bool,

    #[arg(long)]
    no_override: bool,
}

//   -i, --informative <query>     Display all available information for each shown result

fn help() {
    println!("{}\n", msgs::HELP_CMD);
    std::process::exit(0);
}

fn version_command() {
    println!("{}\n", msgs::VERSION_CMD);
    std::process::exit(0);
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    if args.queries.is_empty() && !args.version && !args.help && !args.db_version && !args.update_db
    {
        println!("\nPlease provide a query, or run suzuri --help for instructions on usage.");
        std::process::exit(0);
    } else if args.help {
        help();
    } else if args.version {
        version_command();
    } else if args.update_db {
        db::init_db();
        println!("\nDatabase updated to version {}.\n", DB_VERSION);
        std::process::exit(0);
    } else if args.db_version {
        db::db_version()?;
        std::process::exit(0);
    }

    let query_args = QueryOptions::new(args.verbose, args.common);
    let conn: Connection;
    if args.no_override {
        conn = db::empty_db().unwrap();
    } else {
        conn = db::init_db().unwrap();
    }

    for q in &args.queries {
        handle_query(q, &query_args, &conn);
    }

    // println!("{}", serde_json::to_string_pretty(&json).unwrap());
    println!();
    Ok(())
}
