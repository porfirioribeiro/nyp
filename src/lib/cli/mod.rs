use crate::lib::cli::ni::NiApp;
use clap::{ArgMatches, Command, FromArgMatches, Subcommand};

pub mod ni;

pub fn parse_cli() -> Apps {
    let cmd = Command::new(env!("CARGO_CRATE_NAME"))
        .multicall(true)
        .subcommand(Apps::augment_subcommands(
            Command::new("nyp")
                .arg_required_else_help(true)
                .subcommand_value_name("APP")
                .subcommand_help_heading("APPS"),
        ));

    let cli: Command = Apps::augment_subcommands(cmd);

    let mut matches: &ArgMatches = &cli.get_matches();

    let subcommand = matches.subcommand();
    if let Some(("nyp", cmd)) = subcommand {
        matches = cmd;
    }

    Apps::from_arg_matches(matches)
        .map_err(|err| err.exit())
        .unwrap()
}

#[derive(Subcommand, Debug)]
pub enum Apps {
    Ni(NiApp),
}

// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// pub struct Args {
//     /// Name of the person to greet
//     #[clap(short, long, value_parser, default_value = "World")]
//     pub name: String,
//
//     /// Number of times to greet
//     #[clap(short, long, value_parser, default_value_t = 1)]
//     pub count: u8,
// }
//
// #[derive(Subcommand, Debug)]
// pub enum NypLn {
//     #[clap(arg_required_else_help = true, visible_alias("c"))]
//     Clone {
//         /// The remote to clone
//         #[clap(value_parser)]
//         remote: String,
//     },
// }
//
// #[derive(Debug, Subcommand)]
// pub enum Commands {
//     Test(Args),
//     /// Clones repos
//     #[clap(arg_required_else_help = true, visible_alias("c"))]
//     Clone {
//         /// The remote to clone
//         #[clap(value_parser)]
//         remote: String,
//     },
// }
