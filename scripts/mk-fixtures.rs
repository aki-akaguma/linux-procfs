// rustc --out-dir target scripts/mk-fixtures.rs

use std::fs;
use std::path;

fn main() {
    mk_sys_cpu();
    mk_proc();
    mk_pids();
}

fn mk_sys_cpu() {
    let max_cpu_num = get_max_cpu_num();
    //
    for cpu_num in 0..max_cpu_num {
        let to0 = path::Path::new(&format!("fixtures/01/sys/devices/system/cpu/cpu{}/cpufreq", cpu_num)).to_path_buf();
        let _r = fs::create_dir_all(&to0).unwrap();
        let from0 = path::Path::new(&format!("/sys/devices/system/cpu/cpu{}/cpufreq", cpu_num)).to_path_buf();
        //
        let list = ["cpuinfo_cur_freq", "cpuinfo_max_freq"];
        for fnm in list.iter() {
            let from = from0.join(fnm);
            let to = to0.join(fnm);
            let _r = fs::copy(from, to).unwrap();
        }
        //
        let from1 = from0.join("stats");
        let to1 = to0.join("stats");
        let _r = fs::create_dir_all(&to1).unwrap();
        let list = ["time_in_state"];
        for fnm in list.iter() {
            let from = from1.join(fnm);
            let to = to1.join(fnm);
            let _r = fs::copy(from, to).unwrap();
        }
    }
}

fn mk_proc() {
    let to0 = path::Path::new("fixtures/01/proc").to_path_buf();
    let _r = fs::create_dir_all(&to0);
    let from0 = path::Path::new("/proc").to_path_buf();
    //
    let list = ["diskstats", "loadavg", "meminfo", "stat", "uptime", "vmstat"];
    for fnm in list.iter() {
        let from = from0.join(fnm);
        let to = to0.join(fnm);
        let _r = fs::copy(from, to).unwrap();
    }
    //
    let from1 = from0.join("net");
    let to1 = to0.join("net");
    let _r = fs::create_dir_all(&to1).unwrap();
    let list = ["dev"];
    for fnm in list.iter() {
        let from = from1.join(fnm);
        let to = to1.join(fnm);
        let _r = fs::copy(from, to).unwrap();
    }
}

fn mk_pids() {
    let pids = get_pids();
    //
    for pid in pids.iter() {
        let to0 = path::Path::new(&format!("fixtures/01/proc/{}", pid)).to_path_buf();
        let _r = fs::create_dir_all(&to0);
        let from0 = path::Path::new(&format!("/proc/{}", pid)).to_path_buf();
        //
        let list = ["stat", "statm", "status", "cmdline"];
        for fnm in list.iter() {
            let from = from0.join(fnm);
            let to = to0.join(fnm);
            let _r = fs::copy(from, to).unwrap();
        }
    }
}

fn get_pids() -> Vec<i32> {
    let mut v_pid = Vec::new();
    //
    for entry in fs::read_dir("/proc").unwrap() {
        let entry = entry.unwrap();
        let os_name = entry.file_name();
        let name = os_name.to_string_lossy();
        if let Some(_) = name.as_bytes().iter().position(|&b| b < b'0' || b > b'9') {
            continue;
        }
        let pid: i32 = name.parse().unwrap();
        v_pid.push(pid);
    }
    v_pid.sort();
    //
    v_pid
}

fn get_max_cpu_num() -> i32 {
    let mut max_num = 0;
    //
    for entry in fs::read_dir("/sys/devices/system/cpu").unwrap() {
        let entry = entry.unwrap();
        let os_name = entry.file_name();
        let name = os_name.to_string_lossy();
        if !name.starts_with("cpu") {
            continue;
        }
        if let Some(_) = name.as_bytes().iter().skip(3).position(|&b| b < b'0' || b > b'9') {
            continue;
        }
        max_num += 1;
    }
    max_num
}
