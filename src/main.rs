use std::{collections::BinaryHeap, path::PathBuf};

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
        .filter_map(|id| {
            let mut result = None;
            for handler in &handlers {
                if let Ok(Some(parsed)) = handler.parse.call::<Option<String>>(id.clone()) {
                    result = handler.fetch.call::<Table>(parsed).ok();
                    break;
                }
            }

            result
        })
        .collect();

    println!("{results:?}");

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
