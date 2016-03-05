extern crate libc;
use libc::{hostent, c_int, size_t};

pub type nss_status = c_int;

// const char *restrict name, struct hostent *restrict result_buf, char *restrict buf, size_t buflen, struct hostent **restrict result, int *restrict h_errnop
#[no_mangle]
pub extern fn __nss_signpost_gethostbyname_r(name: *const u8,
                                             result_buf: *mut hostent,
                                             buf: *const u8,
                                             buflen: size_t,
                                             errnop: *mut c_int,
                                             h_errnop: *mut c_int) -> nss_status {
    return 1
}
