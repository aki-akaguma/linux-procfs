// linux source:
//   block/genhd.c >= v2.6.18
// https://elixir.bootlin.com/linux/v2.6.18/source/block/genhd.c#L550

use crate::diskstats::DiskStat;
use crate::diskstats::DiskStats;
use crate::util::{find_to_pos, skip_to_pos};
use cfg_iif::cfg_iif;

#[allow(unused_imports)]
use crate::util::find_to_opt;

#[derive(Debug, Default, Clone)]
pub struct DiskStatsParser();

impl DiskStatsParser {
    pub fn parse(&mut self, sl: &[u8]) -> DiskStats {
        let mut diskstats = DiskStats::default();
        if sl.is_empty() {
            return diskstats;
        }
        //
        let mut pos1: usize = 0;
        let mut pos2: usize;
        let mut pos_end: usize;
        let mut idx: usize = 0;
        'disk_loop: loop {
            if pos1 >= sl.len() {
                break 'disk_loop;
            }
            //
            let haystack = &sl[pos1..];
            let needle = b"\n";
            pos_end = pos1
                + 1
                + match find_to_opt(haystack, needle) {
                    Some(pos) => pos,
                    None => break 'disk_loop,
                };
            //
            if idx >= diskstats.disks.len() {
                diskstats.disks.resize(idx + 1, DiskStat::default());
            }
            let disk_ref: &mut DiskStat = match diskstats.disks.get_mut(idx) {
                Some(disk) => disk,
                None => unreachable!(),
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
                macro_rules! myscan {
                    (skip_spaces) => {{
                        pos1 += skip_to_pos(&sl[pos1..pos_end], b' ');
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
                    (or, $needle1:expr, $needle2:expr) => {{
                        let len1_opt = {
                            let haystack = &sl[pos1..pos_end];
                            let needle = $needle1;
                            find_to_opt(haystack, needle)
                        };
                        let len2_opt = {
                            let haystack = &sl[pos1..pos_end];
                            let needle = $needle2;
                            find_to_opt(haystack, needle)
                        };
                        //
                        let s = {
                            let len = {
                                match len1_opt {
                                    Some(len1) => match len2_opt {
                                        Some(len2) => {
                                            if len1 < len2 {
                                                len1
                                            } else {
                                                len2
                                            }
                                        }
                                        None => len1,
                                    },
                                    None => match len2_opt {
                                        Some(len2) => len2,
                                        None => {
                                            unreachable!();
                                        }
                                    },
                                }
                            };
                            pos2 = pos1 + len;
                            let s = &sl[pos1..pos2];
                            pos1 = pos2 + 1;
                            s
                        };
                        let input = String::from_utf8_lossy(s);
                        input.as_ref().parse().unwrap()
                    }};
                }
                cfg_iif!(feature = "has_diskstats_device_number" {
                    myscan!(skip_spaces);
                    disk_ref.major_num = myscan!(b" ");
                    myscan!(skip_spaces);
                    disk_ref.minor_num = myscan!(b" ");
                } else {
                    // device major number
                    myscan!(skip_spaces);
                    myscan!(skip, b" ");
                    // device mainor number
                    myscan!(skip_spaces);
                    myscan!(skip, b" ");
                });
                // device name
                {
                    let s = myscan!(skip, b" ");
                    disk_ref.name = String::from_utf8_lossy(s).into_owned();
                }
                // read & write
                {
                    disk_ref.rio = myscan!(b" ");
                    disk_ref.rmerge = myscan!(b" ");
                    disk_ref.rblk = myscan!(b" ");
                    disk_ref.ruse = myscan!(b" ");
                    //
                    disk_ref.wio = myscan!(b" ");
                    disk_ref.wmerge = myscan!(b" ");
                    disk_ref.wblk = myscan!(b" ");
                    disk_ref.wuse = myscan!(b" ");
                }
                //
                cfg_iif!(feature = "has_diskstats_running" {
                    disk_ref.running = myscan!(b" ");
                } else {
                    myscan!(skip, b" "); // running
                });
                //
                cfg_iif!(feature = "has_diskstats_use" {
                    disk_ref.use_ = myscan!(b" ");
                } else {
                    myscan!(skip, b" "); // use
                });
                //
                {
                    disk_ref.aveq = myscan!(or, b" ", b"\n");
                }
                if sl[pos2] != b'\n' {
                    let _ = myscan!(skip, b"\n");
                }
                let _ = pos1;
            }
            //
            idx += 1;
            pos1 = pos2 + 1;
        }
        diskstats.disks.resize(idx, DiskStat::default());
        //
        diskstats
    }
}
