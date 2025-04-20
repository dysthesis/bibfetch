use std::path::PathBuf;

use anyhow::anyhow;

pub struct Plugin {
    name: String,
    instance: wasmtime::component::Instance,
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

        // Enable components, which I think is what we want?
        // TODO: actually figure out what component vs. module is
        let mut config = wasmtime::Config::default();
        config.wasm_component_model(true);

        let engine = wasmtime::Engine::new(&config)?;
        let component = wasmtime::component::Component::from_file(&engine, path)?;
        let store = wasmtime::Store::new(&engine, ());
        let linker = wasmtime::component::Linker::new(&engine);

        let instance = linker.instantiate(store, &component)?;

        Ok(Plugin { name, instance })
    }
}
