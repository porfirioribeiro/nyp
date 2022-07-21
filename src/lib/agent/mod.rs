pub mod pnpm;
pub mod yarn;

use self::pnpm::PNPM_AGENT;
use self::yarn::YARN_AGENT;

use crate::lib::cli::ni::NiApp;
use crate::lib::exec;
use crate::lib::fs_utils::find_file_up;
use anyhow::Error;
use std::env;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Stdio;
use subprocess::Exec;

#[derive(Debug)]
pub struct AgentCmd {
    agent: &'static str,
    run: &'static str,
    install: &'static str,
    frozen: &'static str,
    global: &'static str,
    add: &'static str,
    upgrade: &'static str,
    upgrade_interactive: &'static str,
    execute: &'static str,
    uninstall: &'static str,
    global_uninstall: &'static str,
}

#[derive(Debug)]
pub struct AgentOpt {
    lock_file: &'static str,
    install_page: &'static str,
    commands: AgentCmd,
}

pub struct AgentExecutor {
    path: PathBuf,
    inner: &'static dyn Agent,
    opt: &'static AgentOpt,
}

impl AgentExecutor {
    pub fn install(&self) -> anyhow::Result<()> {
        exec::run(self.opt.commands.install.replace("{0}", "").trim())?;

        Ok(())
    }

    pub fn install_frozen(&self) -> anyhow::Result<()> {
        exec::run(self.opt.commands.frozen.replace("{0}", "").trim())?;

        Ok(())
    }
}

pub trait Agent {
    fn opt(&self) -> &AgentOpt;
}

pub fn find_agent() -> anyhow::Result<AgentExecutor> {
    let path = env::current_dir()?;

    if let Some(_) = match_agent(&path, &YARN_AGENT) {
        return Ok(AgentExecutor {
            path,
            inner: &YARN_AGENT,
            opt: YARN_AGENT.opt(),
        });
    } else if let Some(_) = match_agent(&path, &PNPM_AGENT) {
        return Ok(AgentExecutor {
            path,
            inner: &PNPM_AGENT,
            opt: PNPM_AGENT.opt(),
        });
    }

    anyhow::bail!("Do not care")
}

fn match_agent(path: &PathBuf, x: &'static dyn Agent) -> Option<PathBuf> {
    find_file_up(&path, x.opt().lock_file)
}
