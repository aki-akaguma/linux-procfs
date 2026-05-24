// linux source:
//   fs/proc/array.c >= v2.6.18
// cmdline => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/base.c#L431

use crate::pidentries::PidCmdline;
use crate::scanner::ProcScanner;
use crate::ProcResult;

#[derive(Debug, Default, Clone)]
pub struct PidCmdlineParser();
impl PidCmdlineParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<PidCmdline> {
        let mut cmdline = PidCmdline::default();
        if sl.is_empty() {
            return Ok(cmdline);
        }

        let mut sc = ProcScanner::new(sl);
        let mut args = Vec::new();
        
        while !sc.is_empty() {
            if sc.check(b'\0') {
                let arg = sc.scan_until(b'\0')?;
                if !arg.is_empty() {
                    args.push(String::from_utf8_lossy(arg));
                }
            } else {
                // Last part without trailing null
                let remaining = sc.remainder();
                if !remaining.is_empty() {
                    args.push(String::from_utf8_lossy(remaining));
                }
                break;
            }
        }
        
        cmdline.cmdline = args.join(" ").trim().to_string();

        Ok(cmdline)
    }
}
