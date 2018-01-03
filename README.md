# rlua-table-derive
This crate provides a custom derive for a FromLuaTable trait 


To derive just add FromLuaTable to the derive list for a struct. Default must be implemented for a type to use this macro.

An example usage can be found in the examples directory. I have copied it here for convenience:



    #[macro_use] extern crate rlua_table_derive;
    extern crate rlua;


    #[derive(Default, Debug, Clone, FromLuaTable)]
    pub struct Thing{
        x: f32,
        y: f32,

        name: String,
    }

    trait FromLuaTable {
        fn from_lua_table(table: &rlua::Table) -> Self;
    }

    const LUA_STRING: &str = "
    thing = {
        x=2,
        name='john',
    }
    ";

    fn main() {
        let lua = rlua::Lua::new();
        lua.eval::<()>(LUA_STRING, Some("baked in")).unwrap();

        let default = Thing::default();

        let table = lua.globals().get("thing").unwrap();
        let from_lua = Thing::from_lua_table(&table);

        print!("Default: {:?}\n", default);
        print!("From lua: {:?}\n", from_lua);
    }
