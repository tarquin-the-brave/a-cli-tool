#[derive(structopt::StructOpt, schemars::JsonSchema)]
pub struct Cli {
    #[structopt(short, long)]
    config: std::path::PathBuf,

    #[structopt(long)]
    dry_run: bool,

    #[structopt(subcommand)]
    subcommand: Subcommands,
}

#[derive(structopt::StructOpt, schemars::JsonSchema)]
pub enum Subcommands {
    Foo,
    Bar,
    Baz,
}
