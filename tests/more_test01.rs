use linux_procfs::System;

macro_rules! base_path_amd {
    () => {
        "fixtures/linux-4.4.0-cpu-amd"
    };
}

#[test]
fn test_stat_cguest_not_present() {
    let mut sys = System::new(base_path_amd!());
    let _stat = sys.get_stat();

    // Assert that cguest is 0 when the feature is enabled but not present in the fixture
    #[cfg(feature = "has_stat_cguest")]
    {
        assert_eq!(stat.cpu.cguest, 0);
        for cpu in stat.cpus.iter() {
            assert_eq!(cpu.cguest, 0);
        }
    }
}
