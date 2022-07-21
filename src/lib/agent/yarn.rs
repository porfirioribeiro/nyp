use super::{Agent, AgentCmd, AgentOpt};

pub struct AgentYarn {
    opt: AgentOpt,
}

pub static YARN_AGENT: AgentYarn = AgentYarn {
    opt: AgentOpt {
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
    },
};

impl Agent for AgentYarn {
    fn opt(&self) -> &AgentOpt {
        &self.opt
    }
}
