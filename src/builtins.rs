use std::marker::PhantomData;

use mlua::{FromLuaMulti, IntoLuaMulti, Lua, MaybeSend};
pub struct Builtin<F, A, R>
where
    F: Fn(&Lua, A) -> mlua::Result<R> + MaybeSend + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
{
    name: String,
    function: F,
    _phantomdata: PhantomData<(A, R)>,
}

impl<F, A, R> Builtin<F, A, R>
where
    F: Fn(&Lua, A) -> mlua::Result<R> + MaybeSend + Clone + 'static,
    A: FromLuaMulti,
    R: IntoLuaMulti,
{
    pub fn from(name: String, function: F) -> Self {
        Builtin {
            function,
            name,
            _phantomdata: PhantomData,
        }
    }
    pub fn register(&self, lua: &Lua) -> mlua::Result<()> {
        let function = lua.create_function(self.function.clone())?;
        lua.globals().set(self.name.to_owned(), function)?;
        Ok(())
    }
}
