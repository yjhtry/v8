use deno_core::v8::{self, HandleScope, TryCatch};

use crate::LocalValue;

pub fn execute_script<'a>(
    scope: &mut HandleScope<'a>,
    code: impl AsRef<str>,
) -> Result<LocalValue<'a>, LocalValue<'a>> {
    let scope = &mut TryCatch::new(scope);
    let code = v8::String::new(scope, code.as_ref()).unwrap();

    v8::Script::compile(scope, code, None)
        .and_then(|script| script.run(scope))
        .map_or_else(|| Err(scope.stack_trace().unwrap()), Ok)
}
