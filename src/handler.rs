use std::{fs::read_to_string, path::PathBuf};

use anyhow::anyhow;
use mlua::{Function, Lua, Table};

#[derive(Debug)]
/// A handler for a type of identifier. This is derived from the Lua plugin for this handler.
///
/// * `lua`:   The resulting Lua instance
/// * `table`: The full returned Lua table.
/// * `name`:  The name of the handler.
/// * `priority`: This defines the ordering of handlers; the lower the number, the sooner it is
/// tried. For now, this is defined in the plugin itself; however, this could eventually be
/// overridable in a config if one is ever implemented.
/// * `parse`: The Lua function which parses the identifier.
/// * `fetch`: The Lua function which fetches the metadata for the identifier.
pub struct Handler {
    // We need this to persist the table and functions
    lua: Lua,
    table: Table,
    pub name: String,
    pub priority: u8,
    pub parse: Function,
    pub fetch: Function,
}

impl Ord for Handler {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Handler {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Handler {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Handler {}

impl TryFrom<PathBuf> for Handler {
    type Error = anyhow::Error;
    fn try_from(path: PathBuf) -> anyhow::Result<Self> {
        // TODO: Figure out a better way to handle this other than unwrap_or_default()
        let lua = Lua::new();
        let fetch = lua.create_function(move |lua, url: String| {
            let resp = ureq::get(&url)
                .header("Accept", "application/json")
                .call()
                .map_err(|e| anyhow!(e))?
                .body_mut()
                .read_to_string()
                .map_err(|e| anyhow!(e))?;

            // Parse JSON into serde_json::Value
            let json: serde_json::Value = serde_json::from_str(&resp).map_err(|e| anyhow!(e))?;

            // // Convert JSON object into a Lua table
            // let table = lua.create_table()?;
            // if let serde_json::Value::Object(map) = json {
            //     for (k, v) in map {
            //         let val = match v {
            //             serde_json::Value::String(s) => mlua::Value::String(lua.create_string(&s)?),
            //             serde_json::Value::Number(n) => {
            //                 mlua::Value::Number(n.as_f64().unwrap_or_default())
            //             }
            //             serde_json::Value::Bool(b) => mlua::Value::Boolean(b),
            //             _ => mlua::Value::Nil,
            //         };
            //         table.set(k, val)?;
            //     }
            // }
            let target = json.get("message").unwrap_or(&json);

            fn to_lua(lua: &Lua, v: &serde_json::Value) -> Result<mlua::Value, mlua::Error> {
                match v {
                    serde_json::Value::Null => Ok(mlua::Value::Nil),
                    serde_json::Value::Bool(b) => Ok(mlua::Value::Boolean(*b)),
                    serde_json::Value::Number(n) => {
                        Ok(mlua::Value::Number(n.as_f64().unwrap_or_default()))
                    }
                    serde_json::Value::String(s) => Ok(mlua::Value::String(lua.create_string(s)?)),
                    serde_json::Value::Array(arr) => {
                        let tbl = lua.create_table()?;
                        for (i, item) in arr.iter().enumerate() {
                            tbl.set(i + 1, to_lua(lua, item)?)?;
                        }
                        Ok(mlua::Value::Table(tbl))
                    }
                    serde_json::Value::Object(map) => {
                        let tbl = lua.create_table()?;
                        for (k, item) in map {
                            tbl.set(k.as_str(), to_lua(lua, item)?)?;
                        }
                        Ok(mlua::Value::Table(tbl))
                    }
                }
            }
            to_lua(lua, target)
        })?;
        lua.globals().set("fetch", fetch)?;
        let table: Table = lua.load(read_to_string(path)?).eval()?;

        let parse = table.get("parse")?;
        let fetch = table.get("fetch")?;
        let info: Table = table.get("info")?;
        let name = info.get("name")?;
        let priority = info.get("priority")?;
        Ok(dbg!(Handler {
            lua,
            name,
            table,
            parse,
            fetch,
            priority,
        }))
    }
}
