use super::{Agent, AgentCmd, AgentOpt};

pub struct AgentPnpm {
    opt: AgentOpt,
}

pub static PNPM_AGENT: AgentPnpm = AgentPnpm {
    opt: AgentOpt {
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
    },
};

impl Agent for AgentPnpm {
    fn opt(&self) -> &AgentOpt {
        &self.opt
    }
}
