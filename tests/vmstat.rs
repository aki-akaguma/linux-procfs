use linux_procfs::System;

macro_rules! base_path_intel {
    () => {
        "fixtures/linux-4.4.0-cpu-intel"
    };
}

macro_rules! base_path_amd {
    () => {
        "fixtures/linux-4.4.0-cpu-amd"
    };
}

macro_rules! base_path_5_4 {
    () => {
        "fixtures/linux-5.4.0-vcpu"
    };
}

macro_rules! base_path_5_4_intel {
    () => {
        "fixtures/linux-5.4.0-cpu-intel"
    };
}

#[test]
fn test_vmstat_intel() {
    let mut sys = System::new(base_path_intel!());
    let vmstat = sys.get_vmstat();
    //
    #[cfg(feature = "has_vmstat_nr_free_pages")]
    assert_eq!(vmstat.nr_free_pages, 78396);
    #[cfg(feature = "has_vmstat_nr_active")]
    assert_eq!(vmstat.nr_active, 0);
    #[cfg(feature = "has_vmstat_nr_inactive")]
    assert_eq!(vmstat.nr_inactive, 0);
    //
    #[cfg(feature = "has_vmstat_nr_anon_pages")]
    assert_eq!(vmstat.nr_anon_pages, 1067815);
    #[cfg(feature = "has_vmstat_nr_mapped")]
    assert_eq!(vmstat.nr_mapped, 147609);
    #[cfg(feature = "has_vmstat_nr_file_pages")]
    assert_eq!(vmstat.nr_file_pages, 562927);
    //
    #[cfg(feature = "has_vmstat_nr_slab")]
    assert_eq!(vmstat.nr_slab, 0);
    #[cfg(feature = "has_vmstat_nr_slab_reclaimable")]
    assert_eq!(vmstat.nr_slab_reclaimable, 195022);
    #[cfg(feature = "has_vmstat_nr_slab_unreclaimable")]
    assert_eq!(vmstat.nr_slab_unreclaimable, 19708);
    //
    #[cfg(feature = "has_vmstat_nr_page_table_pages")]
    assert_eq!(vmstat.nr_page_table_pages, 21037);
    #[cfg(feature = "has_vmstat_nr_dirty")]
    assert_eq!(vmstat.nr_dirty, 402);
    #[cfg(feature = "has_vmstat_nr_writeback")]
    assert_eq!(vmstat.nr_writeback, 0);
    #[cfg(feature = "has_vmstat_nr_unstable")]
    assert_eq!(vmstat.nr_unstable, 0);
    #[cfg(feature = "has_vmstat_nr_bounce")]
    assert_eq!(vmstat.nr_bounce, 0);
    //
    #[cfg(feature = "has_vmstat_nr_vmscan_write")]
    assert_eq!(vmstat.nr_vmscan_write, 155527);
    //
    #[cfg(feature = "has_vmstat_nr_writeback_temp")]
    assert_eq!(vmstat.nr_writeback_temp, 0);
    //
    #[cfg(feature = "has_vmstat_numa_hit")]
    assert_eq!(vmstat.numa_hit, 682866235);
    #[cfg(feature = "has_vmstat_numa_miss")]
    assert_eq!(vmstat.numa_miss, 0);
    #[cfg(feature = "has_vmstat_numa_foreign")]
    assert_eq!(vmstat.numa_foreign, 0);
    #[cfg(feature = "has_vmstat_numa_interleave")]
    assert_eq!(vmstat.numa_interleave, 20170);
    #[cfg(feature = "has_vmstat_numa_local")]
    assert_eq!(vmstat.numa_local, 682866235);
    #[cfg(feature = "has_vmstat_numa_other")]
    assert_eq!(vmstat.numa_other, 0);
    //
    assert_eq!(vmstat.pgpgin, 24297991);
    assert_eq!(vmstat.pgpgout, 75849328);
    assert_eq!(vmstat.pswpin, 3995);
    assert_eq!(vmstat.pswpout, 4826);
    //
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_dma, 10);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_dma32, 409664394);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_normal, 601399918);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_high, 0);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgfree")]
    assert_eq!(vmstat.pgfree, 1011152442);
    #[cfg(feature = "has_vmstat_pgactivate")]
    assert_eq!(vmstat.pgactivate, 18910152);
    #[cfg(feature = "has_vmstat_pgdeactivate")]
    assert_eq!(vmstat.pgdeactivate, 6516433);
    //
    #[cfg(feature = "has_vmstat_pgfault")]
    assert_eq!(vmstat.pgfault, 262137501);
    #[cfg(feature = "has_vmstat_pgmajfault")]
    assert_eq!(vmstat.pgmajfault, 100617);
    //
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_dma, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_dma32, 3378964);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_normal, 4877917);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_high, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_dma, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_dma32, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_normal, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_high, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_dma, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_dma32, 3720965);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_normal, 5681445);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_high, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_dma, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_dma32, 1145);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_normal, 1461);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_high, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pginodesteal")]
    assert_eq!(vmstat.pginodesteal, 1);
    #[cfg(feature = "has_vmstat_slabs_scanned")]
    assert_eq!(vmstat.slabs_scanned, 9621288);
    #[cfg(feature = "has_vmstat_kswapd_steal")]
    assert_eq!(vmstat.kswapd_steal, 0);
    #[cfg(feature = "has_vmstat_kswapd_inodesteal")]
    assert_eq!(vmstat.kswapd_inodesteal, 12274);
    #[cfg(feature = "has_vmstat_pageoutrun")]
    assert_eq!(vmstat.pageoutrun, 33944);
    #[cfg(feature = "has_vmstat_allocstall")]
    assert_eq!(vmstat.allocstall, 49);
    //
    #[cfg(feature = "has_vmstat_pgrotated")]
    assert_eq!(vmstat.pgrotated, 9456);
    //
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    assert_eq!(vmstat.htlb_buddy_alloc_success, 0);
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    assert_eq!(vmstat.htlb_buddy_alloc_fail, 0);
}

#[test]
fn test_vmstat_amd() {
    let mut sys = System::new(base_path_amd!());
    let vmstat = sys.get_vmstat();
    //
    #[cfg(feature = "has_vmstat_nr_free_pages")]
    assert_eq!(vmstat.nr_free_pages, 44248);
    #[cfg(feature = "has_vmstat_nr_active")]
    assert_eq!(vmstat.nr_active, 0);
    #[cfg(feature = "has_vmstat_nr_inactive")]
    assert_eq!(vmstat.nr_inactive, 0);
    //
    #[cfg(feature = "has_vmstat_nr_anon_pages")]
    assert_eq!(vmstat.nr_anon_pages, 1345800);
    #[cfg(feature = "has_vmstat_nr_mapped")]
    assert_eq!(vmstat.nr_mapped, 7057);
    #[cfg(feature = "has_vmstat_nr_file_pages")]
    assert_eq!(vmstat.nr_file_pages, 460914);
    //
    #[cfg(feature = "has_vmstat_nr_slab")]
    assert_eq!(vmstat.nr_slab, 0);
    #[cfg(feature = "has_vmstat_nr_slab_reclaimable")]
    assert_eq!(vmstat.nr_slab_reclaimable, 96316);
    #[cfg(feature = "has_vmstat_nr_slab_unreclaimable")]
    assert_eq!(vmstat.nr_slab_unreclaimable, 37009);
    //
    #[cfg(feature = "has_vmstat_nr_page_table_pages")]
    assert_eq!(vmstat.nr_page_table_pages, 6725);
    #[cfg(feature = "has_vmstat_nr_dirty")]
    assert_eq!(vmstat.nr_dirty, 27);
    #[cfg(feature = "has_vmstat_nr_writeback")]
    assert_eq!(vmstat.nr_writeback, 0);
    #[cfg(feature = "has_vmstat_nr_unstable")]
    assert_eq!(vmstat.nr_unstable, 0);
    #[cfg(feature = "has_vmstat_nr_bounce")]
    assert_eq!(vmstat.nr_bounce, 0);
    //
    #[cfg(feature = "has_vmstat_nr_vmscan_write")]
    assert_eq!(vmstat.nr_vmscan_write, 55634);
    //
    #[cfg(feature = "has_vmstat_nr_writeback_temp")]
    assert_eq!(vmstat.nr_writeback_temp, 0);
    //
    #[cfg(feature = "has_vmstat_numa_hit")]
    assert_eq!(vmstat.numa_hit, 37156043);
    #[cfg(feature = "has_vmstat_numa_miss")]
    assert_eq!(vmstat.numa_miss, 0);
    #[cfg(feature = "has_vmstat_numa_foreign")]
    assert_eq!(vmstat.numa_foreign, 0);
    #[cfg(feature = "has_vmstat_numa_interleave")]
    assert_eq!(vmstat.numa_interleave, 20999);
    #[cfg(feature = "has_vmstat_numa_local")]
    assert_eq!(vmstat.numa_local, 37156043);
    #[cfg(feature = "has_vmstat_numa_other")]
    assert_eq!(vmstat.numa_other, 0);
    //
    assert_eq!(vmstat.pgpgin, 25793399);
    assert_eq!(vmstat.pgpgout, 28384245);
    assert_eq!(vmstat.pswpin, 123);
    assert_eq!(vmstat.pswpout, 934);
    //
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_dma, 2);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_dma32, 13520953);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_normal, 26989638);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_high, 0);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgfree")]
    assert_eq!(vmstat.pgfree, 41200695);
    #[cfg(feature = "has_vmstat_pgactivate")]
    assert_eq!(vmstat.pgactivate, 4443132);
    #[cfg(feature = "has_vmstat_pgdeactivate")]
    assert_eq!(vmstat.pgdeactivate, 6493081);
    //
    #[cfg(feature = "has_vmstat_pgfault")]
    assert_eq!(vmstat.pgfault, 9188879);
    #[cfg(feature = "has_vmstat_pgmajfault")]
    assert_eq!(vmstat.pgmajfault, 12282);
    //
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_dma, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_dma32, 2009291);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_normal, 3675370);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_high, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_dma, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_dma32, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_normal, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_high, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_dma, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_dma32, 3350246);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_normal, 5933793);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_high, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_dma, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_dma32, 49044);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_normal, 344425);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_high, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pginodesteal")]
    assert_eq!(vmstat.pginodesteal, 172);
    #[cfg(feature = "has_vmstat_slabs_scanned")]
    assert_eq!(vmstat.slabs_scanned, 1412480);
    #[cfg(feature = "has_vmstat_kswapd_steal")]
    assert_eq!(vmstat.kswapd_steal, 0);
    #[cfg(feature = "has_vmstat_kswapd_inodesteal")]
    assert_eq!(vmstat.kswapd_inodesteal, 2264);
    #[cfg(feature = "has_vmstat_pageoutrun")]
    assert_eq!(vmstat.pageoutrun, 52230);
    #[cfg(feature = "has_vmstat_allocstall")]
    assert_eq!(vmstat.allocstall, 647);
    //
    #[cfg(feature = "has_vmstat_pgrotated")]
    assert_eq!(vmstat.pgrotated, 2533);
    //
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    assert_eq!(vmstat.htlb_buddy_alloc_success, 0);
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    assert_eq!(vmstat.htlb_buddy_alloc_fail, 0);
}

#[test]
fn test_vmstat_5_4() {
    let mut sys = System::new(base_path_5_4!());
    let vmstat = sys.get_vmstat();
    //
    #[cfg(feature = "has_vmstat_nr_free_pages")]
    assert_eq!(vmstat.nr_free_pages, 52441);
    #[cfg(feature = "has_vmstat_nr_active")]
    assert_eq!(vmstat.nr_active, 0);
    #[cfg(feature = "has_vmstat_nr_inactive")]
    assert_eq!(vmstat.nr_inactive, 0);
    //
    #[cfg(feature = "has_vmstat_nr_anon_pages")]
    assert_eq!(vmstat.nr_anon_pages, 39881);
    #[cfg(feature = "has_vmstat_nr_mapped")]
    assert_eq!(vmstat.nr_mapped, 37955);
    #[cfg(feature = "has_vmstat_nr_file_pages")]
    assert_eq!(vmstat.nr_file_pages, 118781);
    //
    #[cfg(feature = "has_vmstat_nr_slab")]
    assert_eq!(vmstat.nr_slab, 0);
    #[cfg(feature = "has_vmstat_nr_slab_reclaimable")]
    assert_eq!(vmstat.nr_slab_reclaimable, 0);
    #[cfg(feature = "has_vmstat_nr_slab_unreclaimable")]
    assert_eq!(vmstat.nr_slab_unreclaimable, 0);
    //
    #[cfg(feature = "has_vmstat_nr_page_table_pages")]
    assert_eq!(vmstat.nr_page_table_pages, 0);
    #[cfg(feature = "has_vmstat_nr_dirty")]
    assert_eq!(vmstat.nr_dirty, 610);
    #[cfg(feature = "has_vmstat_nr_writeback")]
    assert_eq!(vmstat.nr_writeback, 0);
    #[cfg(feature = "has_vmstat_nr_unstable")]
    assert_eq!(vmstat.nr_unstable, 0);
    #[cfg(feature = "has_vmstat_nr_bounce")]
    assert_eq!(vmstat.nr_bounce, 0);
    //
    #[cfg(feature = "has_vmstat_nr_vmscan_write")]
    assert_eq!(vmstat.nr_vmscan_write, 1846);
    //
    #[cfg(feature = "has_vmstat_nr_writeback_temp")]
    assert_eq!(vmstat.nr_writeback_temp, 0);
    //
    #[cfg(feature = "has_vmstat_numa_hit")]
    assert_eq!(vmstat.numa_hit, 0);
    #[cfg(feature = "has_vmstat_numa_miss")]
    assert_eq!(vmstat.numa_miss, 0);
    #[cfg(feature = "has_vmstat_numa_foreign")]
    assert_eq!(vmstat.numa_foreign, 0);
    #[cfg(feature = "has_vmstat_numa_interleave")]
    assert_eq!(vmstat.numa_interleave, 0);
    #[cfg(feature = "has_vmstat_numa_local")]
    assert_eq!(vmstat.numa_local, 0);
    #[cfg(feature = "has_vmstat_numa_other")]
    assert_eq!(vmstat.numa_other, 0);
    //
    assert_eq!(vmstat.pgpgin, 923059);
    assert_eq!(vmstat.pgpgout, 832504);
    assert_eq!(vmstat.pswpin, 89);
    assert_eq!(vmstat.pswpout, 1846);
    //
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_dma, 30523);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_dma32, 4157878);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_normal, 0);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_high, 0);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgfree")]
    assert_eq!(vmstat.pgfree, 4241284);
    #[cfg(feature = "has_vmstat_pgactivate")]
    assert_eq!(vmstat.pgactivate, 231452);
    #[cfg(feature = "has_vmstat_pgdeactivate")]
    assert_eq!(vmstat.pgdeactivate, 151249);
    //
    #[cfg(feature = "has_vmstat_pgfault")]
    assert_eq!(vmstat.pgfault, 2321977);
    #[cfg(feature = "has_vmstat_pgmajfault")]
    assert_eq!(vmstat.pgmajfault, 4189);
    //
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_dma, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_dma32, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_normal, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_high, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_dma, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_dma32, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_normal, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_high, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_dma, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_dma32, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_normal, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_high, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_dma, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_dma32, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_normal, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_high, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pginodesteal")]
    assert_eq!(vmstat.pginodesteal, 0);
    #[cfg(feature = "has_vmstat_slabs_scanned")]
    assert_eq!(vmstat.slabs_scanned, 44317);
    #[cfg(feature = "has_vmstat_kswapd_steal")]
    assert_eq!(vmstat.kswapd_steal, 0);
    #[cfg(feature = "has_vmstat_kswapd_inodesteal")]
    assert_eq!(vmstat.kswapd_inodesteal, 542);
    #[cfg(feature = "has_vmstat_pageoutrun")]
    assert_eq!(vmstat.pageoutrun, 64);
    #[cfg(feature = "has_vmstat_allocstall")]
    assert_eq!(vmstat.allocstall, 0);
    //
    #[cfg(feature = "has_vmstat_pgrotated")]
    assert_eq!(vmstat.pgrotated, 1851);
    //
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    assert_eq!(vmstat.htlb_buddy_alloc_success, 0);
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    assert_eq!(vmstat.htlb_buddy_alloc_fail, 0);
}

#[test]
fn test_vmstat_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    let vmstat = sys.get_vmstat();
    //
    #[cfg(feature = "has_vmstat_nr_free_pages")]
    assert_eq!(vmstat.nr_free_pages, 457563);
    #[cfg(feature = "has_vmstat_nr_active")]
    assert_eq!(vmstat.nr_active, 0);
    #[cfg(feature = "has_vmstat_nr_inactive")]
    assert_eq!(vmstat.nr_inactive, 0);
    //
    #[cfg(feature = "has_vmstat_nr_anon_pages")]
    assert_eq!(vmstat.nr_anon_pages, 815551);
    #[cfg(feature = "has_vmstat_nr_mapped")]
    assert_eq!(vmstat.nr_mapped, 166697);
    #[cfg(feature = "has_vmstat_nr_file_pages")]
    assert_eq!(vmstat.nr_file_pages, 692737);
    //
    #[cfg(feature = "has_vmstat_nr_slab")]
    assert_eq!(vmstat.nr_slab, 0);
    #[cfg(feature = "has_vmstat_nr_slab_reclaimable")]
    assert_eq!(vmstat.nr_slab_reclaimable, 0);
    #[cfg(feature = "has_vmstat_nr_slab_unreclaimable")]
    assert_eq!(vmstat.nr_slab_unreclaimable, 0);
    //
    #[cfg(feature = "has_vmstat_nr_page_table_pages")]
    assert_eq!(vmstat.nr_page_table_pages, 0);
    #[cfg(feature = "has_vmstat_nr_dirty")]
    assert_eq!(vmstat.nr_dirty, 3860);
    #[cfg(feature = "has_vmstat_nr_writeback")]
    assert_eq!(vmstat.nr_writeback, 0);
    #[cfg(feature = "has_vmstat_nr_unstable")]
    assert_eq!(vmstat.nr_unstable, 0);
    #[cfg(feature = "has_vmstat_nr_bounce")]
    assert_eq!(vmstat.nr_bounce, 0);
    //
    #[cfg(feature = "has_vmstat_nr_vmscan_write")]
    assert_eq!(vmstat.nr_vmscan_write, 1081114);
    //
    #[cfg(feature = "has_vmstat_nr_writeback_temp")]
    assert_eq!(vmstat.nr_writeback_temp, 0);
    //
    #[cfg(feature = "has_vmstat_numa_hit")]
    assert_eq!(vmstat.numa_hit, 0);
    #[cfg(feature = "has_vmstat_numa_miss")]
    assert_eq!(vmstat.numa_miss, 0);
    #[cfg(feature = "has_vmstat_numa_foreign")]
    assert_eq!(vmstat.numa_foreign, 0);
    #[cfg(feature = "has_vmstat_numa_interleave")]
    assert_eq!(vmstat.numa_interleave, 0);
    #[cfg(feature = "has_vmstat_numa_local")]
    assert_eq!(vmstat.numa_local, 0);
    #[cfg(feature = "has_vmstat_numa_other")]
    assert_eq!(vmstat.numa_other, 0);
    //
    assert_eq!(vmstat.pgpgin, 147334845);
    assert_eq!(vmstat.pgpgout, 82164268);
    assert_eq!(vmstat.pswpin, 759176);
    assert_eq!(vmstat.pswpout, 1081114);
    //
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_dma, 10);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_dma32, 206740683);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_normal, 496372556);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_high, 0);
    #[cfg(feature = "has_vmstat_pgalloc")]
    assert_eq!(vmstat.pgalloc_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgfree")]
    assert_eq!(vmstat.pgfree, 703847060);
    #[cfg(feature = "has_vmstat_pgactivate")]
    assert_eq!(vmstat.pgactivate, 23293905);
    #[cfg(feature = "has_vmstat_pgdeactivate")]
    assert_eq!(vmstat.pgdeactivate, 19312525);
    //
    #[cfg(feature = "has_vmstat_pgfault")]
    assert_eq!(vmstat.pgfault, 276009539);
    #[cfg(feature = "has_vmstat_pgmajfault")]
    assert_eq!(vmstat.pgmajfault, 360973);
    //
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_dma, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_dma32, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_normal, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_high, 0);
    #[cfg(feature = "has_vmstat_pgrefill")]
    assert_eq!(vmstat.pgrefill_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_dma, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_dma32, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_normal, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_high, 0);
    #[cfg(feature = "has_vmstat_pgsteal")]
    assert_eq!(vmstat.pgsteal_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_dma, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_dma32, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_normal, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_high, 0);
    #[cfg(feature = "has_vmstat_pgscan_kswapd")]
    assert_eq!(vmstat.pgscan_kswapd_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_dma, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_dma32, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_normal, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_high, 0);
    #[cfg(feature = "has_vmstat_pgscan_direct")]
    assert_eq!(vmstat.pgscan_direct_movable, 0);
    //
    #[cfg(feature = "has_vmstat_pginodesteal")]
    assert_eq!(vmstat.pginodesteal, 323);
    #[cfg(feature = "has_vmstat_slabs_scanned")]
    assert_eq!(vmstat.slabs_scanned, 136214679);
    #[cfg(feature = "has_vmstat_kswapd_steal")]
    assert_eq!(vmstat.kswapd_steal, 0);
    #[cfg(feature = "has_vmstat_kswapd_inodesteal")]
    assert_eq!(vmstat.kswapd_inodesteal, 6309368);
    #[cfg(feature = "has_vmstat_pageoutrun")]
    assert_eq!(vmstat.pageoutrun, 13298);
    #[cfg(feature = "has_vmstat_allocstall")]
    assert_eq!(vmstat.allocstall, 0);
    //
    #[cfg(feature = "has_vmstat_pgrotated")]
    assert_eq!(vmstat.pgrotated, 1082888);
    //
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    assert_eq!(vmstat.htlb_buddy_alloc_success, 0);
    #[cfg(feature = "has_vmstat_htlb_buddy_alloc")]
    assert_eq!(vmstat.htlb_buddy_alloc_fail, 0);
}
