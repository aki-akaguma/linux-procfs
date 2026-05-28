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
mod scanner;

mod parser;
mod util;

pub use crate::error::{ProcError, ProcResult};

pub type Pid = i32;

pub const DEFAULT_CAPACITY_LOADAVG: usize = 40;
pub const DEFAULT_CAPACITY_MEMINFO: usize = 1300;
pub const DEFAULT_CAPACITY_STAT: usize = 1500;
pub const DEFAULT_CAPACITY_UPTIME: usize = 50;
pub const DEFAULT_CAPACITY_VMSTAT: usize = 1500;
pub const DEFAULT_CAPACITY_DISKSTATS: usize = 1000;
pub const DEFAULT_CAPACITY_NETDEVS: usize = 1000;
pub const DEFAULT_CAPACITY_PID_STAT: usize = 400;
pub const DEFAULT_CAPACITY_PID_STATM: usize = 40;
pub const DEFAULT_CAPACITY_PID_STATUS: usize = 1100;
pub const DEFAULT_CAPACITY_PID_CMDLINE: usize = 600;
pub const DEFAULT_CAPACITY_PID_COMM: usize = 600;
pub const DEFAULT_CAPACITY_CPUFREQ: usize = 20;
pub const DEFAULT_CAPACITY_CPUFREQ_STATS: usize = 100;

#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub loadavg_cap: usize,
    pub meminfo_cap: usize,
    pub stat_cap: usize,
    pub uptime_cap: usize,
    pub vmstat_cap: usize,
    pub diskstats_cap: usize,
    pub netdevs_cap: usize,
    pub pid_stat_cap: usize,
    pub pid_statm_cap: usize,
    pub pid_status_cap: usize,
    pub pid_cmdline_cap: usize,
    pub pid_comm_cap: usize,
    pub cpufreq_cap: usize,
    pub cpufreq_stats_cap: usize,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            loadavg_cap: DEFAULT_CAPACITY_LOADAVG,
            meminfo_cap: DEFAULT_CAPACITY_MEMINFO,
            stat_cap: DEFAULT_CAPACITY_STAT,
            uptime_cap: DEFAULT_CAPACITY_UPTIME,
            vmstat_cap: DEFAULT_CAPACITY_VMSTAT,
            diskstats_cap: DEFAULT_CAPACITY_DISKSTATS,
            netdevs_cap: DEFAULT_CAPACITY_NETDEVS,
            pid_stat_cap: DEFAULT_CAPACITY_PID_STAT,
            pid_statm_cap: DEFAULT_CAPACITY_PID_STATM,
            pid_status_cap: DEFAULT_CAPACITY_PID_STATUS,
            pid_cmdline_cap: DEFAULT_CAPACITY_PID_CMDLINE,
            pid_comm_cap: DEFAULT_CAPACITY_PID_COMM,
            cpufreq_cap: DEFAULT_CAPACITY_CPUFREQ,
            cpufreq_stats_cap: DEFAULT_CAPACITY_CPUFREQ_STATS,
        }
    }
}

/// system interface of linux-procfs
pub struct System {
    base_path: PathBuf,
    fb: util::FileBuffer,
    config: SystemConfig,
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
        Self::with_config(base_path, SystemConfig::default())
    }

    pub fn with_config<P: AsRef<Path>>(base_path: P, config: SystemConfig) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            fb: util::FileBuffer::new(),
            config,
        }
    }
    //
    /// `/proc/loadavg`
    pub fn get_loadavg(&mut self) -> ProcResult<loadavg::LoadAvg> {
        let proc_fb = util::ProcFb {
            capacity: self.config.loadavg_cap,
            name: "loadavg",
        };
        let slice = proc_fb.try_update(&self.base_path, &mut self.fb)?;
        parser::loadavg::LoadAvgParser::default().parse(slice)
    }
    //
    /// `/proc/meminfo`
    pub fn get_meminfo(&mut self) -> ProcResult<meminfo::MemInfo> {
        let proc_fb = util::ProcFb {
            capacity: self.config.meminfo_cap,
            name: "meminfo",
        };
        let slice = proc_fb.try_update(&self.base_path, &mut self.fb)?;
        parser::meminfo::MemInfoParser::default().parse(slice)
    }
    //
    /// `/proc/stat`
    pub fn get_stat(&mut self) -> ProcResult<stat::Stat> {
        let proc_fb = util::ProcFb {
            capacity: self.config.stat_cap,
            name: "stat",
        };
        let slice = proc_fb.try_update(&self.base_path, &mut self.fb)?;
        parser::stat::StatParser::default().parse(slice)
    }
    //
    /// `/proc/uptime`
    pub fn get_uptime(&mut self) -> ProcResult<uptime::Uptime> {
        let proc_fb = util::ProcFb {
            capacity: self.config.uptime_cap,
            name: "uptime",
        };
        let slice = proc_fb.try_update(&self.base_path, &mut self.fb)?;
        parser::uptime::UptimeParser::default().parse(slice)
    }
    //
    /// `/proc/vmstat`
    pub fn get_vmstat(&mut self) -> ProcResult<vmstat::VmStat> {
        let proc_fb = util::ProcFb {
            capacity: self.config.vmstat_cap,
            name: "vmstat",
        };
        let slice = proc_fb.try_update(&self.base_path, &mut self.fb)?;
        parser::vmstat::VmStatParser::default().parse(slice)
    }
    //
    /// `/proc/diskstats`
    pub fn get_diskstats(&mut self) -> ProcResult<diskstats::DiskStats> {
        let proc_fb = util::ProcFb {
            capacity: self.config.diskstats_cap,
            name: "diskstats",
        };
        let slice = proc_fb.try_update(&self.base_path, &mut self.fb)?;
        parser::diskstats::DiskStatsParser::default().parse(slice)
    }
    //
    /// `/proc/net/dev`
    pub fn get_netdevs(&mut self) -> ProcResult<netdevs::NetDevs> {
        let proc_fb = util::ProcFb {
            capacity: self.config.netdevs_cap,
            name: "net/dev",
        };
        let slice = proc_fb.try_update(&self.base_path, &mut self.fb)?;
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
        let sys_cpufreq_cur = util::SysCpuFb {
            capacity: self.config.cpufreq_cap,
            name: "cpufreq/cpuinfo_cur_freq",
        };
        let sys_cpufreq_max = util::SysCpuFb {
            capacity: self.config.cpufreq_cap,
            name: "cpufreq/cpuinfo_max_freq",
        };
        let sys_cpufreq_stats_time_in_state = util::SysCpuFb {
            capacity: self.config.cpufreq_stats_cap,
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
                let slice = match sys_cpufreq_cur.try_update_with_cpu_num(
                    &self.base_path,
                    &mut self.fb,
                    idx,
                ) {
                    Ok(s) => s,
                    Err(_) => &[],
                };
                parser::cpufreqs::CpuFreqMaxParser::default().parse(slice)?
            };
            cpufreq.max = {
                let slice = match sys_cpufreq_max.try_update_with_cpu_num(
                    &self.base_path,
                    &mut self.fb,
                    idx,
                ) {
                    Ok(s) => s,
                    Err(_) => &[],
                };
                parser::cpufreqs::CpuFreqMaxParser::default().parse(slice)?
            };
            cpufreq.time_in_states = {
                let slice = match sys_cpufreq_stats_time_in_state.try_update_with_cpu_num(
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
            pidentry.stat = match self.get_pidentry_stat(pid) {
                Ok(a) => match a {
                    Some(b) => b,
                    None => continue,
                },
                Err(e) if e.is_not_found() => continue,
                Err(e) => return Err(e),
            };
            pidentry.statm = match self.get_pidentry_statm(pid) {
                Ok(a) => match a {
                    Some(b) => b,
                    None => continue,
                },
                Err(e) if e.is_not_found() => continue,
                Err(e) => return Err(e),
            };
            pidentry.status = match self.get_pidentry_status(pid) {
                Ok(a) => match a {
                    Some(b) => b,
                    None => continue,
                },
                Err(e) if e.is_not_found() => continue,
                Err(e) => return Err(e),
            };
            pidentry.cmdline = match self.get_pidentry_cmdline(pid) {
                Ok(a) => match a {
                    Some(b) => b,
                    None => continue,
                },
                Err(e) if e.is_not_found() => continue,
                Err(e) => return Err(e),
            };
            pidentry.is_empty = false;
        }
        Ok(pids)
    }
    //
    /// `/proc/<pid>/stat`
    pub fn get_pidentry_stat(&mut self, pid: Pid) -> ProcResult<Option<pidentries::PidStat>> {
        let pidprocs_stat = util::PidFb {
            capacity: self.config.pid_stat_cap,
            name: "stat",
        };
        let slice = pidprocs_stat.try_update_with_pid(&self.base_path, &mut self.fb, pid)?;
        if !slice.is_empty() {
            Ok(Some(
                parser::pidstat::PidStatParser::default().parse(slice)?,
            ))
        } else {
            Ok(None)
        }
    }
    //
    /// `/proc/<pid>/statm`
    pub fn get_pidentry_statm(&mut self, pid: Pid) -> ProcResult<Option<pidentries::PidStatm>> {
        let pidprocs_statm = util::PidFb {
            capacity: self.config.pid_statm_cap,
            name: "statm",
        };
        let slice = pidprocs_statm.try_update_with_pid(&self.base_path, &mut self.fb, pid)?;
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
        let pidprocs_status = util::PidFb {
            capacity: self.config.pid_status_cap,
            name: "status",
        };
        let slice = pidprocs_status.try_update_with_pid(&self.base_path, &mut self.fb, pid)?;
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
        let pidprocs_cmdline = util::PidFb {
            capacity: self.config.pid_cmdline_cap,
            name: "cmdline",
        };
        let slice = pidprocs_cmdline.try_update_with_pid(&self.base_path, &mut self.fb, pid)?;
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
        let pidprocs_comm = util::PidFb {
            capacity: self.config.pid_comm_cap,
            name: "comm",
        };
        let slice = pidprocs_comm.try_update_with_pid(&self.base_path, &mut self.fb, pid)?;
        if !slice.is_empty() {
            Ok(Some(
                parser::pidcmdline::PidCmdlineParser::default().parse(slice)?,
            ))
        } else {
            Ok(None)
        }
    }
}
