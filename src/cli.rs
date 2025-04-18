//!

use std::path::PathBuf;

const DEFAULT_HANDLERS_PATH: &str = "~/.config/bibfetch/handlers";

#[derive(Debug)]
/// Command-line arguments.
///
/// * `identifiers`: A comma-separated list of identifiers to fetch.
/// * `handler`: Optionally, specify which handlers to use.
/// * `handlers_path`: The path where the handlers can be found.
pub struct Args {
    identifiers: Vec<String>,
    handler: Option<String>,
    handlers_path: PathBuf,
}

impl Args {
    /// Parse the command-line arguments
    pub fn parse() -> Result<Self, lexopt::Error> {
        use lexopt::prelude::*;

        let mut identifiers = None;
        let mut handler = None;
        let mut handlers_path = DEFAULT_HANDLERS_PATH.to_string();

        // Keep on matching on the command line arguments to parse it
        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('p') | Long("handlers-path") => {
                    handlers_path = parser.value()?.parse()?;
                }

                Short('H') | Long("handler") => {
                    handler = Some(parser.value()?.parse()?);
                }

                Value(val) if identifiers.is_none() => {
                    identifiers = Some(val.string()?.split(',').map(|x| x.to_string()).collect());
                }

                _ => return Err(arg.unexpected()),
            }
        }

        let handlers_path = PathBuf::from(handlers_path);

        if !handlers_path.exists() {
            return Err("Path to handlers does not exist!".into());
        }

        Ok(Args {
            identifiers: identifiers.ok_or("Missing argument IDENTIFIERS!")?,
            handler,
            handlers_path,
        })
    }
}
