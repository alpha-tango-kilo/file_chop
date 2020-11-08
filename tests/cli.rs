#[cfg(test)]
mod cli_tests {
    use assert_cmd::Command;
    use tempfile::Builder;
    use std::io::Write;

    const EXECTUABLE: &str = "file_chop";

    fn get_cmd() -> Command {
        Command::cargo_bin(EXECTUABLE)
            .expect("Executable's name has changed?")
    }

    #[test]
    fn help() {
        get_cmd().assert().failure();
        get_cmd().arg("-h").assert().success();
        get_cmd().arg("--help").assert().success();
    }

    #[test]
    fn argument_formation() {
        get_cmd().arg("-C").assert().failure();

        get_cmd().arg("-Cn").assert().failure();
        get_cmd().args(&["-C", "-n"]).assert().failure();
        get_cmd().args(&["-C", "--number"]).assert().failure();

        get_cmd().args(&["-Cn", "3"]).assert().failure();
        get_cmd().args(&["-C", "-n", "3"]).assert().failure();
        get_cmd().args(&["-C", "--number", "3"]).assert().failure();

        let mut file = Builder::new()
            .prefix("argument_formation_test_")
            .suffix(".txt")
            .tempfile()
            .expect("Failed to create tempfile");
        file.write_all("Some text that'll give the file enough size for testing".as_bytes())
            .expect("Failed to write test data to tempfile");

        let name = file.path().to_str().unwrap();

        println!("{:?}", name);

        get_cmd().args(&["-Cn", name]).assert().failure();
        get_cmd().args(&["-C", "-n", name]).assert().failure();
        get_cmd().args(&["-C", "--number", name]).assert().failure();

        get_cmd().args(&["-Cn", "3", name]).assert().success();
        get_cmd().args(&["-C", "-n", "3", name]).assert().success();
        get_cmd().args(&["-C", "--number", "3", name]).assert().success();
    }
}
