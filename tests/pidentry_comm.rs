use linux_procfs::System;

macro_rules! base_path_5_4_intel {
    () => {
        "fixtures/linux-5.4.0-cpu-intel"
    };
}

#[test]
fn test_pidentry_comm_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    //
    let pidentry_comm = sys.get_pidentry_comm(740660).unwrap().unwrap();
    assert_eq!(pidentry_comm.cmdline, "make-test-fixtu");
    let pidentry_cmdline = sys.get_pidentry_cmdline(740660).unwrap().unwrap();
    assert_eq!(
        pidentry_cmdline.cmdline,
        "/bin/sh ./scripts/make-test-fixtures-for-hmon.sh"
    );
    //
    let pidentry_comm = sys.get_pidentry_comm(740638).unwrap().unwrap();
    assert_eq!(pidentry_comm.cmdline, "tumblerd");
    let pidentry_cmdline = sys.get_pidentry_cmdline(740638).unwrap().unwrap();
    assert_eq!(
        pidentry_cmdline.cmdline,
        "/usr/lib/x86_64-linux-gnu/tumbler-1/tumblerd"
    );
    //
    let pidentry_comm = sys.get_pidentry_comm(738244).unwrap().unwrap();
    assert_eq!(pidentry_comm.cmdline, "chrome");
    let pidentry_cmdline = sys.get_pidentry_cmdline(738244).unwrap().unwrap();
    assert_eq!(pidentry_cmdline.cmdline, "/snap/chromium/1328/usr/lib/chromium-browser/chrome --type=renderer --field-trial-handle=1880753760203401862,4523857862519602211,131072 --disable-gpu-compositing --lang=ja --enable-auto-reload --num-raster-threads=2 --enable-main-frame-before-activation --renderer-client-id=25 --no-v8-untrusted-code-mitigations --shared-files=v8_snapshot_data:100");
}
