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

macro_rules! assert_eq_disk {
    (
        $disk:expr =>
        [
            $major_num:tt
            $minor_num:tt
            $name:tt
            $rio:tt
            $rmerge:tt
            $rblk:tt
            $ruse:tt
            $wio:tt
            $wmerge:tt
            $wblk:tt
            $wuse:tt
            $running:tt
            $use_:tt
            $aveq:tt
        ]
    ) => {
        let disk = &$disk;
        //
        #[cfg(feature = "has_diskstats_device_number")]
        assert_eq!(disk.major_num, $major_num);
        #[cfg(feature = "has_diskstats_device_number")]
        assert_eq!(disk.minor_num, $minor_num);
        //
        assert_eq!(disk.name, $name);
        assert_eq!(disk.rio, $rio);
        assert_eq!(disk.rmerge, $rmerge);
        assert_eq!(disk.rblk, $rblk);
        assert_eq!(disk.ruse, $ruse);
        assert_eq!(disk.wio, $wio);
        assert_eq!(disk.wmerge, $wmerge);
        assert_eq!(disk.wblk, $wblk);
        assert_eq!(disk.wuse, $wuse);
        //
        #[cfg(feature = "has_diskstats_running")]
        assert_eq!(disk.running, $running);
        #[cfg(feature = "has_diskstats_use")]
        assert_eq!(disk.use_, $use_);
        //
        assert_eq!(disk.aveq, $aveq);
    };
    (
        $disk:expr =>
        [
            $major_num:tt
            $minor_num:tt
            $name:tt
            $rio:tt
            $rmerge:tt
            $rblk:tt
            $ruse:tt
            $wio:tt
            $wmerge:tt
            $wblk:tt
            $wuse:tt
            $running:tt
            $use_:tt
            $aveq:tt
            $discards:tt
            $discardsmerge:tt
            $discardsblk:tt
            $discardsuse:tt
        ]
    ) => {
        assert_eq_disk!($disk => [$major_num $minor_num $name $rio $rmerge $rblk $ruse $wio $wmerge $wblk $wuse $running $use_ $aveq]);
    };
}

#[test]
fn test_diskstats_intel() {
    let mut sys = System::new(base_path_intel!());
    let diskstats = sys.get_diskstats();
    //
    assert_eq_disk!(diskstats.disks[0] => [7 0 "loop0" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[31] => [7 31 "loop31" 0 0 0 0 0 0 0 0 0 0 0]);
    //
    assert_eq_disk!(diskstats.disks[32] => [8 0 "sda" 1351124 1366720 48595982 6243054 1912435 2217615 151698656 87153961 0 8210969 93524121]);
    assert_eq_disk!(diskstats.disks[33] => [8 1 "sda1" 334419 224502 19822474 2620508 198063 178301 11351880 7868360 0 2318747 10498594]);
    assert_eq_disk!(diskstats.disks[34] => [8 2 "sda2" 953 3121 38480 4376 491 4335 38608 5887 0 6356 10259]);
    assert_eq_disk!(diskstats.disks[35] => [8 4 "sda4" 6 0 36 416 0 0 0 0 0 416 416]);
    assert_eq_disk!(diskstats.disks[36] => [8 5 "sda5" 1015692 1139097 28730848 3617116 1220773 2034979 140308168 78699061 0 6041620 82433436]);
    //
    assert_eq_disk!(diskstats.disks[37] => [11 0 "sr0" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[38] => [2 0 "fd0" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[39] => [8 16 "sdb" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[40] => [8 32 "sdc" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[41] => [8 48 "sdd" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[42] => [8 64 "sde" 0 0 0 0 0 0 0 0 0 0 0]);
}

#[test]
fn test_diskstats_amd() {
    let mut sys = System::new(base_path_amd!());
    let diskstats = sys.get_diskstats();
    //
    assert_eq_disk!(diskstats.disks[0] => [7 0 "loop0" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[31] => [7 31 "loop31" 0 0 0 0 0 0 0 0 0 0 0]);
    //
    assert_eq_disk!(diskstats.disks[32] => [8 0 "sda" 122467 111487 5480568 413956 16712 14586 2540186 862436 0 296680 1276372]);
    assert_eq_disk!(diskstats.disks[33] => [8 1 "sda1" 163 58 10618 3140 10 5 42 328 0 2980 3468]);
    assert_eq_disk!(diskstats.disks[34] => [8 2 "sda2" 6730 1663 226210 50884 909 1346 18048 18740 0 29308 69620]);
    assert_eq_disk!(diskstats.disks[35] => [8 3 "sda3" 18580 8785 713626 99432 868 1565 74936 25328 0 49988 124748]);
    assert_eq_disk!(diskstats.disks[36] => [8 4 "sda4" 3 0 6 232 0 0 0 0 0 232 232]);
    assert_eq_disk!(diskstats.disks[37] => [8 5 "sda5" 73 0 6504 1968 0 0 0 0 0 1656 1968]);
    assert_eq_disk!(diskstats.disks[38] => [8 6 "sda6" 14712 556 2622152 39560 8526 7855 2350680 766396 0 72032 805920]);
    assert_eq_disk!(diskstats.disks[39] => [8 7 "sda7" 10881 6259 520392 62116 6264 3787 96112 51000 0 67148 113112]);
    assert_eq_disk!(diskstats.disks[40] => [8 8 "sda8" 67978 90538 1292338 146896 8 2 80 96 0 119932 147028]);
    assert_eq_disk!(diskstats.disks[41] => [8 9 "sda9" 3264 3628 85266 8892 10 26 288 308 0 7772 9200]);
    assert_eq_disk!(diskstats.disks[42] => [8 32 "sdc" 510892 26315 46066746 5560808 935987 26034 54228224 11214772 0 5669356 16775216]);
    assert_eq_disk!(diskstats.disks[43] => [8 33 "sdc1" 510811 26315 46063058 5560672 561151 26034 54228224 6955252 0 3096688 12515496]);
    assert_eq_disk!(diskstats.disks[44] => [8 16 "sdb" 688 229 22418 42852 5 0 24 44 0 42640 42896]);
    assert_eq_disk!(diskstats.disks[45] => [8 17 "sdb1" 607 229 18730 42736 3 0 24 8 0 42488 42744]);
    //
    assert_eq_disk!(diskstats.disks[46] => [11 0 "sr0" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[47] => [8 48 "sdd" 344 0 17034 8344 5 0 24 100 0 8408 8444]);
    assert_eq_disk!(diskstats.disks[48] => [8 49 "sdd1" 313 0 14906 8304 3 0 24 40 0 8308 8344]);
    assert_eq_disk!(diskstats.disks[49] => [251 0 "zram0" 326 0 2608 0 227 0 1816 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[50] => [251 1 "zram1" 332 0 2656 0 239 0 1912 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[51] => [251 2 "zram2" 325 0 2600 0 223 0 1784 4 0 4 4]);
    assert_eq_disk!(diskstats.disks[52] => [251 3 "zram3" 332 0 2656 0 249 0 1992 4 0 4 4]);
}

#[test]
fn test_diskstats_5_4() {
    let mut sys = System::new(base_path_5_4!());
    let diskstats = sys.get_diskstats();
    //
    assert_eq_disk!(diskstats.disks[0] => [7 0 "loop0" 42 0 676 6 0 0 0 0 0 16 0]);
    assert_eq_disk!(diskstats.disks[31] => [7 31 "loop31" 0 0 0 0 0 0 0 0 0 0 0]);
    //
    assert_eq_disk!(diskstats.disks[32] => [11 0 "sr0" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[33] => [252 0 "vda" 17018 5433 1808850 7643 45019 16555 1665008 120203 0 41240 85300]);
    assert_eq_disk!(diskstats.disks[34] => [252 1 "vda1" 80 0 640 19 0 0 0 0 0 80 0]);
    assert_eq_disk!(diskstats.disks[35] => [252 2 "vda2" 16835 5433 1803682 7588 44678 16555 1665008 120093 0 41108 85284]);
}

#[test]
fn test_diskstats_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    let diskstats = sys.get_diskstats();
    //
    assert_eq_disk!(diskstats.disks[0] => [7 0 "loop0" 514 0 3326 388 0 0 0 0 0 237 350 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[15] => [7 15 "loop15" 0 0 0 0 0 0 0 0 0 0 0]);
    //
    assert_eq_disk!(diskstats.disks[16] => [8 0 "sda" 1216630 2162899 47845388 377918 117916 568658 11948368 78877 0 644078 105581 24 0 70336512 1290]);
    assert_eq_disk!(diskstats.disks[17] => [8 1 "sda1" 28 0 224 10 0 0 0 0 0 15 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[18] => [8 2 "sda2" 1216548 2162899 47842100 377882 107816 568658 11948368 72056 0 643921 104339 24 0 70336512 1290]);
    assert_eq_disk!(diskstats.disks[19] => [8 16 "sdb" 11460059 3144618 236280768 24092509 974696 4791235 128815456 4756499 0 5857710 25793982 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[20] => [8 17 "sdb1" 11086187 2334404 196841618 21934339 541757 4344642 68116064 2106205 0 5252221 21296637 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[21] => [8 20 "sdb4" 3 0 12 38 0 0 0 0 0 6 37 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[22] => [8 21 "sdb5" 373830 810214 39436290 2148419 420954 446593 60699392 2367123 0 640920 4210397 0 0 0 0]);
    //
    assert_eq_disk!(diskstats.disks[23] => [11 0 "sr0" 0 0 0 0 0 0 0 0 0 0 0]);
    //
    assert_eq_disk!(diskstats.disks[24] => [8 32 "sdc" 237422 269250 8040160 84541 281916 819040 23564712 156439 0 283390 81961 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[25] => [8 33 "sdc1" 28 0 224 7 0 0 0 0 0 21 1 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[26] => [8 34 "sdc2" 63 0 4960 31 0 0 0 0 0 39 9 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[27] => [8 35 "sdc3" 237283 269250 8032032 84488 262771 819040 23564712 143869 0 283049 78503 0 0 0 0]);
    //
    assert_eq_disk!(diskstats.disks[28] => [2 0 "fd0" 0 0 0 0 0 0 0 0 0 0 0]);
    //
    assert_eq_disk!(diskstats.disks[29] => [253 0 "dm-0" 1263557 0 30782434 806632 683430 0 12014672 1184232 0 440664 1990864 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[30] => [253 1 "dm-1" 508176 0 8027698 264428 1096966 0 23713128 1440559 0 282957 1704987 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[31] => [253 2 "dm-2" 13468842 0 196839394 40823058 4885492 0 68142752 39819429 0 5273386 80642487 0 0 0 0]);
    //
    assert_eq_disk!(diskstats.disks[32] => [8 80 "sdf" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[33] => [8 96 "sdg" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[34] => [8 64 "sde" 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_disk!(diskstats.disks[35] => [8 48 "sdd" 0 0 0 0 0 0 0 0 0 0 0]);
}
