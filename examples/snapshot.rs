use std::{fs, path::Path};

use clap::Parser;
use v8_learn::JsRuntime;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Parser)]
enum Action {
    Build,
    Run,
}

const BLOB_PATH: &str = "./snapshot.blob";

fn main() {
    JsRuntime::init();

    let args = Args::parse();

    match args.action {
        Action::Build => {
            snapshot_build(BLOB_PATH);
        }
        Action::Run => {
            snapshot_run(BLOB_PATH);
        }
    }
}

fn snapshot_build(path: impl AsRef<Path>) {
    let blob = JsRuntime::create_snapshot();
    fs::write(path, blob).unwrap();
}

fn snapshot_run(path: impl AsRef<Path>) {
    let blob = fs::read(path).unwrap();
    let mut runtime = JsRuntime::new(Some(blob));
    let code = r#"
        function hello() {
            let result = print({a: 1, b: 2});
            print(result);
            return "hello world";
        }
        hello();
    "#;
    let result = runtime.execute_script(code);

    println!("Result is: {:?}", result);
}
