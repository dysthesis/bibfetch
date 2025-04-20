#![feature(trait_alias)]
#![feature(box_into_inner)]
use std::{collections::BinaryHeap, env, path::PathBuf};

use anyhow::anyhow;
use rayon::iter::{ParallelIterator,IntoParallelRefIterator};
use serde_json::json;

use crate::{
    cli::Args,
    handler::{CheckedPath, Handler}, plugin::Plugin,
};

const DEFAULT_HANDLERS_ENV_KEY: &str = "BIBFETCH_HANDLERS_DIR";
const DEFAULT_PLUGINS_ENV_KEY: &str = "BIBFETCH_PLUGINS_DIR";

mod cli;
mod handler;
mod builtins;
mod plugin;
mod conversion;

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;

    
    // set up plugins
    let plugins_path: String = args.plugins_path.unwrap_or(env::var(DEFAULT_PLUGINS_ENV_KEY).map_err(|_| anyhow!("The environment variable {DEFAULT_PLUGINS_ENV_KEY} is not set!"))?);
    let plugins_path = CheckedPath::try_from(plugins_path)?;
    let plugins = init_plugins(plugins_path)?;
    // set up handlers
    let handlers_path: String = args.handlers_path.unwrap_or( env::var(DEFAULT_HANDLERS_ENV_KEY).map_err(|_| anyhow!("The environment variable {DEFAULT_HANDLERS_ENV_KEY} is not set!"))?);
    let handlers_path = CheckedPath::try_from(handlers_path)?;
    let handlers = init_handlers(handlers_path, plugins).unwrap();




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

pub fn init_handlers(path: CheckedPath, plugins: Vec<Plugin>) -> anyhow::Result<Vec<Handler>> {
    let path: PathBuf = path.into();
    let result = path
        .read_dir()?
        .filter_map(|x| x.ok())
        .map(|entry| Handler::with_plugins(entry.path(), plugins.clone()))
        .filter_map(|x| x.ok())
        .collect::<BinaryHeap<Handler>>()
        .into_sorted_vec();
    Ok(result)
}

pub fn init_plugins(path: CheckedPath) -> anyhow::Result<Vec<Plugin>> {
    let path: PathBuf = path.into();
    let res = path.read_dir()?.filter_map(|entry| match entry {
        Ok(file) => Plugin::try_from(file.path()).ok(),
        // TODO: Figure out proper error handling instead of ignoring this
        Err(_) => None,
    }).collect();

    Ok(res)
}
