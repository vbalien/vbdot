#![feature(absolute_path)]

mod cli;
mod commands;
mod config;
mod ctx;
mod util;

use anyhow::Result;
use clap::Parser;
use cli::Commands;

use crate::{cli::Cli, ctx::Ctx};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut ctx = Ctx::new(cli)?;

    let command = ctx.cli.command.clone();
    match command {
        Some(Commands::Link(args)) => commands::link(&args, &ctx),
        Some(Commands::Add(args)) => commands::add(&args, &mut ctx),
        None => Ok(()),
    }
}
