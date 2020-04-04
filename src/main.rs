use std::convert::TryFrom;

mod command;
mod config;

#[derive(Debug)]
enum Error {
    IO(std::io::Error),
    Config(config::Error),
    Command(command::Error),
}

fn main() -> Result<(), Error> {
    let cwd = std::env::current_dir().map_err(Error::IO)?;
    let config_hierarchy =
        config::ConfigHierarchy::try_from(cwd.as_ref()).map_err(Error::Config)?;
    let command = command::DockerRunCommand::new();
    command
        .run_with_configuration(&config_hierarchy, std::env::args().skip(1))
        .map(|code| std::process::exit(code))
        .map_err(Error::Command)
}
