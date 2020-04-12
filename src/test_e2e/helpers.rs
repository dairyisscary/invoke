use assert_fs::prelude::*;

const CONFIG_FILE_NAME: &str = ".invoke.toml";

const BASIC_CONFIG_TOML: &str = r#"[ls]
image = "alpine:3.11"

[pwd]
image = "alpine:3.11"
"#;

pub struct InvokeHarness {
    command: assert_cmd::Command,
    temp_directory: assert_fs::TempDir,
}

impl InvokeHarness {
    fn new() -> Self {
        let mut command = assert_cmd::Command::cargo_bin("invoke").unwrap();
        let temp_directory = assert_fs::TempDir::new().unwrap();
        command
            .env("INVOKE_TEST", "true")
            .current_dir(temp_directory.path())
            // Add a long timeout so that no matter what the tests will finish.
            .timeout(std::time::Duration::from_secs(10));
        Self {
            command,
            temp_directory,
        }
    }

    pub fn with_no_config() -> Self {
        Self::new()
    }

    pub fn with_basic_config() -> Self {
        let harness = Self::new();
        harness
            .temp_directory
            .child(CONFIG_FILE_NAME)
            .write_str(BASIC_CONFIG_TOML)
            .unwrap();
        harness
    }

    pub fn command(&mut self) -> &mut assert_cmd::Command {
        &mut self.command
    }

    pub fn change_working_directory(
        &mut self,
        sub_directory: impl AsRef<std::path::Path>,
    ) -> &mut Self {
        let child = self.temp_directory.child(sub_directory);
        self.command.current_dir(child.path());
        self
    }

    pub fn add_sub_directory(&mut self, sub_directory: impl AsRef<std::path::Path>) -> &mut Self {
        self.temp_directory
            .child(sub_directory)
            .create_dir_all()
            .unwrap();
        self
    }
}
