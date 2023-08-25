use bevy::prelude::bevy_main;
use game::LocaleLangs;
use jni::objects::JObject;
use jni::*;

fn open_url(url: &str) {
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let context = unsafe { JObject::from_raw(ctx.context().cast()) };
    let mut env = vm.attach_current_thread().unwrap();

    let url = env.new_string(url).unwrap();

    env.call_method(
        context,
        "openUrl",
        "(Ljava/lang/String;)V",
        &[(&url).into()],
    )
    .unwrap();
}

fn get_lang() -> game::LocaleLangs {
    let ctx = ndk_context::android_context();
    let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
    let mut env = vm.attach_current_thread().unwrap();

    let lang = env.find_class("java/util/Locale").unwrap();
    let lang = env
        .call_static_method(lang, "getDefault", "()Ljava/util/Locale;", &[])
        .unwrap();
    let lang = env
        .call_method(
            lang.l().unwrap(),
            "getLanguage",
            "()Ljava/lang/String;",
            &[],
        )
        .unwrap();
    let lang = lang.l().unwrap();
    let lang = env.get_string((&lang).into()).unwrap();
    let lang = lang.to_str().unwrap();
    let lang = lang.to_lowercase();

    match lang.as_str() {
        "es" => LocaleLangs::ES,
        _ => LocaleLangs::EN,
    }
}

#[bevy_main]
fn main() {
    println!("Starting launcher: Mobile");
    game::app(true, get_lang(), open_url).run();
}
