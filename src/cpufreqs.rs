#[derive(Debug, Default, Clone)]
pub struct CpuFreqs {
    pub cpufreqs: Vec<CpuFreq>,
}

#[derive(Debug, Default, Clone)]
pub struct CpuFreq {
    pub cur: u32,
    pub max: u32,
    pub time_in_states: Vec<TimeInState>,
}

#[derive(Debug, Default, Clone)]
pub struct TimeInState {
    pub step: u32,
    pub value: u64,
}
