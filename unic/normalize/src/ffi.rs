#[link(name = "icuuc")]
extern "C" {
    pub fn unorm2_getNFCInstance_67(pErrorCode: *mut libc::c_int) -> *mut libc::c_void;
    pub fn unorm2_getNFDInstance_67(pErrorCode: *mut libc::c_int) -> *mut libc::c_void;
    pub fn unorm2_close_67(norm2: *mut libc::c_void);
    pub fn unorm2_normalize_67(
        norm2: *mut libc::c_void,
        src: *const u16,
        length: i32,
        dest: *mut u16,
        capacity: i32,
        pErrorCode: *mut libc::c_int,
    ) -> i32;
}

