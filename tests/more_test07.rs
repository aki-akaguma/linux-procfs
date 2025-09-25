use linux_procfs::System;

macro_rules! base_path_empty_cmdline {
    () => {
        "fixtures/test-pidentry-empty-cmdline"
    };
}

macro_rules! base_path_no_stat {
    () => {
        "fixtures/test-pidentry-no-stat"
    };
}

#[test]
fn test_pidentry_empty_cmdline() {
    let mut sys = System::new(base_path_empty_cmdline!());
    // To test empty cmdline, we need to mock the entire /proc/12345 directory
    // For simplicity, we'll just test that get_pidentry_cmdline returns an empty string
    // when the file is empty.
    let cmdline = sys.get_pidentry_cmdline(12345);
    assert!(cmdline.is_some());
    assert_eq!(cmdline.unwrap().cmdline, "");
}

#[test]
fn test_pidentry_no_stat() {
    let mut sys = System::new(base_path_no_stat!());
    // When the stat file is empty, get_pidentry_stat should return None
    let stat = sys.get_pidentry_stat(54321);
    assert!(stat.is_none());
}
