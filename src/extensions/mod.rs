use deno_core::{
    serde_json, serde_v8,
    v8::{self, FunctionCallbackArguments, HandleScope, ReturnValue},
};

use crate::utils::execute_script;

const GLUE: &str = include_str!("glue.js");

pub struct Extensions;

impl Extensions {
    pub fn install(scope: &mut HandleScope) {
        let bindings = v8::Object::new(scope);
        let name = v8::String::new(scope, "print").unwrap();
        let func = v8::Function::new(scope, print).unwrap();

        bindings.set(scope, name.into(), func.into()).unwrap();

        if let Ok(result) = execute_script(scope, GLUE) {
            let func = v8::Local::<v8::Function>::try_from(result).unwrap();
            let v = v8::undefined(scope).into();
            let args = [bindings.into()];

            func.call(scope, v, &args).unwrap();
        }
    }
}

fn print(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let result: serde_json::Value = serde_v8::from_v8(scope, args.get(0)).unwrap();

    println!("Rust says: {result:#?}");

    rv.set(serde_v8::to_v8(scope, result).unwrap());
}
