use crate::error::{ProcError, ProcResult};
use crate::util::{find_to_opt, skip_to_opt};

pub(crate) trait FromBytes: Sized {
    fn from_bytes(bytes: &[u8]) -> ProcResult<Self>;
}

impl FromBytes for u64 {
    fn from_bytes(bytes: &[u8]) -> ProcResult<Self> {
        if bytes.is_empty() {
            return Err(ProcError::ParseError);
        }
        let mut res = 0u64;
        for &b in bytes {
            if b.is_ascii_digit() {
                res = res * 10 + (b - b'0') as u64;
            } else {
                return Err(ProcError::ParseError);
            }
        }
        Ok(res)
    }
}

impl FromBytes for u32 {
    fn from_bytes(bytes: &[u8]) -> ProcResult<Self> {
        u64::from_bytes(bytes).map(|v| v as u32)
    }
}

impl FromBytes for usize {
    fn from_bytes(bytes: &[u8]) -> ProcResult<Self> {
        u64::from_bytes(bytes).map(|v| v as usize)
    }
}

impl FromBytes for i32 {
    fn from_bytes(bytes: &[u8]) -> ProcResult<Self> {
        if bytes.is_empty() {
            return Err(ProcError::ParseError);
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
                return Err(ProcError::ParseError);
            }
        }
        Ok(if is_neg { -res } else { res })
    }
}

impl FromBytes for i64 {
    fn from_bytes(bytes: &[u8]) -> ProcResult<Self> {
        if bytes.is_empty() {
            return Err(ProcError::ParseError);
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
                return Err(ProcError::ParseError);
            }
        }
        Ok(if is_neg { -res } else { res })
    }
}

impl FromBytes for i8 {
    fn from_bytes(bytes: &[u8]) -> ProcResult<Self> {
        i32::from_bytes(bytes).map(|v| v as i8)
    }
}

impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8]) -> ProcResult<Self> {
        let s = std::str::from_utf8(bytes).map_err(|_| ProcError::ParseError)?;
        s.parse().map_err(|_| ProcError::ParseError)
    }
}

impl FromBytes for String {
    fn from_bytes(bytes: &[u8]) -> ProcResult<Self> {
        std::str::from_utf8(bytes)
            .map(|s| s.to_string())
            .map_err(|_| ProcError::ParseError)
    }
}

pub(crate) struct ProcScanner<'a> {
    slice: &'a [u8],
    cursor: usize,
}

impl<'a> ProcScanner<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self { slice, cursor: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.cursor >= self.slice.len()
    }

    /// Scan until delimiter and return the slice before it.
    pub fn scan_until(&mut self, delimiter: u8) -> ProcResult<&'a [u8]> {
        let haystack = &self.slice[self.cursor..];
        let pos = find_to_opt(haystack, &[delimiter]).ok_or_else(|| {
            ProcError::UnexpectedFormat(format!("Delimiter '{}' not found", delimiter as char))
        })?;

        let start = self.cursor;
        let end = self.cursor + pos;
        self.cursor = end + 1;
        Ok(&self.slice[start..end])
    }

    /// Combined scan and parse.
    pub(crate) fn next<T: FromBytes>(&mut self, delimiter: u8) -> ProcResult<T> {
        let bytes = self.scan_until(delimiter)?;
        T::from_bytes(bytes)
    }

    /// Check if delimiter exists before the next newline or end of slice.
    pub fn check(&self, delimiter: u8) -> bool {
        let haystack = &self.slice[self.cursor..];
        if let Some(pos) = haystack.iter().position(|&b| b == delimiter) {
            if delimiter == b'\n' {
                return true;
            }
            if let Some(nl_pos) = haystack.iter().position(|&b| b == b'\n') {
                return pos < nl_pos;
            }
            return true;
        }
        false
    }

    /// Move cursor to the position after the first occurrence of needle.
    pub fn find_and_jump(&mut self, needle: &[u8]) -> ProcResult<()> {
        let haystack = &self.slice[self.cursor..];
        let pos = find_to_opt(haystack, needle).ok_or_else(|| {
            ProcError::UnexpectedFormat(format!(
                "Needle '{}' not found",
                String::from_utf8_lossy(needle)
            ))
        })?;
        self.cursor += pos + needle.len();
        Ok(())
    }

    /// Move cursor to the position after the first occurrence of needle, if it exists.
    pub fn find_and_jump_opt(&mut self, needle: &[u8]) -> bool {
        let haystack = &self.slice[self.cursor..];
        if let Some(pos) = find_to_opt(haystack, needle) {
            self.cursor += pos + needle.len();
            true
        } else {
            false
        }
    }

    pub fn skip_spaces(&mut self) {
        let haystack = &self.slice[self.cursor..];
        match skip_to_opt(haystack, b' ') {
            Some(pos) => self.cursor += pos,
            None => self.cursor = self.slice.len(),
        }
    }

    pub fn scan_until_any(&mut self, delimiters: &[u8]) -> ProcResult<&'a [u8]> {
        let haystack = &self.slice[self.cursor..];
        let mut min_pos = None;
        for &d in delimiters {
            if let Some(pos) = haystack.iter().position(|&b| b == d) {
                if min_pos.is_none() || pos < min_pos.unwrap() {
                    min_pos = Some(pos);
                }
            }
        }
        let pos =
            min_pos.ok_or_else(|| ProcError::UnexpectedFormat("No delimiter found".into()))?;

        let start = self.cursor;
        let end = self.cursor + pos;
        self.cursor = end + 1;
        Ok(&self.slice[start..end])
    }

    pub(crate) fn next_any<T: FromBytes>(&mut self, delimiters: &[u8]) -> ProcResult<T> {
        let bytes = self.scan_until_any(delimiters)?;
        T::from_bytes(bytes)
    }

    pub fn last_delimiter(&self) -> u8 {
        if self.cursor > 0 && self.cursor <= self.slice.len() {
            self.slice[self.cursor - 1]
        } else {
            0
        }
    }

    pub fn scan_until_last(&mut self, delimiter: u8) -> ProcResult<&'a [u8]> {
        let haystack = &self.slice[self.cursor..];
        let pos = haystack
            .iter()
            .rposition(|&b| b == delimiter)
            .ok_or_else(|| {
                ProcError::UnexpectedFormat(format!(
                    "Delimiter '{}' not found from end",
                    delimiter as char
                ))
            })?;

        let start = self.cursor;
        let end = self.cursor + pos;
        self.cursor = end + 1;
        Ok(&self.slice[start..end])
    }

    pub fn remainder(&self) -> &'a [u8] {
        &self.slice[self.cursor..]
    }
}
