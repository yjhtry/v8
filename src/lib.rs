use once_cell::sync::OnceCell;

use deno_core::{
    serde_json, serde_v8,
    v8::{self, CreateParams, FunctionCodeHandling, HandleScope, Isolate, OwnedIsolate},
};
use extensions::{Extensions, EXTERNAL_REFERENCES};
use state::JsRuntimeState;
use utils::execute_script;

mod extensions;
mod state;
mod utils;

pub struct JsRuntime {
    pub isolate: OwnedIsolate,
}

type LocalValue<'a> = v8::Local<'a, v8::Value>;

impl JsRuntime {
    pub fn init() {
        static V8_INITIAL: OnceCell<()> = OnceCell::new();

        V8_INITIAL.get_or_init(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });
    }

    pub fn new(snapshot: Option<Vec<u8>>) -> Self {
        let mut params = CreateParams::default().external_references(&**EXTERNAL_REFERENCES);
        let mut initialized = false;

        if let Some(snapshot) = snapshot {
            params = params.snapshot_blob(snapshot);
            initialized = true;
        }
        let isolate = v8::Isolate::new(params);

        JsRuntime::init_isolate(isolate, initialized)
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

    pub fn create_snapshot() -> Vec<u8> {
        let mut snapshot_creator = Isolate::snapshot_creator(Some(&EXTERNAL_REFERENCES));

        {
            let scope = &mut v8::HandleScope::new(&mut snapshot_creator);
            let context = v8::Context::new(scope);
            let scope = &mut v8::ContextScope::new(scope, context);

            Extensions::install(scope);

            scope.set_default_context(context);
        }

        match snapshot_creator.create_blob(FunctionCodeHandling::Keep) {
            Some(blob) => blob.to_vec(),
            None => panic!("Failed to create snapshot"),
        }
    }

    pub fn init_isolate(mut isolate: OwnedIsolate, initialized: bool) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);

        if !initialized {
            let context = JsRuntimeState::get_context(&mut isolate);
            let handle_scope = &mut HandleScope::with_context(&mut isolate, context);

            Extensions::install(handle_scope);
        }

        JsRuntime { isolate }
    }
}
