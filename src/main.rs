extern crate core;

pub mod agent;
pub mod cli;
pub mod exec;
pub mod exit;

use crate::cli::ni::run_ni;
use crate::cli::parse_cli;
use crate::cli::Apps;

fn main() -> anyhow::Result<()> {
    let apps = parse_cli();

    match apps {
        Apps::Ni(args) => run_ni(args)?,
    }

    Ok(())
}
