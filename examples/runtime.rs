use v8_learn::JsRuntime;

fn main() {
    JsRuntime::init();

    let mut runtime = JsRuntime::new(None);

    let code = r#"
        async function hello() {
            print("Hello, World!");

            let res = fetch("https://www.rust-lang.org/");

            print(res);

            return { message: "Hello, Rust!" };
        }

        hello()
    "#;

    let result = runtime.execute_script(code, true);

    println!("{:?}", result);
}
