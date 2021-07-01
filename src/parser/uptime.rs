use crate::uptime::Uptime;
use crate::util::find_to_pos;

#[derive(Debug, Default, Clone)]
pub struct UptimeParser();

impl UptimeParser {
    pub fn parse(&mut self, sl: &[u8]) -> Uptime {
        let mut uptime = Uptime::default();
        if sl.is_empty() {
            return uptime;
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
                        pos1 + find_to_pos(haystack, needle)
                    };
                    let s = &sl[pos1..pos2];
                    pos1 = pos2 + 1;
                    s
                }};
                ($needle:expr) => {{
                    let s = myscan!(skip, $needle);
                    let input = String::from_utf8_lossy(s);
                    input.as_ref().parse().unwrap()
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
        uptime
    }
}
