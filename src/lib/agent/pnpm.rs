use super::{utils::PkgDirectories, Agent, AgentCmd, AgentOpt};
use crate::lib::agent::utils::match_agent;
use crate::lib::agent::AgentExecutor;
use std::path::{Path, PathBuf};

pub struct AgentPnpm;

#[allow(dead_code)]

static PNPM_AGENT_OPT: AgentOpt = AgentOpt {
    lock_file: "pnpm-lock.yaml",
    install_page: "https://pnpm.js.org/en/installation",
    commands: AgentCmd {
        agent: "pnpm {0}",
        run: "pnpm run {0}",
        install: "pnpm i {0}",
        frozen: "pnpm i --frozen-lockfile",
        global: "pnpm add -g {0}",
        add: "pnpm add {0}",
        upgrade: "pnpm update {0}",
        upgrade_interactive: "pnpm update -i {0}",
        execute: "pnpm dlx {0}",
        uninstall: "pnpm remove {0}",
        global_uninstall: "pnpm remove --global {0}",
    },
};

impl Agent for AgentPnpm {
    fn make(path: PkgDirectories) -> AgentExecutor {
        AgentExecutor {
            path: path.lock,
            opt: &PNPM_AGENT_OPT,
        }
    }

    fn find(path: &Path) -> Option<AgentExecutor> {
        match_agent(path, &PNPM_AGENT_OPT).map(Self::make)
    }
}
