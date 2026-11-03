use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use libc::{c_void, RTLD_NEXT};

#[no_mangle]
pub extern "C" fn execve(
    filename: *const c_char,
    argv: *const *const c_char,
    envp: *const *const c_char,
) -> c_int {
    unsafe {
        let real_execve: extern "C" fn(
            *const c_char,
            *const *const c_char,
            *const *const c_char,
        ) -> c_int = std::mem::transmute(libc::dlsym(RTLD_NEXT, b"execve\0".as_ptr() as *const c_char));

        if !filename.is_null() {
            let c_str = CStr::from_ptr(filename);
            if let Ok(path_str) = c_str.to_str() {
                if path_str.contains("bwrap") {
                    let mut i = 0;
                    while !(*argv.offset(i)).is_null() {
                        let arg_str = CStr::from_ptr(*argv.offset(i)).to_str().unwrap_or("");
                        if arg_str == "--" {
                            let next_idx = i + 1;
                            if !(*argv.offset(next_idx)).is_null() {
                                return real_execve(*argv.offset(next_idx), argv.offset(next_idx), envp);
                            }
                        }
                        i += 1;
                    }
                }
            }
        }

        real_execve(filename, argv, envp)
    }
}
