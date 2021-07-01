// linux source:
//   fs/proc_misc.c <= v2.6.27
//   fs/meminfo.c   >= v2.6.28
// https://elixir.bootlin.com/linux/v2.6.27/source/fs/proc/proc_misc.c
// https://elixir.bootlin.com/linux/v2.6.28/source/fs/proc/meminfo.c

use crate::meminfo::MemInfo;
use crate::util::{find_to_pos, skip_to_pos};
use cfg_iif::cfg_iif;

#[allow(unused_imports)]
use crate::util::find_to_opt;

#[derive(Debug, Default, Clone)]
pub struct MemInfoParser();

impl MemInfoParser {
    pub fn parse(&mut self, sl: &[u8]) -> MemInfo {
        let mut meminfo = MemInfo::default();
        if sl.is_empty() {
            return meminfo;
        }
        //
        let mut pos1: usize = 0;
        let mut pos2: usize;
        //
        macro_rules! myscan {
            (check, $needle:expr) => {{
                {
                    let haystack = &sl[pos1..];
                    let needle = $needle;
                    match find_to_opt(haystack, needle) {
                        Some(_pos) => true,
                        None => false,
                    }
                }
            }};
            (skip_spaces) => {{
                pos1 += skip_to_pos(&sl[pos1..], b' ');
            }};
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
        macro_rules! myscan_field {
            ($target:tt <= $needle:tt) => {{
                let haystack = &sl[pos1..];
                let needle = $needle;
                pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
                myscan!(skip_spaces);
                meminfo.$target = myscan!(b" ");
            }};
            ($target:tt <= ? $needle:tt) => {{
                let haystack = &sl[pos1..];
                let needle = $needle;
                if myscan!(check, needle) {
                    pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
                    myscan!(skip_spaces);
                    meminfo.$target = myscan!(b" ");
                }
            }};
        }
        //
        // content of /proc/meminfo
        // "MemTotal:        8174580 kB\n"
        // on linux:
        //      "MemTotal:     %8lu kB\n" ...
        //      "VmallocChunk: %8lu kB\n"
        //
        myscan_field!(mem_total_kb <= b"MemTotal:");
        myscan_field!(mem_free_kb <= b"MemFree:");
        //
        cfg_iif!(feature = "has_meminfo_mem_available" {
            myscan_field!(mem_available_kb <=? b"MemAvailable:");
        });
        //
        myscan_field!(buffers_kb <= b"Buffers:");
        myscan_field!(cached_kb <= b"Cached:");
        myscan_field!(swap_cached_kb <= b"SwapCached:");
        myscan_field!(active_kb <= b"Active:");
        myscan_field!(inactive_kb <= b"Inactive:");
        //
        cfg_iif!(feature = "has_meminfo_active_anon" {
            myscan_field!(active_anon_kb <=? b"Active(anon):");
        });
        //
        cfg_iif!(feature = "has_meminfo_inactive_anon" {
            myscan_field!(inactive_anon_kb <=? b"Inactive(anon):");
        });
        //
        cfg_iif!(feature = "has_meminfo_active_file" {
            myscan_field!(active_file_kb <=? b"Active(file):");
        });
        //
        cfg_iif!(feature = "has_meminfo_inactive_file" {
            myscan_field!(inactive_file_kb <=? b"Inactive(file):");
        });
        //
        cfg_iif!(feature = "has_meminfo_unevictable" {
            myscan_field!(unevictable_kb <=? b"Unevictable:");
        });
        //
        cfg_iif!(feature = "has_meminfo_mlocked" {
            myscan_field!(mlocked_kb <=? b"Mlocked:");
        });
        //
        cfg_iif!(feature = "has_meminfo_high_total" {
            myscan_field!(high_total_kb <=? b"HighTotal:");
        });
        //
        cfg_iif!(feature = "has_meminfo_high_free" {
            myscan_field!(high_free_kb <=? b"HighFree:");
        });
        //
        cfg_iif!(feature = "has_meminfo_low_total" {
            myscan_field!(low_total_kb <=? b"LowTotal:");
        });
        //
        cfg_iif!(feature = "has_meminfo_low_free" {
            myscan_field!(low_free_kb <=? b"LowFree:");
        });
        //
        cfg_iif!(feature = "has_meminfo_mmap_copy" {
            myscan_field!(mmap_copy_kb <=? b"MmapCopy:");
        });
        //
        myscan_field!(swap_total_kb <= b"SwapTotal:");
        myscan_field!(swap_free_kb <= b"SwapFree:");
        //
        cfg_iif!(feature = "has_meminfo_dirty" {
            myscan_field!(dirty_kb <=? b"Dirty:");
        });
        //
        cfg_iif!(feature = "has_meminfo_writeback" {
            myscan_field!(writeback_kb <=? b"Writeback:");
        });
        //
        cfg_iif!(feature = "has_meminfo_anon_pages" {
            myscan_field!(anon_pages_kb <=? b"AnonPages:");
        });
        //
        cfg_iif!(feature = "has_meminfo_mapped" {
            myscan_field!(mapped_kb <=? b"Mapped:");
        });
        //
        cfg_iif!(feature = "has_meminfo_slab" {
            myscan_field!(slab_kb <=? b"Slab:");
        });
        cfg_iif!(feature = "has_meminfo_slab_reclaimable" {
            // "slab reclaimable" (dentry and inode structures)
            myscan_field!(slab_reclaimable_kb <=? b"SReclaimable:");
        });
        cfg_iif!(feature = "has_meminfo_slab_unreclaim" {
            myscan_field!(slab_unreclaim_kb <=? b"SUnreclaim:");
        });
        //
        cfg_iif!(feature = "has_meminfo_page_tables" {
            myscan_field!(page_tables_kb <=? b"PageTables:");
        });
        //
        cfg_iif!(feature = "has_meminfo_nfs_unstable" {
            myscan_field!(nfs_unstable_kb <=? b"NFS_Unstable:");
        });
        //
        cfg_iif!(feature = "has_meminfo_bounce" {
            myscan_field!(bounce_kb <=? b"Bounce:");
        });
        //
        cfg_iif!(feature = "has_meminfo_commit_limit" {
            myscan_field!(commit_limit_kb <=? b"CommitLimit:");
        });
        //
        cfg_iif!(feature = "has_meminfo_commit_as" {
            myscan_field!(commited_as_kb <=? b"Committed_AS:");
        });
        //
        cfg_iif!(feature = "has_meminfo_vmalloc_total" {
            myscan_field!(vmalloc_total_kb <=? b"VmallocTotal:");
        });
        //
        cfg_iif!(feature = "has_meminfo_vmalloc_used" {
            myscan_field!(vmalloc_used_kb <=? b"VmallocUsed:");
        });
        //
        cfg_iif!(feature = "has_meminfo_vmalloc_chunk" {
            myscan_field!(vmalloc_chunk_kb <=? b"VmallocChunk:");
        });
        //
        let _ = pos1;
        //
        meminfo
    }
}
