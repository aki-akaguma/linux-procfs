// linux source:
//   block/genhd.c >= v2.6.18
// https://elixir.bootlin.com/linux/v2.6.18/source/block/genhd.c#L550

use crate::diskstats::DiskStat;
use crate::diskstats::DiskStats;
use crate::scanner::ProcScanner;
use crate::ProcResult;
use cfg_iif::cfg_iif;

#[derive(Debug, Default, Clone)]
pub struct DiskStatsParser();

impl DiskStatsParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<DiskStats> {
        let mut diskstats = DiskStats::default();
        if sl.is_empty() {
            return Ok(diskstats);
        }
        //
        let mut sc = ProcScanner::new(sl);
        let mut idx: usize = 0;
        while !sc.is_empty() {
            //
            if idx >= diskstats.disks.len() {
                diskstats.disks.resize(idx + 1, DiskStat::default());
            }
            let disk_ref: &mut DiskStat = match diskstats.disks.get_mut(idx) {
                Some(disk) => disk,
                None => return Err(crate::ProcError::InternalError),
            };
            {
                // content of /proc/diskstats
                //   "   8       0 sda 808426 1058879 43196702 7472309 1536575 1841370 131099984 42740862 0 8137730 50228410\n"
                //   "   8       5 sda5 500283 837502 22925858 3932966 1183846 1552668 116130520 33073524 0 5715397 37007467\n"
                // on linux:
                //   "%4d %4d %s %lu %lu %llu %u %lu %lu %llu %u %u %u %u\n"
                //
                // on linux 5.4.0
                //   " 252       0 vda 14907 5047 1609802 6814 37573 12037 1280416 107798 0 34264 78540 0 0 0 0\n"
                //
                cfg_iif!(feature = "has_diskstats_device_number" {
                    sc.skip_spaces();
                    disk_ref.major_num = sc.next(b' ')?;
                    sc.skip_spaces();
                    disk_ref.minor_num = sc.next(b' ')?;
                } else {
                    // device major number
                    sc.skip_spaces();
                    sc.scan_until(b' ')?;
                    // device minor number
                    sc.skip_spaces();
                    sc.scan_until(b' ')?;
                });
                // device name
                {
                    sc.skip_spaces();
                    let s = sc.scan_until(b' ')?;
                    disk_ref.name = String::from_utf8_lossy(s).into_owned();
                }
                // read & write
                {
                    sc.skip_spaces();
                    disk_ref.rio = sc.next(b' ')?;
                    sc.skip_spaces();
                    disk_ref.rmerge = sc.next(b' ')?;
                    sc.skip_spaces();
                    disk_ref.rblk = sc.next(b' ')?;
                    sc.skip_spaces();
                    disk_ref.ruse = sc.next(b' ')?;
                    //
                    sc.skip_spaces();
                    disk_ref.wio = sc.next(b' ')?;
                    sc.skip_spaces();
                    disk_ref.wmerge = sc.next(b' ')?;
                    sc.skip_spaces();
                    disk_ref.wblk = sc.next(b' ')?;
                    sc.skip_spaces();
                    disk_ref.wuse = sc.next(b' ')?;
                }
                //
                cfg_iif!(feature = "has_diskstats_running" {
                    sc.skip_spaces();
                    disk_ref.running = sc.next(b' ')?;
                } else {
                    sc.skip_spaces();
                    sc.scan_until(b' ')?; // running
                });
                //
                cfg_iif!(feature = "has_diskstats_use" {
                    sc.skip_spaces();
                    disk_ref.use_ = sc.next(b' ')?;
                } else {
                    sc.skip_spaces();
                    sc.scan_until(b' ')?; // use
                });
                //
                {
                    sc.skip_spaces();
                    disk_ref.aveq = sc.next_any(b" \n")?;
                    if sc.last_delimiter() != b'\n' {
                        sc.scan_until(b'\n')?;
                    }
                }
            }
            //
            idx += 1;
        }
        diskstats.disks.resize(idx, DiskStat::default());
        //
        Ok(diskstats)
    }
}
