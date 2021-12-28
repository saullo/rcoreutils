fn main() {
    let tty = nix::unistd::ttyname(0);
    if tty.is_err() {
        let msg = std::ffi::CString::new("tty").unwrap();
        let msg = msg.as_ptr() as *const std::os::raw::c_char;

        unsafe {
            libc::perror(msg);
        }
    }

    let msg = tty.unwrap().into_os_string().into_string().unwrap();
    println!("{}", msg);
}
