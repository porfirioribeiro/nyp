use subprocess::{Exec, ExitStatus};

pub(crate) fn run(cmd: &str) -> anyhow::Result<ExitStatus> {
    let stream = Exec::shell(cmd).popen()?.wait()?;

    if stream.success() {
        println!("Exist success ");
    } else {
        println!("Oh no");
    }

    Ok(stream)
}
