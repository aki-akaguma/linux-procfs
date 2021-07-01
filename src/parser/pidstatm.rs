// linux source:
//   fs/proc/array.c >= v2.6.18
// statm => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/array.c#L476

use crate::pidentries::PidStatm;
use crate::util::find_to_pos;

#[derive(Debug, Default, Clone)]
pub struct PidStatmParser();
impl PidStatmParser {
    pub fn parse(&mut self, sl: &[u8]) -> PidStatm {
        let mut statm = PidStatm::default();
        if sl.is_empty() {
            return statm;
        }
        //
        let mut pos1: usize = 0;
        let mut pos2: usize;
        {
            //
            // content of statm
            //   4319 435 401 121 0 1079 0
            // on linux:
            //      "%d %d %d %d %d %d %d\n"
            //
            macro_rules! myscan {
                () => {{
                    pos2 = {
                        let haystack = &sl[pos1..];
                        let needle = b" ";
                        pos1 + find_to_pos(haystack, needle)
                    };
                    let s = &sl[pos1..pos2];
                    pos1 = pos2 + 1;
                    let input = String::from_utf8_lossy(s);
                    input.as_ref().parse().unwrap()
                }};
            }
            statm.size = myscan!();
            statm.resident = myscan!();
            statm.share = myscan!();
            statm.text = myscan!();
            statm.lib = myscan!();
            statm.data = myscan!();
            let _ = pos1;
        }
        statm
    }
}
