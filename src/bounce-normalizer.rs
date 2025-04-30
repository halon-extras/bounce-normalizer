#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use halon_rust_tokenizer::Tokenizer;

#[no_mangle]
pub extern "C" fn Halon_version(
) -> u32 {
    HALONMTA_PLUGIN_VERSION
}

use std::{
    ffi::CStr,
    ffi::c_void,
    ffi::c_char,
    ptr::null_mut
};

pub extern "C" fn bounce_normalizer(
    _hhc: *mut HalonHSLContext,
    _args: *mut HalonHSLArguments,
    ret: *mut HalonHSLValue,
) {
    unsafe {
        // get first argument
        let arg = HalonMTA_hsl_argument_get(_args, 0);
        if arg == null_mut() {
            return
        }

        // convert to a string
        let mut input: *mut c_char = null_mut();
        let input_ptr: *mut *mut c_char = &mut input;
        let ok = HalonMTA_hsl_value_get(arg, HALONMTA_HSL_TYPE_STRING as i32, input_ptr as *mut c_void, null_mut());
        if !ok {
            libc::free(input as *mut c_void);
        }
        let input_cstr: &CStr = CStr::from_ptr(input);
        let input_str = String::from_utf8_lossy(input_cstr.to_bytes()).to_string();

        let tokenizer = Tokenizer::new().expect("Failed to create tokenizer");
        match tokenizer.normalize(input_str.as_str()) {
            Ok(normalized) => {
                // set as return value
                let output: std::ffi::CString = std::ffi::CString::new(normalized).unwrap();
                HalonMTA_hsl_value_set(ret, HALONMTA_HSL_TYPE_STRING as i32, output.as_ptr() as *mut std::ffi::c_void, 0);
            }
            Err(_e) => {
                // do nothing
            }
        }

    }
}

#[no_mangle]
pub extern "C" fn Halon_hsl_register(hhrc: *mut HalonHSLRegisterContext
) -> bool {
    let func_name = std::ffi::CString::new("bounce_normalizer").unwrap();
    unsafe {
        HalonMTA_hsl_module_register_function(hhrc, func_name.as_ptr(), Some(bounce_normalizer));
    }
    return true
}
