extern crate clap;
extern crate stl_io;

use std::error::Error;
use std::fs::File;
use clap::{Arg, App};

pub struct Config {
    pub stl_filename: String,
    pub img_filename: String,
}

impl Config {
    pub fn new() -> Config {
        // Define command line arguments
        let matches = App::new(env!("CARGO_PKG_NAME"))
                              .version(env!("CARGO_PKG_VERSION"))
                              .author(env!("CARGO_PKG_AUTHORS"))
                              .arg(Arg::with_name("STL_FILE")
                                       .help("STL file")
                                       .required(true)
                                       .index(1))
                              .arg(Arg::with_name("IMG_FILE")
                                       .help("Thumbnail image")
                                       .required(true)
                                       .index(2))
                              .get_matches();

        let stl_filename = matches.value_of("STL_FILE").unwrap().to_string();
        let img_filename = matches.value_of("IMG_FILE").unwrap().to_string();

        Config {stl_filename, img_filename}
    }
}

pub fn run(config: &Config) -> Result<(), Box<Error>> {
    let mut stl_file = File::open(&config.stl_filename)?;
    let stl = stl_io::read_stl(&mut stl_file)?;

    println!("{:?}", stl);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::ErrorKind;
    use super::*;

    #[test]
    fn cube() {
        let config = Config {
            stl_filename: "cube.stl".to_string(),
            img_filename: "cube.png".to_string()
        };

        match fs::remove_file(&config.img_filename) {
            Ok(_) => (),
            Err(ref error) if error.kind() == ErrorKind::NotFound => (),
            Err(_) => {
                panic!("Couldn't clean files before testing");
            }
        }

        run(&config)
            .expect("Error in run function");

        let size = fs::metadata(config.img_filename)
            .expect("No file created")
            .len();

        assert_ne!(0, size);
    }
}
