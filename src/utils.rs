use deno_core::v8::{self, HandleScope, TryCatch};

use crate::LocalValue;

pub fn execute_script<'a>(
    scope: &mut HandleScope<'a>,
    code: impl AsRef<str>,
    is_module: bool,
) -> Result<LocalValue<'a>, LocalValue<'a>> {
    let scope = &mut TryCatch::new(scope);
    let code = v8::String::new(scope, code.as_ref()).unwrap();

    if is_module {
        let source = v8::script_compiler::Source::new(code, Some(&create_origin(scope, "module")));
        let module = v8::script_compiler::compile_module(scope, source).unwrap();
        module.instantiate_module(scope, |_, _, _, _| None).unwrap();

        let result = module.evaluate(scope).unwrap();
        let promise = v8::Local::<v8::Promise>::try_from(result).unwrap();

        match promise.state() {
            v8::PromiseState::Fulfilled => {
                println!("Promise fulfilled");
                let result = promise.result(scope);
                return Ok(result);
            }
            v8::PromiseState::Rejected => {
                println!("Promise rejected");
                let result = promise.result(scope);
                return Err(result);
            }
            v8::PromiseState::Pending => {
                println!("Promise pending");
                return Err(result);
            }
        }
    } else {
        v8::Script::compile(scope, code, None)
            .and_then(|script| script.run(scope))
            .map_or_else(|| Err(scope.stack_trace().unwrap()), Ok)
    }
}

fn create_origin<'s>(scope: &mut HandleScope<'s>, url: &str) -> v8::ScriptOrigin<'s> {
    let name: v8::Local<v8::Value> = v8::String::new(scope, url).unwrap().into();
    v8::ScriptOrigin::new(
        scope,
        name.clone(),
        0,
        0,
        false,
        0,
        name,
        false,
        false,
        true,
    )
}
