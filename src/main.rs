#![allow(dead_code)]
mod cli;
mod config1;
mod config2;
mod config3;
mod config4;
mod config5;
mod config6;
mod config7;

fn main() {
    // use structopt::StructOpt as _;
    // cli::Cli::from_args();

    println!(
        "{}",
        serde_json::to_string_pretty(&schemars::schema_for!(config7::Config)).unwrap()
    );

    // println!(
    //     "{}",
    //     serde_json::to_string_pretty(&schemars::schema_for!(cli::Cli)).unwrap()
    // );
}
