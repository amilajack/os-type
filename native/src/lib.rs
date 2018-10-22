#[macro_use]
extern crate neon;
extern crate os_type;

use neon::prelude::*;

fn current_platform(mut cx: FunctionContext) -> JsResult<JsString> {
    let os = os_type::current_platform();
    Ok(cx.string(format!("{} {:?}", os.version, os.os_type)))
}

register_module!(mut cx, {
    cx.export_function("currentPlatform", current_platform)
});
