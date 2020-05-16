use libc::c_char;
use libc::c_void;
use libc::uid_t;

#[no_mangle]
pub unsafe extern "C" fn getuid() -> uid_t {
    get_getuid().unwrap()()
}

fn get_getuid() -> Result<unsafe extern "C" fn() -> uid_t, &'static str> {
    use std::sync::Once;
    static mut REAL_PTR: *const u8 = 0 as *const u8;
    static mut ONCE: Once = Once::new();
    static mut ERROR_MSG: String = String::new();
    unsafe {
        ONCE.call_once(|| {
            let rtld_next = -1_isize as *mut c_void;
            let ptr = libc::dlsym(rtld_next, "getuid\0".as_ptr() as *const c_char);
            if ptr.is_null() {
                ERROR_MSG = std::ffi::CStr::from_ptr(libc::dlerror())
                    .to_str()
                    .unwrap()
                    .to_owned();
            }
            REAL_PTR = ptr as *const u8;
        });
        if !ERROR_MSG.is_empty() {
            return Err(&ERROR_MSG);
        }
        Ok(std::mem::transmute(REAL_PTR))
    }
}
pub unsafe fn root_user_id() -> uid_t {
    0
}
