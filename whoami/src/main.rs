//! # whoami
//! 显示当前进程所激活的用户名

use clap::App;
use clap::{crate_authors, crate_description, crate_name, crate_version};

fn cli_parser() -> App<'static, 'static> {
    App::new(crate_name!())
        .author(crate_authors!(";"))
        .version(crate_version!())
        .about(crate_description!())
}

fn main() {
    cli_parser().get_matches();

    match os_whoami() {
        Ok(name) => println!("{}", name),
        Err(e) => panic!("{:?}", e),
    }
}

#[cfg(target_os = "linux")]
use self::posix::os_whoami;

#[cfg(target_os = "linux")]
mod posix {
    use std::io::Error;

    pub fn os_whoami() -> Result<String, Error> {
        let name = unsafe {
            // ANCHOR: c_err1
            let uid = geteuid();
            if uid == std::u32::MAX {
                return Err(Error::last_os_error());
            }
            // ANCHOR_END: c_err1

            // ANCHOR: c_err2
            let pw = getpwuid(uid);
            if pw.is_null() {
                return Err(Error::last_os_error());
            }
            // ANCHOR_END: c_err2

            // ANCHOR: c_str
            let name = std::ffi::CStr::from_ptr((*pw).name).to_bytes();
            let name = String::from_utf8_lossy(name).to_string();
            // ANCHOR_END: c_str
            name
        };

        Ok(name)
    }

    // ANCHOR: c_types
    use std::os::raw::c_char;
    /// 根据 POSIX 标准，uid_t, gid_t 是 32 位整数，但未提及有无符号；
    /// 在 glibc 中的实现是 unsigned int
    use std::os::raw::{c_uint as uid_t, c_uint as gid_t};
    // ANCHOR_END: c_types

    /// 参考文档：[The Data Structure that Describes a User](https://ftp.gnu.org/old-gnu/Manuals/glibc-2.2.3/html_node/libc_608.html)
    /// 用来描述 /etc/passwd 中的条目
    // ANCHOR: struct_align
    #[repr(C)]
    struct Passwd {
        name: *const c_char,
        passwd: *const c_char,
        uid: uid_t,
        gid: gid_t,
        gecos: *const c_char,
        dir: *const c_char,
        shell: *const c_char,
    }
    // ANCHOR_END: struct_align

    // ANCHOR: extern_c
    extern "C" {
        /// `uid_t geteuid(void);`
        fn geteuid() -> uid_t;
        /// `struct passwd* geteuid(uid_t);`
        fn getpwuid(uid: uid_t) -> *const Passwd;
    }
    // ANCHOR_END: extern_c
}
