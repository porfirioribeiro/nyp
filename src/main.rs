extern crate core;

use crate::lib::cli::ni::run_ni;
use crate::lib::cli::Apps;
use lib::cli::parse_cli;

mod lib;

fn main() -> anyhow::Result<()> {
    let apps = parse_cli();

    match apps {
        Apps::Ni(args) => run_ni(args)?,
    }

    Ok(())
}
