// linux source:
//   fs/proc/array.c >= v2.6.18
// statm => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/array.c#L476

use crate::pidentries::PidStat;
use crate::util::find_to_pos;
use crate::util::rfind_to_pos;
use cfg_iif::cfg_iif;

#[allow(unused_imports)]
use crate::util::find_to_opt;

#[derive(Debug, Default, Clone)]
pub struct PidStatParser();
impl PidStatParser {
    pub fn parse(&mut self, sl: &[u8]) -> PidStat {
        let mut stat = PidStat::default();
        if sl.is_empty() {
            return stat;
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
            let s = &sl[0..pos1 - 1];
            let input = String::from_utf8_lossy(s);
            stat.pid = input.as_ref().parse().unwrap();
        }
        // comm
        {
            pos2 = {
                let haystack = &sl[pos1..];
                let needle = b")";
                pos1 + rfind_to_pos(haystack, needle)
            };
            cfg_iif!(feature = "has_pidentry_stat_comm" {
                let s = &sl[pos1 + 1..pos2];
                stat.comm = String::from_utf8_lossy(s).into_owned();
            });
            //
            pos2 += 2;
        }
        // %c state
        {
            cfg_iif!(feature = "has_pidentry_stat_state" {
                stat.state = sl[pos2];
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
                    let input = String::from_utf8_lossy(s);
                    input.as_ref().parse().unwrap()
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
        stat
    }
}
