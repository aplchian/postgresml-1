use owo_colors::OwoColorize;
use std::fs::File;
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::process::Command;

macro_rules! unwrap_or_exit {
    ($i:expr) => {
        match $i {
            Ok(v) => v,
            Err(e) => {
                error!("{}:{}:{} {e}", file!(), line!(), column!());

                std::process::exit(1);
            }
        }
    };
}

macro_rules! debug1 {
    ($e:expr) => {
        debug!("{}:{}:{} {}", file!(), line!(), column!(), $e);
    };
}

pub(crate) use debug1;
pub(crate) use unwrap_or_exit;

pub fn info(value: &str) {
    println!("{}", value.green());
}

pub fn error(value: &str) {
    println!("{}", value.red());
}

pub fn warn(value: &str) {
    println!("{}", value.yellow());
}

pub fn execute_command(command: &mut Command) -> std::io::Result<String> {
    let output = match command.output() {
        Ok(output) => output,
        Err(err) => {
            return Err(err);
        }
    };

    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let stdout = String::from_utf8_lossy(&output.stderr).to_string();

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr).to_string();
        debug!(
            "{} failed: {}",
            command.get_program().to_str().unwrap(),
            error,
        );

        return Err(std::io::Error::new(ErrorKind::Other, error));
    }

    if !stderr.is_empty() {
        warn!("{}", stderr);
    }

    if !stdout.is_empty() {
        info!("{}", stdout);
    }

    Ok(stdout)
}

pub fn write_to_file(path: &Path, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
