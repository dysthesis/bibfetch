use std::path::PathBuf;

use anyhow::Ok;
use mlua::Table;

use crate::{
    cli::{Args, HandlersPath},
    handler::Handler,
};

mod cli;
mod handler;

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;
    let handlers = init_handlers(args.handlers_path).unwrap();
    let results: Vec<Table> = args
        .identifiers
        .iter()
        .map(|id| {
            let handler = handlers.first().expect("No handlers found!");
            let parsed = handler
                .parse
                .call::<Option<String>>(id.clone())
                .unwrap_or_else(|e| panic!("Failed to parse with handler {}: {}", handler.name, e))
                .unwrap();
            handler.fetch.call::<Table>(parsed).unwrap()
        })
        .collect();

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
