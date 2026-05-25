//#![allow(dead_code)]
use memx::memnechr;
use naive_opt::string_search_bytes;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

use super::Pid;

pub(crate) struct FileBuffer {
    pub buffer: Vec<u8>,
}

impl FileBuffer {
    pub(crate) fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(3000),
        }
    }
    pub(crate) fn least_capacity(&mut self, n: usize) {
        if self.buffer.capacity() < n {
            self.buffer.clear();
            self.buffer.reserve(n);
        }
    }
    pub(crate) fn clear(&mut self) {
        self.buffer.clear();
    }
    pub(crate) fn read_from_file(&mut self, file_handle: &mut fs::File) -> io::Result<usize> {
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
pub(crate) struct ProcFb {
    pub name: &'static str,
    pub capacity: usize,
}

impl ProcFb {
    pub(crate) fn try_update<'a>(
        &self,
        base_path: &Path,
        fb: &'a mut FileBuffer,
    ) -> io::Result<&'a [u8]> {
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

pub(crate) struct PidFb {
    pub name: &'static str,
    pub capacity: usize,
}

impl PidFb {
    pub(crate) fn try_update_with_pid<'a>(
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

pub(crate) struct SysCpuFb {
    pub name: &'static str,
    pub capacity: usize,
}

impl SysCpuFb {
    pub(crate) fn try_update_with_cpu_num<'a>(
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

pub(crate) fn find_to_opt(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    string_search_bytes(haystack, needle)
}

pub(crate) fn skip_to_opt(buffer: &[u8], byte: u8) -> Option<usize> {
    memnechr(buffer, byte)
}
