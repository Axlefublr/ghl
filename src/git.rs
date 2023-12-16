use std::error::Error;
use std::process::Command;

fn sh(command: &mut Command) -> Result<String, Box<dyn Error>> {
    let output = command.output()?;
    if !output.status.success() {
        Err(String::from_utf8(output.stderr)?.trim())?
    } else {
        Ok(String::from_utf8(output.stdout)?.trim().to_owned())
    }
}

pub fn branch() -> Result<String, Box<dyn Error>> {
    sh(Command::new("git").arg("branch").arg("--show-current"))
}
