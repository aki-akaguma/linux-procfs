// linux source:
//   fs/proc/array.c >= v2.6.18
// statm => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/array.c#L476

use crate::pidentries::PidStat;
use crate::scanner::ProcScanner;
use crate::ProcResult;
use cfg_iif::cfg_iif;

#[cfg(feature = "has_pidentry_stat_comm")]
use crate::scanner::FromBytes;

#[derive(Debug, Default, Clone)]
pub(crate) struct PidStatParser();
impl PidStatParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<PidStat> {
        let mut stat = PidStat::default();
        if sl.is_empty() {
            return Ok(stat);
        }

        let mut sc = ProcScanner::new(sl);

        // pid
        stat.pid = sc.next(b' ')?;
        // comm
        sc.find_and_jump(b"(")?;
        cfg_iif!(feature = "has_pidentry_stat_comm" {
            stat.comm = FromBytes::from_bytes(sc.scan_until_last(b')')?)?;
        } else {
            sc.scan_until_last(b')')?;
        });
        // %c state
        sc.find_and_jump(b" ")?;
        cfg_iif!(feature = "has_pidentry_stat_state" {
             let state_bytes = sc.scan_until(b' ')?;
             if !state_bytes.is_empty() {
                 stat.state = state_bytes[0];
             }
        } else {
             sc.scan_until(b' ')?;
        });

        // now cursor is at start of ppid
        stat.ppid = sc.next(b' ')?;
        stat.pgrp = sc.next(b' ')?;

        cfg_iif!(feature = "has_pidentry_stat_session" {
            stat.session = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });

        cfg_iif!(feature = "has_pidentry_stat_tty_nr" {
            stat.tty_nr = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });

        cfg_iif!(feature = "has_pidentry_stat_tpgid" {
            stat.tpgid = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_flags" {
            stat.flags = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });

        cfg_iif!(feature = "has_pidentry_stat_minflt" {
            stat.minflt = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_cminflt" {
            stat.cminflt = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_majflt" {
            stat.majflt = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_cmajflt" {
            stat.cmajflt = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });

        stat.utime = sc.next(b' ')?;
        stat.stime = sc.next(b' ')?;
        stat.cutime = sc.next(b' ')?;
        stat.cstime = sc.next(b' ')?;

        cfg_iif!(feature = "has_pidentry_stat_priority" {
            stat.priority = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });

        stat.nice = sc.next(b' ')?;
        stat.num_threads = sc.next(b' ')?;
        sc.scan_until(b' ')?; // self.unused_1

        stat.starttime = sc.next(b' ')?;

        cfg_iif!(feature = "has_pidentry_stat_vsize" {
            stat.vsize = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_rss" {
            stat.rss = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_rlim" {
            stat.rlim = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });

        cfg_iif!(feature = "has_pidentry_stat_startcode" {
            stat.startcode = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_endcode" {
            stat.endcode = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_startstack" {
            stat.startstack = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_kstesp" {
            stat.kstesp = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_ksteip" {
            stat.ksteip = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });

        cfg_iif!(feature = "has_pidentry_stat_signal" {
            stat.signal = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_blocked" {
            stat.blocked = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_sigignore" {
            stat.sigignore = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_sigcatch" {
            stat.sigcatch = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });

        sc.scan_until(b' ')?; // self.unused_2
        sc.scan_until(b' ')?; // self.unused_3
        sc.scan_until(b' ')?; // self.unused_4

        cfg_iif!(feature = "has_pidentry_stat_exit_signal" {
            stat.exit_signal = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_processor" {
            stat.processor = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_rt_priority" {
            stat.rt_priority = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_policy" {
            stat.policy = sc.next(b' ')?;
        } else {
            sc.scan_until(b' ')?;
        });
        cfg_iif!(feature = "has_pidentry_stat_delayacct" {
            // The last field might be followed by ' ' or '\n' or EOF.
            // sc.next(b' ')? will fail if there is no space.
            // But usually there is a newline.
            if sc.check(b' ') {
                stat.delayacct_blkio_ticks = sc.next(b' ')?;
            } else {
                stat.delayacct_blkio_ticks = sc.next(b'\n')?;
            }
        } else {
            if sc.check(b' ') {
                sc.scan_until(b' ')?;
            } else {
                sc.scan_until(b'\n')?;
            }
        });

        Ok(stat)
    }
}
