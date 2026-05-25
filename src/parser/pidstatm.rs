// linux source:
//   fs/proc/array.c >= v2.6.18
// statm => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/array.c#L476

use crate::pidentries::PidStatm;
use crate::scanner::ProcScanner;
use crate::ProcResult;

#[derive(Debug, Default, Clone)]
pub(crate) struct PidStatmParser();
impl PidStatmParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<PidStatm> {
        let mut statm = PidStatm::default();
        if sl.is_empty() {
            return Ok(statm);
        }

        let mut sc = ProcScanner::new(sl);

        statm.size = sc.next(b' ')?;
        statm.resident = sc.next(b' ')?;
        statm.share = sc.next(b' ')?;
        statm.text = sc.next(b' ')?;
        statm.lib = sc.next(b' ')?;
        statm.data = sc.next(b' ')?;

        // Skip the 7th field (dt) if it exists
        if sc.check(b' ') {
            let _ = sc.scan_until(b' ');
        } else {
            let _ = sc.scan_until(b'\n');
        }

        Ok(statm)
    }
}
