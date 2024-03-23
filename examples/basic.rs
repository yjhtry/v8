use deno_core::{
    serde_v8,
    v8::{self, CreateParams, HandleScope, Isolate},
};

#[derive(Debug, serde::Deserialize)]
struct Data {
    pub message: String,
}

fn main() {
    init();

    let params = CreateParams::default();
    let isolate = &mut Isolate::new(params);

    let handle_scope = &mut HandleScope::new(isolate);
    let context = v8::Context::new(handle_scope);
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    let source = r#"
        function hello() {
            return { message: "Hello, World!" };
        }

        hello()
    "#;

    let code = v8::String::new(context_scope, source).unwrap();
    let script = v8::Script::compile(context_scope, code, None).unwrap();
    let result = script.run(context_scope).unwrap();

    let value: Data = serde_v8::from_v8(context_scope, result).unwrap();

    println!("{:?}", value.message)
}
fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}
