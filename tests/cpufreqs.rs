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

macro_rules! assert_eq_time_in_state {
    ($tis:expr => [$step:tt $value:tt]) => {
        let tis = &$tis;
        assert_eq!(tis.step, $step);
        assert_eq!(tis.value, $value);
    };
}

#[test]
fn test_cpufreqs_intel() {
    let mut sys = System::new(base_path_intel!());
    let max_cpu_num = sys.get_max_cpu_num();
    assert_eq!(max_cpu_num, 4);
    let cpufreqs = sys.get_cpufreqs(max_cpu_num);
    //
    assert_eq!(cpufreqs.cpufreqs[0].cur, 1870000);
    assert_eq!(cpufreqs.cpufreqs[0].max, 2403000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[0] => [2403000 3246210]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[1] => [2136000 2535951]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[2] => [1870000 7527266]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[3] => [1603000 33113100]);
    //
    assert_eq!(cpufreqs.cpufreqs[1].cur, 1870000);
    assert_eq!(cpufreqs.cpufreqs[1].max, 2403000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[0] => [2403000 3619917]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[1] => [2136000 1612091]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[2] => [1870000 5545859]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[3] => [1603000 35644660]);
    //
    assert_eq!(cpufreqs.cpufreqs[2].cur, 2136000);
    assert_eq!(cpufreqs.cpufreqs[2].max, 2403000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[0] => [2403000 4117556]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[1] => [2136000 1840436]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[2] => [1870000 6053471]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[3] => [1603000 34411063]);
    //
    assert_eq!(cpufreqs.cpufreqs[3].cur, 1603000);
    assert_eq!(cpufreqs.cpufreqs[3].max, 2403000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[0] => [2403000 4146582]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[1] => [2136000 1959969]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[2] => [1870000 6282662]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[3] => [1603000 34033314]);
}

#[test]
fn test_cpufreqs_amd() {
    let mut sys = System::new(base_path_amd!());
    let max_cpu_num = sys.get_max_cpu_num();
    assert_eq!(max_cpu_num, 4);
    let cpufreqs = sys.get_cpufreqs(max_cpu_num);
    //
    assert_eq!(cpufreqs.cpufreqs[0].cur, 2600000);
    assert_eq!(cpufreqs.cpufreqs[0].max, 2600000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[0] => [2600000 3859247]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[1] => [1900000 2383908]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[2] => [1400000 5079736]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[3] => [800000 3574662]);
    //
    assert_eq!(cpufreqs.cpufreqs[1].cur, 1400000);
    assert_eq!(cpufreqs.cpufreqs[1].max, 2600000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[0] => [2600000 3578210]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[1] => [1900000 2328768]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[2] => [1400000 5081434]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[3] => [800000 3909139]);
    //
    assert_eq!(cpufreqs.cpufreqs[2].cur, 800000);
    assert_eq!(cpufreqs.cpufreqs[2].max, 2600000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[0] => [2600000 3798695]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[1] => [1900000 2318342]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[2] => [1400000 5070897]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[3] => [800000 3709618]);
    //
    assert_eq!(cpufreqs.cpufreqs[3].cur, 1400000);
    assert_eq!(cpufreqs.cpufreqs[3].max, 2600000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[0] => [2600000 3763479]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[1] => [1900000 2354890]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[2] => [1400000 5101863]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[3] => [800000 3677320]);
}

#[test]
fn test_cpufreqs_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    let max_cpu_num = sys.get_max_cpu_num();
    assert_eq!(max_cpu_num, 4);
    let cpufreqs = sys.get_cpufreqs(max_cpu_num);
    //
    assert_eq!(cpufreqs.cpufreqs[0].cur, 2403000);
    assert_eq!(cpufreqs.cpufreqs[0].max, 2403000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[0] => [2403000 2606190]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[1] => [2136000 2022157]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[2] => [1870000 9230453]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[0].time_in_states[3] => [1603000 10607598]);
    //
    assert_eq!(cpufreqs.cpufreqs[1].cur, 1870000);
    assert_eq!(cpufreqs.cpufreqs[1].max, 2403000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[0] => [2403000 1401483]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[1] => [2136000 2025987]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[2] => [1870000 9639583]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[1].time_in_states[3] => [1603000 11399346]);
    //
    assert_eq!(cpufreqs.cpufreqs[2].cur, 1870000);
    assert_eq!(cpufreqs.cpufreqs[2].max, 2403000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[0] => [2403000 1518462]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[1] => [2136000 2292317]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[2] => [1870000 10271074]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[2].time_in_states[3] => [1603000 10384547]);
    //
    assert_eq!(cpufreqs.cpufreqs[3].cur, 2136000);
    assert_eq!(cpufreqs.cpufreqs[3].max, 2403000);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[0] => [2403000 1547880]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[1] => [2136000 2234251]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[2] => [1870000 9946724]);
    assert_eq_time_in_state!(cpufreqs.cpufreqs[3].time_in_states[3] => [1603000 10737548]);
}
