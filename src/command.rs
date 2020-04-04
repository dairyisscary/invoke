use crate::config;
use std::process::{Command, Stdio};

const ROOT_WORKING_DIRECTORY: &str = "/opt/invoke-cwd";

#[derive(Debug)]
pub enum Error {
    MissingBinaryArgument,
    MissingCommandConfig,
    IO(std::io::Error),
}

#[derive(Debug)]
pub struct DockerRunCommand {
    command: Command,
}

impl DockerRunCommand {
    pub fn new() -> Self {
        let mut command = Command::new("docker");
        command
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .arg("run")
            .arg("--rm")
            .arg("-it");
        DockerRunCommand { command }
    }

    pub fn run_with_configuration(
        self,
        config_hierarchy: &config::ConfigHierarchy,
        mut arguments: impl Iterator<Item = String>,
    ) -> Result<i32, Error> {
        let mut command = self.command;
        let command_name = arguments
            .next()
            .ok_or_else(|| Error::MissingBinaryArgument)?;
        let config = config_hierarchy
            .get_command_config(&command_name)
            .ok_or_else(|| Error::MissingCommandConfig)?;

        command
            .arg("-w")
            .arg(config.working_directory_with_root(ROOT_WORKING_DIRECTORY))
            .arg("-v")
            .arg(format!(
                "{}:{}",
                config.root_path_argument(),
                ROOT_WORKING_DIRECTORY
            ))
            .arg(config.image_argument())
            .arg(command_name);
        for arg in arguments {
            command.arg(arg);
        }

        command
            .status()
            .map(|status| status.code().unwrap_or(0))
            .map_err(Error::IO)
    }
}
