pub(crate) fn puts(str: &str, kp: fn(s: *const u8)) {
    let mut buf = [0 as u8; 128];
    let mut e_index = 0;
    for (index, ch) in str.bytes().enumerate() {
        buf[index % 127] = ch;
        if index != 0 && index % 126 == 0 {
            buf[127] = 0;
            kp(buf.as_ptr())
        }
        e_index = index;
    }
    if e_index % 126 != 0 {
        buf[e_index % 127 + 1] = 0;
        kp(buf.as_ptr())
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    lazy_static! {
        static ref OUT: Mutex<String> = Mutex::new(String::new());
    }

    #[test]
    fn test_puts() {
        fn kputs(s: *const u8) {
            unsafe {
                use std::ffi::CStr;
                let a = CStr::from_ptr(s as _);
                let mut out = OUT.lock().unwrap();
                out.push_str(a.to_str().unwrap())
            }
        }

        let data = r"
            This is a very long string.
            This is a very long string.
            This is a very long string.
            This is a very long string.
            This is a very long string.
            This is a very long string.
        ";

        puts(data, kputs);

        assert!(data.len() > 128);
        let out = OUT.lock().unwrap();
        assert_eq!(data.cmp(out.as_str()), Ordering::Equal);
    }
}