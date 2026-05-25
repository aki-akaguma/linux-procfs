// linux source:
//   fs/proc/array.c >= v2.6.18
// status => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/array.c#L296
// status vm => https://elixir.bootlin.com/linux/v2.6.18/source/fs/proc/task_mmu.c#L14

use crate::pidentries::PidStatus;
use crate::scanner::ProcScanner;
use crate::ProcResult;
use cfg_iif::cfg_iif;

#[derive(Debug, Default, Clone)]
pub(crate) struct PidStatusParser();
impl PidStatusParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<PidStatus> {
        let mut status = PidStatus::default();
        if sl.is_empty() {
            return Ok(status);
        }

        let mut sc = ProcScanner::new(sl);

        // Name
        cfg_iif!(feature = "has_pidentry_status_name" {
            if sc.find_and_jump_opt(b"Name:\t") {
                status.name = sc.next(b'\n')?;
            }
        });

        // State
        cfg_iif!(feature = "has_pidentry_status_state" {
            if sc.find_and_jump_opt(b"State:\t") {
                let state_bytes = sc.scan_until(b' ')?;
                if !state_bytes.is_empty() {
                    status.state = state_bytes[0];
                }
                sc.scan_until(b'\n')?; // skip rest of line
            }
        });

        // Tgid
        cfg_iif!(feature = "has_pidentry_status_tgid" {
            if sc.find_and_jump_opt(b"Tgid:\t") {
                status.tgid = sc.next(b'\n')?;
            }
        });

        // Ngid
        cfg_iif!(feature = "has_pidentry_status_ngid" {
            if sc.find_and_jump_opt(b"Ngid:\t") {
                status.ngid = sc.next(b'\n')?;
            }
        });

        // Pid
        if sc.find_and_jump_opt(b"Pid:\t") {
            status.pid = sc.next(b'\n')?;
        }

        // PPid
        if sc.find_and_jump_opt(b"PPid:\t") {
            status.ppid = sc.next(b'\n')?;
        }

        // TracerPid
        cfg_iif!(feature = "has_pidentry_status_tracer_pid" {
            if sc.find_and_jump_opt(b"TracerPid:\t") {
                status.tracer_pid = sc.next(b'\n')?;
            }
        });

        // Uid: 0 0 0 0
        if sc.find_and_jump_opt(b"Uid:\t") {
            status.ruid = sc.next(b'\t')?;
            status.euid = sc.next(b'\t')?;
            status.suid = sc.next(b'\t')?;
            status.fuid = sc.next(b'\n')?;
        }

        // Gid: 0 0 0 0
        if sc.find_and_jump_opt(b"Gid:\t") {
            status.rgid = sc.next(b'\t')?;
            status.egid = sc.next(b'\t')?;
            status.sgid = sc.next(b'\t')?;
            status.fgid = sc.next(b'\n')?;
        }

        // VmPeak, VmSize, VmLck, ...
        // some kernel threads do not have Vm*, so we use find_and_jump_opt

        cfg_iif!(feature = "has_pidentry_status_vm_peak" {
            if sc.find_and_jump_opt(b"VmPeak:\t") {
                sc.skip_spaces();
                status.vm_peak = sc.next(b' ')?;
                sc.scan_until(b'\n')?; // skip " kB\n"
            }
        });

        if sc.find_and_jump_opt(b"VmSize:\t") {
            sc.skip_spaces();
            status.vm_size = sc.next(b' ')?;
            sc.scan_until(b'\n')?;
        }

        if sc.find_and_jump_opt(b"VmLck:\t") {
            sc.skip_spaces();
            status.vm_lck = sc.next(b' ')?;
            sc.scan_until(b'\n')?;
        }

        cfg_iif!(feature = "has_pidentry_status_vm_pin" {
            if sc.find_and_jump_opt(b"VmPin:\t") {
                sc.skip_spaces();
                status.vm_pin = sc.next(b' ')?;
                sc.scan_until(b'\n')?;
            }
        });

        cfg_iif!(feature = "has_pidentry_status_vm_hwm" {
            if sc.find_and_jump_opt(b"VmHWM:\t") {
                sc.skip_spaces();
                status.vm_hwm = sc.next(b' ')?;
                sc.scan_until(b'\n')?;
            }
        });

        if sc.find_and_jump_opt(b"VmRSS:\t") {
            sc.skip_spaces();
            status.vm_rss = sc.next(b' ')?;
            sc.scan_until(b'\n')?;
        }

        cfg_iif!(feature = "has_pidentry_status_rss_anon" {
            if sc.find_and_jump_opt(b"RssAnon:\t") {
                sc.skip_spaces();
                status.rss_anon = sc.next(b' ')?;
                sc.scan_until(b'\n')?;
            }
        });

        cfg_iif!(feature = "has_pidentry_status_rss_file" {
            if sc.find_and_jump_opt(b"RssFile:\t") {
                sc.skip_spaces();
                status.rss_file = sc.next(b' ')?;
                sc.scan_until(b'\n')?;
            }
        });

        cfg_iif!(feature = "has_pidentry_status_rss_shmem" {
            if sc.find_and_jump_opt(b"RssShmem:\t") {
                sc.skip_spaces();
                status.rss_shmem = sc.next(b' ')?;
                sc.scan_until(b'\n')?;
            }
        });

        if sc.find_and_jump_opt(b"VmData:\t") {
            sc.skip_spaces();
            status.vm_data = sc.next(b' ')?;
            sc.scan_until(b'\n')?;
        }

        if sc.find_and_jump_opt(b"VmStk:\t") {
            sc.skip_spaces();
            status.vm_stk = sc.next(b' ')?;
            sc.scan_until(b'\n')?;
        }

        if sc.find_and_jump_opt(b"VmExe:\t") {
            sc.skip_spaces();
            status.vm_exe = sc.next(b' ')?;
            sc.scan_until(b'\n')?;
        }

        if sc.find_and_jump_opt(b"VmLib:\t") {
            sc.skip_spaces();
            status.vm_lib = sc.next(b' ')?;
            sc.scan_until(b'\n')?;
        }

        if sc.find_and_jump_opt(b"VmPTE:\t") {
            sc.skip_spaces();
            status.vm_pte = sc.next(b' ')?;
            sc.scan_until(b'\n')?;
        }

        cfg_iif!(feature = "has_pidentry_status_vm_pmd" {
            if sc.find_and_jump_opt(b"VmPMD:\t") {
                sc.skip_spaces();
                status.vm_pmd = sc.next(b' ')?;
                sc.scan_until(b'\n')?;
            }
        });

        if sc.find_and_jump_opt(b"VmSwap:\t") {
            sc.skip_spaces();
            status.vm_swap = sc.next(b' ')?;
            sc.scan_until(b'\n')?;
        }

        Ok(status)
    }
}
