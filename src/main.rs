use getopt::prelude::*;

use std::{
    error::Error,
    fs,
};

fn main() -> Result<(), Box<dyn Error>> {
    let _config = Config::new(std::env::args().collect())?;
    Ok(())
}

struct Config {
    no_parts: usize,
    files: Vec<String>,
}

impl Config {
    fn new(mut args: Vec<String>) -> Result<Config, Box<dyn Error>> {
        let mut opts = Parser::new(&args, "p:");

        let mut no_parts: usize = 10; // default value
        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('p', Some(n)) => no_parts = n.parse()?,
                    _ => unimplemented!("Print docs"),
                }
            }
        }

        let files = Config::remove_invalid_paths(args.split_off(opts.index()));

        println!("Splitting each of the following files into {} parts:\n{:#?}", no_parts, files);

        if files.len() == 0 {
            Err("No valid file arguments given".into())
        } else {
            Ok(Config {
                files,
                no_parts,
            })
        }
    }

    fn remove_invalid_paths(file_list: Vec<String>) -> Vec<String> {
        file_list.into_iter()
            .filter(|p| {
                // Check if path is a directory, if not, drop it
                fs::metadata(p).map(|md| { md.is_file() }).unwrap_or(false)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // There will be tests - promise!
}
