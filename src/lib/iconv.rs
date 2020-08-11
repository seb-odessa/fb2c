use std::io;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;
use libc::{c_char, size_t, c_int, c_void};
use result::Fb2Result;
use result::Fb2Error;
use std::collections::HashMap;

type Hash = HashMap<String, Converter>;
static mut CONV: Option<Hash> = None;


pub fn to_utf8(encoding: &str, data: &[u8]) -> Fb2Result<String> {
    let enc = encoding.to_lowercase();
    unsafe {
        let conv: &mut Hash = CONV.get_or_insert(Hash::new());
        if !conv.contains_key(&enc) {
            let converter = Converter::new(&enc, "utf-8")?;
            conv.insert(enc.clone(), converter);
        }
        if let Some(converter) = conv.get_mut(&enc) {
            let buffer = converter.utf8(&data)?;
            return Ok(String::from_utf8_lossy(&buffer).to_string());
        }
    }
    Err(Fb2Error::UnableToMakeUtf8)
}



#[allow(non_camel_case_types)]
type iconv_t = *mut c_void;

extern "C" {
    fn iconv_open(__tocode: *const c_char, __fromcode: *const c_char) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn iconv_close(__cd: iconv_t) -> c_int;
}

pub struct Converter {
    cd: iconv_t,
}
impl Converter {
    pub fn new(from: &str, to: &str) -> Fb2Result<Converter> {
        let from_encoding = CString::new(from).unwrap(); // Null ptr is not allowed in safe Rust
        let to_encoding = CString::new(to).unwrap();
        let handle = unsafe { iconv_open(to_encoding.as_ptr(), from_encoding.as_ptr()) };
        if handle as isize == -1 {
            return Err(Fb2Error::Custom(format!(
                "Error creating conversion descriptor from {:} to {:}",
                from,
                to
            )));
        }
        Ok(Converter { cd: handle })
    }

    /// Convert from input into UTF-8.
    /// Returns Fb2Result<Vec<u8>>l.
    pub fn utf8(&self, src: &[u8]) -> Fb2Result<Vec<u8>> {
        let expected_len = 4 * src.len();
        let mut dst = Vec::with_capacity(expected_len);
        dst.resize(expected_len, 0u8);
        let (_, length, ret) = self.convert(&src, &mut dst);
        if 0 != ret {
            return Err(Fb2Error::Custom(
                String::from("Unable to convert input buffer"),
            ));
        }
        if length == dst.len() {
            return Err(Fb2Error::Custom(String::from(
                "Unable to convert input buffer. Not enough destination buffer length",
            )));
        }
        dst.resize(length, 0u8);
        return Ok(dst);
    }

    /// Convert from input into output.
    /// Returns (bytes_read, bytes_written, errno).
    pub fn convert(&self, input: &[u8], output: &mut [u8]) -> (usize, usize, c_int) {
        let input_left = input.len() as size_t;
        let output_left = output.len() as size_t;

        if input_left > 0 && output_left > 0 {
            let input_ptr = input.as_ptr();
            let output_ptr = output.as_ptr();

            let ret = unsafe {
                iconv(
                    self.cd,
                    mem::transmute(&input_ptr),
                    mem::transmute(&input_left),
                    mem::transmute(&output_ptr),
                    mem::transmute(&output_left),
                )
            };
            let bytes_read = input.len() - input_left as usize;
            let bytes_written = output.len() - output_left as usize;

            return (
                bytes_read,
                bytes_written,
                if ret as isize == -1 {
                    io::Error::last_os_error().raw_os_error().unwrap() as c_int
                } else {
                    0
                },
            );
        } else if input_left == 0 && output_left > 0 {
            let output_ptr = output.as_ptr();

            let ret = unsafe {
                iconv(
                    self.cd,
                    ptr::null_mut::<*mut c_char>(),
                    mem::transmute(&input_left),
                    mem::transmute(&output_ptr),
                    mem::transmute(&output_left),
                )
            };

            let bytes_written = output.len() - output_left as usize;

            return (
                0,
                bytes_written,
                if ret as isize == -1 {
                    io::Error::last_os_error().raw_os_error().unwrap() as c_int
                } else {
                    0
                },
            );
        } else {
            let ret = unsafe {
                iconv(
                    self.cd,
                    ptr::null_mut::<*mut c_char>(),
                    mem::transmute(&input_left),
                    ptr::null_mut::<*mut c_char>(),
                    mem::transmute(&output_left),
                )
            };

            return (
                0,
                0,
                if ret as isize == -1 {
                    io::Error::last_os_error().raw_os_error().unwrap() as c_int
                } else {
                    0
                },
            );
        }
    }
}

impl Drop for Converter {
    fn drop(&mut self) {
        unsafe { iconv_close(self.cd) };
    }
}
