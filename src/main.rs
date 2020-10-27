mod cli;
mod config;

fn main() {
    use structopt::StructOpt as _;
    cli::Cli::from_args();
}
