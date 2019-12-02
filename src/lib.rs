#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(specialization)]
#![recursion_limit = "2048"]

extern crate swc;

use std::{path::Path, sync::Arc};
use swc::{
    common::{
        errors::{ColorConfig, Handler},
        SourceMap,
    },
    config::Options,
};
use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[no_mangle]
pub extern fn compile(s: *const c_char) -> *const c_char {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();

    let cm = Arc::<SourceMap>::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let c = swc::Compiler::new(cm.clone(), handler);

    let fm = cm
        .load_file(Path::new(r_str))
        .expect("failed to load file");

    let output = c.process_js_file(
        fm,
        &Options {
            ..Default::default()
        },
    )
        .expect("failed to process file");

    let s = CString::new(output.code).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);

    return p;
}
