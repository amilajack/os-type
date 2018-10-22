#[macro_use]
extern crate neon;
extern crate os_type;

use neon::prelude::*;

fn map_os_type_to_string<'a>(os_type: os_type::OSType) -> &'a str {
    match os_type {
        Unknown => "unknown",
        Redhat => "redhat",
        OSX => "osx",
        Ubuntu => "ubuntu",
        Debian => "debian",
        Arch => "arch",
        Manjaro => "manjaro",
        CentOS => "centos",
        OpenSUSE => "opensuse"
    }
}

fn current_platform(mut cx: FunctionContext) -> JsResult<JsObject> {
    let os = os_type::current_platform();
    let object = JsObject::new(&mut cx);
    let os_type_js_string = cx.string(map_os_type_to_string(os.os_type));
    let js_string_version = cx.string(os.version);
    object.set(&mut cx, "version", js_string_version).unwrap();
    object.set(&mut cx, "osType", os_type_js_string).unwrap();
    Ok(object)
}

register_module!(mut cx, {
    cx.export_function("currentPlatform", current_platform)
});
