use linux_procfs::System;

macro_rules! base_path_cguest {
    () => {
        "fixtures/test-cguest"
    };
}

#[test]
fn test_stat_cguest_present() {
    let mut sys = System::new(base_path_cguest!());
    let _stat = sys.get_stat();

    // Assert that cguest has the expected value when the feature is enabled and present in the fixture
    #[cfg(feature = "has_stat_cguest")]
    {
        assert_eq!(stat.cpu.cguest, 12345);
        assert_eq!(stat.cpus[0].cguest, 1234);
        assert_eq!(stat.cpus[1].cguest, 5678);
        assert_eq!(stat.cpus[2].cguest, 9012);
        assert_eq!(stat.cpus[3].cguest, 3456);
    }
}
