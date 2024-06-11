/*!
The linux-procfs crate is the data snap library for the `/proc/` filesystem on the linux os.

This crate reads from `/proc` filesystem, scanne it, stores the value into the struct and returns it. This scans and holds only the required values by specifying `feature`.

# Feature

- minimum support rustc 1.58.1 (db9d1b20b 2022-01-20)

# Example

## Example 1: load average

```
use linux_procfs::System;
let mut sys = System::new("/");
let loadavg = sys.get_loadavg();
println!("{}, {}, {}, {}", loadavg.a1, loadavg.a5, loadavg.a15, loadavg.last_pid);
```

## Example 2: disk stats

```
use linux_procfs::System;
let mut sys = System::new("/");
let diskstats = sys.get_diskstats();
for disk in diskstats.disks {
    println!("{}, {}, {}", disk.name, disk.rblk, disk.wblk);
}
```
*/

use std::fs;
use std::path::{Path, PathBuf};

pub mod loadavg;
pub mod meminfo;
pub mod stat;
pub mod uptime;
pub mod vmstat;

pub mod diskstats;
pub mod netdevs;

pub mod cpufreqs;
pub mod pidentries;

mod parser;
mod util;

pub type Pid = i32;

/// system interface of linux-procfs
pub struct System {
    base_path: PathBuf,
    fb: util::FileBuffer,
}

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
    pub fn get_loadavg(&mut self) -> loadavg::LoadAvg {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 40,
            name: "loadavg",
        };
        let slice = PROC_FB.update(&self.base_path, &mut self.fb);
        parser::loadavg::LoadAvgParser::default().parse(slice)
    }
    //
    /// `/proc/meminfo`
    pub fn get_meminfo(&mut self) -> meminfo::MemInfo {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 1300,
            name: "meminfo",
        };
        let slice = PROC_FB.update(&self.base_path, &mut self.fb);
        parser::meminfo::MemInfoParser::default().parse(slice)
    }
    //
    /// `/proc/stat`
    pub fn get_stat(&mut self) -> stat::Stat {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 1500,
            name: "stat",
        };
        let slice = PROC_FB.update(&self.base_path, &mut self.fb);
        parser::stat::StatParser::default().parse(slice)
    }
    //
    /// `/proc/uptime`
    pub fn get_uptime(&mut self) -> uptime::Uptime {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 30,
            name: "uptime",
        };
        let slice = PROC_FB.update(&self.base_path, &mut self.fb);
        parser::uptime::UptimeParser::default().parse(slice)
    }
    //
    /// `/proc/vmstat`
    pub fn get_vmstat(&mut self) -> vmstat::VmStat {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 2600,
            name: "vmstat",
        };
        let slice = PROC_FB.update(&self.base_path, &mut self.fb);
        parser::vmstat::VmStatParser::default().parse(slice)
    }
    //
    /// `/proc/diskstats`
    pub fn get_diskstats(&mut self) -> diskstats::DiskStats {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 3000,
            name: "diskstats",
        };
        let slice = PROC_FB.update(&self.base_path, &mut self.fb);
        parser::diskstats::DiskStatsParser::default().parse(slice)
    }
    //
    /// `/proc/net/dev`
    pub fn get_netdevs(&mut self) -> netdevs::NetDevs {
        static PROC_FB: util::ProcFb = util::ProcFb {
            capacity: 2000,
            name: "net/dev",
        };
        let slice = PROC_FB.update(&self.base_path, &mut self.fb);
        parser::netdevs::NetDevsParser::default().parse(slice)
    }
    //
    /// maximum cpu number
    pub fn get_max_cpu_num(&mut self) -> usize {
        let mut max_cpu_num = 0;
        //
        let cpu_path = format!(
            "{}/sys/devices/system/cpu",
            &self.base_path.to_str().unwrap()
        );
        for entry in fs::read_dir(cpu_path).unwrap() {
            let entry = entry.unwrap();
            let os_name = entry.file_name();
            let name = os_name.to_string_lossy();
            if name.starts_with("cpu") && name.len() > 3 {
                let sl = &name[3..];
                if let Ok(n) = sl.parse::<usize>() {
                    if n > max_cpu_num {
                        max_cpu_num = n;
                    }
                }
            }
        }
        //
        max_cpu_num + 1
    }
    //
    /// `/sys/devices/system/cpu/cpu0/cpufreq/`
    pub fn get_cpufreqs(&mut self, max_cpu_num: usize) -> cpufreqs::CpuFreqs {
        static SYS_CPUFREQ_CUR: util::SysCpuFb = util::SysCpuFb {
            capacity: 10,
            name: "cpufreq/cpuinfo_cur_freq",
        };
        static SYS_CPUFREQ_MAX: util::SysCpuFb = util::SysCpuFb {
            capacity: 10,
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
                let slice = SYS_CPUFREQ_CUR.update_with_cpu_num(&self.base_path, &mut self.fb, idx);
                parser::cpufreqs::CpuFreqMaxParser::default().parse(slice)
            };
            cpufreq.max = {
                let slice = SYS_CPUFREQ_MAX.update_with_cpu_num(&self.base_path, &mut self.fb, idx);
                parser::cpufreqs::CpuFreqMaxParser::default().parse(slice)
            };
            cpufreq.time_in_states = {
                let slice = SYS_CPUFREQ_STATS_TIME_IN_STATE.update_with_cpu_num(
                    &self.base_path,
                    &mut self.fb,
                    idx,
                );
                parser::cpufreqs::CpuFreqStatsTimeInStateParser::default().parse(slice)
            };
        }
        cpufreqs
    }
    //
    /// `/proc/<pid>/`
    pub fn get_pids(&mut self) -> Vec<Pid> {
        let mut v_pid = Vec::new();
        //
        let proc_path = format!("{}/proc", &self.base_path.to_str().unwrap());
        for entry in fs::read_dir(proc_path).unwrap() {
            let entry = entry.unwrap();
            let os_name = entry.file_name();
            let name = os_name.to_string_lossy();
            if name.as_bytes().iter().any(|&b| !b.is_ascii_digit()) {
                continue;
            }
            let pid: Pid = name.parse().unwrap();
            v_pid.push(pid);
        }
        v_pid.sort_unstable();
        //
        v_pid
    }
    //
    /// `/proc/<pid>/{stat, statm, status, cmdline}`
    pub fn get_pidentries(&mut self) -> pidentries::PidEntries {
        let pid_vec = self.get_pids();
        //
        let mut pids = pidentries::PidEntries::default();
        pids.pidentries
            .resize(pid_vec.len(), pidentries::PidEntry::default());
        for (idx, &pid) in pid_vec.iter().enumerate() {
            let pidentry = &mut pids.pidentries[idx];
            pidentry.is_empty = true;
            //
            pidentry.stat = match self.get_pidentry_stat(pid) {
                Some(a) => a,
                None => continue,
            };
            pidentry.statm = match self.get_pidentry_statm(pid) {
                Some(a) => a,
                None => continue,
            };
            pidentry.status = match self.get_pidentry_status(pid) {
                Some(a) => a,
                None => continue,
            };
            pidentry.cmdline = match self.get_pidentry_cmdline(pid) {
                Some(a) => a,
                None => continue,
            };
            //
            pidentry.is_empty = false;
        }
        pids
    }
    //
    /// `/proc/<pid>/stat`
    pub fn get_pidentry_stat(&mut self, pid: Pid) -> Option<pidentries::PidStat> {
        static PIDPROCS_STAT: util::PidFb = util::PidFb {
            capacity: 400,
            name: "stat",
        };
        let slice = PIDPROCS_STAT.update_with_pid(&self.base_path, &mut self.fb, pid);
        if !slice.is_empty() {
            Some(parser::pidstat::PidStatParser::default().parse(slice))
        } else {
            None
        }
    }
    //
    /// `/proc/<pid>/statm`
    pub fn get_pidentry_statm(&mut self, pid: Pid) -> Option<pidentries::PidStatm> {
        static PIDPROCS_STATM: util::PidFb = util::PidFb {
            capacity: 40,
            name: "statm",
        };
        let slice = PIDPROCS_STATM.update_with_pid(&self.base_path, &mut self.fb, pid);
        if !slice.is_empty() {
            Some(parser::pidstatm::PidStatmParser::default().parse(slice))
        } else {
            None
        }
    }
    //
    /// `/proc/<pid>/status`
    pub fn get_pidentry_status(&mut self, pid: Pid) -> Option<pidentries::PidStatus> {
        static PIDPROCS_STATUS: util::PidFb = util::PidFb {
            capacity: 1100,
            name: "status",
        };
        let slice = PIDPROCS_STATUS.update_with_pid(&self.base_path, &mut self.fb, pid);
        if !slice.is_empty() {
            Some(parser::pidstatus::PidStatusParser::default().parse(slice))
        } else {
            None
        }
    }
    //
    /// `/proc/<pid>/cmdline`
    pub fn get_pidentry_cmdline(&mut self, pid: Pid) -> Option<pidentries::PidCmdline> {
        static PIDPROCS_CMDLINE: util::PidFb = util::PidFb {
            capacity: 600,
            name: "cmdline",
        };
        let slice = PIDPROCS_CMDLINE.update_with_pid(&self.base_path, &mut self.fb, pid);
        if !slice.is_empty() {
            Some(parser::pidcmdline::PidCmdlineParser::default().parse(slice))
        } else {
            None
        }
    }
    //
    /// `/proc/<pid>/comm`
    pub fn get_pidentry_comm(&mut self, pid: Pid) -> Option<pidentries::PidCmdline> {
        static PIDPROCS_COMM: util::PidFb = util::PidFb {
            capacity: 600,
            name: "comm",
        };
        let slice = PIDPROCS_COMM.update_with_pid(&self.base_path, &mut self.fb, pid);
        if !slice.is_empty() {
            Some(parser::pidcmdline::PidCmdlineParser::default().parse(slice))
        } else {
            None
        }
    }
}
