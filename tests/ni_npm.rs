use assert_cmd::Command;
use std::path::PathBuf;

#[test]
fn install_pnpm() {
    let mut cmd = get_exe();
    cmd.run_cmd_at("pnpm", "ni").assert()
        .success()
        .stdout("Lockfile is up-to-date, resolution step is skipped\nAlready up-to-date\n\nExist success \n")
        .stderr("");
}

#[test]
fn install_yarn() {
    let mut cmd = get_exe();
    cmd.run_cmd_at("yarn", "ni")
        .assert()
        .success()
        .stdout(predicates::str::starts_with("yarn install v"))
        .stderr(predicates::str::is_empty());
}

fn get_exe() -> Command {
    Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
}

fn fixture_dir(fixture_path: &str) -> PathBuf {
    std::env::current_dir()
        .unwrap()
        .join("tests")
        .join("fixtures")
        .join(fixture_path)
}

trait CommandEx {
    fn run_cmd_at(&mut self, fixture_path: &str, subcommand: &str) -> &mut Command;
}

impl CommandEx for Command {
    fn run_cmd_at(&mut self, fixture_path: &str, subcommand: &str) -> &mut Command {
        self.current_dir(fixture_dir(fixture_path)).arg(subcommand)
    }
}
