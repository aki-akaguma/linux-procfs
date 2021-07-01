#[derive(Debug, Default, Clone)]
pub struct Cpu {
    pub name: String,
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
    pub steal: u64,
    // up to here, on linux v2.6.18
    pub guest: u64,
    pub guest_nice: u64,
}

#[derive(Debug, Default, Clone)]
pub struct Stat {
    //
    pub cpu: Cpu,
    //
    pub cpus: Vec<Cpu>,
    //
    pub ctxt: u64,
    //
    #[cfg(feature = "has_stat_btime")]
    pub btime: u32,
    //
    // total forks
    pub processes: u32,
    //
    // nr_running
    #[cfg(feature = "has_stat_procs_running")]
    pub procs_running: u32,
    //
    // nr_iowait
    #[cfg(feature = "has_stat_procs_blocked")]
    pub procs_blocked: u32,
}

impl Stat {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            cpus: Vec::with_capacity(n),
            ..Self::default()
        }
    }
}
