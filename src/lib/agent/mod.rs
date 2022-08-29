#[allow(unused)]
pub mod pnpm;
mod utils;
pub mod yarn;

use crate::lib::agent::pnpm::AgentPnpm;
use crate::lib::agent::utils::prompt_agent;
use crate::lib::agent::yarn::AgentYarn;
use crate::lib::exec;
use anyhow::{anyhow, Context};
use std::env;
use std::path::{Path, PathBuf};
use subprocess::ExitStatus;

#[allow(unused)]
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

#[allow(unused)]
#[derive(Debug)]
pub struct AgentOpt {
    lock_file: &'static str,
    install_page: &'static str,
    commands: AgentCmd,
}

#[allow(unused)]
#[derive(Debug)]
pub struct AgentExecutor {
    path: PathBuf,
    opt: &'static AgentOpt,
}

impl AgentExecutor {
    pub fn find() -> anyhow::Result<AgentExecutor> {
        let cur_dir = env::current_dir()?;

        AgentYarn::find(&cur_dir)
            .or_else(|| AgentPnpm::find(&cur_dir))
            .or_else(|| prompt_agent(cur_dir))
            .context(anyhow!("Could not find a valid agent to run the command"))
    }

    pub fn install(&self) -> anyhow::Result<ExitStatus> {
        exec::run(self.opt.commands.install.replace("{0}", "").trim())
    }

    pub fn install_frozen(&self) -> anyhow::Result<ExitStatus> {
        exec::run(self.opt.commands.frozen.replace("{0}", "").trim())
    }

    pub fn add(
        &self,
        pkg: String,
        dev: bool,
        peer: bool,
        optional: bool,
    ) -> anyhow::Result<ExitStatus> {
        let mut c: Vec<&str> = vec![];
        if dev {
            c.push("--save-optional");
        }
        if peer {
            c.push("--save-peer");
        }
        if optional {
            c.push("--save-optional");
        }

        c.push(&pkg);

        let string = c.join(" ");
        println!("cmd{}", &string);
        exec::run(self.opt.commands.add.replace("{0}", &string).trim())
    }
}

trait Agent {
    fn make(path: PathBuf) -> AgentExecutor;
    fn find(path: &Path) -> Option<AgentExecutor>;
}
