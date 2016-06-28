// Copyright (c) 2015 gdbwire_sys developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

extern crate gdbwire_sys;

use std::ffi::{CStr, CString};
use std::os::raw;
use std::ptr;

fn main() {
    let callbacks = gdbwire_sys::gdbmi_parser_callbacks {
        context: ptr::null_mut(),
        gdbmi_output_callback: Some(parser_callback),
    };
    unsafe {
        let parser = gdbwire_sys::gdbmi_parser_create(callbacks);
        assert!(!parser.is_null());
        main_loop(parser);
        gdbwire_sys::gdbmi_parser_destroy(parser);
    }
}

unsafe fn main_loop(parser: *mut gdbwire_sys::gdbmi_parser) {
    let mut line = String::new();
    loop {
        if let Ok(size) = std::io::stdin().read_line(&mut line) {
            let cline = CString::new(line.clone()).unwrap();
            let result = gdbwire_sys::gdbmi_parser_push_data(parser, cline.as_ptr(), size);
            assert!(result == gdbwire_sys::gdbwire_result::GDBWIRE_OK);
        } else {
            break;
        }
        line.clear();
    }
}

unsafe extern "C" fn parser_callback(context: *mut raw::c_void,
                                     output: *mut gdbwire_sys::gdbmi_output) {
    assert!(context.is_null());
    assert!(!output.is_null());

    if (*output).kind == gdbwire_sys::gdbmi_output_kind::GDBMI_OUTPUT_PARSE_ERROR {
        let line = CStr::from_ptr((*output).line);
        println!("\n  Parse Error: {}", line.to_string_lossy());
    }

    assert!((*output).kind != gdbwire_sys::gdbmi_output_kind::GDBMI_OUTPUT_PARSE_ERROR);
    assert!((*output).next.is_null());
    gdbwire_sys::gdbmi_output_free(output);
}
