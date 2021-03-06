use std::process::Command;

use log::info;

pub mod build;
pub mod config;
pub mod generate;
pub mod publish;
pub mod whoami;

pub use self::config::config;
pub use build::build;
pub use generate::generate;
pub use publish::preview::preview;
pub use publish::preview::HTTPMethod;
pub use publish::publish;
pub use whoami::whoami;

/// Run the given command and return its stdout.
pub fn run(mut command: Command, command_name: &str) -> Result<(), failure::Error> {
    info!("Running {:?}", command);

    let status = command.status()?;

    if status.success() {
        Ok(())
    } else {
        failure::bail!(
            "failed to execute `{}`: exited with {}",
            command_name,
            status
        )
    }
}
