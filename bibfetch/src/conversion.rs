use anyhow::anyhow;
use mlua::{Lua, Value as LuaValue};
use wasmtime::component::Val as ComponentVal;

/// Local extension trait for converting any mlua::LuaValue into a Wasm `Val`.
pub trait LuaValueExt {
    fn try_to_wasm_val(&self) -> anyhow::Result<ComponentVal>;
}

impl LuaValueExt for LuaValue {
    fn try_to_wasm_val(&self) -> anyhow::Result<ComponentVal> {
        match self {
            // Nil ⇒ Option(None)
            LuaValue::Nil => Ok(ComponentVal::Option(None)),
            // Boolean ⇒ Bool
            LuaValue::Boolean(b) => Ok(ComponentVal::Bool(*b)),
            // Integer ⇒ S64
            LuaValue::Integer(i) => Ok(ComponentVal::S64(*i)),
            // Number ⇒ Float64
            LuaValue::Number(n) => Ok(ComponentVal::Float64(*n)),
            // String ⇒ String
            LuaValue::String(s) => {
                let s_ref = s
                    .to_str()
                    .map_err(|e| anyhow!("Invalid UTF-8 in Lua string: {}", e))?;
                Ok(ComponentVal::String(s_ref.to_owned()))
            }
            LuaValue::Table(t) => {
                // Iterate only string keys, error on non-string
                let mut fields = Vec::new();
                for pair in t.pairs::<String, LuaValue>() {
                    let (key, val) =
                        pair.map_err(|e| anyhow!("Error reading table pair: {}", e))?;
                    let cv = val.try_to_wasm_val()?;
                    fields.push((key, cv));
                }
                Ok(ComponentVal::Record(fields))
            }
            LuaValue::Error(err_box) => {
                // Convert the Lua error into a string payload
                let msg = err_box.to_string();
                // Wrap that string as a Val and then into the Err side of a Result
                let payload = ComponentVal::String(msg);
                Ok(ComponentVal::Result(Err(Some(Box::new(payload)))))
            }
            other => Err(anyhow!("cannot map LuaValue::{:?} to component Val", other)),
        }
    }
}

pub trait ComponentValExt {
    fn to_lua_value(&self, lua: &Lua) -> anyhow::Result<LuaValue>;
}

impl ComponentValExt for ComponentVal {
    fn to_lua_value(&self, lua: &Lua) -> anyhow::Result<LuaValue> {
        match self {
            // Option(None) ⇒ Nil
            ComponentVal::Option(None) => Ok(LuaValue::Nil),
            // Option(Some(inner)) ⇒ recurse
            ComponentVal::Option(Some(inner)) => inner.to_lua_value(lua),
            // Bool ⇒ Boolean
            ComponentVal::Bool(b) => Ok(LuaValue::Boolean(*b)),
            // S64 ⇒ Integer
            ComponentVal::S64(i) => Ok(LuaValue::Integer(*i)),
            // U64 ⇒ Integer (clamped/truncated)
            ComponentVal::U64(u) => Ok(LuaValue::Integer(*u as i64)),
            // Float64 ⇒ Number
            ComponentVal::Float64(f) => Ok(LuaValue::Number(*f)),
            // String ⇒ String
            ComponentVal::String(s) => {
                let lua_str = lua.create_string(s)?;
                Ok(LuaValue::String(lua_str))
            }
            ComponentVal::Record(fields) => {
                let table: mlua::Table = lua.create_table()?;
                for (key, val) in fields.iter() {
                    let lv = val.to_lua_value(lua)?;
                    table.set(key.as_str(), lv)?;
                }
                Ok(LuaValue::Table(table))
            }

            // TODO: See if you can convert the other types

            // All other variants are unsupported in this mapping
            other => Err(anyhow!(
                "cannot map ComponentValue::{:?} to component Val",
                other
            )),
        }
    }
}
