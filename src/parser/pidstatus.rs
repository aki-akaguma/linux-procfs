// linux source:
//   fs/proc/array.c >= v2.6.18
// status => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/array.c#L296
// status vm => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/task_mmu.c#L14

use crate::pidentries::PidStatus;
use crate::util::{find_to_pos, skip_to_pos};
use cfg_iif::cfg_iif;

#[allow(unused_imports)]
use crate::util::find_to_opt;

#[derive(Debug, Default, Clone)]
pub struct PidStatusParser();
impl PidStatusParser {
    pub fn parse(&mut self, sl: &[u8]) -> PidStatus {
        let mut status = PidStatus::default();
        if sl.is_empty() {
            return status;
        }
        //
        //
        let mut pos1: usize;
        let mut pos2: usize = 0;
        // Name
        cfg_iif!(feature = "has_pidentry_status_name" {
            macro_rules! myscan {
                (skip, $needle:expr) => {{
                    pos1 = {
                        let haystack = &sl[pos2..];
                        let needle = $needle;
                        pos2 + $needle.len() + find_to_pos(haystack, needle)
                    };
                    pos2 = {
                        let haystack = &sl[pos1..];
                        let needle = b"\n";
                        pos1 + find_to_pos(haystack, needle)
                    };
                }};
                ($needle:expr) => {{
                    myscan!(skip, $needle);
                    let s = &sl[pos1..pos2];
                    let input = String::from_utf8_lossy(s);
                    input.into_owned()
                }};
            }
            status.name = myscan!(b"Name:\t");
        });
        cfg_iif!(feature = "has_pidentry_status_state" {
            macro_rules! myscan {
                (skip, $needle:expr) => {{
                    pos1 = {
                        let haystack = &sl[pos2..];
                        let needle = $needle;
                        pos2 + $needle.len() + find_to_pos(haystack, needle)
                    };
                    pos2 = {
                        let haystack = &sl[pos1..];
                        let needle = b"\n";
                        pos1 + find_to_pos(haystack, needle)
                    };
                }};
                ($needle:expr) => {{
                    myscan!(skip, $needle);
                    if pos1 < pos2 {
                        sl[pos1]
                    } else {
                        0
                    }
                }};
            }
            status.state = myscan!(b"State:\t");
        });
        // Tgid, Ngid, Pid, PPid
        {
            macro_rules! myscan {
                (skip, $needle:expr) => {{
                    pos1 = {
                        let haystack = &sl[pos2..];
                        let needle = $needle;
                        pos2 + $needle.len() + find_to_pos(haystack, needle)
                    };
                    pos2 = {
                        let haystack = &sl[pos1..];
                        let needle = b"\n";
                        pos1 + find_to_pos(haystack, needle)
                    };
                }};
                ($needle:expr) => {{
                    myscan!(skip, $needle);
                    let s = &sl[pos1..pos2];
                    let input = String::from_utf8_lossy(s);
                    input.as_ref().parse().unwrap()
                }};
            }
            cfg_iif!(feature = "has_pidentry_status_tgid" {
                status.tgid = myscan!(b"Tgid:\t");
            });
            cfg_iif!(feature = "has_pidentry_status_ngid" {
                status.ngid = myscan!(b"Ngid:\t");
            });
            {
                status.pid = myscan!(b"Pid:\t");
                status.ppid = myscan!(b"PPid:\t");
            }
            cfg_iif!(feature = "has_pidentry_status_tracer_pid" {
                status.tracer_pid = myscan!(b"TracerPid:\t");
            });
            let _ = pos2;
        }
        // Uid, Gid
        {
            macro_rules! myscan {
                ($needle:expr) => {{
                    pos2 = {
                        let haystack = &sl[pos1..];
                        let needle = $needle;
                        pos1 + find_to_pos(haystack, needle)
                    };
                    let s = &sl[pos1..pos2];
                    pos1 = pos2 + 1;
                    let input = String::from_utf8_lossy(s);
                    input.as_ref().parse().unwrap()
                }};
            }
            pos1 = {
                let haystack = &sl[pos1..];
                let needle = b"Uid:\t";
                pos1 + needle.len() + find_to_pos(haystack, needle)
            };
            status.ruid = myscan!(b"\t");
            status.euid = myscan!(b"\t");
            status.suid = myscan!(b"\t");
            status.fuid = myscan!(b"\n");
            //
            pos1 = {
                let haystack = &sl[pos1..];
                let needle = b"Gid:\t";
                pos1 + needle.len() + find_to_pos(haystack, needle)
            };
            status.rgid = myscan!(b"\t");
            status.egid = myscan!(b"\t");
            status.sgid = myscan!(b"\t");
            status.fgid = myscan!(b"\n");
        }
        // VmPeak, VmSize, VmLck, ...
        {
            macro_rules! myscan {
                (check, $needle:expr) => {{
                    {
                        let haystack = &sl[pos1..];
                        let needle = $needle;
                        match find_to_opt(haystack, needle) {
                            Some(_pos) => true,
                            None => false,
                        }
                    }
                }};
                (skip_spaces) => {{
                    pos1 += skip_to_pos(&sl[pos1..], b' ');
                }};
                (skip, $needle:expr) => {{
                    pos2 = {
                        let haystack = &sl[pos1..];
                        let needle = $needle;
                        pos1 + find_to_pos(haystack, needle)
                    };
                    let s = &sl[pos1..pos2];
                    pos1 = pos2 + 1;
                    s
                }};
                ($needle:expr) => {{
                    let s = myscan!(skip, $needle);
                    let input = String::from_utf8_lossy(s);
                    input.as_ref().parse().unwrap()
                }};
            }
            macro_rules! myscan_field {
                ($target:tt <= $needle:tt) => {{
                    let haystack = &sl[pos1..];
                    let needle = $needle;
                    pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
                    myscan!(skip_spaces);
                    status.$target = myscan!(b" ");
                }};
                ($target:tt <= ? $needle:tt) => {{
                    let haystack = &sl[pos1..];
                    let needle = $needle;
                    if myscan!(check, needle) {
                        pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
                        myscan!(skip_spaces);
                        status.$target = myscan!(b" ");
                    }
                }};
            }
            //
            // content of vm
            //   VmPeak:\t   17276 kB
            //   VmSize:\t   17276 kB
            // on linux:
            //      "VmPeak:\t%8lu kB\n" ...
            //      "VmSize:\t%8lu kB\n"
            //
            if !myscan!(check, b"VmPeak:\t") {
                // some kernel thread do not has Vm, but no error.
                return status;
            }
            cfg_iif!(feature = "has_pidentry_status_vm_peak" {
                myscan_field!(vm_peak <= b"VmPeak:\t");
            });
            //
            myscan_field!(vm_size <= b"VmSize:\t");
            myscan_field!(vm_lck <= b"VmLck:\t");
            //
            cfg_iif!(feature = "has_pidentry_status_vm_pin" {
                myscan_field!(vm_pin <=? b"VmPin:\t");
            });
            cfg_iif!(feature = "has_pidentry_status_vm_hwm" {
                myscan_field!(vm_hwm <= b"VmHWM:\t");
            });
            //
            myscan_field!(vm_rss <= b"VmRSS:\t");
            //
            cfg_iif!(feature = "has_pidentry_status_rss_anon" {
                myscan_field!(rss_anon <=? b"RssAnon:\t");
            });
            cfg_iif!(feature = "has_pidentry_status_rss_file" {
                myscan_field!(rss_file <=? b"RssFile:\t");
            });
            cfg_iif!(feature = "has_pidentry_status_rss_shmem" {
                myscan_field!(rss_shmem <=? b"RssShmem:\t");
            });
            //
            myscan_field!(vm_data <= b"VmData:\t");
            myscan_field!(vm_stk <= b"VmStk:\t");
            myscan_field!(vm_exe <= b"VmExe:\t");
            myscan_field!(vm_lib <= b"VmLib:\t");
            myscan_field!(vm_pte <= b"VmPTE:\t");
            //
            cfg_iif!(feature = "has_pidentry_status_vm_pmd" {
                myscan_field!(vm_pmd <=? b"VmPMD:\t");
            });
            //
            myscan_field!(vm_swap <=? b"VmSwap:\t");
            let _ = pos1;
        }
        //
        status
    }
}
