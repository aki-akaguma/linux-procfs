use crate::error::ProcError;
use crate::uptime::Uptime;
use crate::util::{find_to_opt, FromBytes};
use crate::ProcResult;

#[derive(Debug, Default, Clone)]
pub struct UptimeParser();

impl UptimeParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<Uptime> {
        let mut uptime = Uptime::default();
        if sl.is_empty() {
            return Ok(uptime);
        }
        //
        let mut pos1: usize = 0;
        let pos2: usize;
        {
            macro_rules! myscan {
                (skip, $needle:expr) => {{
                    pos2 = {
                        let haystack = &sl[pos1..];
                        let needle = $needle;
                        pos1 + find_to_opt(haystack, needle).ok_or_else(|| {
                            ProcError::UnexpectedFormat("Delimiter not found".into())
                        })?
                    };
                    let s = &sl[pos1..pos2];
                    pos1 = pos2 + 1;
                    s
                }};
                ($needle:expr) => {{
                    let s = myscan!(skip, $needle);
                    FromBytes::from_bytes(s)?
                }};
            }
            //
            // content of /proc/uptime
            //   "611310.75 2218313.30\n"
            // on linux:
            //   "%lu.%02lu %lu.%02lu\n"
            uptime.seconds = myscan!(b" ");
            let _ = pos1;
        }
        //
        Ok(uptime)
    }
}
