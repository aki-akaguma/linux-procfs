use linux_procfs::System;

macro_rules! base_path_stat_features {
    () => {
        "fixtures/test-stat-features"
    };
}

#[test]
fn test_stat_features_present() {
    let mut sys = System::new(base_path_stat_features!());
    let _stat = sys.get_stat();

    #[cfg(feature = "has_stat_btime")]
    {
        assert_eq!(stat.btime, 1527662995);
    }

    #[cfg(feature = "has_stat_procs_running")]
    {
        assert_eq!(stat.procs_running, 2);
    }

    #[cfg(feature = "has_stat_procs_blocked")]
    {
        assert_eq!(stat.procs_blocked, 0);
    }
}
