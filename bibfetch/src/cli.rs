//!

use anyhow::anyhow;

#[derive(Debug)]
/// Command-line arguments.
///
/// * `identifiers`: A comma-separated list of identifiers to fetch.
/// * `handler`: Optionally, specify which handlers to use.
/// * `handlers_path`: The path where the handlers can be found.
pub struct Args {
    pub identifiers: Vec<String>,
    pub handler: Option<String>,
    pub handlers_path: Option<String>,
    pub plugins_path: Option<String>,
}

impl Args {
    /// Parse the command-line arguments.
    ///
    /// This method guarantees that `handlers_path` exists and
    pub fn parse() -> anyhow::Result<Self> {
        use lexopt::prelude::*;

        let mut identifiers = None;
        let mut handler = None;
        let mut handlers_path = None;
        let mut plugins_path = None;

        // Keep on matching on the command line arguments to parse it
        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('p') | Long("handlers-path") => {
                    handlers_path = Some(parser.value()?.parse::<String>()?);
                }

                Short('H') | Long("handler") => {
                    handler = Some(parser.value()?.parse()?);
                }

                Short('P') | Long("plugins-path") => {
                    plugins_path = Some(parser.value()?.parse::<String>()?);
                }

                Value(val) if identifiers.is_none() => {
                    identifiers = Some(val.string()?.split(',').map(|x| x.to_string()).collect());
                }

                _ => return Err(arg.unexpected().into()),
            }
        }

        Ok(Args {
            identifiers: identifiers.ok_or(anyhow!("Missing argument IDENTIFIERS!"))?,
            handler,
            handlers_path,
            plugins_path,
        })
    }
}
