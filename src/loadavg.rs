use super::Pid;

#[derive(Debug, Default, Clone)]
pub struct LoadAvg {
    pub a1: f64,
    pub a5: f64,
    pub a15: f64,
    //
    pub last_pid: Pid,
}
