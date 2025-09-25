use linux_procfs::System;

macro_rules! base_path_cpufreqs_zero {
    () => {
        "fixtures/test-cpufreqs-zero"
    };
}

#[test]
fn test_cpufreqs_zero() {
    let mut sys = System::new(base_path_cpufreqs_zero!());
    let max_cpu_num = sys.get_max_cpu_num();
    assert_eq!(max_cpu_num, 1); // Only cpu0 exists in this fixture
    let cpufreqs = sys.get_cpufreqs(max_cpu_num);

    assert_eq!(cpufreqs.cpufreqs[0].cur, 0);
    assert_eq!(cpufreqs.cpufreqs[0].max, 0);
    assert!(cpufreqs.cpufreqs[0].time_in_states.is_empty());
}
