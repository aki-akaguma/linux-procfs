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

macro_rules! assert_eq_cpu {
    (
        $cpu:expr =>
        [
            $name:tt
            $user:tt
            $nice:tt
            $system:tt
            $idle:tt
            $iowait:tt
            $irq:tt
            $softirq:tt
            $steal:tt
            $quest:tt
            $guest_nice:tt
        ]
    ) => {
        assert_eq!($cpu.name, $name);
        assert_eq!($cpu.user, $user);
        assert_eq!($cpu.nice, $nice);
        assert_eq!($cpu.system, $system);
        assert_eq!($cpu.idle, $idle);
        assert_eq!($cpu.iowait, $iowait);
        assert_eq!($cpu.irq, $irq);
        assert_eq!($cpu.softirq, $softirq);
        assert_eq!($cpu.steal, $steal);
        assert_eq!($cpu.guest, $quest);
        assert_eq!($cpu.guest_nice, $guest_nice);
    };
}

#[test]
fn test_stat_intel() {
    let mut sys = System::new(base_path_intel!());
    let stat = sys.get_stat();
    //
    assert_eq_cpu!(stat.cpu => ["cpu" 13450128 1201947 2371263 166843529 846927 0 119211 0 0 0]);
    assert_eq_cpu!(stat.cpus[0] => ["cpu0" 3660732 216845 509911 41618446 231264 0 6742 0 0 0]);
    assert_eq_cpu!(stat.cpus[1] => ["cpu1" 2841663 323291 610689 42212197 136006 0 52944 0 0 0]);
    assert_eq_cpu!(stat.cpus[2] => ["cpu2" 3470335 323127 629365 41434691 290532 0 47304 0 0 0]);
    assert_eq_cpu!(stat.cpus[3] => ["cpu3" 3477397 338682 621295 41578195 189125 0 12219 0 0 0]);
    //
    assert_eq!(stat.ctxt, 2058923176);
    //
    #[cfg(feature = "has_stat_btime")]
    assert_eq!(stat.btime, 1527331745);
    //
    assert_eq!(stat.processes, 377914);
    //
    #[cfg(feature = "has_stat_procs_running")]
    assert_eq!(stat.procs_running, 1);
    //
    #[cfg(feature = "has_stat_procs_blocked")]
    assert_eq!(stat.procs_blocked, 0);
}

#[test]
fn test_stat_amd() {
    let mut sys = System::new(base_path_amd!());
    let stat = sys.get_stat();
    //
    assert_eq_cpu!(stat.cpu => ["cpu" 13357385 2432 2460563 42834808 342959 0 36466 0 11961063 0]);
    assert_eq_cpu!(stat.cpus[0] => ["cpu0" 3381999 448 616970 10688006 77220 0 3074 0 3040704 0]);
    assert_eq_cpu!(stat.cpus[1] => ["cpu1" 3324287 706 605184 10714660 71842 0 23652 0 2964194 0]);
    assert_eq_cpu!(stat.cpus[2] => ["cpu2" 3299165 550 617601 10731889 104207 0 6330 0 2948328 0]);
    assert_eq_cpu!(stat.cpus[3] => ["cpu3" 3351934 727 620806 10700252 89688 0 3409 0 3007836 0]);
    //
    assert_eq!(stat.ctxt, 1366164509);
    //
    #[cfg(feature = "has_stat_btime")]
    assert_eq!(stat.btime, 1527662995);
    //
    assert_eq!(stat.processes, 40022);
    //
    #[cfg(feature = "has_stat_procs_running")]
    assert_eq!(stat.procs_running, 2);
    //
    #[cfg(feature = "has_stat_procs_blocked")]
    assert_eq!(stat.procs_blocked, 0);
}

#[test]
fn test_stat_5_4() {
    let mut sys = System::new(base_path_5_4!());
    let stat = sys.get_stat();
    //
    assert_eq_cpu!(stat.cpu => ["cpu" 9289 834 7030 12789586 1391 0 2458 10622 0 0]);
    assert_eq_cpu!(stat.cpus[0] => ["cpu0" 4505 342 3770 6387284 908 0 2080 6843 0 0]);
    assert_eq_cpu!(stat.cpus[1] => ["cpu1" 4783 492 3259 6402301 482 0 378 3779 0 0]);
    //
    assert_eq!(stat.ctxt, 6501704);
    //
    #[cfg(feature = "has_stat_btime")]
    assert_eq!(stat.btime, 1591548333);
    //
    assert_eq!(stat.processes, 8563);
    //
    #[cfg(feature = "has_stat_procs_running")]
    assert_eq!(stat.procs_running, 1);
    //
    #[cfg(feature = "has_stat_procs_blocked")]
    assert_eq!(stat.procs_blocked, 0);
}

#[test]
fn test_stat_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    let stat = sys.get_stat();
    //
    assert_eq_cpu!(stat.cpu => ["cpu" 12717643 5463599 4106560 71307156 2056153 0 424001 0 0 0]);
    assert_eq_cpu!(stat.cpus[0] => ["cpu0" 3146141 1411073 1073853 17285153 1086180 0 28023 0 0 0]);
    assert_eq_cpu!(stat.cpus[1] => ["cpu1" 3170758 1336570 1007552 18255781 162099 0 83078 0 0 0]);
    assert_eq_cpu!(stat.cpus[2] => ["cpu2" 3362943 1356370 1010815 17850431 403320 0 16831 0 0 0]);
    assert_eq_cpu!(stat.cpus[3] => ["cpu3" 3037798 1359584 1014338 17915789 404553 0 296067 0 0 0]);
    //
    assert_eq!(stat.ctxt, 4115667701);
    //
    #[cfg(feature = "has_stat_btime")]
    assert_eq!(stat.btime, 1601988491);
    //
    assert_eq!(stat.processes, 742671);
    //
    #[cfg(feature = "has_stat_procs_running")]
    assert_eq!(stat.procs_running, 1);
    //
    #[cfg(feature = "has_stat_procs_blocked")]
    assert_eq!(stat.procs_blocked, 0);
}
