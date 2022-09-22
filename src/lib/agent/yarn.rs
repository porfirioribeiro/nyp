use super::{utils::PkgDirectories, Agent, AgentCmd, AgentOpt};
use crate::lib::agent::utils::match_agent;
use crate::lib::agent::AgentExecutor;
use std::path::{Path, PathBuf};

pub struct AgentYarn;

pub static YARN_AGENT_OPT: AgentOpt = AgentOpt {
    lock_file: "yarn.lock",
    install_page: "https://classic.yarnpkg.com/en/docs/install",
    commands: AgentCmd {
        agent: "yarn {0}",
        run: "yarn run {0}",
        install: "yarn install {0}",
        frozen: "yarn install --frozen-lockfile",
        global: "yarn global add {0}",
        add: "yarn add {0}",
        upgrade: "yarn upgrade {0}",
        upgrade_interactive: "yarn upgrade-interactive {0}",
        execute: "yarn dlx {0}",
        uninstall: "yarn remove {0}",
        global_uninstall: "yarn global remove {0}",
    },
};

impl Agent for AgentYarn {
    fn make(path: PkgDirectories) -> AgentExecutor {
        AgentExecutor {
            path: path.lock,
            opt: &YARN_AGENT_OPT,
        }
    }

    fn find(path: &Path) -> Option<AgentExecutor> {
        match_agent(path, &YARN_AGENT_OPT).map(Self::make)
    }
}
