use std::process::Command;

use proc_macro2::Span;
use syn::{Error, Result};

pub fn run_command(args: Vec<&str>, path: &std::path::Path) -> Result<()> {
    execute_command(Command::new(&args[0]).args(&args[1..]).current_dir(path))?;

    Ok(())
}

fn execute_command(command: &mut Command) -> Result<std::process::ExitStatus> {
    let status = command.status().map_err(|error| {
        Error::new(
            Span::call_site(),
            format!("failed to execute command: {}", error),
        )
    })?;

    verbose_command_error(status).map_err(|message| Error::new(Span::call_site(), message))
}

fn verbose_command_error(
    status: std::process::ExitStatus,
) -> std::result::Result<std::process::ExitStatus, String> {
    if status.success() {
        Ok(status)
    } else if let Some(status) = status.code() {
        Err(format!("external command exited with status {}", status))
    } else {
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::process::ExitStatusExt;
            if let Some(signal) = status.signal() {
                Err(format!("external command killed by signal {}", signal))
            } else {
                Err(format!("external command failed, but did not exit and was not killed by a signal, this can only be a bug in std::process"))
            }
        }
        #[cfg(not(target_family = "unix"))]
        {
            Err(format!("external command killed by signal"))
        }
    }
}
