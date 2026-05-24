// linux source:
//   fs/proc/array.c >= v2.6.18
// statm => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/array.c#L476

use crate::pidentries::PidStat;
use crate::util::{find_to_pos, rfind_to_pos, FromBytes};
use crate::ProcResult;
use cfg_iif::cfg_iif;

#[allow(unused_imports)]
use crate::util::find_to_opt;

#[derive(Debug, Default, Clone)]
pub struct PidStatParser();
impl PidStatParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<PidStat> {
        let mut stat = PidStat::default();
        if sl.is_empty() {
            return Ok(stat);
        }
        //
        let mut pos1: usize;
        let mut pos2: usize;
        //
        // pid
        {
            pos1 = {
                let haystack = &sl[0..];
                let needle = b"(";
                find_to_pos(haystack, needle)
            };
            if pos1 == 0 || pos1 >= sl.len() {
                return Err(crate::ProcError::ParseError);
            }
            let s = &sl[0..pos1 - 1];
            stat.pid = FromBytes::from_bytes(s)?;
        }
        // comm
        {
            pos2 = {
                let haystack = &sl[pos1..];
                let needle = b")";
                pos1 + rfind_to_pos(haystack, needle)
            };
            if pos2 >= sl.len() {
                return Err(crate::ProcError::ParseError);
            }
            cfg_iif!(feature = "has_pidentry_stat_comm" {
                let s = &sl[pos1 + 1..pos2];
                stat.comm = FromBytes::from_bytes(s)?;
            });
            //
            pos2 += 2;
        }
        // %c state
        {
            cfg_iif!(feature = "has_pidentry_stat_state" {
                if pos2 < sl.len() {
                    stat.state = sl[pos2];
                }
            });
            pos1 = pos2 + 2;
        }
        //
        {
            macro_rules! myscan {
                (skip) => {{
                    pos2 = {
                        let haystack = &sl[pos1..];
                        let needle = b" ";
                        pos1 + find_to_pos(haystack, needle)
                    };
                    let s = &sl[pos1..pos2];
                    pos1 = pos2 + 1;
                    s
                }};
                () => {{
                    let s = myscan!(skip);
                    FromBytes::from_bytes(s)?
                }};
            }
            stat.ppid = myscan!();
            stat.pgrp = myscan!();
            //
            cfg_iif!(feature = "has_pidentry_stat_session" {
                stat.session = myscan!();
            } else {
                myscan!(skip);
            });
            //
            cfg_iif!(feature = "has_pidentry_stat_tty_nr" {
                stat.tty_nr = myscan!();
            } else {
                myscan!(skip);
            });
            //
            cfg_iif!(feature = "has_pidentry_stat_tpgid" {
                stat.tpgid = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_flags" {
                stat.flags = myscan!();
            } else {
                myscan!(skip);
            });
            //
            cfg_iif!(feature = "has_pidentry_stat_minflt" {
                stat.minflt = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_cminflt" {
                stat.cminflt = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_majflt" {
                stat.majflt = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_cmajflt" {
                stat.cmajflt = myscan!();
            } else {
                myscan!(skip);
            });
            //
            stat.utime = myscan!();
            stat.stime = myscan!();
            stat.cutime = myscan!();
            stat.cstime = myscan!();
            //
            cfg_iif!(feature = "has_pidentry_stat_priority" {
                stat.priority = myscan!();
            } else {
                myscan!(skip);
            });
            //
            stat.nice = myscan!();
            stat.num_threads = myscan!();
            myscan!(skip); // self.unused_1 = myscan!();
                           //
            stat.starttime = myscan!();
            //
            cfg_iif!(feature = "has_pidentry_stat_vsize" {
                stat.vsize = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_rss" {
                stat.rss = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_rlim" {
                stat.rlim = myscan!();
            } else {
                myscan!(skip);
            });
            //
            cfg_iif!(feature = "has_pidentry_stat_startcode" {
                stat.startcode = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_endcode" {
                stat.endcode = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_startstack" {
                stat.startstack = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_kstesp" {
                stat.kstesp = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_ksteip" {
                stat.ksteip = myscan!();
            } else {
                myscan!(skip);
            });
            //
            cfg_iif!(feature = "has_pidentry_stat_signal" {
                stat.signal = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_blocked" {
                stat.blocked = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_sigignore" {
                stat.sigignore = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_sigcatch" {
                stat.sigcatch = myscan!();
            } else {
                myscan!(skip);
            });
            //
            myscan!(skip); // self.unused_2 = myscan!();
            myscan!(skip); // self.unused_3 = myscan!();
            myscan!(skip); // self.unused_4 = myscan!();
                           //
            cfg_iif!(feature = "has_pidentry_stat_exit_signal" {
                stat.exit_signal = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_processor" {
                stat.processor = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_rt_priority" {
                stat.rt_priority = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_policy" {
                stat.policy = myscan!();
            } else {
                myscan!(skip);
            });
            cfg_iif!(feature = "has_pidentry_stat_delayacct" {
                stat.delayacct_blkio_ticks = myscan!();
            } else {
                myscan!(skip);
            });
            let _ = pos1;
        }
        Ok(stat)
    }
}
