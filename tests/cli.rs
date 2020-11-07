#[cfg(test)]
mod cli_tests {
    use assert_cmd::Command;

    const EXECTUABLE: &str = "file_chop";

    fn get_cmd() -> Command {
        Command::cargo_bin(EXECTUABLE)
            .expect("Executable's name has changed?")
    }

    #[test]
    fn help() {
        get_cmd().assert().success();
        get_cmd().arg("-h").assert().success();
        get_cmd().arg("--help").assert().success();
    }
}
