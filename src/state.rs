use std::{cell::RefCell, rc::Rc};

use deno_core::v8::{Context, Global, HandleScope, Isolate};

type GlobalContext = Global<Context>;
type JsRuntimeStateRef = Rc<RefCell<JsRuntimeState>>;

pub struct JsRuntimeState {
    context: Option<GlobalContext>,
}

impl JsRuntimeState {
    pub fn new(isolate: &mut Isolate) -> JsRuntimeStateRef {
        let context = {
            let handle_scope = &mut HandleScope::new(isolate);
            let context = Context::new(handle_scope);

            Global::new(handle_scope, context)
        };

        Rc::new(RefCell::new(JsRuntimeState {
            context: Some(context),
        }))
    }

    pub fn get_context(isolate: &mut Isolate) -> GlobalContext {
        let state = isolate.get_slot::<JsRuntimeStateRef>().unwrap().clone();

        let context = &state.borrow().context;

        context.as_ref().unwrap().clone()
    }
}
