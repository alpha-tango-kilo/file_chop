#[cfg(test)]
mod cli_tests {
    use assert_cmd::Command;

    const EXECTUABLE: &str = "file_chop";

    #[test]
    fn help() {
        let mut cmd = Command::cargo_bin(EXECTUABLE)
            .expect("Executable's name has changed?");
        cmd.arg("-h").assert().success();
    }
}
