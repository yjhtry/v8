use v8_learn::{JsRuntime, JsRuntimeParams};

fn main() {
    JsRuntime::init();
    let mut runtime = JsRuntime::new(JsRuntimeParams::default());

    let code = r#"
        function hello() {
            print("Hello, World!");

            return { message: "Hello, Rust!" };
        }

        hello();
    "#;

    let result = runtime.execute_script(code);

    println!("{:?}", result);
}
