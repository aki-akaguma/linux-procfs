#[derive(Debug, Default, Clone)]
pub struct MemInfo {
    //
    pub mem_total_kb: usize,
    //
    pub mem_free_kb: usize,
    //
    //{{{ >= linux v3.14
    //
    #[cfg(feature = "has_meminfo_mem_available")]
    pub mem_available_kb: usize,
    //}}}
    //
    pub buffers_kb: usize,
    //
    pub cached_kb: usize,
    //
    pub swap_cached_kb: usize,
    //
    pub active_kb: usize,
    //
    pub inactive_kb: usize,
    //
    //{{{ >= linux v2.6.28
    //
    #[cfg(feature = "has_meminfo_active_anon")]
    pub active_anon_kb: usize,
    //
    #[cfg(feature = "has_meminfo_inactive_anon")]
    pub inactive_anon_kb: usize,
    //
    #[cfg(feature = "has_meminfo_active_file")]
    pub active_file_kb: usize,
    //
    #[cfg(feature = "has_meminfo_inactive_file")]
    pub inactive_file_kb: usize,
    //}}}
    //
    //{{{ >= linux v2.6.28 && CONFIG_UNEVICTABLE_LRU
    //
    #[cfg(feature = "has_meminfo_unevictable")]
    pub unevictable_kb: usize,
    //
    #[cfg(feature = "has_meminfo_mlocked")]
    pub mlocked_kb: usize,
    //}}}
    //
    //{{{ >= linux v2.6.18 && CONFIG_HIGHMEM
    //
    #[cfg(feature = "has_meminfo_high_total")]
    pub high_total_kb: usize,
    //
    #[cfg(feature = "has_meminfo_high_free")]
    pub high_free_kb: usize,
    //
    #[cfg(feature = "has_meminfo_low_total")]
    pub low_total_kb: usize,
    //
    #[cfg(feature = "has_meminfo_low_free")]
    pub low_free_kb: usize,
    //}}}
    //{{{ >= linux v2.6.29 && CONFIG_MMU
    //
    #[cfg(feature = "has_meminfo_mmap_copy")]
    pub mmap_copy_kb: usize,
    //}}}
    //
    pub swap_total_kb: usize,
    //
    pub swap_free_kb: usize,
    //
    #[cfg(feature = "has_meminfo_dirty")]
    pub dirty_kb: usize,
    #[cfg(feature = "has_meminfo_writeback")]
    pub writeback_kb: usize,
    #[cfg(feature = "has_meminfo_anon_pages")]
    pub anon_pages_kb: usize,
    #[cfg(feature = "has_meminfo_mapped")]
    pub mapped_kb: usize,
    #[cfg(feature = "has_meminfo_slab")]
    pub slab_kb: usize,
    #[cfg(feature = "has_meminfo_slab_reclaimable")]
    pub slab_reclaimable_kb: usize,
    #[cfg(feature = "has_meminfo_slab_unreclaim")]
    pub slab_unreclaim_kb: usize,
    #[cfg(feature = "has_meminfo_page_tables")]
    pub page_tables_kb: usize,
    #[cfg(feature = "has_meminfo_nfs_unstable")]
    pub nfs_unstable_kb: usize,
    #[cfg(feature = "has_meminfo_bounce")]
    pub bounce_kb: usize,
    #[cfg(feature = "has_meminfo_commit_limit")]
    pub commit_limit_kb: usize,
    #[cfg(feature = "has_meminfo_commit_as")]
    pub commited_as_kb: usize,
    #[cfg(feature = "has_meminfo_vmalloc_total")]
    pub vmalloc_total_kb: usize,
    #[cfg(feature = "has_meminfo_vmalloc_used")]
    pub vmalloc_used_kb: usize,
    #[cfg(feature = "has_meminfo_vmalloc_chunk")]
    pub vmalloc_chunk_kb: usize,
}
//
/*
linux v2.6.18
//
Dirty:              1608 kB
Writeback:             0 kB
AnonPages:       4271260 kB
Mapped:           590436 kB
Shmem:            118376 kB
Slab:             858920 kB
SReclaimable:     780088 kB
SUnreclaim:        78832 kB
KernelStack:       17392 kB
PageTables:        84148 kB
NFS_Unstable:          0 kB
Bounce:                0 kB
WritebackTmp:          0 kB
CommitLimit:    37641712 kB
Committed_AS:   12620632 kB
VmallocTotal:   34359738367 kB
VmallocUsed:           0 kB
VmallocChunk:          0 kB
HardwareCorrupted:     0 kB
AnonHugePages:         0 kB
CmaTotal:              0 kB
CmaFree:               0 kB
HugePages_Total:       0
HugePages_Free:        0
HugePages_Rsvd:        0
HugePages_Surp:        0
Hugepagesize:       2048 kB
DirectMap4k:      992960 kB
DirectMap2M:     7395328 kB
*/
//
