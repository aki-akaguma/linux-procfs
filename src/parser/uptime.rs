use crate::scanner::ProcScanner;
use crate::uptime::Uptime;
use crate::ProcResult;

#[derive(Debug, Default, Clone)]
pub(crate) struct UptimeParser();

impl UptimeParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<Uptime> {
        let mut uptime = Uptime::default();
        if sl.is_empty() {
            return Ok(uptime);
        }
        //
        let mut sc = ProcScanner::new(sl);
        //
        // content of /proc/uptime
        //   "611310.75 2218313.30\n"
        // on linux:
        //   "%lu.%02lu %lu.%02lu\n"
        uptime.seconds = sc.next(b' ')?;
        //
        Ok(uptime)
    }
}
