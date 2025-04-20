use std::{fs, path::PathBuf};

use anyhow::anyhow;

struct Plugin {
    name: String,
    instance: wasmtime::Instance,
}

impl TryFrom<PathBuf> for Plugin {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let name = path
            .file_stem()
            .ok_or(anyhow!("Failed to get plugin name from file name!"))?
            .to_str()
            .ok_or(anyhow!("Failed to convert name to &str!"))?
            .to_string();

        let engine = wasmtime::Engine::default();
        let module = wasmtime::Module::from_file(&engine, path)?;
        let mut store = wasmtime::Store::new(&engine, ());

        let instance = wasmtime::Instance::new(&mut store, &module, &[])?;
        Ok(Plugin { name, instance })
    }
}
