use linux_procfs::System;

macro_rules! base_path_uptime_zero {
    () => {
        "fixtures/test-uptime-zero"
    };
}

macro_rules! base_path_uptime_large {
    () => {
        "fixtures/test-uptime-large"
    };
}

#[test]
fn test_uptime_zero() {
    let mut sys = System::new(base_path_uptime_zero!());
    let uptime = sys.get_uptime();

    assert_eq!(uptime.seconds, 0.00);
}

#[test]
fn test_uptime_large() {
    let mut sys = System::new(base_path_uptime_large!());
    let uptime = sys.get_uptime();

    assert_eq!(uptime.seconds, 999999999.99);
}
