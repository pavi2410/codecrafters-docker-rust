use anyhow::{Context, Result};
use std::process::Stdio;
use std::os::unix::fs;
use tempfile::tempdir;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let tmp_dir = tempdir()?;
    fs::chroot(tmp_dir.into_path())?;
    std::env::set_current_dir("/")?;

    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    std::fs::copy(command, tmp_dir.path().join(command));

    let output = std::process::Command::new(command)
        .args(command_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;
    
    std::process::exit(output.code().unwrap_or(1));
}
