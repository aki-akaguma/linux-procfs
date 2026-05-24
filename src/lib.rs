/*!
The linux-procfs crate is the data snap library for the `/proc/` filesystem on the linux os.

This crate reads from `/proc` filesystem, scanne it, stores the value into the struct and returns it. This scans and holds only the required values.

# Usage

## Setup

Add the following to your `Cargo.toml`:

```toml
[dependencies]
linux-procfs = "0.3.17"
```

## Example 1: load average

```
use linux_procfs::System;
let mut sys = System::new("/");
let loadavg = sys.get_loadavg().unwrap();
println!("{}, {}, {}, {}", loadavg.a1, loadavg.a5, loadavg.a15, loadavg.last_pid);
```

## Example 2: disk stats

```
use linux_procfs::System;
let mut sys = System::new("/");
let diskstats = sys.get_diskstats().unwrap();
for disk in diskstats.disks {
    println!("{}, {}, {}", disk.name, disk.rblk, disk.wblk);
}
```
*/

pub mod loadavg;
pub mod meminfo;
pub mod stat;
pub mod uptime;
pub mod vmstat;

pub mod diskstats;
pub mod netdevs;

pub mod cpufreqs;
pub mod pidentries;

pub mod error;

mod parser;
mod util;

pub use crate::error::{ProcError, ProcResult};

pub type Pid = i32;

/// system interface of linux-procfs
pub struct System {
    base_path: PathBuf,
    fb: util::FileBuffer,
}

use std::fs;
use std::path::{Path, PathBuf};

impl System {
    /// create instance
    ///
    /// Example:
    /// ```
    ///use linux_procfs::System;
    ///let mut sys = System::new("/");
    /// ```
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            fb: util::FileBuffer::new(),
        }
    }
    //
    /// `/proc/loadavg`
    pub fn get_loadavg(&mut self) -> ProcResult<loadavg::LoadAvg> {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 40,
            name: "loadavg",
        };
        let slice = PROC_FB.try_update(&self.base_path, &mut self.fb)?;
        parser::loadavg::LoadAvgParser::default().parse(slice)
    }
    //
    /// `/proc/meminfo`
    pub fn get_meminfo(&mut self) -> ProcResult<meminfo::MemInfo> {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 1300,
            name: "meminfo",
        };
        let slice = PROC_FB.try_update(&self.base_path, &mut self.fb)?;
        parser::meminfo::MemInfoParser::default().parse(slice)
    }
    //
    /// `/proc/stat`
    pub fn get_stat(&mut self) -> ProcResult<stat::Stat> {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 1500,
            name: "stat",
        };
        let slice = PROC_FB.try_update(&self.base_path, &mut self.fb)?;
        parser::stat::StatParser::default().parse(slice)
    }
    //
    /// `/proc/uptime`
    pub fn get_uptime(&mut self) -> ProcResult<uptime::Uptime> {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 50,
            name: "uptime",
        };
        let slice = PROC_FB.try_update(&self.base_path, &mut self.fb)?;
        parser::uptime::UptimeParser::default().parse(slice)
    }
    //
    /// `/proc/vmstat`
    pub fn get_vmstat(&mut self) -> ProcResult<vmstat::VmStat> {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 1500,
            name: "vmstat",
        };
        let slice = PROC_FB.try_update(&self.base_path, &mut self.fb)?;
        parser::vmstat::VmStatParser::default().parse(slice)
    }
    //
    /// `/proc/diskstats`
    pub fn get_diskstats(&mut self) -> ProcResult<diskstats::DiskStats> {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 1000,
            name: "diskstats",
        };
        let slice = PROC_FB.try_update(&self.base_path, &mut self.fb)?;
        parser::diskstats::DiskStatsParser::default().parse(slice)
    }
    //
    /// `/proc/net/dev`
    pub fn get_netdevs(&mut self) -> ProcResult<netdevs::NetDevs> {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 1000,
            name: "net/dev",
        };
        let slice = PROC_FB.try_update(&self.base_path, &mut self.fb)?;
        parser::netdevs::NetDevsParser::default().parse(slice)
    }
    //
    /// maximum cpu number
    pub fn get_max_cpu_num(&mut self) -> ProcResult<usize> {
        let mut max_cpu_num = 0;
        let cpu_path = self.base_path.join("sys/devices/system/cpu");
        for entry in fs::read_dir(cpu_path).map_err(|_| crate::ProcError::PermissionDenied)? {
            let entry = entry.map_err(|_| crate::ProcError::InternalError)?;
            let os_name = entry.file_name();
            let name = os_name.to_string_lossy();
            if !name.starts_with("cpu") {
                continue;
            }
            let s_num = &name[3..];
            if s_num.as_bytes().iter().any(|&b| !b.is_ascii_digit()) {
                continue;
            }
            if let Ok(num) = s_num.parse::<usize>() {
                if max_cpu_num < num {
                    max_cpu_num = num;
                }
            }
        }
        Ok(max_cpu_num + 1)
    }
    //
    /// `/sys/devices/system/cpu/cpu*/cpufreq/`
    pub fn get_cpufreqs(&mut self, max_cpu_num: usize) -> ProcResult<cpufreqs::CpuFreqs> {
        static SYS_CPUFREQ_CUR: util::SysCpuFb = util::SysCpuFb {
            capacity: 20,
            name: "cpufreq/cpuinfo_cur_freq",
        };
        static SYS_CPUFREQ_MAX: util::SysCpuFb = util::SysCpuFb {
            capacity: 20,
            name: "cpufreq/cpuinfo_max_freq",
        };
        static SYS_CPUFREQ_STATS_TIME_IN_STATE: util::SysCpuFb = util::SysCpuFb {
            capacity: 100,
            name: "cpufreq/stats/time_in_state",
        };
        //
        let mut cpufreqs = cpufreqs::CpuFreqs::default();
        cpufreqs
            .cpufreqs
            .resize(max_cpu_num, cpufreqs::CpuFreq::default());
        for idx in 0..max_cpu_num {
            let cpufreq = &mut cpufreqs.cpufreqs[idx];
            cpufreq.cur = {
                let slice = match SYS_CPUFREQ_CUR.try_update_with_cpu_num(&self.base_path, &mut self.fb, idx) {
                    Ok(s) => s,
                    Err(_) => &[],
                };
                parser::cpufreqs::CpuFreqMaxParser::default().parse(slice)?
            };
            cpufreq.max = {
                let slice = match SYS_CPUFREQ_MAX.try_update_with_cpu_num(&self.base_path, &mut self.fb, idx) {
                    Ok(s) => s,
                    Err(_) => &[],
                };
                parser::cpufreqs::CpuFreqMaxParser::default().parse(slice)?
            };
            cpufreq.time_in_states = {
                let slice = match SYS_CPUFREQ_STATS_TIME_IN_STATE.try_update_with_cpu_num(
                    &self.base_path,
                    &mut self.fb,
                    idx,
                ) {
                    Ok(s) => s,
                    Err(_) => &[],
                };
                parser::cpufreqs::CpuFreqStatsTimeInStateParser::default().parse(slice)?
            };
        }
        Ok(cpufreqs)
    }
    //
    /// `/proc/<pid>/`
    pub fn get_pids(&mut self) -> ProcResult<Vec<Pid>> {
        let mut v_pid = Vec::new();
        //
        let proc_path = self.base_path.join("proc");
        for entry in fs::read_dir(proc_path).map_err(|_| crate::ProcError::PermissionDenied)? {
            let entry = entry.map_err(|_| crate::ProcError::InternalError)?;
            let os_name = entry.file_name();
            let name = os_name.to_string_lossy();
            if name.as_bytes().iter().any(|&b| !b.is_ascii_digit()) {
                continue;
            }
            if let Ok(pid) = name.parse::<Pid>() {
                v_pid.push(pid);
            }
        }
        v_pid.sort_unstable();
        //
        Ok(v_pid)
    }
    //
    /// `/proc/<pid>/{stat, statm, status, cmdline}`
    pub fn get_pidentries(&mut self) -> ProcResult<pidentries::PidEntries> {
        let pid_vec = self.get_pids()?;
        //
        let mut pids = pidentries::PidEntries::default();
        pids.pidentries
            .resize(pid_vec.len(), pidentries::PidEntry::default());
        for (idx, &pid) in pid_vec.iter().enumerate() {
            let pidentry = &mut pids.pidentries[idx];
            pidentry.is_empty = true;
            //
            pidentry.stat = match self.get_pidentry_stat(pid)? {
                Some(a) => a,
                None => continue,
            };
            pidentry.statm = match self.get_pidentry_statm(pid)? {
                Some(a) => a,
                None => continue,
            };
            pidentry.status = match self.get_pidentry_status(pid)? {
                Some(a) => a,
                None => continue,
            };
            pidentry.cmdline = match self.get_pidentry_cmdline(pid)? {
                Some(a) => a,
                None => continue,
            };
            pidentry.is_empty = false;
        }
        Ok(pids)
    }
    //
    /// `/proc/<pid>/stat`
    pub fn get_pidentry_stat(&mut self, pid: Pid) -> ProcResult<Option<pidentries::PidStat>> {
        static PIDPROCS_STAT: util::PidFb = util::PidFb {
            capacity: 400,
            name: "stat",
        };
        let slice = match PIDPROCS_STAT.try_update_with_pid(&self.base_path, &mut self.fb, pid) {
            Ok(s) => s,
            Err(_) => return Ok(None),
        };
        if !slice.is_empty() {
            Ok(Some(parser::pidstat::PidStatParser::default().parse(slice)?))
        } else {
            Ok(None)
        }
    }
    //
    /// `/proc/<pid>/statm`
    pub fn get_pidentry_statm(&mut self, pid: Pid) -> ProcResult<Option<pidentries::PidStatm>> {
        static PIDPROCS_STATM: util::PidFb = util::PidFb {
            capacity: 40,
            name: "statm",
        };
        let slice = match PIDPROCS_STATM.try_update_with_pid(&self.base_path, &mut self.fb, pid) {
            Ok(s) => s,
            Err(_) => return Ok(None),
        };
        if !slice.is_empty() {
            Ok(Some(
                parser::pidstatm::PidStatmParser::default().parse(slice)?,
            ))
        } else {
            Ok(None)
        }
    }
    //
    /// `/proc/<pid>/status`
    pub fn get_pidentry_status(&mut self, pid: Pid) -> ProcResult<Option<pidentries::PidStatus>> {
        static PIDPROCS_STATUS: util::PidFb = util::PidFb {
            capacity: 1100,
            name: "status",
        };
        let slice = match PIDPROCS_STATUS.try_update_with_pid(&self.base_path, &mut self.fb, pid) {
            Ok(s) => s,
            Err(_) => return Ok(None),
        };
        if !slice.is_empty() {
            Ok(Some(
                parser::pidstatus::PidStatusParser::default().parse(slice)?,
            ))
        } else {
            Ok(None)
        }
    }
    //
    /// `/proc/<pid>/cmdline`
    pub fn get_pidentry_cmdline(&mut self, pid: Pid) -> ProcResult<Option<pidentries::PidCmdline>> {
        static PIDPROCS_CMDLINE: util::PidFb = util::PidFb {
            capacity: 600,
            name: "cmdline",
        };
        let slice = match PIDPROCS_CMDLINE.try_update_with_pid(&self.base_path, &mut self.fb, pid) {
            Ok(s) => s,
            Err(_) => return Ok(None),
        };
        if !slice.is_empty() {
            Ok(Some(
                parser::pidcmdline::PidCmdlineParser::default().parse(slice)?,
            ))
        } else {
            Ok(None)
        }
    }
    //
    /// `/proc/<pid>/comm`
    pub fn get_pidentry_comm(&mut self, pid: Pid) -> ProcResult<Option<pidentries::PidCmdline>> {
        static PIDPROCS_COMM: util::PidFb = util::PidFb {
            capacity: 600,
            name: "comm",
        };
        let slice = match PIDPROCS_COMM.try_update_with_pid(&self.base_path, &mut self.fb, pid) {
            Ok(s) => s,
            Err(_) => return Ok(None),
        };
        if !slice.is_empty() {
            Ok(Some(
                parser::pidcmdline::PidCmdlineParser::default().parse(slice)?,
            ))
        } else {
            Ok(None)
        }
    }
}
