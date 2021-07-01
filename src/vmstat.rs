#[derive(Debug, Default, Clone)]
pub struct VmStat {
    //{{{ >= linux v2.6.21
    #[cfg(feature = "has_vmstat_nr_free_pages")]
    pub nr_free_pages: usize,
    #[cfg(feature = "has_vmstat_nr_active")]
    pub nr_active: usize,
    #[cfg(feature = "has_vmstat_nr_inactive")]
    pub nr_inactive: usize,
    //}}}
    //{{{ >= linux v2.6.18
    #[cfg(feature = "has_vmstat_nr_anon_pages")]
    pub nr_anon_pages: usize,
    #[cfg(feature = "has_vmstat_nr_mapped")]
    pub nr_mapped: usize,
    #[cfg(feature = "has_vmstat_nr_file_pages")]
    pub nr_file_pages: usize,
    #[cfg(feature = "has_vmstat_nr_slab")]
    pub nr_slab: usize,
    //}}}
    //{{{ >= linux v2.6.19
    #[cfg(feature = "has_vmstat_nr_slab_reclaimable")]
    pub nr_slab_reclaimable: usize,
    #[cfg(feature = "has_vmstat_nr_slab_unreclaimable")]
    pub nr_slab_unreclaimable: usize,
    //}}}
    //{{{ >= linux v2.6.18
    #[cfg(feature = "has_vmstat_nr_page_table_pages")]
    pub nr_page_table_pages: usize,
    #[cfg(feature = "has_vmstat_nr_dirty")]
    pub nr_dirty: usize,
    #[cfg(feature = "has_vmstat_nr_writeback")]
    pub nr_writeback: usize,
    #[cfg(feature = "has_vmstat_nr_unstable")]
    pub nr_unstable: usize,
    #[cfg(feature = "has_vmstat_nr_bounce")]
    pub nr_bounce: usize,
    //}}}
    //{{{ >= linux v2.6.19
    #[cfg(feature = "has_vmstat_nr_vmscan_write")]
    pub nr_vmscan_write: usize,
    //}}}
    //{{{ >= linux v2.6.26
    #[cfg(feature = "has_vmstat_nr_writeback_temp")]
    pub nr_writeback_temp: usize,
    //}}}
    //
    //{{{ >= linux v2.6.18 && CONFIG_NUMA
    #[cfg(feature = "has_vmstat_numa_hit")]
    pub numa_hit: usize,
    #[cfg(feature = "has_vmstat_numa_miss")]
    pub numa_miss: usize,
    #[cfg(feature = "has_vmstat_numa_foreign")]
    pub numa_foreign: usize,
    #[cfg(feature = "has_vmstat_numa_interleave")]
    pub numa_interleave: usize,
    #[cfg(feature = "has_vmstat_numa_local")]
    pub numa_local: usize,
    #[cfg(feature = "has_vmstat_numa_other")]
    pub numa_other: usize,
    //}}}
    //
    //{{{ >= linux v2.6.18 && CONFIG_VM_EVENT_COUNTERS
    pub pgpgin: usize,
    pub pgpgout: usize,
    pub pswpin: usize,
    pub pswpout: usize,
    //
    #[cfg(feature = "has_vmstat_pgalloc")]
    pub pgalloc_dma: usize,
    #[cfg(feature = "has_vmstat_pgalloc")]
    pub pgalloc_dma32: usize,
    #[cfg(feature = "has_vmstat_pgalloc")]
    pub pgalloc_normal: usize,
    #[cfg(feature = "has_vmstat_pgalloc")]
    pub pgalloc_high: usize,
    #[cfg(feature = "has_vmstat_pgalloc")]
    pub pgalloc_movable: usize,
    //
    #[cfg(feature = "has_vmstat_pgfree")]
    pub pgfree: usize,
    #[cfg(feature = "has_vmstat_pgactivate")]
    pub pgactivate: usize,
    #[cfg(feature = "has_vmstat_pgdeactivate")]
    pub pgdeactivate: usize,
    //
    #[cfg(feature = "has_vmstat_pgfault")]
    pub pgfault: usize,
    #[cfg(feature = "has_vmstat_pgmajfault")]
    pub pgmajfault: usize,
    //
    #[cfg(feature = "has_vmstat_pgrefill")]
    pub pgrefill_dma: usize,
    #[cfg(feature = "has_vmstat_pgrefill")]
    pub pgrefill_dma32: usize,
    #[cfg(feature = "has_vmstat_pgrefill")]
    pub pgrefill_normal: usize,
    #[cfg(feature = "has_vmstat_pgrefill")]
    pub pgrefill_high: usize,
    #[cfg(feature = "has_vmstat_pgrefill")]
    pub pgrefill_movable: usize,
    //
    #[cfg(feature = "has_vmstat_pgsteal")]
    pub pgsteal_dma: usize,
    #[cfg(feature = "has_vmstat_pgsteal")]
    pub pgsteal_dma32: usize,
    #[cfg(feature = "has_vmstat_pgsteal")]
    pub pgsteal_normal: usize,
    #[cfg(feature = "has_vmstat_pgsteal")]
    pub pgsteal_high: usize,
    #[cfg(feature = "has_vmstat_pgsteal")]
    pub pgsteal_movable: usize,
    //
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    pub pgscan_kswapd_dma: usize,
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    pub pgscan_kswapd_dma32: usize,
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    pub pgscan_kswapd_normal: usize,
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    pub pgscan_kswapd_high: usize,
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    pub pgscan_kswapd_movable: usize,
    //
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    pub pgscan_direct_dma: usize,
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    pub pgscan_direct_dma32: usize,
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    pub pgscan_direct_normal: usize,
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    pub pgscan_direct_high: usize,
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    pub pgscan_direct_movable: usize,
    //
    #[cfg(feature = "has_vmstat_pginodesteal")]
    pub pginodesteal: usize,
    #[cfg(feature = "has_vmstat_slabs_scanned")]
    pub slabs_scanned: usize,
    #[cfg(feature = "has_vmstat_kswapd_steal")]
    pub kswapd_steal: usize,
    #[cfg(feature = "has_vmstat_kswapd_inodesteal")]
    pub kswapd_inodesteal: usize,
    #[cfg(feature = "has_vmstat_pageoutrun")]
    pub pageoutrun: usize,
    #[cfg(feature = "has_vmstat_allocstall")]
    pub allocstall: usize,
    //
    #[cfg(feature = "has_vmstat_pgrotated")]
    pub pgrotated: usize,
    //}}}
    //{{{ >= linux v2.6.26 && CONFIG_VM_EVENT_COUNTERS && CONFIG_HUGETLB_PAGE
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    pub htlb_buddy_alloc_success: usize,
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    pub htlb_buddy_alloc_fail: usize,
    //}}}
}
/*
 * _movable >= linux v2.6.23
*/
