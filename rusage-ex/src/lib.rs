pub mod rusage;

use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("libc error({errno}): {message}")]
    LibcError { errno: i32, message: String },
}

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::str;

const TMPBUF_SZ: usize = 128;

pub fn error_string(errno: i32) -> String {
    extern "C" {
        #[cfg_attr(
            any(target_os = "linux", target_env = "newlib"),
            link_name = "__xpg_strerror_r"
        )]
        fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: libc::size_t) -> c_int;
    }

    let mut buf = [0 as c_char; TMPBUF_SZ];

    let p = buf.as_mut_ptr();
    unsafe {
        if strerror_r(errno as c_int, p, buf.len()) < 0 {
            panic!("strerror_r failure");
        }

        let p = p as *const _;
        str::from_utf8(CStr::from_ptr(p).to_bytes())
            .unwrap()
            .to_owned()
    }
}
