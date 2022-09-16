mod cli;
mod command;
mod data_query;
mod helper;
mod print;

use {
    crate::cli::Cli,
    clap::Parser,
    data_query::DataQuery,
    std::{fmt::Debug, path::PathBuf},
};

#[derive(Parser, Debug)]
#[clap(name = "gluesql", about, version)]
struct Args {
    /// SQL file to execute
    #[clap(short, long, value_parser)]
    execute: Option<PathBuf>,
}

pub fn run() {
    let args = Args::parse();

    let output = std::io::stdout();
    let storage = DataQuery::new();
    let mut cli = Cli::new(storage, output);

    if let Some(execute) = args.execute {
        cli.load(execute).unwrap();
    } else {
        if let Err(e) = cli.run() {
            eprintln!("{}", e);
        }
    }
}
