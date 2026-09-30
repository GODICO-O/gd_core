use jni::EnvUnowned;
use jni::objects::{JClass, JString};
use jni::sys::jint;

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_godico_gdlauncher_host_MainActivity_gd_1core_1init(
    _unowned_env: EnvUnowned,
    _class: JClass,
) -> jint {
    1
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_godico_gdlauncher_host_MainActivity_gd_1core_1get_1version<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass,
) -> JString<'local> {
    unowned_env.with_env(|env| {
        let output = "v0.1.0-debug";
        env.new_string(output)
            .expect("Gagal membuat JNI String!")
    })
}
