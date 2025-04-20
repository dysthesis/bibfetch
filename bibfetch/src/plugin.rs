use std::{path::PathBuf, sync::Arc};

use anyhow::anyhow;
use parking_lot::Mutex;

use crate::conversion::{ComponentValExt, LuaValueExt};

#[derive(Clone)]
pub struct Plugin {
    pub name: String,
    pre: wasmtime::component::InstancePre<()>,
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
        let linker = wasmtime::component::Linker::new(&engine);

        let pre = linker.instantiate_pre(&component)?;

        Ok(Plugin { name, pre })
    }
}

impl Plugin {
    pub fn get_typed_func<Params, Results>(
        &mut self,
        name: impl wasmtime::component::InstanceExportLookup,
    ) -> anyhow::Result<wasmtime::component::TypedFunc<Params, Results>>
    where
        Params: wasmtime::component::ComponentNamedList + wasmtime::component::Lower,
        Results: wasmtime::component::ComponentNamedList + wasmtime::component::Lift,
    {
        let mut store = wasmtime::Store::new(&self.pre.engine(), ());
        self.pre
            .instantiate(&mut store)?
            .get_typed_func::<Params, Results>(&mut store, name)
    }

    pub fn get_func(
        &mut self,
        name: impl wasmtime::component::InstanceExportLookup,
    ) -> anyhow::Result<wasmtime::component::Func> {
        let mut store = wasmtime::Store::new(&self.pre.engine(), ());
        let res = self
            .pre
            .instantiate(&mut store)
            .map_err(|e| anyhow!("Failed to instantiate plugin: {e}"))?
            .get_func(&mut store, name)
            .ok_or_else(|| anyhow!("Function does not exist"))?;
        Ok(res)
    }

    pub fn call(
        &mut self,
        params: &[wasmtime::component::Val],
        results: &mut [wasmtime::component::Val],
    ) -> anyhow::Result<()> {
        let mut store = wasmtime::Store::new(&self.pre.engine(), ());
        self.get_func(self.name.clone())?
            .call(&mut store, params, results)
    }

    pub fn register(this: Arc<Mutex<Self>>, lua: &mlua::Lua) -> anyhow::Result<()> {
        let name = {
            let guard = this.lock();
            guard.name.clone()
        };

        let plugin_handle = Arc::clone(&this);

        let f = lua.create_function_mut(move |lua, params: mlua::MultiValue| {
            let wasm_args: Vec<_> = params
                .iter()
                .filter_map(|p| p.try_to_wasm_val().ok())
                .collect();

            let mut plugin = plugin_handle.lock();
            let mut results = Vec::with_capacity(4);
            plugin
                .call(&wasm_args, &mut results)
                .map_err(|e| anyhow!("Failed to call plugin: {e}"))?;

            let lua_values: Vec<mlua::Value> = results
                .into_iter()
                .filter_map(|v| v.to_lua_value(lua).ok())
                .collect();

            Ok(mlua::MultiValue::from_vec(lua_values))
        })?;

        lua.globals().set(name, f)?;
        Ok(())
    }
}
