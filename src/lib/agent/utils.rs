use crate::lib::agent::pnpm::AgentPnpm;
use crate::lib::agent::yarn::AgentYarn;
use crate::lib::agent::{Agent, AgentExecutor, AgentOpt};
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

pub fn find_file_up(starting_directory: &Path, lock_file_name: &str) -> Option<PathBuf> {
    let mut path: PathBuf = starting_directory.into();
    let pkg_file = Path::new("package.json");
    let lock_file = Path::new(lock_file_name);

    println!("find_file_up: {:?} {:?}", &path, &lock_file);

    let mut pkg_path: Option<PathBuf> = None;

    loop {
        println!("loop: {:?}", &path);
        path.push(pkg_file);

        if path.is_file() && pkg_path.is_none() {
            println!("loop:pkg: {:?}", &path);
            pkg_path = Some(path.clone());
        }
        path.pop();

        path.push(lock_file);

        if path.is_file() {
            println!("loop:lock: {:?} {:?}", &path, &pkg_path);

            break Some(path);
        }

        // remove file && remove parent
        if !(path.pop() && path.pop()) {
            break None;
        }
    }
}
