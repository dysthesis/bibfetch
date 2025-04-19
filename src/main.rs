use std::path::PathBuf;

use anyhow::Ok;

use crate::{
    cli::{Args, HandlersPath},
    handler::Handler,
};

mod cli;
mod handler;

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;
    println!("Found command-line arguments {args:?}");
    let handlers = init_handlers(args.handlers_path).unwrap();
    Ok(())
}

pub fn init_handlers(path: HandlersPath) -> anyhow::Result<Vec<Handler>> {
    let path: PathBuf = path.into();
    let result = path
        .read_dir()?
        .filter_map(|x| x.ok())
        .map(|entry| Handler::try_from(entry.path()))
        .filter_map(|x| x.ok())
        .collect::<Vec<Handler>>();
    Ok(result)
}
