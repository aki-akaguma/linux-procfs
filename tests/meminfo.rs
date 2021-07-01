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
fn test_meminfo_intel() {
    let mut sys = System::new(base_path_intel!());
    let meminfo = sys.get_meminfo();
    //
    assert_eq!(meminfo.mem_total_kb, 8174572);
    assert_eq!(meminfo.mem_free_kb, 313584);
    //
    #[cfg(feature = "has_meminfo_mem_available")]
    assert_eq!(meminfo.mem_available_kb, 2866392);
    //
    assert_eq!(meminfo.buffers_kb, 636260);
    assert_eq!(meminfo.cached_kb, 1577352);
    assert_eq!(meminfo.swap_cached_kb, 38096);
    assert_eq!(meminfo.active_kb, 4905288);
    assert_eq!(meminfo.inactive_kb, 1585140);
    //
    #[cfg(feature = "has_meminfo_active_anon")]
    assert_eq!(meminfo.active_anon_kb, 3761252);
    #[cfg(feature = "has_meminfo_inactive_anon")]
    assert_eq!(meminfo.inactive_anon_kb, 651500);
    #[cfg(feature = "has_meminfo_active_file")]
    assert_eq!(meminfo.active_file_kb, 1144036);
    #[cfg(feature = "has_meminfo_inactive_file")]
    assert_eq!(meminfo.inactive_file_kb, 933640);
    //
    #[cfg(feature = "has_meminfo_unevictable")]
    assert_eq!(meminfo.unevictable_kb, 17576);
    #[cfg(feature = "has_meminfo_mlocked")]
    assert_eq!(meminfo.mlocked_kb, 17576);
    //
    #[cfg(feature = "has_meminfo_high_total")]
    assert_eq!(meminfo.high_total_kb, 0);
    #[cfg(feature = "has_meminfo_low_free")]
    assert_eq!(meminfo.high_free_kb, 0);
    #[cfg(feature = "has_meminfo_low_total")]
    assert_eq!(meminfo.low_total_kb, 0);
    #[cfg(feature = "has_meminfo_low_free")]
    assert_eq!(meminfo.low_free_kb, 0);
    //
    #[cfg(feature = "has_meminfo_mmap_copy")]
    assert_eq!(meminfo.mmap_copy_kb, 0);
    //
    assert_eq!(meminfo.swap_total_kb, 33554428);
    assert_eq!(meminfo.swap_free_kb, 33086876);
    //
    #[cfg(feature = "has_meminfo_dirty")]
    assert_eq!(meminfo.dirty_kb, 1608);
    #[cfg(feature = "has_meminfo_writeback")]
    assert_eq!(meminfo.writeback_kb, 0);
    #[cfg(feature = "has_meminfo_anon_pages")]
    assert_eq!(meminfo.anon_pages_kb, 4271260);
    #[cfg(feature = "has_meminfo_mapped")]
    assert_eq!(meminfo.mapped_kb, 590436);
    #[cfg(feature = "has_meminfo_slab")]
    assert_eq!(meminfo.slab_kb, 858920);
    #[cfg(feature = "has_meminfo_slab_reclaimable")]
    assert_eq!(meminfo.slab_reclaimable_kb, 780088);
    #[cfg(feature = "has_meminfo_slab_unreclaim")]
    assert_eq!(meminfo.slab_unreclaim_kb, 78832);
    #[cfg(feature = "has_meminfo_page_tables")]
    assert_eq!(meminfo.page_tables_kb, 84148);
    #[cfg(feature = "has_meminfo_nfs_unstable")]
    assert_eq!(meminfo.nfs_unstable_kb, 0);
    #[cfg(feature = "has_meminfo_bounce")]
    assert_eq!(meminfo.bounce_kb, 0);
    #[cfg(feature = "has_meminfo_commit_limit")]
    assert_eq!(meminfo.commit_limit_kb, 37641712);
    #[cfg(feature = "has_meminfo_commit_as")]
    assert_eq!(meminfo.commited_as_kb, 12620632);
    #[cfg(feature = "has_meminfo_vmalloc_total")]
    assert_eq!(meminfo.vmalloc_total_kb, 34359738367);
    #[cfg(feature = "has_meminfo_vmalloc_used")]
    assert_eq!(meminfo.vmalloc_used_kb, 0);
    #[cfg(feature = "has_meminfo_vmalloc_chunk")]
    assert_eq!(meminfo.vmalloc_chunk_kb, 0);
}

#[test]
fn test_meminfo_amd() {
    let mut sys = System::new(base_path_amd!());
    let meminfo = sys.get_meminfo();
    //
    assert_eq!(meminfo.mem_total_kb, 8174996);
    assert_eq!(meminfo.mem_free_kb, 176992);
    //
    #[cfg(feature = "has_meminfo_mem_available")]
    assert_eq!(meminfo.mem_available_kb, 2083860);
    //
    assert_eq!(meminfo.buffers_kb, 352576);
    assert_eq!(meminfo.cached_kb, 1482064);
    assert_eq!(meminfo.swap_cached_kb, 9016);
    assert_eq!(meminfo.active_kb, 4047548);
    assert_eq!(meminfo.inactive_kb, 3171400);
    //
    #[cfg(feature = "has_meminfo_active_anon")]
    assert_eq!(meminfo.active_anon_kb, 3103596);
    #[cfg(feature = "has_meminfo_inactive_anon")]
    assert_eq!(meminfo.inactive_anon_kb, 2286744);
    #[cfg(feature = "has_meminfo_active_file")]
    assert_eq!(meminfo.active_file_kb, 943952);
    #[cfg(feature = "has_meminfo_inactive_file")]
    assert_eq!(meminfo.inactive_file_kb, 884656);
    //
    #[cfg(feature = "has_meminfo_unevictable")]
    assert_eq!(meminfo.unevictable_kb, 0);
    #[cfg(feature = "has_meminfo_mlocked")]
    assert_eq!(meminfo.mlocked_kb, 0);
    //
    #[cfg(feature = "has_meminfo_high_total")]
    assert_eq!(meminfo.high_total_kb, 0);
    #[cfg(feature = "has_meminfo_low_free")]
    assert_eq!(meminfo.high_free_kb, 0);
    #[cfg(feature = "has_meminfo_low_total")]
    assert_eq!(meminfo.low_total_kb, 0);
    #[cfg(feature = "has_meminfo_low_free")]
    assert_eq!(meminfo.low_free_kb, 0);
    //
    #[cfg(feature = "has_meminfo_mmap_copy")]
    assert_eq!(meminfo.mmap_copy_kb, 0);
    //
    assert_eq!(meminfo.swap_total_kb, 20687548);
    assert_eq!(meminfo.swap_free_kb, 20486800);
    //
    #[cfg(feature = "has_meminfo_dirty")]
    assert_eq!(meminfo.dirty_kb, 108);
    #[cfg(feature = "has_meminfo_writeback")]
    assert_eq!(meminfo.writeback_kb, 0);
    #[cfg(feature = "has_meminfo_anon_pages")]
    assert_eq!(meminfo.anon_pages_kb, 5383200);
    #[cfg(feature = "has_meminfo_mapped")]
    assert_eq!(meminfo.mapped_kb, 28228);
    #[cfg(feature = "has_meminfo_slab")]
    assert_eq!(meminfo.slab_kb, 533300);
    #[cfg(feature = "has_meminfo_slab_reclaimable")]
    assert_eq!(meminfo.slab_reclaimable_kb, 385264);
    #[cfg(feature = "has_meminfo_slab_unreclaim")]
    assert_eq!(meminfo.slab_unreclaim_kb, 148036);
    #[cfg(feature = "has_meminfo_page_tables")]
    assert_eq!(meminfo.page_tables_kb, 26900);
    #[cfg(feature = "has_meminfo_nfs_unstable")]
    assert_eq!(meminfo.nfs_unstable_kb, 0);
    #[cfg(feature = "has_meminfo_bounce")]
    assert_eq!(meminfo.bounce_kb, 0);
    #[cfg(feature = "has_meminfo_commit_limit")]
    assert_eq!(meminfo.commit_limit_kb, 24775044);
    #[cfg(feature = "has_meminfo_commit_as")]
    assert_eq!(meminfo.commited_as_kb, 9335696);
    #[cfg(feature = "has_meminfo_vmalloc_total")]
    assert_eq!(meminfo.vmalloc_total_kb, 34359738367);
    #[cfg(feature = "has_meminfo_vmalloc_used")]
    assert_eq!(meminfo.vmalloc_used_kb, 0);
    #[cfg(feature = "has_meminfo_vmalloc_chunk")]
    assert_eq!(meminfo.vmalloc_chunk_kb, 0);
}

#[test]
fn test_meminfo_5_4() {
    let mut sys = System::new(base_path_5_4!());
    let meminfo = sys.get_meminfo();
    //
    assert_eq!(meminfo.mem_total_kb, 1004672);
    assert_eq!(meminfo.mem_free_kb, 209496);
    //
    #[cfg(feature = "has_meminfo_mem_available")]
    assert_eq!(meminfo.mem_available_kb, 569296);
    //
    assert_eq!(meminfo.buffers_kb, 40496);
    assert_eq!(meminfo.cached_kb, 433976);
    assert_eq!(meminfo.swap_cached_kb, 552);
    assert_eq!(meminfo.active_kb, 361240);
    assert_eq!(meminfo.inactive_kb, 254728);
    //
    #[cfg(feature = "has_meminfo_active_anon")]
    assert_eq!(meminfo.active_anon_kb, 72532);
    #[cfg(feature = "has_meminfo_inactive_anon")]
    assert_eq!(meminfo.inactive_anon_kb, 79992);
    #[cfg(feature = "has_meminfo_active_file")]
    assert_eq!(meminfo.active_file_kb, 288708);
    #[cfg(feature = "has_meminfo_inactive_file")]
    assert_eq!(meminfo.inactive_file_kb, 174736);
    //
    #[cfg(feature = "has_meminfo_unevictable")]
    assert_eq!(meminfo.unevictable_kb, 18516);
    #[cfg(feature = "has_meminfo_mlocked")]
    assert_eq!(meminfo.mlocked_kb, 18516);
    //
    #[cfg(feature = "has_meminfo_high_total")]
    assert_eq!(meminfo.high_total_kb, 0);
    #[cfg(feature = "has_meminfo_low_free")]
    assert_eq!(meminfo.high_free_kb, 0);
    #[cfg(feature = "has_meminfo_low_total")]
    assert_eq!(meminfo.low_total_kb, 0);
    #[cfg(feature = "has_meminfo_low_free")]
    assert_eq!(meminfo.low_free_kb, 0);
    //
    #[cfg(feature = "has_meminfo_mmap_copy")]
    assert_eq!(meminfo.mmap_copy_kb, 0);
    //
    assert_eq!(meminfo.swap_total_kb, 2097148);
    assert_eq!(meminfo.swap_free_kb, 2089712);
    //
    #[cfg(feature = "has_meminfo_dirty")]
    assert_eq!(meminfo.dirty_kb, 2372);
    #[cfg(feature = "has_meminfo_writeback")]
    assert_eq!(meminfo.writeback_kb, 0);
    #[cfg(feature = "has_meminfo_anon_pages")]
    assert_eq!(meminfo.anon_pages_kb, 159548);
    #[cfg(feature = "has_meminfo_mapped")]
    assert_eq!(meminfo.mapped_kb, 151820);
    #[cfg(feature = "has_meminfo_slab")]
    assert_eq!(meminfo.slab_kb, 124068);
    #[cfg(feature = "has_meminfo_slab_reclaimable")]
    assert_eq!(meminfo.slab_reclaimable_kb, 47516);
    #[cfg(feature = "has_meminfo_slab_unreclaim")]
    assert_eq!(meminfo.slab_unreclaim_kb, 76552);
    #[cfg(feature = "has_meminfo_page_tables")]
    assert_eq!(meminfo.page_tables_kb, 3616);
    #[cfg(feature = "has_meminfo_nfs_unstable")]
    assert_eq!(meminfo.nfs_unstable_kb, 0);
    #[cfg(feature = "has_meminfo_bounce")]
    assert_eq!(meminfo.bounce_kb, 0);
    #[cfg(feature = "has_meminfo_commit_limit")]
    assert_eq!(meminfo.commit_limit_kb, 2599484);
    #[cfg(feature = "has_meminfo_commit_as")]
    assert_eq!(meminfo.commited_as_kb, 710160);
    #[cfg(feature = "has_meminfo_vmalloc_total")]
    assert_eq!(meminfo.vmalloc_total_kb, 34359738367);
    #[cfg(feature = "has_meminfo_vmalloc_used")]
    assert_eq!(meminfo.vmalloc_used_kb, 14368);
    #[cfg(feature = "has_meminfo_vmalloc_chunk")]
    assert_eq!(meminfo.vmalloc_chunk_kb, 0);
}

#[test]
fn test_meminfo_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    let meminfo = sys.get_meminfo();
    //
    assert_eq!(meminfo.mem_total_kb, 8152520);
    assert_eq!(meminfo.mem_free_kb, 1830280);
    //
    #[cfg(feature = "has_meminfo_mem_available")]
    assert_eq!(meminfo.mem_available_kb, 3988368);
    //
    assert_eq!(meminfo.buffers_kb, 199940);
    assert_eq!(meminfo.cached_kb, 2367832);
    assert_eq!(meminfo.swap_cached_kb, 203176);
    assert_eq!(meminfo.active_kb, 3448216);
    assert_eq!(meminfo.inactive_kb, 2380416);
    //
    #[cfg(feature = "has_meminfo_active_anon")]
    assert_eq!(meminfo.active_anon_kb, 2701192);
    #[cfg(feature = "has_meminfo_inactive_anon")]
    assert_eq!(meminfo.inactive_anon_kb, 819916);
    #[cfg(feature = "has_meminfo_active_file")]
    assert_eq!(meminfo.active_file_kb, 747024);
    #[cfg(feature = "has_meminfo_inactive_file")]
    assert_eq!(meminfo.inactive_file_kb, 1560500);
    //
    #[cfg(feature = "has_meminfo_unevictable")]
    assert_eq!(meminfo.unevictable_kb, 12964);
    #[cfg(feature = "has_meminfo_mlocked")]
    assert_eq!(meminfo.mlocked_kb, 12964);
    //
    #[cfg(feature = "has_meminfo_high_total")]
    assert_eq!(meminfo.high_total_kb, 0);
    #[cfg(feature = "has_meminfo_low_free")]
    assert_eq!(meminfo.high_free_kb, 0);
    #[cfg(feature = "has_meminfo_low_total")]
    assert_eq!(meminfo.low_total_kb, 0);
    #[cfg(feature = "has_meminfo_low_free")]
    assert_eq!(meminfo.low_free_kb, 0);
    //
    #[cfg(feature = "has_meminfo_mmap_copy")]
    assert_eq!(meminfo.mmap_copy_kb, 0);
    //
    assert_eq!(meminfo.swap_total_kb, 8388600);
    assert_eq!(meminfo.swap_free_kb, 7351824);
    //
    #[cfg(feature = "has_meminfo_dirty")]
    assert_eq!(meminfo.dirty_kb, 15440);
    #[cfg(feature = "has_meminfo_writeback")]
    assert_eq!(meminfo.writeback_kb, 0);
    #[cfg(feature = "has_meminfo_anon_pages")]
    assert_eq!(meminfo.anon_pages_kb, 3262288);
    #[cfg(feature = "has_meminfo_mapped")]
    assert_eq!(meminfo.mapped_kb, 666788);
    #[cfg(feature = "has_meminfo_slab")]
    assert_eq!(meminfo.slab_kb, 312588);
    #[cfg(feature = "has_meminfo_slab_reclaimable")]
    assert_eq!(meminfo.slab_reclaimable_kb, 141996);
    #[cfg(feature = "has_meminfo_slab_unreclaim")]
    assert_eq!(meminfo.slab_unreclaim_kb, 170592);
    #[cfg(feature = "has_meminfo_page_tables")]
    assert_eq!(meminfo.page_tables_kb, 53272);
    #[cfg(feature = "has_meminfo_nfs_unstable")]
    assert_eq!(meminfo.nfs_unstable_kb, 0);
    #[cfg(feature = "has_meminfo_bounce")]
    assert_eq!(meminfo.bounce_kb, 0);
    #[cfg(feature = "has_meminfo_commit_limit")]
    assert_eq!(meminfo.commit_limit_kb, 12464860);
    #[cfg(feature = "has_meminfo_commit_as")]
    assert_eq!(meminfo.commited_as_kb, 11538224);
    #[cfg(feature = "has_meminfo_vmalloc_total")]
    assert_eq!(meminfo.vmalloc_total_kb, 34359738367);
    #[cfg(feature = "has_meminfo_vmalloc_used")]
    assert_eq!(meminfo.vmalloc_used_kb, 41172);
    #[cfg(feature = "has_meminfo_vmalloc_chunk")]
    assert_eq!(meminfo.vmalloc_chunk_kb, 0);
}
