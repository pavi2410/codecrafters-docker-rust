use anyhow::{Context, Result};
use std::process::Stdio;
use std::os::unix::fs;
use tempfile::tempdir;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let tmp_dir = tempdir()?;

    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    let to = tmp_dir.path().join(command.strip_prefix("/").unwrap_or(command));
    std::fs::create_dir_all(to.parent().unwrap())?;
    std::fs::copy(command, to)?;

    let dev_null = tmp_dir.path().join("dev/null");
    std::fs::create_dir_all(dev_null.parent().unwrap())?;
    std::fs::File::create(dev_null)?;

    fs::chroot(tmp_dir.path())?;
    // std::env::set_current_dir("/")?;

    // Print all available files in the chroot directory
    for entry in std::fs::read_dir("/")? {
        let entry = entry?;
        println!("{}", entry.path().display());
    }

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
