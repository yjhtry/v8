use once_cell::sync::OnceCell;

use deno_core::{
    serde_json, serde_v8,
    v8::{self, CreateParams, HandleScope, OwnedIsolate},
};
use extensions::Extensions;
use state::JsRuntimeState;
use utils::execute_script;

mod extensions;
mod state;
mod utils;

pub struct JsRuntime {
    isolate: OwnedIsolate,
}

type LocalValue<'a> = v8::Local<'a, v8::Value>;

#[derive(Default)]
pub struct JsRuntimeParams(CreateParams);

impl JsRuntimeParams {
    pub fn new(_snapshot: Option<Vec<u8>>) -> Self {
        Self(CreateParams::default())
    }

    pub fn into_inner(self) -> CreateParams {
        self.0
    }
}

impl JsRuntime {
    pub fn init() {
        static V8_INITIAL: OnceCell<()> = OnceCell::new();

        V8_INITIAL.get_or_init(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });
    }
    pub fn new(params: JsRuntimeParams) -> Self {
        let isolate = v8::Isolate::new(params.into_inner());

        JsRuntime::init_isolate(isolate)
    }

    pub fn execute_script(
        &mut self,
        code: impl AsRef<str>,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let context = JsRuntimeState::get_context(&mut self.isolate);
        let handle_scope = &mut HandleScope::with_context(&mut self.isolate, context);

        match execute_script(handle_scope, code) {
            Ok(value) => Ok(serde_v8::from_v8(handle_scope, value).unwrap()),
            Err(err) => Err(serde_v8::from_v8(handle_scope, err).unwrap()),
        }
    }

    pub fn new_with_snapshot(params: JsRuntimeParams, snapshot: &[u8]) -> Self {
        todo!()
    }

    pub fn create_snapshot(&self) -> Vec<u8> {
        todo!()
    }

    pub fn init_isolate(mut isolate: OwnedIsolate) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);

        {
            let context = JsRuntimeState::get_context(&mut isolate);
            let handle_scope = &mut HandleScope::with_context(&mut isolate, context);

            Extensions::install(handle_scope);
        }

        JsRuntime { isolate }
    }
}
