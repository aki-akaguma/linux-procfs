use crate::loadavg::LoadAvg;
use crate::util::find_to_pos;

#[derive(Debug, Default, Clone)]
pub struct LoadAvgParser();

impl LoadAvgParser {
    pub fn parse(&mut self, sl: &[u8]) -> LoadAvg {
        let mut loadavg = LoadAvg::default();
        if sl.is_empty() {
            return loadavg;
        }
        //
        let mut pos1: usize = 0;
        let mut pos2: usize;
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
            // content of /proc/loadavg
            //   "0.36 0.45 0.34 1/957 8585\n"
            // on linux:
            //   "%d.%02d %d.%02d %d.%02d %ld/%d %d\n"
            //   LOAD_INT(a), LOAD_FRAC(a), ...
            //   nr_running(), nr_threads, last_pid
            loadavg.a1 = myscan!(b" ");
            loadavg.a5 = myscan!(b" ");
            loadavg.a15 = myscan!(b" ");
            myscan!(skip, b" ");
            loadavg.last_pid = myscan!(b"\n");
            let _ = pos1;
        }
        //
        loadavg
    }
}
