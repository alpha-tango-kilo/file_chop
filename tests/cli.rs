#[cfg(test)]
mod cli_tests {
    use assert_cmd::Command;
    use tempfile::{Builder, NamedTempFile};
    use std::io::Write;

    const EXECTUABLE: &str = "file_chop";

    fn get_bin() -> Command {
        Command::cargo_bin(EXECTUABLE)
            .expect("Executable's name has changed?")
    }

    fn get_small_file() -> (NamedTempFile, String) {
        let mut file = Builder::new()
            .prefix("file_chop_tempfile_")
            .suffix(".txt")
            .tempfile()
            .expect("Failed to create tempfile");
        file.write_all("Some text that'll give the file enough size for testing".as_bytes())
            .expect("Failed to write test data to tempfile");
        let name = String::from(file.path().to_str().unwrap());
        (file, name)
    }

    #[test]
    fn help() {
        get_bin().assert().failure();
        get_bin().arg("-h").assert().success();
        get_bin().arg("--help").assert().success();
    }

    #[test]
    fn argument_formation() {
        get_bin().arg("-C").assert().failure();

        get_bin().arg("-Cn").assert().failure();
        get_bin().args(&["-C", "-n"]).assert().failure();
        get_bin().args(&["-C", "--number"]).assert().failure();

        get_bin().args(&["-Cn", "3"]).assert().failure();
        get_bin().args(&["-C", "-n", "3"]).assert().failure();
        get_bin().args(&["-C", "--number", "3"]).assert().failure();

        let (_file_1, name_1) = get_small_file();
        let (_file_2, name_2) = get_small_file();

        // Test one file
        // Bad args
        get_bin().args(&["-Cn", &name_1]).assert().failure();
        get_bin().args(&["-C", "-n", &name_1]).assert().failure();
        get_bin().args(&["-C", "--number", &name_1]).assert().failure();

        // Good args
        get_bin().args(&["-Cn", "3", &name_1]).assert().success();
        get_bin().args(&["-C", "-n", "3", &name_1]).assert().success();
        get_bin().args(&["-C", "--number", "3", &name_1]).assert().success();

        // Test two files
        // Bad args
        get_bin().args(&["-Cn", &name_1, &name_2]).assert().failure();
        get_bin().args(&["-C", "-n", &name_1, &name_2]).assert().failure();
        get_bin().args(&["-C", "--number", &name_1, &name_2]).assert().failure();

        // Good args
        get_bin().args(&["-Cn", "3", &name_1, &name_2]).assert().success();
        get_bin().args(&["-C", "-n", "3", &name_1, &name_2]).assert().success();
        get_bin().args(&["-C", "--number", "3", &name_1, &name_2]).assert().success();
    }

    #[test]
    fn files_validation() {
        let (_file, name) = get_small_file();
        get_bin().args(&["-Cn", "2", &name]).assert().success();
        get_bin().args(&["-Cn", "2", "\"%#<>\""]).assert().failure();
    }

    #[test]
    fn number_validation() {
        let (_file, name) = get_small_file();
        for n in 0usize..10 {
            let result = get_bin().args(&["-Cn", &format!("{}", n), &name]).assert();
            if n < 2 {
                result.failure();
            } else {
                result.success();
            }
        }
    }
}
