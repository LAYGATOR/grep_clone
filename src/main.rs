use std::env;
use colored::Colorize;
use std::process;

use grep_clone::Config;

fn main() 
{
    //Taking CLI arguments.
    let arguments: Vec<String> = env::args().collect();

    //Passing the arguments to the Config constructor (Config::new) .
    let config = Config::new(&arguments).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err.red());
        process::exit(0);
    });

    
    println!("Trying to find: {:?}", config.query);
    println!("in file: {:?}\n", config.file_name);

    //Calling grep_clone::run function and handling the errors.
    match grep_clone::run(config)
    {
        Err(err) => {eprintln!("An Error ocurred: {}", err.to_string().red()); process::exit(0);},
        Ok(_) => (),
    }

}
