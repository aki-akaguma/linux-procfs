use crate::error::ProcError;
use crate::scanner::ProcScanner;
use crate::stat::Cpu;
use crate::stat::Stat;
use crate::ProcResult;
use cfg_iif::cfg_iif;

#[derive(Debug, Default, Clone)]
pub(crate) struct StatParser {
    cpus_cap: usize,
}

impl StatParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<Stat> {
        let mut stat = if self.cpus_cap > 0 {
            Stat::with_capacity(self.cpus_cap)
        } else {
            Stat::default()
        };
        if sl.is_empty() {
            return Ok(stat);
        }
        //
        let mut sc = ProcScanner::new(sl);
        //
        // cpu
        sc.find_and_jump(b"cpu  ")?;
        stat.cpu.name = "cpu".to_string();
        //
        {
            // content of /proc/stat
            //   "cpu  15170491 3654501 2781868 219713999 930079 0 101596 0 0 0\n"
            // on linux:
            //   "cpu  %llu %llu %llu %llu %llu %llu %llu %llu\n"
            stat.cpu.user = sc.next(b' ')?;
            stat.cpu.nice = sc.next(b' ')?;
            stat.cpu.system = sc.next(b' ')?;
            stat.cpu.idle = sc.next(b' ')?;
            //
            stat.cpu.iowait = sc.next(b' ')?;
            stat.cpu.irq = sc.next(b' ')?;
            stat.cpu.softirq = sc.next(b' ')?;
            if !sc.check(b' ') {
                stat.cpu.steal = sc.next(b'\n')?;
            // up to here, on linux v2.6.18
            } else {
                stat.cpu.steal = sc.next(b' ')?;
                //
                stat.cpu.guest = sc.next(b' ')?;
                if !sc.check(b' ') {
                    stat.cpu.guest_nice = sc.next(b'\n')?;
                // up to here, on linux vx.x.x
                } else {
                    stat.cpu.guest_nice = sc.next(b' ')?;
                    if sc.check(b' ') {
                        cfg_iif!(feature = "has_stat_cguest" {
                            stat.cpu.cguest = sc.next(b'\n')?;
                        } else {
                            sc.scan_until(b'\n')?;
                        });
                    } else {
                        sc.scan_until(b'\n')?;
                    }
                }
            }
        }
        // cpu0, cpu1, cpu2 ... cpuN
        while sc.find_and_jump_opt(b"cpu") {
            // content of /proc/stat
            //   "cpu0 4291443 513175 595878 55021783 194990 0 6946 0 0 0\n"
            // on linux:
            //   "cpu%d %llu %llu %llu %llu %llu %llu %llu %llu\n"
            //
            let idx: usize = sc.next(b' ')?;
            if idx >= stat.cpus.len() {
                stat.cpus.resize(idx + 1, Cpu::default());
            }
            let cpu_ref: &mut Cpu = stat
                .cpus
                .get_mut(idx)
                .ok_or_else(|| ProcError::UnexpectedFormat("Failed to get CPU mut ref".into()))?;
            //
            cpu_ref.name = format!("cpu{idx}");
            //
            cpu_ref.user = sc.next(b' ')?;
            cpu_ref.nice = sc.next(b' ')?;
            cpu_ref.system = sc.next(b' ')?;
            cpu_ref.idle = sc.next(b' ')?;
            //
            cpu_ref.iowait = sc.next(b' ')?;
            cpu_ref.irq = sc.next(b' ')?;
            cpu_ref.softirq = sc.next(b' ')?;
            if !sc.check(b' ') {
                cpu_ref.steal = sc.next(b'\n')?;
            // up to here, on linux v2.6.18
            } else {
                cpu_ref.steal = sc.next(b' ')?;
                //
                cpu_ref.guest = sc.next(b' ')?;
                if !sc.check(b' ') {
                    cpu_ref.guest_nice = sc.next(b'\n')?;
                } else {
                    cpu_ref.guest_nice = sc.next(b' ')?;
                    cfg_iif!(feature = "has_stat_cguest" {
                        cpu_ref.cguest = sc.next(b'\n')?;
                    } else {
                        sc.scan_until(b'\n')?;
                    });
                }
            }
        }
        //
        // intr: nothing todo
        //
        //
        // content of /proc/stat
        //   "ctxt 1175115190\n"
        // on linux:
        //   "ctxt %llu\n"
        if sc.find_and_jump_opt(b"ctxt ") {
            stat.ctxt = sc.next(b'\n')?;
        }
        //
        cfg_iif!(feature = "has_stat_btime" {
            if sc.find_and_jump_opt(b"btime ") {
                stat.btime = sc.next(b'\n')?;
            }
        });
        //
        if sc.find_and_jump_opt(b"processes ") {
            stat.processes = sc.next(b'\n')?;
        }
        //
        cfg_iif!(feature = "has_stat_procs_running" {
            if sc.find_and_jump_opt(b"procs_running ") {
                stat.procs_running = sc.next(b'\n')?;
            }
        });
        //
        cfg_iif!(feature = "has_stat_procs_blocked" {
            if sc.find_and_jump_opt(b"procs_blocked ") {
                stat.procs_blocked = sc.next(b'\n')?;
            }
        });
        //
        // optimize the capacity size of the vector
        stat.cpus.shrink_to_fit();
        self.cpus_cap = stat.cpus.len();
        //
        Ok(stat)
    }
}
