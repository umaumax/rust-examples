use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use std::env;

extern "C" fn ld_preload_initialise_fn() {
    // NOTE: run before main()
    let init_process_flag = match env::var("STOP_THE_WORLD_INIT") {
        Ok(v) => v.parse::<i32>().unwrap() != 0,
        Err(_) => true,
    };
    if init_process_flag {
        signal::kill(Pid::from_raw(std::process::id() as i32), Signal::SIGSTOP).unwrap();
    }
}

extern "C" fn ld_preload_deinitialise_fn() {
    // NOTE: run after main()
    let term_process_flag = match env::var("STOP_THE_WORLD_TERM") {
        Ok(v) => v.parse::<i32>().unwrap() != 0,
        Err(_) => false,
    };
    if term_process_flag {
        signal::kill(Pid::from_raw(std::process::id() as i32), Signal::SIGSTOP).unwrap();
    }
}

#[cfg(any(target_os = "macos"))]
#[no_mangle]
#[link_section = "__DATA,__mod_init_func"]
pub static LD_PRELOAD_INITIALISE_RUST: extern "C" fn() = self::ld_preload_initialise_fn;

#[cfg(not(target_os = "macos"))]
#[no_mangle]
#[link_section = ".init_array"]
pub static LD_PRELOAD_INITIALISE_RUST: extern "C" fn() = self::ld_preload_initialise_fn;

#[cfg(any(target_os = "macos"))]
#[no_mangle]
#[link_section = "__DATA,__mod_term_func"]
pub static LD_PRELOAD_DEINITIALISE_RUST: extern "C" fn() = self::ld_preload_deinitialise_fn;

#[cfg(not(target_os = "macos"))]
#[no_mangle]
#[link_section = ".fini_array"]
pub static LD_PRELOAD_DEINITIALISE_RUST: extern "C" fn() = self::ld_preload_deinitialise_fn;
