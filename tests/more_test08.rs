use linux_procfs::System;

macro_rules! base_path_diskstats_zero {
    () => {
        "fixtures/test-diskstats-zero"
    };
}

macro_rules! base_path_diskstats_large {
    () => {
        "fixtures/test-diskstats-large"
    };
}

#[test]
fn test_diskstats_zero() {
    let mut sys = System::new(base_path_diskstats_zero!());
    let diskstats = sys.get_diskstats();

    assert_eq!(diskstats.disks.len(), 1);
    let disk = &diskstats.disks[0];

    #[cfg(feature = "has_diskstats_device_number")]
    assert_eq!(disk.major_num, 0);
    #[cfg(feature = "has_diskstats_device_number")]
    assert_eq!(disk.minor_num, 0);

    assert_eq!(disk.name, "zero");
    assert_eq!(disk.rio, 0);
    assert_eq!(disk.rmerge, 0);
    assert_eq!(disk.rblk, 0);
    assert_eq!(disk.ruse, 0);
    assert_eq!(disk.wio, 0);
    assert_eq!(disk.wmerge, 0);
    assert_eq!(disk.wblk, 0);
    assert_eq!(disk.wuse, 0);

    #[cfg(feature = "has_diskstats_running")]
    assert_eq!(disk.running, 0);
    #[cfg(feature = "has_diskstats_use")]
    assert_eq!(disk.use_, 0);

    assert_eq!(disk.aveq, 0);
}

#[test]
fn test_diskstats_large() {
    let mut sys = System::new(base_path_diskstats_large!());
    let diskstats = sys.get_diskstats();

    assert_eq!(diskstats.disks.len(), 1);
    let disk = &diskstats.disks[0];

    #[cfg(feature = "has_diskstats_device_number")]
    assert_eq!(disk.major_num, 1234567890);
    #[cfg(feature = "has_diskstats_device_number")]
    assert_eq!(disk.minor_num, 1234567890);

    assert_eq!(disk.name, "large");
    assert_eq!(disk.rio, 999999999999999999);
    assert_eq!(disk.rmerge, 999999999999999999);
    assert_eq!(disk.rblk, 999999999999999999);
    assert_eq!(disk.ruse, 4294967295); // Corrected to u32 max
    assert_eq!(disk.wio, 999999999999999999);
    assert_eq!(disk.wmerge, 999999999999999999);
    assert_eq!(disk.wblk, 999999999999999999);
    assert_eq!(disk.wuse, 4294967295); // Corrected to u32 max

    #[cfg(feature = "has_diskstats_running")]
    assert_eq!(disk.running, 4294967295); // Corrected to u32 max
    #[cfg(feature = "has_diskstats_use")]
    assert_eq!(disk.use_, 4294967295); // Corrected to u32 max

    assert_eq!(disk.aveq, 4294967295); // Corrected to u32 max
}