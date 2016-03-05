extern crate libc;
use libc::{getaddrinfo, addrinfo, c_char, c_int};
use std::ffi::CString;

#[link(name = "libc")]
fn main() {
    unsafe {
        let name = CString::new("localhost").unwrap();
        //let hints = addrinfo {ai_family: AF_INET, ai_socktype: 0, ai_protocol:0, ai_flags: 0};
        let mut result = 0 as *mut addrinfo;
        getaddrinfo(name.as_ptr(), 0 as *const c_char, 0 as *const addrinfo, &mut result);
        print_addrinfo(result)
    }
}

fn print_addrinfo(info_p: *mut addrinfo) {
    unsafe {
        if info_p as c_int != 0 {
            let info = *info_p;
            let address = (*info.ai_addr).sa_data;
            println!("{}.{}.{}.{}", address[2], address[3], address[4], address[5]);
            print_addrinfo(info.ai_next);
        }
    }
}
