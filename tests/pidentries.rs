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

macro_rules! base_path_5_4_intel {
    () => {
        "fixtures/linux-5.4.0-cpu-intel"
    };
}

macro_rules! assert_eq_stat {
    (
        $stat:expr =>
        [
            $pid:tt
            $comm:tt
            $state:tt
            $ppid:tt
            $pgrp:tt
            $session:tt
            $tty_nr:tt
            $($tok:tt)*
        ]
    ) => {
        let stat = &$stat;
        //
        assert_eq!(stat.pid, $pid);
        //
        #[cfg(feature = "has_pidentry_stat_comm")]
        assert_eq!(stat.comm, $comm);
        #[cfg(feature = "has_pidentry_stat_state")]
        assert_eq!(stat.state, $state);
        //
        assert_eq!(stat.ppid, $ppid);
        assert_eq!(stat.pgrp, $pgrp);
        //
        #[cfg(feature = "has_pidentry_stat_session")]
        assert_eq!(stat.session, $session);
        #[cfg(feature = "has_pidentry_stat_tty_nr")]
        assert_eq!(stat.tty_nr, $tty_nr);
        //
        assert_eq_stat!($stat => b01_tpgid[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b01_tpgid[
            -$tpgid:tt
            $($tok:tt)*
        ]
    ) => {
        let _stat = &$stat;
        //
        #[cfg(feature = "has_pidentry_stat_tpgid")]
        assert_eq!(_stat.tpgid, -$tpgid);
        //
        assert_eq_stat!($stat => b02[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b01_tpgid[
            $tpgid:tt
            $($tok:tt)*
        ]
    ) => {
        let _stat = &$stat;
        //
        #[cfg(feature = "has_pidentry_stat_tpgid")]
        assert_eq!(_stat.tpgid, $tpgid);
        //
        assert_eq_stat!($stat => b02[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b02[
            $flags:tt
            $minflt:tt
            $cminflt:tt
            $majflt:tt
            $cmajflt:tt
            $utime:tt
            $stime:tt
            $cutime:tt
            $cstime:tt
            $($tok:tt)*
        ]
    ) => {
        let stat = &$stat;
        //
        #[cfg(feature = "has_pidentry_stat_flags")]
        assert_eq!(stat.flags, $flags);
        //
        #[cfg(feature = "has_pidentry_stat_minflt")]
        assert_eq!(stat.minflt, $minflt);
        #[cfg(feature = "has_pidentry_stat_cminflt")]
        assert_eq!(stat.cminflt, $cminflt);
        #[cfg(feature = "has_pidentry_stat_majflt")]
        assert_eq!(stat.majflt, $majflt);
        #[cfg(feature = "has_pidentry_stat_cmajflt")]
        assert_eq!(stat.cmajflt, $cmajflt);
        //
        assert_eq!(stat.utime, $utime);
        assert_eq!(stat.stime, $stime);
        assert_eq!(stat.cutime, $cutime);
        assert_eq!(stat.cstime, $cstime);
        //
        assert_eq_stat!($stat => b02_priority[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b02_priority[
            -$priority:tt
            $($tok:tt)*
        ]
    ) => {
        let _stat = &$stat;
        //
        #[cfg(feature = "has_pidentry_stat_priority")]
        assert_eq!(_stat.priority, -$priority);
        //
        assert_eq_stat!($stat => b03_nice[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b02_priority[
            $priority:tt
            $($tok:tt)*
        ]
    ) => {
        let _stat = &$stat;
        //
        #[cfg(feature = "has_pidentry_stat_priority")]
        assert_eq!(_stat.priority, $priority);
        //
        assert_eq_stat!($stat => b03_nice[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b03_nice[
            -$nice:tt
            $($tok:tt)*
        ]
    ) => {
        let stat = &$stat;
        //
        assert_eq!(stat.nice, -$nice);
        //
        assert_eq_stat!($stat => b04[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b03_nice[
            $nice:tt
            $($tok:tt)*
        ]
    ) => {
        let stat = &$stat;
        //
        assert_eq!(stat.nice, $nice);
        //
        assert_eq_stat!($stat => b04[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b04[
            $num_threads:tt
            $unused_1:tt
            $starttime:tt
            $vsize:tt
            $rss:tt
            $rlim:tt
            $startcode:tt
            $endcode:tt
            $startstack:tt
            $kstesp:tt
            $ksteip:tt
            $signal:tt
            $blocked:tt
            $sigignore:tt
            $sigcatch:tt
            $unused_2:tt
            $unused_3:tt
            $unused_4:tt
            $($tok:tt)*
        ]
    ) => {
        let stat = &$stat;
        //
        assert_eq!(stat.num_threads, $num_threads);
        assert_eq!(stat.starttime, $starttime);
        //
        #[cfg(feature = "has_pidentry_stat_vsize")]
        assert_eq!(stat.vsize, $vsize);
        #[cfg(feature = "has_pidentry_stat_rss")]
        assert_eq!(stat.rss, $rss);
        #[cfg(feature = "has_pidentry_stat_rlim")]
        assert_eq!(stat.rlim, $rlim);
        //
        #[cfg(feature = "has_pidentry_stat_startcode")]
        assert_eq!(stat.startcode, $startcode);
        #[cfg(feature = "has_pidentry_stat_endcode")]
        assert_eq!(stat.endcode, $endcode);
        #[cfg(feature = "has_pidentry_stat_startstack")]
        assert_eq!(stat.startstack, $startstack);
        #[cfg(feature = "has_pidentry_stat_kstesp")]
        assert_eq!(stat.kstesp, $kstesp);
        #[cfg(feature = "has_pidentry_stat_ksteip")]
        assert_eq!(stat.ksteip, $ksteip);
        //
        #[cfg(feature = "has_pidentry_stat_signal")]
        assert_eq!(stat.signal, $signal);
        #[cfg(feature = "has_pidentry_stat_blocked")]
        assert_eq!(stat.blocked, $blocked);
        #[cfg(feature = "has_pidentry_stat_sigignore")]
        assert_eq!(stat.sigignore, $sigignore);
        #[cfg(feature = "has_pidentry_stat_sigcatch")]
        assert_eq!(stat.sigcatch, $sigcatch);
        //
        assert_eq_stat!($stat => b05[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b05[
            $exit_signal:tt
            $processor:tt
            $rt_priority:tt
            $policy:tt
            $delayacct_blkio_ticks:tt
            $($tok:tt)*
        ]
    ) => {
        let _stat = &$stat;
        //
        #[cfg(feature = "has_pidentry_stat_exit_signal")]
        assert_eq!(_stat.exit_signal, $exit_signal);
        #[cfg(feature = "has_pidentry_stat_processor")]
        assert_eq!(_stat.processor, $processor);
        #[cfg(feature = "has_pidentry_stat_rt_priority")]
        assert_eq!(_stat.rt_priority, $rt_priority);
        #[cfg(feature = "has_pidentry_stat_policy")]
        assert_eq!(_stat.policy, $policy);
        #[cfg(feature = "has_pidentry_stat_delayacct")]
        assert_eq!(_stat.delayacct_blkio_ticks, $delayacct_blkio_ticks);
        //
        assert_eq_stat!($stat => b06[ $($tok)* ]);
    };
    (
        $stat:expr =>
        b06[
            $unused_5:tt
            $unused_6:tt
            $unused_7:tt
            $unused_8:tt
            $unused_9:tt
            $unused_10:tt
            $unused_11:tt
            $unused_12:tt
            $unused_13:tt
            $unused_14:tt
        ]
    ) => {
        let _stat = &$stat;
    };
}

macro_rules! assert_eq_statm {
    ($statm:expr => [$size:tt $resident:tt $share:tt $text:tt $lib:tt $data:tt $none:tt]) => {
        let statm = &$statm;
        //
        assert_eq!(statm.size, $size);
        assert_eq!(statm.resident, $resident);
        assert_eq!(statm.share, $share);
        assert_eq!(statm.text, $text);
        assert_eq!(statm.lib, $lib);
        assert_eq!(statm.data, $data);
    };
}

macro_rules! assert_eq_status {
    ($status:expr => tgid[$tgid:tt $ngid:tt]) => {
        let _status = &$status;
        //
        #[cfg(feature = "has_pidentry_status_tgid")]
        assert_eq!(_status.tgid, $tgid);
        #[cfg(feature = "has_pidentry_status_ngid")]
        assert_eq!(_status.ngid, $ngid);
    };
    ($status:expr => pid[$pid:tt $ppid:tt]) => {
        let status = &$status;
        //
        assert_eq!(status.pid, $pid);
        assert_eq!(status.ppid, $ppid);
    };
    (
        $status:expr => uid[$ruid:tt $euid:tt $suid:tt $fuid:tt $rgid:tt $egid:tt $sgid:tt $fgid:tt]
    ) => {
        let status = &$status;
        //
        assert_eq!(status.ruid, $ruid);
        assert_eq!(status.euid, $euid);
        assert_eq!(status.suid, $suid);
        assert_eq!(status.fuid, $fuid);
        assert_eq!(status.rgid, $rgid);
        assert_eq!(status.egid, $egid);
        assert_eq!(status.sgid, $sgid);
        assert_eq!(status.fgid, $fgid);
    };
    (
        $status:expr => vm[
            $vm_peak:tt
            $vm_size:tt
            $vm_lck:tt
            $vm_pin:tt
            $vm_hwm:tt
            $vm_rss:tt
            $vm_data:tt
            $vm_stk:tt
            $vm_exe:tt
            $vm_lib:tt
            $vm_pte:tt
            $vm_pmd:tt
            $vm_swap:tt
        ]
    ) => {
        let status = &$status;
        //
        #[cfg(feature = "has_pidentry_status_vm_peak")]
        assert_eq!(status.vm_peak, $vm_peak);
        //
        assert_eq!(status.vm_size, $vm_size);
        assert_eq!(status.vm_lck, $vm_lck);
        //
        #[cfg(feature = "has_pidentry_status_vm_pin")]
        assert_eq!(status.vm_pin, $vm_pin);
        //
        #[cfg(feature = "has_pidentry_status_vm_hwm")]
        assert_eq!(status.vm_hwm, $vm_hwm);
        //
        assert_eq!(status.vm_rss, $vm_rss);
        assert_eq!(status.vm_data, $vm_data);
        assert_eq!(status.vm_stk, $vm_stk);
        assert_eq!(status.vm_exe, $vm_exe);
        assert_eq!(status.vm_lib, $vm_lib);
        assert_eq!(status.vm_pte, $vm_pte);
        //
        #[cfg(feature = "has_pidentry_status_vm_pmd")]
        assert_eq!(status.vm_pmd, $vm_pmd);
        //
        assert_eq!(status.vm_swap, $vm_swap);
    };
    (
        $status:expr => vm[
            $vm_peak:tt
            $vm_size:tt
            $vm_lck:tt
            $vm_pin:tt
            $vm_hwm:tt
            $vm_rss:tt
            $rss_anon:tt
            $rss_file:tt
            $rss_shmem:tt
            $vm_data:tt
            $vm_stk:tt
            $vm_exe:tt
            $vm_lib:tt
            $vm_pte:tt
            $vm_swap:tt
        ]
    ) => {
        assert_eq_status!($status => vm[$vm_peak $vm_size $vm_lck $vm_pin $vm_hwm $vm_rss $vm_data $vm_stk $vm_exe $vm_lib $vm_pte 0 $vm_swap]);
        //
        let _status = &$status;
        #[cfg(feature = "has_pidentry_status_rss_anon")]
        assert_eq!(_status.rss_anon, $rss_anon);
        #[cfg(feature = "has_pidentry_status_rss_file")]
        assert_eq!(_status.rss_file, $rss_file);
        #[cfg(feature = "has_pidentry_status_rss_shmem")]
        assert_eq!(_status.rss_shmem, $rss_shmem);
    };
}

#[test]
fn test_pidentries_intel() {
    let mut sys = System::new(base_path_intel!());
    let pidentries = sys.get_pidentries();
    //
    assert_eq!(pidentries.pidentries.len(), 366);
    //
    {
        let pidentry = &pidentries.pidentries[0];
        //
        assert_eq_stat!(pidentry.stat => [1 "systemd" b'S' 0 1 1 0 -1 4194560 359541 12902718 160 4799 3471 763 14114 12191 20 0 1 0 3 190058496 1210 18446744073709551615 94479697154048 94479698579168 140734880879312 140734880874848 140486857591315 0 671173123 4096 1260 1 0 0 17 3 0 0 1224 0 0 94479698586752 94479698732776 94479725228032 140734880886591 140734880886609 140734880886609 140734880886765 0]);
        //
        assert_eq_statm!(pidentry.statm => [46401 1210 717 348 0 37365 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "systemd");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[1 0]);
        assert_eq_status!(pidentry.status => pid[1 0]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[251148 185604 0 0 6420 4840 149328 132 1392 3684 104 12 284]);
        //
        assert_eq!(pidentry.cmdline.cmdline, "/sbin/init splash");
    }
    //
    {
        let pidentry = &pidentries.pidentries[1];
        //
        assert_eq_stat!(pidentry.stat => [2 "kthreadd" b'S' 0 0 0 0 -1 2129984 0 0 0 0 0 10 0 0 20 0 1 0 3 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 1 0 0 0 3 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        //
        assert_eq_statm!(pidentry.statm => [0 0 0 0 0 0 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "kthreadd");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[2 0]);
        assert_eq_status!(pidentry.status => pid[2 0]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[0 0 0 0 0 0 0 0 0 0 0 0 0]);
        assert_eq!(pidentry.cmdline.cmdline, "");
    }
    {
        let pidentry = &pidentries.pidentries[10];
        //
        assert_eq_stat!(pidentry.stat => [13 "migration/1" b'S' 2 0 0 0 -1 69238848 0 0 0 0 0 47 0 0 -100 0 1 0 5 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 1 0 0 17 1 99 1 0 0 0 0 0 0 0 0 0 0 0]);
        //
        assert_eq_statm!(pidentry.statm => [0 0 0 0 0 0 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "migration/1");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[13 0]);
        assert_eq_status!(pidentry.status => pid[13 2]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[0 0 0 0 0 0 0 0 0 0 0 0 0]);
        assert_eq!(pidentry.cmdline.cmdline, "");
    }
    {
        let pidentry = &pidentries.pidentries[100];
        //
        assert_eq_stat!(pidentry.stat => [151 "deferwq" b'S' 2 0 0 0 -1 69238880 0 0 0 0 0 0 0 0 0 -20 1 0 134 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 1 0 0 17 3 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        //
        assert_eq_statm!(pidentry.statm => [0 0 0 0 0 0 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "deferwq");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[151 0]);
        assert_eq_status!(pidentry.status => pid[151 2]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[0 0 0 0 0 0 0 0 0 0 0 0 0]);
        assert_eq!(pidentry.cmdline.cmdline, "");
    }
    //
    {
        let pidentry = &pidentries.pidentries[200];
        //
        assert_eq_stat!(pidentry.stat => [2808 "dbus-daemon" b'S' 2063 2808 2808 0 -1 4194368 5528 1657 16 1 1125 324 0 1 20 0 1 0 5541 45187072 968 18446744073709551615 94898374189056 94898374403180 140722473904288 140722473902168 140279903709683 0 0 0 16385 1 0 0 17 0 0 0 225 0 0 94898376502600 94898376508240 94898386358272 140722473912379 140722473912453 140722473912453 140722473914339 0]);
        //
        assert_eq_statm!(pidentry.statm => [11032 968 587 53 0 452 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "dbus-daemon");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[2808 0]);
        assert_eq_status!(pidentry.status => pid[2808 2063]);
        assert_eq_status!(pidentry.status => uid[1000 1000 1000 1000 1000 1000 1000 1000]);
        assert_eq_status!(pidentry.status => vm[44160 44128 0 0 4468 3872 1676 132 212 5064 100 16 228]);
        assert_eq!(
            pidentry.cmdline.cmdline,
            "dbus-daemon --fork --session --address=unix:abstract=/tmp/dbus-TQcjW35ED2"
        );
    }
    {
        let pidentry = &pidentries.pidentries[354];
        //
        assert_eq_stat!(pidentry.stat => [28573 "xfreerdp" b'S' 3114 2974 2974 0 -1 4194304 2859 0 0 0 195213 53169 0 0 20 0 13 0 33249374 1017675776 5791 18446744073709551615 4194304 4337444 140725720280336 140725720279888 140497587033933 0 0 4096 1471704831 0 0 0 17 3 0 0 0 0 0 6438088 6965648 40050688 140725720282660 140725720282929 140725720282929 140725720285158 0]);
        //
        assert_eq_statm!(pidentry.statm => [248456 5791 3714 35 0 152311 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "xfreerdp");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[28573 0]);
        assert_eq_status!(pidentry.status => pid[28573 3114]);
        assert_eq_status!(pidentry.status => uid[1000 1000 1000 1000 1000 1000 1000 1000]);
        assert_eq_status!(pidentry.status => vm[1059360 993824 0 0 26068 23164 609112 132 140 52444 620 16 0]);
        assert_eq!(
            pidentry.cmdline.cmdline,
            "/usr/bin/xfreerdp /cert-ignore /sec-tls /async-input /gdi:hw /disp /fonts /window-drag /decorations /menu-anims /themes /wallpaper -bitmap-cache -offscreen-cache /clipboard /sound /printer:Brother-DCP-J525N /size:1780x980 /bpp:32 /u:CC /p:********* /v:trade05-win7.lan"
        );
    }
    {
        let pidentry = &pidentries.pidentries[358];
        //
        assert_eq_stat!(pidentry.stat => [29242 "chromium-browse" b'S' 25334 2974 2974 0 -1 1077936192 78106 0 22 0 457 47 0 0 20 0 13 0 46372725 1802465280 40356 18446744073709551615 94320188768256 94320334987160 140722918441376 140722918438336 140193766487817 0 0 4098 1073827565 0 0 0 17 2 0 0 11 0 0 94320334993984 94320341135936 94320374165504 140722918446480 140722918446637 140722918446637 140722918449101 0]);
        //
        assert_eq_statm!(pidentry.statm => [440055 40356 17720 35698 0 327789 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "chromium-browse");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[29242 0]);
        assert_eq_status!(pidentry.status => pid[29242 25334]);
        assert_eq_status!(pidentry.status => uid[1000 1000 1000 1000 1000 1000 1000 1000]);
        assert_eq_status!(pidentry.status => vm[1834824 1760220 0 0 244700 161424 1311024 132 142792 54796 1556 592 0]);
        assert_eq!(
            pidentry.cmdline.cmdline,
            "/usr/lib/chromium-browser/chromium-browser --type=renderer --field-trial-handle=5707398580160947606,12502303896638068163,131072 --service-pipe-token=0BCB652ABF9A2FA2B8A1739C45B1241F --lang=ja --enable-offline-auto-reload --enable-offline-auto-reload-visible-only --ppapi-flash-path=/usr/lib/adobe-flashplugin/libpepflashplayer.so --ppapi-flash-version=29.0.0.171 --num-raster-threads=2 --enable-main-frame-before-activation --service-request-channel-token=0BCB652ABF9A2FA2B8A1739C45B1241F --renderer-client-id=278 --shared-files=v8_context_snapshot_data:100,v8_natives_data:101"
        );
    }
    {
        let pidentry = &pidentries.pidentries[365];
        //
        assert_eq_stat!(pidentry.stat => [29726 "mk-fixtures" b'R' 29725 29725 21543 34842 29725 4194560 151 0 0 0 1 7 0 0 39 19 1 0 46422662 17690624 401 18446744073709551615 94817992482816 94817992975832 140730511007136 140730510997016 140713119515904 0 0 4096 1088 0 0 0 17 2 0 0 0 0 0 94817995074592 94817995092256 94818007777280 140730511009829 140730511009850 140730511009850 140730511011811 0]);
        //
        assert_eq_statm!(pidentry.statm => [4319 401 366 121 0 1079 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "mk-fixtures");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'R');
        //
        assert_eq_status!(pidentry.status => tgid[29726 0]);
        assert_eq_status!(pidentry.status => pid[29726 29725]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[17276 17276 0 0 1604 1604 4184 132 484 2168 52 12 0]);
        assert_eq!(pidentry.cmdline.cmdline, "./target/mk-fixtures");
    }
}

#[test]
fn test_pidentries_amd() {
    let mut sys = System::new(base_path_amd!());
    let pidentries = sys.get_pidentries();
    //
    assert_eq!(pidentries.pidentries.len(), 212);
    //
    {
        let pidentry = &pidentries.pidentries[0];
        //
        assert_eq_stat!(pidentry.stat => [1 "systemd" b'S' 0 1 1 0 -1 4194560 19379 423381 206 856 209 552 1859 1070 20 0 1 0 3 39550976 1265 18446744073709551615 94348624252928 94348625678048 140734322392992 140734322389272 139625883249139 0 671173123 4096 1260 1 0 0 17 2 0 0 267 0 0 94348625685632 94348625831656 94348654657536 140734322401102 140734322401113 140734322401113 140734322401261 0]);
        //
        assert_eq_statm!(pidentry.statm => [9656 1265 640 348 0 620 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "systemd");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[1 0]);
        assert_eq_status!(pidentry.status => pid[1 0]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[39272 38624 0 0 6188 5060 2348 132 1392 3684 96 12 184]);
        assert_eq!(pidentry.cmdline.cmdline, "/sbin/init");
    }
    //
    {
        let pidentry = &pidentries.pidentries[1];
        //
        assert_eq_stat!(pidentry.stat => [2 "kthreadd" b'S' 0 0 0 0 -1 2129984 0 0 0 0 0 2 0 0 20 0 1 0 3 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 1 0 0 0 2 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        //
        assert_eq_statm!(pidentry.statm => [0 0 0 0 0 0 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "kthreadd");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[2 0]);
        assert_eq_status!(pidentry.status => pid[2 0]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[0 0 0 0 0 0 0 0 0 0 0 0 0]);
        assert_eq!(pidentry.cmdline.cmdline, "");
    }
    {
        let pidentry = &pidentries.pidentries[10];
        //
        assert_eq_stat!(pidentry.stat => [13 "ksoftirqd/1" b'S' 2 0 0 0 -1 69238848 0 0 0 0 6 432 0 0 20 0 1 0 7 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 1 0 0 17 1 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        //
        assert_eq_statm!(pidentry.statm => [0 0 0 0 0 0 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "ksoftirqd/1");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[13 0]);
        assert_eq_status!(pidentry.status => pid[13 2]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[0 0 0 0 0 0 0 0 0 0 0 0 0]);
        assert_eq!(pidentry.cmdline.cmdline, "");
    }
    {
        let pidentry = &pidentries.pidentries[100];
        //
        assert_eq_stat!(pidentry.stat => [256 "ext4-rsv-conver" b'S' 2 0 0 0 -1 69238880 0 0 0 0 0 0 0 0 0 -20 1 0 775 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 1 0 0 17 3 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        //
        assert_eq_statm!(pidentry.statm => [0 0 0 0 0 0 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "ext4-rsv-conver");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[256 0]);
        assert_eq_status!(pidentry.status => pid[256 2]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[0 0 0 0 0 0 0 0 0 0 0 0 0]);
        assert_eq!(pidentry.cmdline.cmdline, "");
    }
    //
    {
        let pidentry = &pidentries.pidentries[129];
        //
        assert_eq_stat!(pidentry.stat => [784 "dbus-daemon" b'S' 1 784 784 0 -1 4194560 297 0 9 0 31 6 0 0 20 0 1 0 1463 43933696 538 18446744073709551615 93960038334464 93960038548588 140724676233088 140724676230968 140113835370995 0 0 4096 16385 1 0 0 17 3 0 0 229 0 0 93960040648008 93960040653648 93960063815680 140724676235012 140724676235103 140724676235103 140724676235235 0]);
        //
        assert_eq_statm!(pidentry.statm => [10726 538 432 53 0 146 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "dbus-daemon");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[784 0]);
        assert_eq_status!(pidentry.status => pid[784 1]);
        assert_eq_status!(pidentry.status => uid[102 102 102 102 105 105 105 105]);
        assert_eq_status!(pidentry.status => vm[42908 42904 0 0 3768 2152 452 132 212 5064 104 12 44]);
        assert_eq!(
            pidentry.cmdline.cmdline,
            "/usr/bin/dbus-daemon --system --address=systemd: --nofork --nopidfile --systemd-activation"
        );
    }
    {
        let pidentry = &pidentries.pidentries[154];
        assert_eq_stat!(pidentry.stat => [1749 "apache2" b'S' 1400 1749 1749 0 -1 4194624 692 724 0 2 202 294 2468 2162 20 0 1 0 3018 75476992 760 18446744073709551615 94632803917824 94632804551964 140727758503360 140727758502408 140121210553779 0 0 16781312 134235883 1 0 0 17 3 0 0 18 0 0 94632806650856 94632806675400 94632819376128 140727758511802 140727758511829 140727758511829 140727758512102 0]);
        //
        assert_eq_statm!(pidentry.statm => [18427 760 453 155 0 335 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "apache2");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[1749 0]);
        assert_eq_status!(pidentry.status => pid[1749 1400]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[73708 73708 0 0 4424 3040 1208 132 620 3692 160 16 0]);
        assert_eq!(pidentry.cmdline.cmdline, "/usr/sbin/apache2 -k start");
    }
    {
        let pidentry = &pidentries.pidentries[181];
        assert_eq_stat!(pidentry.stat => [2862 "qemu-system-x86" b'S' 1 2861 2861 0 -1 138412416 1347468 0 1090 0 3721468 439616 0 0 20 0 6 0 86786 5202874368 523116 18446744073709551615 94373517504512 94373524100484 140720890445488 140720890444560 140198844274705 0 268444224 4096 16963 0 0 0 17 1 0 0 0 3247610 0 94373526199024 94373528061664 94373547839488 140720890452522 140720890453879 140720890453879 140720890453980 0]);
        //
        assert_eq_statm!(pidentry.statm => [1270233 523116 860 1611 0 1193337 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "qemu-system-x86");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'R');
        //
        assert_eq_status!(pidentry.status => tgid[2862 0]);
        assert_eq_status!(pidentry.status => pid[2862 1]);
        assert_eq_status!(pidentry.status => uid[106 106 106 106 113 113 113 113]);
        assert_eq_status!(pidentry.status => vm[5634696 5080932 0 0 2166464 2092464 4773216 132 6444 47532 5568 32 62100]);
        assert_eq!(
            pidentry.cmdline.cmdline,
            "qemu-system-x86_64 -enable-kvm -name 61_trade04-win7 -S -machine pc-i440fx-xenial,accel=kvm,usb=off -cpu Opteron_G3,+wdt,+skinit,+ibs,+osvw,+3dnowprefetch,+cr8legacy,+extapic,+cmp_legacy,+3dnow,+3dnowext,+pdpe1gb,+fxsr_opt,+mmxext,+ht,+vme -m 2048 -realtime mlock=off -smp 2,maxcpus=4,sockets=1,cores=1,threads=4 -uuid be2ef962-e6cb-dd63-a1d0-798673d1d466 -no-user-config -nodefaults -chardev socket,id=charmonitor,path=/var/lib/libvirt/qemu/domain-61_trade04-win7/monitor.sock,server,nowait -mon chardev=charmonitor,id=monitor,mode=control -rtc base=localtime -no-shutdown -boot strict=on -device piix3-usb-uhci,id=usb,bus=pci.0,addr=0x1.0x2 -drive file=/z3/VRdisk3/trade/trade04-win7-sd0.img,format=qed,if=none,id=drive-virtio-disk0,cache=writeback -device virtio-blk-pci,scsi=off,bus=pci.0,addr=0x5,drive=drive-virtio-disk0,id=virtio-disk0,bootindex=1 -drive if=none,id=drive-ide0-1-0,readonly=on -device ide-cd,bus=ide.1,unit=0,drive=drive-ide0-1-0,id=ide0-1-0 -netdev tap,fd=27,id=hostnet0,vhost=on,vhostfd=29 -device virtio-net-pci,netdev=hostnet0,id=net0,mac=52:54:00:28:5f:1e,bus=pci.0,addr=0x3 -chardev pty,id=charserial0 -device isa-serial,chardev=charserial0,id=serial0 -device usb-tablet,id=input0 -vnc 127.0.0.1:1 -device VGA,id=video0,vgamem_mb=16,bus=pci.0,addr=0x2 -device virtio-balloon-pci,id=balloon0,bus=pci.0,addr=0x4 -msg timestamp=on"
        );
    }
    {
        let pidentry = &pidentries.pidentries[199];
        assert_eq_stat!(pidentry.stat => [7769 "mk-fixtures" b'R' 7762 7762 7539 34822 7762 4194560 147 0 0 0 1 2 0 0 20 0 1 0 14897830 17690624 435 18446744073709551615 94301772857344 94301773350360 140721373942944 140721373932824 139722310051072 0 0 4096 1088 0 0 0 17 0 0 0 0 0 0 94301775449120 94301775466784 94301808029696 140721373944137 140721373944151 140721373944151 140721373945834 0]);
        //
        assert_eq_statm!(pidentry.statm => [4319 435 401 121 0 1079 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "mk-fixtures");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'R');
        //
        assert_eq_status!(pidentry.status => tgid[7769 0]);
        assert_eq_status!(pidentry.status => pid[7769 7762]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[17276 17276 0 0 1740 1740 4184 132 484 2168 52 12 0]);
        assert_eq!(pidentry.cmdline.cmdline, "./mk-fixtures");
    }
}

#[test]
fn test_pidentries_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    let pidentries = sys.get_pidentries();
    //
    assert_eq!(pidentries.pidentries.len(), 334);
    //
    {
        let pidentry = &pidentries.pidentries[0];
        //
        assert_eq_stat!(pidentry.stat => [1 "systemd" b'S' 0 1 1 0 -1 4194560 53538 63545520 379 69307 1271 1320 1395843 455546 20 0 1 0 21 172408832 1961 18446744073709551615 94808232656896 94808233616997 140735348288944 0 0 0 671173123 4096 1260 1 0 0 17 2 0 0 82 0 0 94808233972624 94808234258472 94808237891584 140735348293396 140735348293414 140735348293414 140735348293613 0]);
        //
        assert_eq_statm!(pidentry.statm => [42092 1961 1154 235 0 5045 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "systemd");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[1 0]);
        assert_eq_status!(pidentry.status => pid[1 0]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[234164 168368 0 0 12684 7844 3228 4616 0 20048 132 940 8740 92 852]);
        //
        assert_eq!(pidentry.cmdline.cmdline, "/sbin/init splash");
    }
    //
    {
        let pidentry = &pidentries.pidentries[1];
        //
        assert_eq_stat!(pidentry.stat => [2 "kthreadd" b'S' 0 0 0 0 -1 2129984 0 0 0 0 0 10 0 0 20 0 1 0 21 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 1 0 0 0 3 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        //
        assert_eq_statm!(pidentry.statm => [0 0 0 0 0 0 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "kthreadd");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[2 0]);
        assert_eq_status!(pidentry.status => pid[2 0]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        assert_eq!(pidentry.cmdline.cmdline, "");
    }
    {
        let pidentry = &pidentries.pidentries[10];
        //
        assert_eq_stat!(pidentry.stat => [14 "cpuhp/0" b'S' 2 0 0 0 -1 69238848 0 0 0 0 0 0 0 0 20 0 1 0 32 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 1 0 0 17 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        //
        assert_eq_statm!(pidentry.statm => [0 0 0 0 0 0 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "cpuhp/0");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[14 0]);
        assert_eq_status!(pidentry.status => pid[14 2]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        assert_eq!(pidentry.cmdline.cmdline, "");
    }
    {
        let pidentry = &pidentries.pidentries[100];
        //
        assert_eq_stat!(pidentry.stat => [285 "jbd2/dm-0-8" b'S' 2 0 0 0 -1 2359360 0 0 0 0 0 201 0 0 20 0 1 0 277 0 0 18446744073709551615 0 0 0 0 0 0 0 2147483647 0 1 0 0 17 2 0 0 1542 0 0 0 0 0 0 0 0 0 0]);
        //
        assert_eq_statm!(pidentry.statm => [0 0 0 0 0 0 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "jbd2/dm-0-8");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[285 0]);
        assert_eq_status!(pidentry.status => pid[285 2]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
        assert_eq!(pidentry.cmdline.cmdline, "");
    }
    //
    {
        let pidentry = &pidentries.pidentries[200];
        //
        assert_eq_stat!(pidentry.stat => [215700 "xfwm4" b'S' 215515 215515 215515 0 -1 4194304 14396 0 1489 0 933989 241335 0 0 20 0 4 0 2584398 776728576 9528 18446744073709551615 93994658922496 93994659238725 140723627936688 0 0 0 0 4096 16899 0 0 0 17 3 0 0 96 0 0 93994659333232 93994659344752 93994663972864 140723627943936 140723627944010 140723627944010 140723627945961 0]);
        //
        assert_eq_statm!(pidentry.statm => [189631 9528 5957 78 0 17026 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "xfwm4");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[215700 0]);
        assert_eq_status!(pidentry.status => pid[215700 215515]);
        assert_eq_status!(pidentry.status => uid[1000 1000 1000 1000 1000 1000 1000 1000]);
        assert_eq_status!(pidentry.status => vm[761652 758524 0 0 94788 38112 14284 23816 12 67964 140 312 58868 484 17164]);
        assert_eq!(
            pidentry.cmdline.cmdline,
            "xfwm4 --display :0.0 --sm-client-id 248c23c55-4fab-429c-8e47-889860d515b9"
        );
    }
    {
        let pidentry = &pidentries.pidentries[333];
        //
        assert_eq_stat!(pidentry.stat => [740660 "make-test-fixtu" b'S' 740659 740659 730124 34828 740659 4194560 13636 251088 0 0 4 26 83 241 39 19 1 0 24466071 2670592 470 8589934592 94761686306816 94761686403061 140723342716544 0 0 0 0 0 65538 1 0 0 17 1 0 0 0 0 0 94761686433584 94761686438464 94761687474176 140723342718844 140723342718893 140723342718893 140723342720975 0]);
        //
        assert_eq_statm!(pidentry.statm => [652 470 439 24 0 80 0]);
        //
        #[cfg(feature = "has_pidentry_status_name")]
        assert_eq!(pidentry.status.name, "make-test-fixtu");
        #[cfg(feature = "has_pidentry_status_state")]
        assert_eq!(pidentry.status.state, b'S');
        //
        assert_eq_status!(pidentry.status => tgid[740660 0]);
        assert_eq_status!(pidentry.status => pid[740660 740659]);
        assert_eq_status!(pidentry.status => uid[0 0 0 0 0 0 0 0]);
        assert_eq_status!(pidentry.status => vm[2668 2608 0 0 1880 1880 124 1756 0 188 132 96 1628 40 0]);
        assert_eq!(
            pidentry.cmdline.cmdline,
            "/bin/sh ./scripts/make-test-fixtures-for-hmon.sh"
        );
    }
}
