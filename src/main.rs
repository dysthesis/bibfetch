use std::{collections::BinaryHeap, path::PathBuf};

use anyhow::anyhow;
use mlua::Table;
use serde_json::json;

use crate::{
    cli::{Args, HandlersPath},
    handler::Handler,
};

mod cli;
mod handler;

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;
    let handlers = init_handlers(args.handlers_path).unwrap();

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
            .iter()
            .filter_map(|id| {
                let parsed = handler.parse(id.clone()).ok()?;
                handler.fetch(parsed).ok()
            })
            .collect()
    } 
    // otherwise, try and guess
        else {
        args.identifiers
            .iter()
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

    println!("{}", json.to_string());

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
