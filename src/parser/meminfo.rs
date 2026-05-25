use crate::meminfo::MemInfo;
use crate::scanner::ProcScanner;
use crate::ProcResult;
use cfg_iif::cfg_iif;

#[derive(Debug, Default, Clone)]
pub(crate) struct MemInfoParser();

impl MemInfoParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<MemInfo> {
        let mut meminfo = MemInfo::default();
        if sl.is_empty() {
            return Ok(meminfo);
        }
        //
        let mut sc = ProcScanner::new(sl);
        //
        macro_rules! myscan_field {
            ($target:tt <= $needle:tt) => {{
                let needle = $needle;
                sc.find_and_jump(needle)?;
                sc.skip_spaces();
                meminfo.$target = sc.next(b' ')?;
            }};
            ($target:tt <= ? $needle:tt) => {{
                let needle = $needle;
                if sc.find_and_jump_opt(needle) {
                    sc.skip_spaces();
                    meminfo.$target = sc.next(b' ')?;
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
        Ok(meminfo)
    }
}
