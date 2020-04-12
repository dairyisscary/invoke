mod helpers;

#[test]
fn with_no_config() {
    helpers::InvokeHarness::with_no_config()
        .command()
        .arg("noconfig")
        .assert()
        .stderr("Error: Config(NotFound)\n")
        .failure();
}

#[test]
fn without_config_for_command() {
    helpers::InvokeHarness::with_basic_config()
        .command()
        .arg("notfound")
        .assert()
        .stderr("Error: Command(MissingCommandConfig)\n")
        .failure();
}

#[test]
fn print_working_directory() {
    helpers::InvokeHarness::with_basic_config()
        .command()
        .arg("pwd")
        .assert()
        .stdout("/opt/invoke-cwd\n")
        .success();
}

#[test]
fn changing_directory() {
    let mut harness = helpers::InvokeHarness::with_basic_config();
    harness
        .add_sub_directory("inner")
        .change_working_directory("inner")
        .command()
        .arg("ls")
        .arg("-a")
        .arg(".")
        .arg("..")
        .assert()
        .stdout(
            ".:
.
..

..:
.
..
.invoke.toml
inner
",
        )
        .success();
}

#[test]
fn with_arguments() {
    let mut harness = helpers::InvokeHarness::with_basic_config();
    harness
        .command()
        .arg("ls")
        .arg("-a")
        .assert()
        .stdout(
            ".
..
.invoke.toml
",
        )
        .success();
}
