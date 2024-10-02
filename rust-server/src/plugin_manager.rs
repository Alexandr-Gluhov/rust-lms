use std::collections::HashMap;

use wasmtime::{Engine, Result, Store, Config};
use wasmtime::component::{ResourceTable, Linker, bindgen, Component};
use wasmtime_wasi::{WasiCtx, WasiView, WasiCtxBuilder};

pub struct PluginManager {
    engine: Engine,
    linker: Linker<MyState>,
    store: Store<MyState>,
    plugins: HashMap<String, Component>
}

impl PluginManager {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = Engine::new(&config).expect("Can't create plugins engine");
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker).expect("Can't add wasmtime to linker sync");

        let store = Store::new(
            &engine,
            MyState {
                ctx: WasiCtxBuilder::new().inherit_stdio().build(),
                table: ResourceTable::new(),
            },
        );

        let plugins = HashMap::new();

        Self { engine, linker, store, plugins }
    }

    pub fn load_plugin(&mut self, plugin_id: &str) -> Result<()> {
        let component = Component::from_file(&self.engine, format!("plugins/{plugin_id}.wasm"))?;
        self.plugins.insert(plugin_id.to_string(), component);
        Ok(())
    }

    pub fn get_plugin(&self, plugin_id: &str) -> Component {
        self.plugins.get(plugin_id).unwrap().clone()
    }

    pub fn has_plugin(&self, plugin_id: &str) -> bool {
        self.plugins.contains_key(plugin_id)
    }

    pub fn instantiate_world(&mut self, component: &Component) -> Result<NewWorld> {
        NewWorld::instantiate(&mut self.store, component, &self.linker)
    }

    pub fn get_store(&mut self) -> &mut Store<MyState> {
        &mut self.store
    }
}

bindgen!({ world: "new-world", path: "lms_plugin_api/wit/world.wit" });

pub struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}