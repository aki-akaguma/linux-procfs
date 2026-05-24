//#![allow(dead_code)]
use memx::memnechr;
use naive_opt::{string_rsearch_bytes, string_search_bytes};
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

use super::Pid;

pub struct FileBuffer {
    pub buffer: Vec<u8>,
}

impl FileBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(3000),
        }
    }
    pub fn least_capacity(&mut self, n: usize) {
        if self.buffer.capacity() < n {
            self.buffer.clear();
            self.buffer.reserve(n);
        }
    }
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
    pub fn read_from_file(&mut self, file_handle: &mut fs::File) -> io::Result<usize> {
        self.buffer.clear();
        #[cfg(not(windows))]
        {
            file_handle.read_to_end(&mut self.buffer)
        }
        #[cfg(windows)]
        {
            let mut buf: Vec<u8> = vec![];
            let _ = file_handle.read_to_end(&mut buf)?;
            for &a in buf.iter() {
                if a != b'\r' {
                    self.buffer.push(a);
                }
            }
            Ok(self.buffer.len())
        }
    }
}
impl Default for FileBuffer {
    fn default() -> Self {
        Self::new()
    }
}
pub struct ProcFb {
    pub name: &'static str,
    pub capacity: usize,
}

impl ProcFb {
    pub fn try_update<'a>(&self, base_path: &Path, fb: &'a mut FileBuffer) -> io::Result<&'a [u8]> {
        fb.clear();
        fb.least_capacity(self.capacity);
        {
            let name = base_path.join("proc").join(self.name);
            let mut fh = fs::OpenOptions::new().read(true).open(&name)?;
            fb.read_from_file(&mut fh)?;
        }
        Ok(fb.buffer.as_slice())
    }
}

pub struct PidFb {
    pub name: &'static str,
    pub capacity: usize,
}

impl PidFb {
    pub fn try_update_with_pid<'a>(
        &self,
        base_path: &Path,
        fb: &'a mut FileBuffer,
        pid: Pid,
    ) -> io::Result<&'a [u8]> {
        fb.clear();
        fb.least_capacity(self.capacity);
        {
            let name = base_path.join("proc").join(pid.to_string()).join(self.name);
            let mut fh = fs::OpenOptions::new().read(true).open(&name)?;
            fb.read_from_file(&mut fh)?;
        }
        Ok(fb.buffer.as_slice())
    }
}

pub struct SysCpuFb {
    pub name: &'static str,
    pub capacity: usize,
}

impl SysCpuFb {
    pub fn try_update_with_cpu_num<'a>(
        &self,
        base_path: &Path,
        fb: &'a mut FileBuffer,
        cpu_num: usize,
    ) -> io::Result<&'a [u8]> {
        fb.clear();
        fb.least_capacity(self.capacity);
        {
            let cpu_dir = format!("cpu{}", cpu_num);
            let name = base_path
                .join("sys")
                .join("devices")
                .join("system")
                .join("cpu")
                .join(cpu_dir)
                .join(self.name);
            let mut fh = fs::OpenOptions::new().read(true).open(&name)?;
            fb.read_from_file(&mut fh)?;
        }
        Ok(fb.buffer.as_slice())
    }
}

//
/*
pub fn _0_find_to_opt(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
*/

pub fn find_to_opt(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    string_search_bytes(haystack, needle)
}

pub fn find_to_pos(haystack: &[u8], needle: &[u8]) -> usize {
    let o = find_to_opt(haystack, needle);
    match o {
        Some(pos) => pos,
        None => {
            /*
            eprintln!("haystack:: \"{}\"", String::from_utf8_lossy(haystack));
            eprintln!("needle:: \"{}\"", String::from_utf8_lossy(needle));
             */
            unreachable!();
        }
    }
}

//
pub fn rfind_to_opt(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    string_rsearch_bytes(haystack, needle)
}

pub fn rfind_to_pos(haystack: &[u8], needle: &[u8]) -> usize {
    let o = rfind_to_opt(haystack, needle);
    match o {
        Some(pos) => pos,
        None => {
            /*
            eprintln!("haystack:: \"{}\"", String::from_utf8_lossy(haystack));
            eprintln!("needle:: \"{}\"", String::from_utf8_lossy(needle));
            */
            unreachable!();
        }
    }
}

//
/*
pub fn _0_skip_to_opt(buffer: &[u8], byte: u8) -> Option<usize> {
    buffer.iter().position(|&x| x != byte)
}
*/

pub fn skip_to_opt(buffer: &[u8], byte: u8) -> Option<usize> {
    memnechr(buffer, byte)
}

pub fn skip_to_pos(buffer: &[u8], byte: u8) -> usize {
    let o = skip_to_opt(buffer, byte);
    match o {
        Some(pos) => pos,
        None => {
            /*
            eprintln!("buffer:: \"{}\"", String::from_utf8_lossy(buffer));
            eprintln!("byte:: \"{}\"", String::from_utf8_lossy(byte));
            */
            unreachable!();
        }
    }
}

pub trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> super::ProcResult<Self>;
}

impl FromBytes for u64 {
    fn from_bytes(bytes: &[u8]) -> super::ProcResult<Self> {
        if bytes.is_empty() {
            return Err(super::ProcError::ParseError);
        }
        let mut res = 0u64;
        for &b in bytes {
            if b.is_ascii_digit() {
                res = res * 10 + (b - b'0') as u64;
            } else {
                return Err(super::ProcError::ParseError);
            }
        }
        Ok(res)
    }
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> super::ProcResult<Self> {
        u64::from_bytes(bytes).map(|v| v as u32)
    }
}

impl FromBytes for usize {
    fn from_bytes(bytes: &[u8]) -> super::ProcResult<Self> {
        u64::from_bytes(bytes).map(|v| v as usize)
    }
}

impl FromBytes for i32 {
    fn from_bytes(bytes: &[u8]) -> super::ProcResult<Self> {
        if bytes.is_empty() {
            return Err(super::ProcError::ParseError);
        }
        let (is_neg, start) = if bytes[0] == b'-' {
            (true, 1)
        } else {
            (false, 0)
        };
        let mut res = 0i32;
        for &b in &bytes[start..] {
            if b.is_ascii_digit() {
                res = res * 10 + (b - b'0') as i32;
            } else {
                return Err(super::ProcError::ParseError);
            }
        }
        Ok(if is_neg { -res } else { res })
    }
}

impl FromBytes for i64 {
    fn from_bytes(bytes: &[u8]) -> super::ProcResult<Self> {
        if bytes.is_empty() {
            return Err(super::ProcError::ParseError);
        }
        let (is_neg, start) = if bytes[0] == b'-' {
            (true, 1)
        } else {
            (false, 0)
        };
        let mut res = 0i64;
        for &b in &bytes[start..] {
            if b.is_ascii_digit() {
                res = res * 10 + (b - b'0') as i64;
            } else {
                return Err(super::ProcError::ParseError);
            }
        }
        Ok(if is_neg { -res } else { res })
    }
}

impl FromBytes for i8 {
    fn from_bytes(bytes: &[u8]) -> super::ProcResult<Self> {
        i32::from_bytes(bytes).map(|v| v as i8)
    }
}

impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8]) -> super::ProcResult<Self> {
        let s = std::str::from_utf8(bytes).map_err(|_| super::ProcError::ParseError)?;
        s.parse().map_err(|_| super::ProcError::ParseError)
    }
}

impl FromBytes for String {
    fn from_bytes(bytes: &[u8]) -> super::ProcResult<Self> {
        std::str::from_utf8(bytes)
            .map(|s| s.to_string())
            .map_err(|_| super::ProcError::ParseError)
    }
}
