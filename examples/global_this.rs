use deno_core::{serde_v8, v8};

fn main() {
    init();

    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
    {
        let handle_scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(handle_scope);
        let scope = &mut v8::ContextScope::new(handle_scope, context);

        let source = r#"
        globalThis.x = 42;

        globalThis.x;
    "#;

        let code = v8::String::new(scope, source).unwrap();
        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();
        let value = serde_v8::from_v8::<i32>(scope, result).unwrap();

        println!("result: {}", value);
    }

    {
        // when use another context, globalThis.x not exists
        let handle_scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(handle_scope);
        let scope = &mut v8::ContextScope::new(handle_scope, context);
        let source2 = "globalThis.x";
        let code2 = v8::String::new(scope, source2).unwrap();
        let script2 = v8::Script::compile(scope, code2, None).unwrap();
        let result2 = script2.run(scope).unwrap();

        let value2 = serde_v8::from_v8::<i32>(scope, result2).unwrap();

        println!("result2: {}", value2);
    }
}

fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}
