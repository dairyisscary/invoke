use serde::Deserialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::{Component, Display, Path, PathBuf};

fn path_diff<'a, 'b>(
    mut larger_path: impl Iterator<Item = Component<'a>> + Clone,
    mut smaller_path: impl Iterator<Item = Component<'b>>,
) -> PathBuf {
    loop {
        let preserved_iter = larger_path.clone();
        match (larger_path.next(), smaller_path.next()) {
            (Some(x), Some(y)) if x == y => (),
            (Some(_), Some(_)) | (None, Some(_)) => {
                // In theory they should never be different and larger should never be smalller...
                panic!("This is a bug in invoke. There is a problem with relative directories.");
            }
            (Some(_), None) | (None, None) => return preserved_iter.collect(),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Deserialize(toml::de::Error),
    NotFound,
}

#[derive(Debug, Deserialize)]
struct UserCommandConfig {
    image: String,
}

#[derive(Debug)]
struct ConfigFile<'a> {
    path: &'a Path,
    relative_dir: PathBuf,
    config: HashMap<String, UserCommandConfig>,
}

#[derive(Debug)]
pub struct CommandConfig<'a> {
    user_config: &'a UserCommandConfig,
    config_file: &'a ConfigFile<'a>,
}

impl<'a> CommandConfig<'a> {
    pub fn root_path_argument(&self) -> Display {
        self.config_file.path.display()
    }

    pub fn image_argument(&self) -> &str {
        &self.user_config.image
    }

    pub fn working_directory_with_root(&self, root: &str) -> String {
        Path::new(root)
            .join(&self.config_file.relative_dir)
            .display()
            .to_string()
    }
}

#[derive(Debug)]
pub struct ConfigHierarchy<'a> {
    config_files: Vec<ConfigFile<'a>>,
}

impl<'a> ConfigHierarchy<'a> {
    pub fn get_command_config(&self, command_name: &str) -> Option<CommandConfig> {
        self.config_files.iter().find_map(|file| {
            file.config
                .get(command_name)
                .map(|user_config| CommandConfig {
                    user_config,
                    config_file: file,
                })
        })
    }
}

impl<'a> TryFrom<&'a Path> for ConfigHierarchy<'a> {
    type Error = Error;

    fn try_from(start_path: &'a Path) -> Result<Self, Self::Error> {
        start_path
            .ancestors()
            .filter_map(|path| {
                let file_path = path.join(Path::new("./.invoke.toml"));
                std::fs::read(&file_path)
                    .map(Some)
                    .or_else(|err| match err.kind() {
                        std::io::ErrorKind::NotFound => Ok(None),
                        _ => Err(Error::IO(err)),
                    })
                    .transpose()
                    .map(|file_bytes_result| {
                        file_bytes_result.and_then(|file_bytes| {
                            toml::from_slice(&file_bytes)
                                .map(|config| {
                                    let relative_dir =
                                        path_diff(start_path.components(), path.components());
                                    ConfigFile {
                                        relative_dir,
                                        path,
                                        config,
                                    }
                                })
                                .map_err(Error::Deserialize)
                        })
                    })
            })
            .collect::<Result<Vec<ConfigFile>, Self::Error>>()
            .and_then(|config_files| {
                if config_files.is_empty() {
                    Err(Error::NotFound)
                } else {
                    Ok(Self { config_files })
                }
            })
    }
}
