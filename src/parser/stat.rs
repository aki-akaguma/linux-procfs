use crate::stat::Cpu;
use crate::stat::Stat;
use crate::util::find_to_opt;
use crate::util::find_to_pos;
use cfg_iif::cfg_iif;

#[derive(Debug, Default, Clone)]
pub struct StatParser {
    cpus_cap: usize,
}

impl StatParser {
    pub fn parse(&mut self, sl: &[u8]) -> Stat {
        let mut stat = if self.cpus_cap > 0 {
            Stat::with_capacity(self.cpus_cap)
        } else {
            Stat::default()
        };
        if sl.is_empty() {
            return stat;
        }
        //
        let mut pos1: usize = 0;
        let mut pos2: usize;
        let mut pos_end: usize;
        //
        // cpu
        let haystack = &sl[pos1..];
        let needle = b"cpu  ";
        pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
        stat.cpu.name = "cpu".to_string();
        //
        let haystack = &sl[pos1..];
        let needle = b"\n";
        pos_end = pos1
            + 1
            + match find_to_opt(haystack, needle) {
                Some(pos) => pos,
                None => unreachable!(),
            };
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
        {
            // content of /proc/stat
            //   "cpu  15170491 3654501 2781868 219713999 930079 0 101596 0 0 0\n"
            // on linux:
            //   "cpu  %llu %llu %llu %llu %llu %llu %llu %llu\n"
            stat.cpu.user = myscan!(b" ");
            stat.cpu.nice = myscan!(b" ");
            stat.cpu.system = myscan!(b" ");
            stat.cpu.idle = myscan!(b" ");
            //
            stat.cpu.iowait = myscan!(b" ");
            stat.cpu.irq = myscan!(b" ");
            stat.cpu.softirq = myscan!(b" ");
            if !myscan!(check, b" ") {
                stat.cpu.steal = myscan!(b"\n");
            // up to here, on linux v2.6.18
            } else {
                stat.cpu.steal = myscan!(b" ");
                //
                stat.cpu.guest = myscan!(b" ");
                if !myscan!(check, b" ") {
                    stat.cpu.guest_nice = myscan!(b"\n");
                // up to here, on linux vx.x.x
                } else {
                    stat.cpu.guest_nice = myscan!(b" ");
                    if myscan!(check, b" ") {
                        cfg_iif!(feature = "has_stat_cguest" {
                            stat.cpu.cguest = myscan!(b"\n");
                        } else {
                            myscan!(skip, b"\n");
                        });
                    } else {
                        myscan!(skip, b"\n");
                    }
                }
            }
            let _ = pos1;
        }
        // cpu0, cpu1, cpu2 ... cpuN
        'cpu_lp: loop {
            // content of /proc/stat
            //   "cpu0 4291443 513175 595878 55021783 194990 0 6946 0 0 0\n"
            // on linux:
            //   "cpu%d %llu %llu %llu %llu %llu %llu %llu %llu\n"
            let haystack = &sl[pos1..];
            let needle = b"cpu";
            pos1 = pos1
                + needle.len()
                + match find_to_opt(haystack, needle) {
                    Some(pos) => pos,
                    None => break 'cpu_lp,
                };
            //
            let haystack = &sl[pos1..];
            let needle = b"\n";
            pos_end = pos1
                + 1
                + match find_to_opt(haystack, needle) {
                    Some(pos) => pos,
                    None => unreachable!(),
                };
            //
            let idx: usize = myscan!(b" ");
            if idx >= stat.cpus.len() {
                stat.cpus.resize(idx + 1, Cpu::default());
            }
            let cpu_ref: &mut Cpu = match stat.cpus.get_mut(idx) {
                Some(cpu) => cpu,
                None => unreachable!(),
            };
            //
            cpu_ref.name = format!("cpu{idx}");
            //
            cpu_ref.user = myscan!(b" ");
            cpu_ref.nice = myscan!(b" ");
            cpu_ref.system = myscan!(b" ");
            cpu_ref.idle = myscan!(b" ");
            //
            cpu_ref.iowait = myscan!(b" ");
            cpu_ref.irq = myscan!(b" ");
            cpu_ref.softirq = myscan!(b" ");
            if !myscan!(check, b" ") {
                cpu_ref.steal = myscan!(b"\n");
            // up to here, on linux v2.6.18
            } else {
                cpu_ref.steal = myscan!(b" ");
                //
                cpu_ref.guest = myscan!(b" ");
                if !myscan!(check, b" ") {
                    cpu_ref.guest_nice = myscan!(b"\n");
                } else {
                    cpu_ref.guest_nice = myscan!(b" ");
                    cfg_iif!(feature = "has_stat_cguest" {
                        cpu_ref.cguest = myscan!(b"\n");
                    } else {
                        myscan!(skip, b"\n");
                    });
                }
            }
            let _ = pos1;
        }
        //
        // intr: nothing todo
        //
        macro_rules! myscan {
            (skip, $needle:expr) => {{
                pos2 = {
                    let haystack = &sl[pos1..];
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
        // content of /proc/stat
        //   "ctxt 1175115190\n"
        // on linux:
        //   "ctxt %llu\n"
        {
            let haystack = &sl[pos1..];
            let needle = b"ctxt ";
            pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
            stat.ctxt = myscan!(b"\n");
        }
        //
        cfg_iif!(feature = "has_stat_btime" {
            let haystack = &sl[pos1..];
            let needle = b"btime ";
            pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
            stat.btime = myscan!(b"\n");
        });
        //
        {
            let haystack = &sl[pos1..];
            let needle = b"processes ";
            pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
            stat.processes = myscan!(b"\n");
        }
        //
        cfg_iif!(feature = "has_stat_procs_running" {
            let haystack = &sl[pos1..];
            let needle = b"procs_running ";
            pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
            stat.procs_running = myscan!(b"\n");
        });
        //
        cfg_iif!(feature = "has_stat_procs_blocked" {
            let haystack = &sl[pos1..];
            let needle = b"procs_blocked ";
            pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
            stat.procs_blocked = myscan!(b"\n");
        });
        let _ = pos1;
        //
        // optimize the capacity size of the vector
        stat.cpus.shrink_to_fit();
        self.cpus_cap = stat.cpus.len();
        //
        stat
    }
}
