use deno_core::{
    serde_json, serde_v8,
    v8::{
        self, ExternalReference, ExternalReferences, FunctionCallbackArguments, HandleScope,
        MapFnTo, ReturnValue,
    },
};

use crate::utils::execute_script;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref EXTERNAL_REFERENCES: ExternalReferences = ExternalReferences::new(&[
        ExternalReference {
            function: MapFnTo::map_fn_to(print),
        },
        ExternalReference {
            function: MapFnTo::map_fn_to(fetch),
        }
    ]);
}

pub struct Extensions;

macro_rules! bindings {
    ($scope:ident, $name:expr, $target:expr) => {
        let code = format!(
            r"(target) => {{
                globalThis.{name} = target;
              }};",
            name = $name
        );

        if let Ok(result) = execute_script($scope, code) {
            let glue_func = v8::Local::<v8::Function>::try_from(result).unwrap();
            let v = v8::undefined($scope).into();
            let args = [$target.into()];

            glue_func.call($scope, v, &args).unwrap();
        }
    };
}

impl Extensions {
    pub fn install(scope: &mut HandleScope) {
        bindings!(scope, "print", v8::Function::new(scope, print).unwrap());
        bindings!(scope, "fetch", v8::Function::new(scope, fetch).unwrap());
        bindings!(
            scope,
            "test_name",
            v8::String::new(scope, "this is test_name").unwrap()
        );
    }
}

fn print(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let result: serde_json::Value = serde_v8::from_v8(scope, args.get(0)).unwrap();

    println!("Rust says: {result:#?}");

    rv.set(serde_v8::to_v8(scope, result).unwrap());
}

fn fetch(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let url: String = serde_v8::from_v8(scope, args.get(0)).unwrap();
    let result = reqwest::blocking::get(&url).unwrap().text().unwrap();

    rv.set(serde_v8::to_v8(scope, result).unwrap());
}
