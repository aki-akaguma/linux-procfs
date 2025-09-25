// linux source:
//   drivers/cpufreq/cpufreq_stats.c >= v2.6.18
// https://elixir.bootlin.com/linux/v2.6.18/source/drivers/cpufreq/cpufreq_stats.c#L83

use crate::cpufreqs::TimeInState;
use crate::util::find_to_pos;

#[allow(unused_imports)]
use crate::util::find_to_opt;

#[derive(Debug, Default, Clone)]
pub struct CpuFreqMaxParser();

impl CpuFreqMaxParser {
    pub fn parse(&mut self, sl: &[u8]) -> u32 {
        // content of /sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq
        //   "2403000\n"
        //
        if sl.is_empty() {
            return 0;
        }
        let s = &sl[0..sl.len() - 1];
        let input = String::from_utf8_lossy(s);
        input.as_ref().parse().unwrap()
    }
}

#[derive(Debug, Default, Clone)]
pub struct CpuFreqStatsTimeInStateParser();

impl CpuFreqStatsTimeInStateParser {
    pub fn parse(&mut self, sl: &[u8]) -> Vec<TimeInState> {
        let mut time_in_states = Vec::new();
        if sl.is_empty() {
            return time_in_states;
        }
        //
        let mut pos1: usize = 0;
        let mut pos2: usize;
        let mut pos_end: usize;
        let mut idx: usize = 0;
        'tis_loop: loop {
            if pos1 >= sl.len() {
                break 'tis_loop;
            }
            //
            let haystack = &sl[pos1..];
            let needle = b"\n";
            pos_end = pos1
                + 1
                + match find_to_opt(haystack, needle) {
                    Some(pos) => pos,
                    None => break 'tis_loop,
                };
            //
            if idx >= time_in_states.len() {
                time_in_states.resize(idx + 1, TimeInState::default());
            }
            let tis_ref: &mut TimeInState = match time_in_states.get_mut(idx) {
                Some(tis) => tis,
                None => unreachable!(),
            };
            //
            // content of /sys/devices/system/cpu/cpu0/cpufreq/stats/time_in_state
            //   "2403000 4202462\n"
            //   "2136000 2726644\n"
            // on linux:
            //   "%u %llu\n"
            //
            macro_rules! myscan {
                (check, $needle:expr) => {{
                    {
                        let haystack = &sl[pos1..pos_end];
                        let needle = $needle;
                        match find_to_opt(haystack, needle) {
                            Some(_pos) => true,
                            None => false,
                        }
                    }
                }};
                (skip, $needle:expr) => {{
                    pos2 = {
                        let haystack = &sl[pos1..pos_end];
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
            //
            if myscan!(check, b" ") {
                tis_ref.step = myscan!(b" ");
                tis_ref.value = myscan!(b"\n");
            } else {
                return vec![];
            }
            let _ = pos1;
            //
            idx += 1;
            pos1 = pos2 + 1;
        }
        time_in_states.resize(idx, TimeInState::default());
        //
        time_in_states
    }
}
