use crate::agent::pnpm::AgentPnpm;
use crate::agent::yarn::AgentYarn;
use crate::agent::{Agent, AgentExecutor, AgentOpt};
use crate::exit::Expected;
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

    let path = PkgDirectories {
        lock: cur_dir.clone(),
        pkg: cur_dir,
    };

    match selection {
        Ok(1) => Some(AgentYarn::make(path)),
        Ok(2) => Some(AgentPnpm::make(path)),
        _ => None,
    }
}

pub struct PkgDirectories {
    pub lock: PathBuf,
    pub pkg: PathBuf,
}

pub fn match_agent(starting_directory: &Path, x: &AgentOpt) -> Option<PkgDirectories> {
    let mut path: PathBuf = starting_directory.into();
    let pkg_file = Path::new("package.json");
    let lock_file = Path::new(x.lock_file);

    // println!("find_file_up: {:?} {:?}", &path, &lock_file);

    let mut pkg_path: Option<PathBuf> = None;

    loop {
        // println!("loop: {:?}", &path);
        path.push(pkg_file);

        if path.is_file() && pkg_path.is_none() {
            // println!("loop:pkg: {:?}", &path);
            pkg_path = Some(path.clone());
        }
        path.pop();

        path.push(lock_file);

        if path.is_file() {
            // println!("loop:lock: {:?} {:?}", &path, &pkg_path);

            let pkg_dir = pkg_path.or_die("found lock file without package.json");

            break Some(PkgDirectories {
                lock: path.parent()?.to_path_buf(),
                pkg: pkg_dir.parent()?.to_path_buf(),
            });
        }

        // remove file && remove parent
        if !(path.pop() && path.pop()) {
            break None;
        }
    }
}
