use linux_procfs::System;

macro_rules! base_path_5_4_intel {
    () => {
        "fixtures/linux-5.4.0-cpu-intel"
    };
}

#[test]
fn test_pidentry_5_4_intel_1() {
    let mut sys = System::new(base_path_5_4_intel!());
    //
    let pid = 740660;
    let _pidentry_stat = sys.get_pidentry_stat(pid).unwrap().unwrap();
    let _pidentry_statm = sys.get_pidentry_statm(pid).unwrap().unwrap();
    let _pidentry_status = sys.get_pidentry_status(pid).unwrap().unwrap();
    //
    let pidentry_comm = sys.get_pidentry_comm(pid).unwrap().unwrap();
    assert_eq!(pidentry_comm.cmdline, "make-test-fixtu");
    let pidentry_cmdline = sys.get_pidentry_cmdline(pid).unwrap().unwrap();
    assert_eq!(
        pidentry_cmdline.cmdline,
        "/bin/sh ./scripts/make-test-fixtures-for-hmon.sh"
    );
}

#[test]
fn test_pidentry_5_4_intel_2() {
    let mut sys = System::new(base_path_5_4_intel!());
    //
    let pid = 999999;
    if let Err(e) = sys.get_pidentry_stat(pid) {
        assert!(e.is_not_found());
    } else {
        unreachable!();
    }
    if let Err(e) = sys.get_pidentry_statm(pid) {
        assert!(e.is_not_found());
    } else {
        unreachable!();
    }
    if let Err(e) = sys.get_pidentry_status(pid) {
        assert!(e.is_not_found());
    } else {
        unreachable!();
    }
    if let Err(e) = sys.get_pidentry_comm(pid) {
        assert!(e.is_not_found());
    } else {
        unreachable!();
    }
    if let Err(e) = sys.get_pidentry_cmdline(pid) {
        assert!(e.is_not_found());
    } else {
        unreachable!();
    }
}
