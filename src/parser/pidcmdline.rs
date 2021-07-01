// linux source:
//   fs/proc/array.c >= v2.6.18
// cmdline => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/base.c#L431

use crate::pidentries::PidCmdline;

#[derive(Debug, Default, Clone)]
pub struct PidCmdlineParser();
impl PidCmdlineParser {
    pub fn parse(&mut self, sl: &[u8]) -> PidCmdline {
        let mut cmdline = PidCmdline::default();
        if sl.is_empty() {
            return cmdline;
        }
        //
        let v: Vec<u8> = sl.iter().map(|&b| if b == 0 { b' ' } else { b }).collect();
        cmdline.cmdline = String::from_utf8_lossy(&v).as_ref().trim().to_string();
        //
        cmdline
    }
}
