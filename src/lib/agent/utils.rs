use crate::lib::agent::pnpm::AgentPnpm;
use crate::lib::agent::yarn::AgentYarn;
use crate::lib::agent::{Agent, AgentExecutor, AgentOpt};
use crate::lib::fs_utils::find_file_up;
use dialoguer::console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use std::path::{Path, PathBuf};

pub fn prompt_agent(cur_dir: PathBuf) -> Option<AgentExecutor> {
    let selections = &["npm", "yarn", "pnpm", "bun"];

    let theme = ColorfulTheme {
        prompt_suffix: style("› - Use arrow-keys. Return to submit.".to_string())
            .for_stderr()
            .black()
            .bright(),
        ..ColorfulTheme::default()
    };

    let selection = Select::with_theme(&theme)
        .with_prompt("Choose the agent")
        .default(0)
        .items(&selections[..])
        .interact();

    match selection {
        Ok(1) => Some(AgentYarn::make(cur_dir)),
        Ok(2) => Some(AgentPnpm::make(cur_dir)),
        _ => None,
    }
}

pub fn match_agent(path: &Path, x: &AgentOpt) -> Option<PathBuf> {
    find_file_up(path, x.lock_file).and_then(|s| s.parent().map(|p| p.to_path_buf()))
}
