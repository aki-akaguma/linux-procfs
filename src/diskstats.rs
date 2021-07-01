#[derive(Debug, Default, Clone)]
pub struct DiskStat {
    #[cfg(feature = "has_diskstats_device_number")]
    pub major_num: i32,
    #[cfg(feature = "has_diskstats_device_number")]
    pub minor_num: i32,
    //
    pub name: String,
    pub rio: u64,
    pub rmerge: u64,
    pub rblk: u64,
    pub ruse: u32,
    pub wio: u64,
    pub wmerge: u64,
    pub wblk: u64,
    pub wuse: u32,
    //
    #[cfg(feature = "has_diskstats_running")]
    pub running: u32,
    #[cfg(feature = "has_diskstats_use")]
    pub use_: u32,
    //
    pub aveq: u32,
}

#[derive(Debug, Default, Clone)]
pub struct DiskStats {
    pub disks: Vec<DiskStat>,
}
