use std::env;
use std::error::Error;
use std::fs;

// a struct that defines the needed arguments
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}
impl Config {
    //Passes the arguments
    pub fn new(mut arguments: env::Args) -> Result<Config, &'static str> {
        arguments.next();

        let query = match arguments.next() {
            Some(argument) => argument,
            None => return Err("Please Provide the query."),
        };
        let file_name = match arguments.next() {
            Some(argument) => argument,
            None => return Err("Please Provide the file name."),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

/*reads the output of the file (config.file_name),
decide to call the find_case_insensitive or find function
,and prints the line with the query.
*/
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.file_name)?;

    let sensitive_or_not = if config.case_sensitive {
        find(&config.query, &file_content)
    } else {
        find_case_insensitive(&config.query, &file_content)
    };

    if !sensitive_or_not.is_empty() {
        for line in sensitive_or_not {
            println!("The line that contains your query: \n{}", line);
        }
    } else {
        print!("Couldn't find: {}", config.query);
        print!("in file: {}", config.file_name);
    }

    Ok(())
}

//Iterate through every line and check if the query is there. Return the lines with query.
pub fn find<'a>(query: &'a str, file_content: &'a str) -> Vec<&'a str> {
    file_content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

//Same as the find function but case insensitive.
pub fn find_case_insensitive<'a>(query: &'a str, file_content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in file_content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //Tests the find function
    //if it will return the line that includes the exact same query.
    fn case_sensitive() {
        let query = "Love Is A Dagger.";
        let file_content = "Love Is A Dagger.
                          It's A Weapon To Be Wielded Far Away Or Up Close. 
                          You Can See Yourself In It. It's Beautiful. Until It Makes You Bleed.";

        assert_eq!(vec!["Love Is A Dagger."], find(query, file_content));
    }

    #[test]
    //Tests the find_case_insensitive function
    //if it will return the line that includes the "same" query.
    fn case_insensitive() {
        let query = "Love Is A DaGGer";
        let file_content = "Love Is A Dagger.
                          It's A Weapon To Be Wielded Far Away Or Up Close. 
                          You Can See Yourself In It. It's Beautiful. Until It Makes You Bleed.";

        assert_eq!(
            vec!["Love Is A Dagger."],
            find_case_insensitive(query, file_content)
        );
    }
}
