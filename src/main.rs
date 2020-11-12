extern crate clap;
use clap::{
    Arg,
    App,
    //load_yaml,
};
extern crate rayon;
use rayon::prelude::*;
use std::fs;
use file_chop::chop;

fn main() {
    //let yaml = load_yaml!("./args/en-gb.yml");
    //let matches = App::from(yaml).get_matches();
    //println!("{:#?}", matches);

    let matches = App::new("file_chop")
        .arg(Arg::with_name("CHOP")
            .short("C")
            .conflicts_with("PLOP")
            .required_unless("PLOP"))
        .arg(Arg::with_name("PLOP")
            .short("P")
            .conflicts_with("CHOP")
            .required_unless("CHOP"))
        .arg(Arg::with_name("number")
            .short("n")
            .long("number")
            .takes_value(true)
            .conflicts_with_all(&["PLOP", "size"])
            .requires("CHOP")
            .validator(|s| {
                match s.parse::<usize>() {
                    Ok(n) => if n > 1 { Ok(()) } else { Err("Number should be greater than 1".into()) },
                    Err(_) => Err("Failed to parse unsigned integer for number of parts to chop file into".into()),
                }
            }))
        .arg(Arg::with_name("files")
            .required(true)
            .min_values(1)
            .validator(|s| {
                match fs::metadata(&s) {
                    Ok(md) => if md.is_file() { Ok(()) } else { Err(format!("Expected {} to be a file", &s))},
                    Err(e) => Err(e.to_string()),
                }
            }))
        .get_matches();

    let files = matches.values_of("files").unwrap();

    if matches.is_present("CHOP") {
        files.par_bridge().map(|path| -> (&str, u64) {
            let size = fs::metadata(path)
                .unwrap() // Checked already by argument checker
                .len();
            if let Some(n) = matches.value_of("number") {
                let divisor = n.parse::<u64>().unwrap(); // Checked already by argument checker
                (path, size / divisor)
            } else {
                panic!("Incomplete arguments for chopping found - this was supposed to have already been checked for");
            }
        }).for_each(|(path, slice_size)| { chop(path, slice_size) });
    } else if matches.is_present("PLOP") {
        // TODO :)
    } else {
        panic!("Program was called without an operation - this was supposed to have already been checked for");
    }
}
