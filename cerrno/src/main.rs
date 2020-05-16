use libc::c_char;
use libc::c_int;

fn main() {
    unsafe {
        libc::open("\0".as_ptr() as *const c_char, libc::O_CREAT);
    }
    println!("{}", get_c_error_msg());
    println!("{}", get_c_error_msg_by_ffi());
}

fn errno() -> c_int {
    unsafe { *libc::__error() }
}

fn get_c_error_msg_by_ffi() -> String {
    unsafe {
        std::ffi::CStr::from_ptr(libc::strerror(errno()))
            .to_str()
            .unwrap()
            .to_owned()
    }
}

fn get_c_error_msg() -> String {
    unsafe { get_c_chars_to_str(libc::strerror(errno())) }
}

fn get_c_chars_to_str(chars: *const c_char) -> String {
    unsafe {
        let strerr = chars as *mut u8;
        let length = libc::strlen(strerr as *const c_char) as usize;

        let mut string = String::with_capacity(length);

        for i in 0..length {
            let car = *strerr.offset(i as isize) as char;
            if car == (0 as char) {
                break;
            }
            string.push(car);
        }

        string
    }
}
