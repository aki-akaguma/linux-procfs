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
fn test_loadavg_intel() {
    let mut sys = System::new(base_path_intel!());
    let loadavg = sys.get_loadavg();
    //
    assert_eq!(loadavg.a1, 0.68);
    assert_eq!(loadavg.a5, 0.50);
    assert_eq!(loadavg.a15, 0.39);
    assert_eq!(loadavg.last_pid, 29726);
}

#[test]
fn test_loadavg_amd() {
    let mut sys = System::new(base_path_amd!());
    let loadavg = sys.get_loadavg();
    //
    assert_eq!(loadavg.a1, 1.33);
    assert_eq!(loadavg.a5, 1.42);
    assert_eq!(loadavg.a15, 1.28);
    assert_eq!(loadavg.last_pid, 7769);
}

#[test]
fn test_loadavg_5_4() {
    let mut sys = System::new(base_path_5_4!());
    let loadavg = sys.get_loadavg();
    //
    assert_eq!(loadavg.a1, 0.03);
    assert_eq!(loadavg.a5, 0.01);
    assert_eq!(loadavg.a15, 0.0);
    assert_eq!(loadavg.last_pid, 8560);
}

#[test]
fn test_loadavg_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    let loadavg = sys.get_loadavg();
    //
    assert_eq!(loadavg.a1, 2.14);
    assert_eq!(loadavg.a5, 2.09);
    assert_eq!(loadavg.a15, 2.04);
    assert_eq!(loadavg.last_pid, 742669);
}
