use crate::loadavg::LoadAvg;
use crate::util::find_to_opt;
use crate::ProcResult;
use crate::error::ProcError;

#[derive(Debug, Default, Clone)]
pub struct LoadAvgParser();

impl LoadAvgParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<LoadAvg> {
        let mut loadavg = LoadAvg::default();
        if sl.is_empty() {
            return Ok(loadavg);
        }
        //
        let mut pos1: usize = 0;
        let mut pos2: usize;
        {
            macro_rules! try_scan {
                (skip, $needle:expr) => {{
                    pos2 = {
                        let haystack = &sl[pos1..];
                        let needle = $needle;
                        pos1 + find_to_opt(haystack, needle)
                            .ok_or_else(|| ProcError::UnexpectedFormat("Delimiter not found".into()))?
                    };
                    let s = &sl[pos1..pos2];
                    pos1 = pos2 + 1;
                    s
                }};
                ($needle:expr) => {{
                    let s = try_scan!(skip, $needle);
                    let input = std::str::from_utf8(s)?;
                    input.parse().map_err(|_| ProcError::UnexpectedFormat(format!("Parse error: {}", input)))?
                }};
            }
            // content of /proc/loadavg
            //   "0.36 0.45 0.34 1/957 8585\n"
            // on linux:
            //   "%d.%02d %d.%02d %d.%02d %ld/%d %d\n"
            //   LOAD_INT(a), LOAD_FRAC(a), ...
            //   nr_running(), nr_threads, last_pid
            loadavg.a1 = try_scan!(b" ");
            loadavg.a5 = try_scan!(b" ");
            loadavg.a15 = try_scan!(b" ");
            try_scan!(skip, b" ");
            loadavg.last_pid = try_scan!(b"\n");
            let _ = pos1;
        }
        //
        Ok(loadavg)
    }
}
