use crate::loadavg::LoadAvg;
use crate::scanner::ProcScanner;
use crate::ProcResult;

#[derive(Debug, Default, Clone)]
pub struct LoadAvgParser();

impl LoadAvgParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<LoadAvg> {
        let mut loadavg = LoadAvg::default();
        if sl.is_empty() {
            return Ok(loadavg);
        }
        //
        let mut sc = ProcScanner::new(sl);
        // content of /proc/loadavg
        //   "0.36 0.45 0.34 1/957 8585\n"
        // on linux:
        //   "%d.%02d %d.%02d %d.%02d %ld/%d %d\n"
        //   LOAD_INT(a), LOAD_FRAC(a), ...
        //   nr_running(), nr_threads, last_pid
        loadavg.a1 = sc.next(b' ')?;
        loadavg.a5 = sc.next(b' ')?;
        loadavg.a15 = sc.next(b' ')?;
        sc.scan_until(b' ')?;
        loadavg.last_pid = sc.next(b'\n')?;
        //
        Ok(loadavg)
    }
}
