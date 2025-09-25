use linux_procfs::System;

macro_rules! base_path_loadavg_zero {
    () => {
        "fixtures/test-loadavg-zero"
    };
}

macro_rules! base_path_loadavg_high {
    () => {
        "fixtures/test-loadavg-high"
    };
}

#[test]
fn test_loadavg_zero() {
    let mut sys = System::new(base_path_loadavg_zero!());
    let loadavg = sys.get_loadavg();

    assert_eq!(loadavg.a1, 0.00);
    assert_eq!(loadavg.a5, 0.00);
    assert_eq!(loadavg.a15, 0.00);
    assert_eq!(loadavg.last_pid, 1000);
}

#[test]
fn test_loadavg_high() {
    let mut sys = System::new(base_path_loadavg_high!());
    let loadavg = sys.get_loadavg();

    assert_eq!(loadavg.a1, 100.00);
    assert_eq!(loadavg.a5, 90.00);
    assert_eq!(loadavg.a15, 80.00);
    assert_eq!(loadavg.last_pid, 99999);
}