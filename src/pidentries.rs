use super::Pid;

#[derive(Debug, Default, Clone)]
pub struct PidEntries {
    pub pidentries: Vec<PidEntry>,
}

#[derive(Debug, Default, Clone)]
pub struct PidEntry {
    pub is_empty: bool,
    pub stat: PidStat,
    pub statm: PidStatm,
    pub status: PidStatus,
    pub cmdline: PidCmdline,
}

// /proc/[number]/stat
#[derive(Debug, Default, Clone)]
pub struct PidStat {
    pub pid: Pid, // process id

    #[cfg(feature = "has_pidentry_stat_comm")]
    pub comm: String, // filename of executable
    #[cfg(feature = "has_pidentry_stat_state")]
    pub state: u8, // process state character ('S':sleeping, 'R', 'D', 'Z', 'T')
    //
    pub ppid: Pid, // pid of parent process
    pub pgrp: i32, // process groud id
    //
    #[cfg(feature = "has_pidentry_stat_session")]
    pub session: i32, // session id
    #[cfg(feature = "has_pidentry_stat_tty_nr")]
    pub tty_nr: i32, // device number of terminal the process uses
    #[cfg(feature = "has_pidentry_stat_tpgid")]
    pub tpgid: i32, // terminal process group id
    //
    #[cfg(feature = "has_pidentry_stat_flags")]
    pub flags: u32, // kernel flags of process
    #[cfg(feature = "has_pidentry_stat_minflt")]
    pub minflt: u32, // count of minor page faults since process start
    #[cfg(feature = "has_pidentry_stat_cminflt")]
    pub cminflt: u32, // cumulative minflt of process and child processes
    #[cfg(feature = "has_pidentry_stat_majflt")]
    pub majflt: u32, // count of major page faults since process start
    #[cfg(feature = "has_pidentry_stat_cmajflt")]
    pub cmajflt: u32, // cumulative majflt of process and child processes
    //
    pub utime: u32,  // user-mode cpu time (clock_t, see also man 2 times)
    pub stime: u32,  // system-mode cpu time
    pub cutime: u32, // cumulative utime of process and child processes
    pub cstime: u32, // cumulative stime of process and child processes
    //
    #[cfg(feature = "has_pidentry_stat_priority")]
    pub priority: i8, // cpu resource priority of process [>=0]
    //
    pub nice: i8,         // nice value ranges from 19 to -19.
    pub num_threads: i32, // count of threads of process (from linux v2.6)
    pub starttime: u64,   // start time of process
    //
    #[cfg(feature = "has_pidentry_stat_vsize")]
    pub vsize: usize, // *virtual memory size in bytes
    #[cfg(feature = "has_pidentry_stat_rss")]
    pub rss: usize, //  resident set size
    #[cfg(feature = "has_pidentry_stat_rlim")]
    pub rlim: usize, // *current limit in bytes on the rss of the process
    //
    #[cfg(feature = "has_pidentry_stat_startcode")]
    pub startcode: usize, //  addresst of beginning of code segment
    #[cfg(feature = "has_pidentry_stat_endcode")]
    pub endcode: usize, //  address of end of code segment
    #[cfg(feature = "has_pidentry_stat_startstack")]
    pub startstack: usize, // *address of the bottom of stack for the process
    #[cfg(feature = "has_pidentry_stat_kstesp")]
    pub kstesp: usize, // *kernel stack pointer
    #[cfg(feature = "has_pidentry_stat_ksteip")]
    pub ksteip: usize, //  kernel instruction pointer
    //
    #[cfg(feature = "has_pidentry_stat_signal")]
    pub signal: u32, // bitmap of pending signals
    #[cfg(feature = "has_pidentry_stat_blocked")]
    pub blocked: u32, // bitmap of blocked signals
    #[cfg(feature = "has_pidentry_stat_sigignore")]
    pub sigignore: u32, // bitmap of ignored signals
    #[cfg(feature = "has_pidentry_stat_sigcatch")]
    pub sigcatch: u32, // bitmap of caught signals
    //
    #[cfg(feature = "has_pidentry_stat_exit_signal")]
    pub exit_signal: i32, // signal to be sent to parent when this process die, (linux v2.1.22)
    #[cfg(feature = "has_pidentry_stat_processor")]
    pub processor: i32, // cpu number last executed on. (linux v2.2.8)
    #[cfg(feature = "has_pidentry_stat_rt_priority")]
    pub rt_priority: u32, // real-time scheduling priority (linux v2.5.19)
    #[cfg(feature = "has_pidentry_stat_policy")]
    pub policy: u32, // scheduling policy (linux v2.5.19)
    #[cfg(feature = "has_pidentry_stat_delayacct")]
    pub delayacct_blkio_ticks: u64, // delay clock ticks of block io (linux v2.6.18)
}

// /proc/[number]/statm
#[derive(Debug, Default, Clone)]
pub struct PidStatm {
    pub size: u32,     // total number of pages of memory
    pub resident: u32, // number of resident set (non-swapped) pages
    pub share: u32,    // number of pages of shared (mmaped) pages
    pub text: u32,     // text resident set size
    pub lib: u32,      // shared0lib resident set size
    pub data: u32,     // data/stack resident set size
}

// /proc/[number]/status
#[derive(Debug, Default, Clone)]
pub struct PidStatus {
    #[cfg(feature = "has_pidentry_status_name")]
    pub name: String,
    //
    #[cfg(feature = "has_pidentry_status_state")]
    pub state: u8,
    //
    #[cfg(feature = "has_pidentry_status_tgid")]
    pub tgid: Pid,
    #[cfg(feature = "has_pidentry_status_ngid")]
    pub ngid: Pid,
    //
    pub pid: Pid,
    pub ppid: Pid,
    //
    #[cfg(feature = "has_pidentry_status_tracer_pid")]
    pub tracer_pid: Pid,
    //
    // Uid
    pub ruid: u32, // real user id
    pub euid: u32, // effective user id
    pub suid: u32, // saved user id
    pub fuid: u32, // filesystem user id
    // Gid
    pub rgid: u32, // real group id
    pub egid: u32, // effective group id
    pub sgid: u32, // saved group id
    pub fgid: u32, // filesystem group id
    //
    #[cfg(feature = "has_pidentry_status_vm_peak")]
    pub vm_peak: usize,
    //
    pub vm_size: usize, // same as vsize in kb
    pub vm_lck: usize,  // locked pages in kb
    //{{{  >= linux v3.2
    #[cfg(feature = "has_pidentry_status_vm_pin")]
    pub vm_pin: usize, //
    //}}}
    //
    #[cfg(feature = "has_pidentry_status_vm_hwm")]
    pub vm_hwm: usize,
    //
    pub vm_rss: usize, // same as rss in kb
    //
    //{{{  >= linux v4.5
    #[cfg(feature = "has_pidentry_status_rss_anon")]
    pub rss_anon: usize,
    #[cfg(feature = "has_pidentry_status_rss_file")]
    pub rss_file: usize,
    #[cfg(feature = "has_pidentry_status_rss_shmem")]
    pub rss_shmem: usize,
    //}}}
    //
    pub vm_data: usize, // data size in kb
    pub vm_stk: usize,  // stack size in kb
    pub vm_exe: usize,  // executable size in kb
    pub vm_lib: usize,  // library size in kb (all pages, not just used ones)
    pub vm_pte: usize,  //
    //
    //{{{  >= linux v4.1
    #[cfg(feature = "has_pidentry_status_vm_pmd")]
    pub vm_pmd: usize,
    //}}}
    //
    //{{{  >= linux v2.6.34
    pub vm_swap: usize,
    //}}}
}

// /proc/[number]/comm
// /proc/[number]/cmdline
#[derive(Debug, Default, Clone)]
pub struct PidCmdline {
    pub cmdline: String,
}
