use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jint;

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_godico_gdlauncher_host_MainActivity_gd_1core_1init(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    1
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_godico_gdlauncher_host_MainActivity_gd_1core_1get_1version<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
) -> JString<'local> {
    let output = "v0.1.0-debug";
    env.new_string(output)
        .expect("Gagal membuat JNI String!")
}
