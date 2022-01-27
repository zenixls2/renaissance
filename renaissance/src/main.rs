use anyhow::Error;
mod oms;
use oms::*;
mod connector;
use clap::{Parser, Subcommand};
use connector::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, help = "strategy / exchange config placement")]
    config_path: String,
    #[clap(short, long, help = "secret key placement")]
    key_path: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
#[clap(author, version, about, long_about = None)]
enum Command {
    #[clap(about = "run the trading strategy for the real money on exchanges")]
    Run {},
    #[clap(about = "test trading strategy offline using dumped data")]
    Backtest {},
    #[clap(about = "test trading strategy with fake money inside exchanges' test environment")]
    PaperTrade {},
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();
    println!("{:?}", args);
    Ok(())
}
