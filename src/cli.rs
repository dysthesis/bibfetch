//!

use anyhow::anyhow;
use std::path::PathBuf;

const DEFAULT_HANDLERS_PATH: &str = "~/.config/bibfetch/handlers";

#[derive(Debug)]
pub struct HandlersPath(PathBuf);

impl TryFrom<String> for HandlersPath {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let path = PathBuf::from(value);

        if !path.exists() {
            return Err(anyhow!("Path to handlers does not exist!"));
        }

        if !path.is_dir() {
            return Err(anyhow!("Path to handlers must be a directory!"));
        }

        Ok(HandlersPath(path))
    }
}

impl Into<PathBuf> for HandlersPath {
    fn into(self) -> PathBuf {
        self.0
    }
}

#[derive(Debug)]
/// Command-line arguments.
///
/// * `identifiers`: A comma-separated list of identifiers to fetch.
/// * `handler`: Optionally, specify which handlers to use.
/// * `handlers_path`: The path where the handlers can be found.
pub struct Args {
    pub identifiers: Vec<String>,
    pub handler: Option<String>,
    pub handlers_path: HandlersPath,
}

impl Args {
    /// Parse the command-line arguments.
    ///
    /// This method guarantees that `handlers_path` exists and
    pub fn parse() -> anyhow::Result<Self> {
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

                _ => return Err(arg.unexpected().into()),
            }
        }

        let handlers_path = HandlersPath::try_from(handlers_path)?;

        Ok(Args {
            identifiers: identifiers.ok_or(anyhow!("Missing argument IDENTIFIERS!"))?,
            handler,
            handlers_path,
        })
    }
}
