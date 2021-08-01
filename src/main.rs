use std::env;
use std::process;

use grep_clone::Config;

fn main() {
    //Passing the arguments to the Config constructor (Config::new) .
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(0);
    });

    println!("\nTrying to find: {:?}", config.query);
    println!("in file: {:?}\n", config.file_name);

    //Calling grep_clone::run function and handling the errors.
    if let Err(err) = grep_clone::run(config) {
        eprintln!("An Error ocurred: {}", err.to_string());
        process::exit(0);
    }
}
