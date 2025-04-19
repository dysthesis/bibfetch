use std::{fs::read_to_string, path::PathBuf};

use mlua::{Function, Lua, Table};

#[derive(Debug)]
/// A handler for a type of identifier. This is derived from the Lua plugin for this handler.
///
/// * `name`:  The name of the handler.
/// * `lua`:   The resulting Lua instance
/// * `table`: The full returned Lua table.
/// * `parse`: The Lua function which parses the identifier.
/// * `fetch`: The Lua function which fetches the metadata for the identifier.
pub struct Handler {
    name: String,
    // We need this to persist the table and functions
    lua: Lua,
    table: Table,
    pub parse: Function,
    pub fetch: Function,
}

impl TryFrom<PathBuf> for Handler {
    type Error = anyhow::Error;
    fn try_from(path: PathBuf) -> anyhow::Result<Self> {
        // TODO: Figure out a better way to handle this other than unwrap_or_default()
        let name = path
            .to_str()
            .unwrap_or_default()
            // Get rid of extension
            .split('.')
            .nth(0)
            .unwrap_or_default()
            // Get rid of any directory
            .split('/')
            .next_back()
            .unwrap_or_default()
            .to_string();
        let lua = Lua::new();
        let table: Table = lua.load(read_to_string(path)?).eval()?;

        let parse = table.get("parse")?;
        let fetch = table.get("fetch")?;
        Ok(Handler {
            lua,
            name,
            table,
            parse,
            fetch,
        })
    }
}
