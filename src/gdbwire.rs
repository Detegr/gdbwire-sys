/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct gdbmi_position {
    pub start_column: ::std::os::raw::c_int,
    pub end_column: ::std::os::raw::c_int,
}
impl ::std::default::Default for gdbmi_position {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum gdbmi_output_kind {
    GDBMI_OUTPUT_OOB = 0,
    GDBMI_OUTPUT_RESULT = 1,
    GDBMI_OUTPUT_PROMPT = 2,
    GDBMI_OUTPUT_PARSE_ERROR = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct gdbmi_output {
    pub kind: gdbmi_output_kind,
    pub variant: Union_Unnamed1,
    pub line: *mut ::std::os::raw::c_char,
    pub next: *mut gdbmi_output,
}
impl ::std::default::Default for gdbmi_output {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Union_Unnamed1 {
    pub _bindgen_data_: [u64; 2usize],
}
impl Union_Unnamed1 {
    pub unsafe fn oob_record(&mut self) -> *mut *mut gdbmi_oob_record {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn result_record(&mut self) -> *mut *mut gdbmi_result_record {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn error(&mut self) -> *mut Struct_Unnamed2 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Struct_Unnamed2 {
    pub token: *mut ::std::os::raw::c_char,
    pub pos: gdbmi_position,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type gdbmi_token_t = *mut ::std::os::raw::c_char;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum gdbmi_result_class {
    GDBMI_DONE = 0,
    GDBMI_RUNNING = 1,
    GDBMI_CONNECTED = 2,
    GDBMI_ERROR = 3,
    GDBMI_EXIT = 4,
    GDBMI_UNSUPPORTED = 5,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct gdbmi_result_record {
    pub token: gdbmi_token_t,
    pub result_class: gdbmi_result_class,
    pub result: *mut gdbmi_result,
}
impl ::std::default::Default for gdbmi_result_record {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum gdbmi_oob_record_kind { GDBMI_ASYNC = 0, GDBMI_STREAM = 1, }
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct gdbmi_oob_record {
    pub kind: gdbmi_oob_record_kind,
    pub variant: Union_Unnamed3,
}
impl ::std::default::Default for gdbmi_oob_record {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Union_Unnamed3 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed3 {
    pub unsafe fn async_record(&mut self) -> *mut *mut gdbmi_async_record {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn stream_record(&mut self) -> *mut *mut gdbmi_stream_record {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed3 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum gdbmi_async_record_kind {
    GDBMI_STATUS = 0,
    GDBMI_EXEC = 1,
    GDBMI_NOTIFY = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum gdbmi_stream_record_kind {
    GDBMI_CONSOLE = 0,
    GDBMI_TARGET = 1,
    GDBMI_LOG = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum gdbmi_async_class {
    GDBMI_ASYNC_DOWNLOAD = 0,
    GDBMI_ASYNC_STOPPED = 1,
    GDBMI_ASYNC_RUNNING = 2,
    GDBMI_ASYNC_THREAD_GROUP_ADDED = 3,
    GDBMI_ASYNC_THREAD_GROUP_REMOVED = 4,
    GDBMI_ASYNC_THREAD_GROUP_STARTED = 5,
    GDBMI_ASYNC_THREAD_GROUP_EXITED = 6,
    GDBMI_ASYNC_THREAD_CREATED = 7,
    GDBMI_ASYNC_THREAD_EXITED = 8,
    GDBMI_ASYNC_THREAD_SELECTED = 9,
    GDBMI_ASYNC_LIBRARY_LOADED = 10,
    GDBMI_ASYNC_LIBRARY_UNLOADED = 11,
    GDBMI_ASYNC_TRACEFRAME_CHANGED = 12,
    GDBMI_ASYNC_TSV_CREATED = 13,
    GDBMI_ASYNC_TSV_MODIFIED = 14,
    GDBMI_ASYNC_TSV_DELETED = 15,
    GDBMI_ASYNC_BREAKPOINT_CREATED = 16,
    GDBMI_ASYNC_BREAKPOINT_MODIFIED = 17,
    GDBMI_ASYNC_BREAKPOINT_DELETED = 18,
    GDBMI_ASYNC_RECORD_STARTED = 19,
    GDBMI_ASYNC_RECORD_STOPPED = 20,
    GDBMI_ASYNC_CMD_PARAM_CHANGED = 21,
    GDBMI_ASYNC_MEMORY_CHANGED = 22,
    GDBMI_ASYNC_UNSUPPORTED = 23,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct gdbmi_async_record {
    pub token: gdbmi_token_t,
    pub kind: gdbmi_async_record_kind,
    pub async_class: gdbmi_async_class,
    pub result: *mut gdbmi_result,
}
impl ::std::default::Default for gdbmi_async_record {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum gdbmi_result_kind {
    GDBMI_CSTRING = 0,
    GDBMI_TUPLE = 1,
    GDBMI_LIST = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct gdbmi_result {
    pub kind: gdbmi_result_kind,
    pub variable: *mut ::std::os::raw::c_char,
    pub variant: Union_Unnamed4,
    pub next: *mut gdbmi_result,
}
impl ::std::default::Default for gdbmi_result {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Union_Unnamed4 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed4 {
    pub unsafe fn cstring(&mut self) -> *mut *mut ::std::os::raw::c_char {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn result(&mut self) -> *mut *mut gdbmi_result {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for Union_Unnamed4 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct gdbmi_stream_record {
    pub kind: gdbmi_stream_record_kind,
    pub cstring: *mut ::std::os::raw::c_char,
}
impl ::std::default::Default for gdbmi_stream_record {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[link(name = "gdbwire", kind = "dylib")]
extern "C" {
    pub fn gdbmi_output_free(param: *mut gdbmi_output);
    pub fn append_gdbmi_output(list: *mut gdbmi_output,
                               item: *mut gdbmi_output) -> *mut gdbmi_output;
    pub fn append_gdbmi_result(list: *mut gdbmi_result,
                               item: *mut gdbmi_result) -> *mut gdbmi_result;
}
