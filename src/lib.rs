use jni::EnvUnowned;
use jni::objects::JClass;
use jni::sys::{jint, jstring};

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_godico_gdlauncher_host_MainActivity_gd_1core_1init(
    _unowned_env: EnvUnowned,
    _class: JClass,
) -> jint {
    1
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_godico_gdlauncher_host_MainActivity_gd_1core_1get_1version(
    unowned_env: EnvUnowned,
    _class: JClass,
) -> jstring {
    let mut env = unsafe { unowned_env.get_env() };
    let output = "v0.1.0-debug";
    let jstr = env.new_string(output).expect("Gagal membuat JNI String!");
    jstr.into_raw()
}
