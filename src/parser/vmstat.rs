// linux source:
//   mm/vmstat.c   >= v2.6.18
// https://elixir.bootlin.com/linux/v2.6.18/source/mm/vmstat.c#L438
// https://elixir.bootlin.com/linux/v2.6.26/source/mm/vmstat.c#L601

use crate::util::{find_to_pos, skip_to_pos};
use crate::vmstat::VmStat;
use cfg_iif::cfg_iif;

#[allow(unused_imports)]
use crate::util::find_to_opt;

#[derive(Debug, Default, Clone)]
pub struct VmStatParser();

impl VmStatParser {
    pub fn parse(&mut self, sl: &[u8]) -> VmStat {
        let mut vmstat = VmStat::default();
        if sl.is_empty() {
            return vmstat;
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
                vmstat.$target = myscan!(b"\n");
            }};
            ($target:tt <= ? $needle:tt) => {{
                let haystack = &sl[pos1..];
                let needle = $needle;
                if myscan!(check, needle) {
                    pos1 = pos1 + needle.len() + find_to_pos(haystack, needle);
                    myscan!(skip_spaces);
                    vmstat.$target = myscan!(b"\n");
                }
            }};
        }
        //
        // content of /proc/vmstat
        //   "pgpgin 21624735\n"
        //   "pgpgout 66647980\n"
        // on linux:
        //   "%s %lu\n"
        //
        cfg_iif!(feature = "has_vmstat_nr_free_pages" {
            myscan_field!(nr_free_pages <=? b"nr_free_pages ");
        });
        let pos1_bk = pos1;
        cfg_iif!(feature = "has_vmstat_nr_inactive" {
            myscan_field!(nr_inactive <=? b"nr_inactive ");
        });
        let _ = pos1;
        pos1 = pos1_bk;
        cfg_iif!(feature = "has_vmstat_nr_active" {
            myscan_field!(nr_active <=? b"nr_active ");
        });
        //
        cfg_iif!(feature = "has_vmstat_nr_anon_pages" {
            myscan_field!(nr_anon_pages <=? b"nr_anon_pages ");
        });
        cfg_iif!(feature = "has_vmstat_nr_mapped" {
            myscan_field!(nr_mapped <=? b"nr_mapped ");
        });
        cfg_iif!(feature = "has_vmstat_nr_file_pages" {
            myscan_field!(nr_file_pages <=? b"nr_file_pages ");
        });
        //
        let pos1_bk = pos1;
        cfg_iif!(feature = "has_vmstat_nr_slab" {
            myscan_field!(nr_slab <=? b"nr_slab ");
        });
        cfg_iif!(feature = "has_vmstat_nr_slab_reclaimable" {
            myscan_field!(nr_slab_reclaimable <=? b"nr_slab_reclaimable ");
        });
        cfg_iif!(feature = "has_vmstat_nr_slab_unreclaimable" {
            myscan_field!(nr_slab_unreclaimable <=? b"nr_slab_unreclaimable ");
        });
        //
        cfg_iif!(feature = "has_vmstat_nr_page_table_pages" {
            myscan_field!(nr_page_table_pages <=? b"nr_page_table_pages ");
        });
        let _ = pos1;
        pos1 = pos1_bk;
        cfg_iif!(feature = "has_vmstat_nr_dirty" {
            myscan_field!(nr_dirty <=? b"nr_dirty ");
        });
        //
        cfg_iif!(feature = "has_vmstat_nr_writeback" {
            myscan_field!(nr_writeback <=? b"nr_writeback ");
        });
        cfg_iif!(feature = "has_vmstat_nr_unstable" {
            myscan_field!(nr_unstable <=? b"nr_unstable ");
        });
        cfg_iif!(feature = "has_vmstat_nr_bounce" {
            myscan_field!(nr_bounce <=? b"nr_bounce ");
        });
        //
        cfg_iif!(feature = "has_vmstat_nr_vmscan_write" {
            myscan_field!(nr_vmscan_write <=? b"nr_vmscan_write ");
        });
        //
        cfg_iif!(feature = "has_vmstat_nr_writeback_temp" {
            myscan_field!(nr_writeback_temp <=? b"nr_writeback_temp ");
        });
        //
        cfg_iif!(feature = "has_vmstat_numa_hit" {
            myscan_field!(numa_hit <=? b"numa_hit ");
        });
        cfg_iif!(feature = "has_vmstat_numa_miss" {
            myscan_field!(numa_miss <=? b"numa_miss ");
        });
        cfg_iif!(feature = "has_vmstat_numa_foreign" {
            myscan_field!(numa_foreign <=? b"numa_foreign ");
        });
        cfg_iif!(feature = "has_vmstat_numa_interleave" {
            myscan_field!(numa_interleave <=? b"numa_interleave ");
        });
        cfg_iif!(feature = "has_vmstat_numa_local" {
            myscan_field!(numa_local <=? b"numa_local ");
        });
        cfg_iif!(feature = "has_vmstat_numa_other" {
            myscan_field!(numa_other <=? b"numa_other ");
        });
        //
        myscan_field!(pgpgin <= b"pgpgin ");
        myscan_field!(pgpgout <= b"pgpgout ");
        myscan_field!(pswpin <= b"pswpin ");
        myscan_field!(pswpout <= b"pswpout ");
        //
        cfg_iif!(feature = "has_vmstat_pgalloc" {
            myscan_field!(pgalloc_dma <=? b"pgalloc_dma ");
        });
        cfg_iif!(feature = "has_vmstat_pgalloc" {
            myscan_field!(pgalloc_dma32 <=? b"pgalloc_dma32 ");
        });
        cfg_iif!(feature = "has_vmstat_pgalloc" {
            myscan_field!(pgalloc_normal <=? b"pgalloc_normal ");
        });
        cfg_iif!(feature = "has_vmstat_pgalloc" {
            myscan_field!(pgalloc_high <=? b"pgalloc_high ");
        });
        cfg_iif!(feature = "has_vmstat_pgalloc" {
            myscan_field!(pgalloc_movable <=? b"pgalloc_movable ");
        });
        //
        cfg_iif!(feature = "has_vmstat_pgfree" {
            myscan_field!(pgfree <=? b"pgfree ");
        });
        cfg_iif!(feature = "has_vmstat_pgactivate" {
            myscan_field!(pgactivate <=? b"pgactivate ");
        });
        cfg_iif!(feature = "has_vmstat_pgdeactivate" {
            myscan_field!(pgdeactivate <=? b"pgdeactivate ");
        });
        //
        cfg_iif!(feature = "has_vmstat_pgfault" {
            myscan_field!(pgfault <=? b"pgfault ");
        });
        cfg_iif!(feature = "has_vmstat_pgmajfault" {
            myscan_field!(pgmajfault <=? b"pgmajfault ");
        });
        //
        cfg_iif!(feature = "has_vmstat_pgrefill" {
            myscan_field!(pgrefill_dma <=? b"pgrefill_dma ");
        });
        cfg_iif!(feature = "has_vmstat_pgrefill" {
            myscan_field!(pgrefill_dma32 <=? b"pgrefill_dma32 ");
        });
        cfg_iif!(feature = "has_vmstat_pgrefill" {
            myscan_field!(pgrefill_normal <=? b"pgrefill_normal ");
        });
        cfg_iif!(feature = "has_vmstat_pgrefill" {
            myscan_field!(pgrefill_high <=? b"pgrefill_high ");
        });
        cfg_iif!(feature = "has_vmstat_pgrefill" {
            myscan_field!(pgrefill_movable <=? b"pgrefill_movable ");
        });
        //
        cfg_iif!(feature = "has_vmstat_pgsteal" {
            myscan_field!(pgsteal_dma <=? b"pgsteal_dma ");
        });
        cfg_iif!(feature = "has_vmstat_pgsteal" {
            myscan_field!(pgsteal_dma32 <=? b"pgsteal_dma32 ");
        });
        cfg_iif!(feature = "has_vmstat_pgsteal" {
            myscan_field!(pgsteal_normal <=? b"pgsteal_normal ");
        });
        cfg_iif!(feature = "has_vmstat_pgsteal" {
            myscan_field!(pgsteal_high <=? b"pgsteal_high ");
        });
        cfg_iif!(feature = "has_vmstat_pgsteal" {
            myscan_field!(pgsteal_movable <=? b"pgsteal_movable ");
        });
        //
        cfg_iif!(feature = "has_vmstat_pgscan_kswapd" {
            myscan_field!(pgscan_kswapd_dma <=? b"pgscan_kswapd_dma ");
        });
        cfg_iif!(feature = "has_vmstat_pgscan_kswapd" {
            myscan_field!(pgscan_kswapd_dma32 <=? b"pgscan_kswapd_dma32 ");
        });
        cfg_iif!(feature = "has_vmstat_pgscan_kswapd" {
            myscan_field!(pgscan_kswapd_normal <=? b"pgscan_kswapd_normal ");
        });
        cfg_iif!(feature = "has_vmstat_pgscan_kswapd" {
            myscan_field!(pgscan_kswapd_high <=? b"pgscan_kswapd_high ");
        });
        cfg_iif!(feature = "has_vmstat_pgscan_kswapd" {
            myscan_field!(pgscan_kswapd_movable <=? b"pgscan_kswapd_movable ");
        });
        //
        cfg_iif!(feature = "has_vmstat_pgscan_direct" {
            myscan_field!(pgscan_direct_dma <=? b"pgscan_direct_dma ");
        });
        cfg_iif!(feature = "has_vmstat_pgscan_direct" {
            myscan_field!(pgscan_direct_dma32 <=? b"pgscan_direct_dma32 ");
        });
        cfg_iif!(feature = "has_vmstat_pgscan_direct" {
            myscan_field!(pgscan_direct_normal <=? b"pgscan_direct_normal ");
        });
        cfg_iif!(feature = "has_vmstat_pgscan_direct" {
            myscan_field!(pgscan_direct_high <=? b"pgscan_direct_high ");
        });
        cfg_iif!(feature = "has_vmstat_pgscan_direct" {
            myscan_field!(pgscan_direct_movable <=? b"pgscan_direct_movable ");
        });
        //
        cfg_iif!(feature = "has_vmstat_pginodesteal" {
            myscan_field!(pginodesteal <=? b"pginodesteal ");
        });
        cfg_iif!(feature = "has_vmstat_slabs_scanned" {
            myscan_field!(slabs_scanned <=? b"slabs_scanned ");
        });
        cfg_iif!(feature = "has_vmstat_kswapd_steal" {
            myscan_field!(kswapd_steal <=? b"kswapd_steal ");
        });
        cfg_iif!(feature = "has_vmstat_kswapd_inodesteal" {
            myscan_field!(kswapd_inodesteal <=? b"kswapd_inodesteal ");
        });
        cfg_iif!(feature = "has_vmstat_pageoutrun" {
            myscan_field!(pageoutrun <=? b"pageoutrun ");
        });
        cfg_iif!(feature = "has_vmstat_allocstall" {
            myscan_field!(allocstall <=? b"allocstall ");
        });
        //
        cfg_iif!(feature = "has_vmstat_pgrotated" {
            myscan_field!(pgrotated <=? b"pgrotated ");
        });
        //
        cfg_iif!(feature = "has_vmstat_htlb_buddy_alloc" {
            myscan_field!(htlb_buddy_alloc_success <=? b"htlb_buddy_alloc_success ");
        });
        cfg_iif!(feature = "has_vmstat_htlb_buddy_alloc" {
            myscan_field!(htlb_buddy_alloc_fail <=? b"htlb_buddy_alloc_fail ");
        });
        //
        let _ = pos1;
        //
        vmstat
    }
}
