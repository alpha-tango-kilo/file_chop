extern crate clap;
use clap::{
    Arg,
    App,
    //load_yaml,
};

fn main() {
    //let yaml = load_yaml!("./args/en-gb.yml");
    //let matches = App::from(yaml).get_matches();
    //println!("{:#?}", matches);

    let matches = App::new("file_chop")
        .arg(Arg::with_name("CHOP")
            .short("C")
            .conflicts_with("PLOP"))
        .arg(Arg::with_name("PLOP")
            .short("P")
            .conflicts_with("CHOP"))
        .arg(Arg::with_name("number")
            .short("n")
            .long("number")
            .takes_value(true)
            .conflicts_with_all(&["PLOP", "size"])
            .requires("CHOP")
            .validator(|s| {
                match s.parse::<usize>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Failed to parse unsigned integer for number of parts to chop file into".into()),
                }
            }))
        .arg(Arg::with_name("file")
            .required(true)
            .min_values(1))
        .get_matches();

    println!("{:#?}", matches);
    println!("{:?}", matches.values_of("file").unwrap());
}
