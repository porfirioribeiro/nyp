extern crate core;

use crate::lib::cli::ni::run_ni;
use crate::lib::cli::Apps;
use lib::cli::parse_cli;
use std::fmt::Debug;
use std::io::{BufRead, BufReader};
use subprocess::{Exec, Redirection};

mod lib;

fn main() -> anyhow::Result<()> {
    let apps = parse_cli();

    match apps {
        Apps::Ni(args) => run_ni(args)?,
    }

    Ok(())
}
