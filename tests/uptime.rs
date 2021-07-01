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
fn test_uptime_intel() {
    let mut sys = System::new(base_path_intel!());
    let uptime = sys.get_uptime();
    //
    assert_eq!(uptime.seconds, 464226.62);
}

#[test]
fn test_uptime_amd() {
    let mut sys = System::new(base_path_amd!());
    let uptime = sys.get_uptime();
    //
    assert_eq!(uptime.seconds, 148978.30);
}

#[test]
fn test_uptime_5_4() {
    let mut sys = System::new(base_path_5_4!());
    let uptime = sys.get_uptime();
    //
    assert_eq!(uptime.seconds, 64216.93);
}

#[test]
fn test_uptime_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    let uptime = sys.get_uptime();
    //
    assert_eq!(uptime.seconds, 244665.07);
}
