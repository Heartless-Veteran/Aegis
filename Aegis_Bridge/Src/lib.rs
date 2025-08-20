use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use deno_core::JsRuntime;
use deno_core::RuntimeOptions;
use tokio::runtime::Builder;

/// This is the function that our generated Kotlin code will call.
/// The name is specially formatted for JNI: Java_package_name_ClassName_methodName
#[no_mangle]
pub extern "system" fn Java_com_aegisapp_AegisBridge_executeJs<'local>(
    // JNI environment for interacting with the JVM
    mut env: JNIEnv<'local>,
    // A reference to the Java class that called this method
    _class: JClass<'local>,
    // The arguments from Kotlin
    script: JString<'local>,
    json_data: JString<'local>,
) -> jstring {
    // 1. Convert JString arguments from Kotlin into standard Rust Strings.
    // We expect these to fail only in catastrophic memory error scenarios.
    let script_str: String = env.get_string(&script).expect("Couldn't get script string").into();
    let json_data_str: String = env.get_string(&json_data).expect("Couldn't get json data string").into();

    // 2. A simple string replacement to inject the Aegis data into the JS script.
    // In a production system, this would be a more robust JSON-based mechanism.
    let final_script = script_str.replace("{user}", &json_data_str);

    // 3. We need a Tokio runtime because the Deno/V8 core is asynchronous.
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    let result_str = rt.block_on(async {
        // 4. Create a new JavaScript runtime (a V8 isolate).
        // It's sandboxed and has no file system or network access by default.
        let mut runtime = JsRuntime::new(RuntimeOptions::default());

        // 5. Execute the script. `deno_core` uses futures.
        let result_global = runtime.execute_script("<aegis>", final_script.into())
            .expect("JS execution failed");
        
        // 6. Get a handle to the result and serialize it back to a JSON string.
        let scope = &mut runtime.handle_scope();
        let local_result = deno_core::v8::Local::new(scope, result_global);
        
        serde_json::to_string(&local_result)
            .unwrap_or_else(|e| format!("{{\"error\":\"Failed to serialize result: {}\"}}", e))
    });

    // 7. Convert the Rust String result back into a JString to return to Kotlin.
    let output = env.new_string(result_str).expect("Couldn't create java string!");
    
    // Release the raw pointer to the JVM.
    output.into_raw()
}
