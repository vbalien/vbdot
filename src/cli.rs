use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    Add(AddArgs),
    Link(LinkArgs),
}

#[derive(Args, Clone)]
pub struct AddArgs {
    pub file_path: String,
}

#[derive(Args, Clone)]
pub struct LinkArgs {
    pub profile: String,
}
