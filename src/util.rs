//#![allow(dead_code)]
use memx::memnechr;
use naive_opt::{string_rsearch_bytes, string_search_bytes};
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

use super::Pid;

pub struct FileBuffer {
    buffer: Vec<u8>,
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
    pub fn update<'a>(&self, base_path: &Path, fb: &'a mut FileBuffer) -> &'a [u8] {
        fb.clear();
        fb.least_capacity(self.capacity);
        {
            let name = format!("{}/proc/{}", base_path.to_str().unwrap(), self.name);
            let mut fh = match fs::OpenOptions::new().read(true).open(&name) {
                Ok(fh) => fh,
                Err(_) => return fb.buffer.as_slice(),
            };
            let _len = match fb.read_from_file(&mut fh) {
                Ok(len) => len,
                Err(err) => {
                    eprintln!("{}: `{}`", err, &name);
                    return fb.buffer.as_slice();
                }
            };
        }
        fb.buffer.as_slice()
    }
}

pub struct PidFb {
    pub name: &'static str,
    pub capacity: usize,
}

impl PidFb {
    pub fn update_with_pid<'a>(
        &self,
        base_path: &Path,
        fb: &'a mut FileBuffer,
        pid: Pid,
    ) -> &'a [u8] {
        fb.clear();
        fb.least_capacity(self.capacity);
        {
            let name = format!("{}/proc/{}/{}", base_path.to_str().unwrap(), pid, self.name);
            let mut fh = match fs::OpenOptions::new().read(true).open(&name) {
                Ok(fh) => fh,
                Err(_) => return fb.buffer.as_slice(),
            };
            let _len = match fb.read_from_file(&mut fh) {
                Ok(len) => len,
                Err(err) => {
                    eprintln!("{}: `{}`", err, &name);
                    return fb.buffer.as_slice();
                }
            };
        }
        fb.buffer.as_slice()
    }
}

pub struct SysCpuFb {
    pub name: &'static str,
    pub capacity: usize,
}

impl SysCpuFb {
    pub fn update_with_cpu_num<'a>(
        &self,
        base_path: &Path,
        fb: &'a mut FileBuffer,
        cpu_num: usize,
    ) -> &'a [u8] {
        fb.clear();
        fb.least_capacity(self.capacity);
        {
            let name = format!(
                "{}/sys/devices/system/cpu/cpu{}/{}",
                base_path.to_str().unwrap(),
                cpu_num,
                self.name
            );
            let mut fh = match fs::OpenOptions::new().read(true).open(&name) {
                Ok(fh) => fh,
                Err(_) => return fb.buffer.as_slice(),
            };
            let _len = match fb.read_from_file(&mut fh) {
                Ok(len) => len,
                Err(err) => {
                    eprintln!("{}: `{}`", err, &name);
                    return fb.buffer.as_slice();
                }
            };
        }
        fb.buffer.as_slice()
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
