use std::{collections::BinaryHeap, env, path::PathBuf};

use anyhow::anyhow;
use rayon::iter::{ParallelIterator,IntoParallelRefIterator};
use serde_json::json;

use crate::{
    cli::Args,
    handler::{Handler, HandlersPath},
};

const DEFAULT_HANDLERS_ENV_KEY: &str = "BIBFETCH_HANDLERS_DIR";
const DEFAULT_PLUGINS_ENV_KEY: &str = "BIBFETCH_PLUGINS_DIR";

mod cli;
mod handler;
mod builtins;
mod plugin;

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;
    let handlers_path: String = args.handlers_path.unwrap_or( env::var(DEFAULT_HANDLERS_ENV_KEY).map_err(|_| anyhow!("The environment variable {DEFAULT_HANDLERS_ENV_KEY} is not set!"))?);
    let handlers_path = HandlersPath::try_from(handlers_path)?;
    let handlers = init_handlers(handlers_path).unwrap();


    let results: Vec<serde_json::Value> = 
        // Check for explicit mention of a specific handler
        if let Some(name) = args.handler {
        let filtered: Vec<Handler> = handlers
            .into_iter()
            .filter(|handler| handler.name == name)
            .collect();
        let handler = filtered
            .first()
            .ok_or(anyhow!(format!("Failed to get handler called {name}")))?;

        args.identifiers
            .par_iter()
            .filter_map(|id| {
                let parsed = handler.parse(id.clone()).ok()?;
                handler.fetch(parsed).ok()
            })
            .collect()
    } 
    // otherwise, try and guess
        else {
        args.identifiers
            .par_iter()
            .filter_map(|id| {
                let mut result = None;
                for handler in &handlers {
                    if let Ok(parsed) = handler.parse(id.clone()) {
                        result = handler.fetch(parsed).ok();
                        break;
                    }
                }
                result
            })
            .collect()
    };

    let json = json!(results);

    println!("{json}");

    Ok(())
}

pub fn init_handlers(path: HandlersPath) -> anyhow::Result<Vec<Handler>> {
    let path: PathBuf = path.into();
    let result = path
        .read_dir()?
        .filter_map(|x| x.ok())
        .map(|entry| Handler::try_from(entry.path()))
        .filter_map(|x| x.ok())
        .collect::<BinaryHeap<Handler>>()
        .into_sorted_vec();
    Ok(result)
}

