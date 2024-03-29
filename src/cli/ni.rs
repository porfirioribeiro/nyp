use crate::agent::AgentExecutor;
use clap::Parser;
use clap_verbosity_flag::Verbosity;

#[derive(Parser, Debug)]
pub struct NiApp {
    #[command(flatten)]
    verbose: Verbosity,

    #[clap(
        short = 'g',
        long = "global",
        value_parser,
        default_value = "false",
        help = "Install as a global package"
    )]
    pub global: bool,

    #[clap(
        short = 'D',
        long = "save-dev",
        value_parser,
        default_value = "false",
        help = "Save package to your `devDependencies`"
    )]
    pub dev: bool,

    #[clap(
        short = 'O',
        long = "save-optional",
        value_parser,
        default_value = "false",
        help = "Save package to your `optionalDependencies`"
    )]
    pub optional: bool,

    #[clap(
        short = 'P',
        long = "save-peer",
        value_parser,
        default_value = "false",
        help = "Save package to your `peerDependencies` and `devDependencies`"
    )]
    pub peer: bool,

    #[clap(
        long = "frozen",
        value_parser,
        default_value = "false",
        help = "Don't generate a lockfile and fail if an update is needed."
    )]
    pub frozen: bool,

    #[clap(value_parser)]
    pub package: Option<String>,
}

pub fn run_ni(app: NiApp) -> anyhow::Result<()> {
    env_logger::Builder::new()
        .filter_level(app.verbose.log_level_filter())
        .init();

    let agent = AgentExecutor::find()?;

    if let Some(package) = app.package {
        agent.add(package, app.dev, app.peer, app.optional)?;
    } else if app.frozen {
        agent.install_frozen()?;
    } else {
        agent.install()?;
    }

    Ok(())
}
