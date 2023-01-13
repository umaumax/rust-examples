use std::mem;
use std::os;

use crate::Error;

pub fn getrusage_children() -> Result<libc::rusage, Error> {
    getrusage(libc::RUSAGE_CHILDREN)
}

pub fn getrusage_thread() -> Result<libc::rusage, Error> {
    getrusage(libc::RUSAGE_THREAD)
}

pub fn getrusage_self() -> Result<libc::rusage, Error> {
    getrusage(libc::RUSAGE_SELF)
}

pub fn getrusage(who: os::raw::c_int) -> Result<libc::rusage, Error> {
    let mut data = unsafe { mem::MaybeUninit::uninit().assume_init() };

    let result = unsafe { libc::getrusage(who, &mut data) };

    if result == -1 {
        let errno: os::raw::c_int = unsafe { *libc::__errno_location() };
        Err(Error::LibcError {
            errno: errno,
            message: crate::error_string(errno),
        })
    } else {
        Ok(data)
    }
}

pub fn calc_time_diff(start_rusage: &libc::rusage, end_rusage: &libc::rusage) -> f64 {
    let start_time = (start_rusage.ru_utime.tv_sec + start_rusage.ru_stime.tv_sec) as f64
        + (start_rusage.ru_utime.tv_usec + start_rusage.ru_stime.tv_usec) as f64 * 0.001 * 0.001;
    let end_time = (end_rusage.ru_utime.tv_sec + end_rusage.ru_stime.tv_sec) as f64
        + (end_rusage.ru_utime.tv_usec + end_rusage.ru_stime.tv_usec) as f64 * 0.001 * 0.001;
    end_time - start_time
}
