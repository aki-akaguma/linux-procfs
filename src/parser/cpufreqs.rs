// linux source:
//   drivers/cpufreq/cpufreq_stats.c >= v2.6.18
// https://elixir.bootlin.com/linux/v2.6.18/source/drivers/cpufreq/cpufreq_stats.c#L83

use crate::cpufreqs::TimeInState;
use crate::scanner::{FromBytes, ProcScanner};
use crate::ProcResult;

#[derive(Debug, Default, Clone)]
pub struct CpuFreqMaxParser();

impl CpuFreqMaxParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<u32> {
        // content of /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq
        //   "2403000\n"
        //
        if sl.is_empty() {
            return Ok(0);
        }
        let mut sc = ProcScanner::new(sl);
        if sc.check(b'\n') {
            sc.next(b'\n')
        } else {
            FromBytes::from_bytes(sl)
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CpuFreqStatsTimeInStateParser();

impl CpuFreqStatsTimeInStateParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<Vec<TimeInState>> {
        let mut time_in_states = Vec::new();
        if sl.is_empty() {
            return Ok(time_in_states);
        }
        //
        let mut sc = ProcScanner::new(sl);
        let mut idx: usize = 0;
        while !sc.is_empty() {
            //
            if idx >= time_in_states.len() {
                time_in_states.resize(idx + 1, TimeInState::default());
            }
            let tis_ref: &mut TimeInState = match time_in_states.get_mut(idx) {
                Some(tis) => tis,
                None => return Err(crate::ProcError::InternalError),
            };
            //
            // content of /sys/devices/system/cpu/cpu0/cpufreq/stats/time_in_state
            //   "2403000 4202462\n"
            //   "2136000 2726644\n"
            // on linux:
            //   "%u %llu\n"
            //
            if sc.check(b' ') {
                tis_ref.step = sc.next(b' ')?;
                tis_ref.value = sc.next(b'\n')?;
            } else {
                return Ok(vec![]);
            }
            //
            idx += 1;
        }
        time_in_states.resize(idx, TimeInState::default());
        //
        Ok(time_in_states)
    }
}
