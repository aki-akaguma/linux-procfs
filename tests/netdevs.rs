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

macro_rules! assert_eq_net {
    (
        $net:expr =>
        [
            $name:tt
            $rx_bytes:tt
            $rx_packets:tt
            $rx_errors:tt
            $rx_dropped_errors:tt
            $rx_fifo_errors:tt
            $rx_frame_errors:tt
            $rx_compressed:tt
            $rx_multicast:tt
            $tx_bytes:tt
            $tx_packets:tt
            $tx_errors:tt
            $tx_dropped_errors:tt
            $tx_fifo_errors:tt
            $tx_collisions:tt
            $tx_carrier_errors:tt
            $tx_compressed:tt
        ]
    ) => {
        let net = &$net;
        //
        assert_eq!(net.name, $name);
        //
        assert_eq!(net.rx_bytes, $rx_bytes);
        assert_eq!(net.rx_packets, $rx_packets);
        //
        #[cfg(feature = "has_netdevs_rx_errors")]
        assert_eq!(net.rx_errors, $rx_errors);
        #[cfg(feature = "has_netdevs_rx_dropped_errors")]
        assert_eq!(net.rx_dropped_errors, $rx_dropped_errors);
        #[cfg(feature = "has_netdevs_rx_fifo_errors")]
        assert_eq!(net.rx_fifo_errors, $rx_fifo_errors);
        #[cfg(feature = "has_netdevs_rx_frame_errors")]
        assert_eq!(net.rx_frame_errors, $rx_frame_errors);
        #[cfg(feature = "has_netdevs_rx_compressed")]
        assert_eq!(net.rx_compressed, $rx_compressed);
        #[cfg(feature = "has_netdevs_rx_multicast")]
        assert_eq!(net.rx_multicast, $rx_multicast);
        //
        assert_eq!(net.tx_bytes, $tx_bytes);
        assert_eq!(net.tx_packets, $tx_packets);
        //
        #[cfg(feature = "has_netdevs_tx_errors")]
        assert_eq!(net.tx_errors, $tx_errors);
        #[cfg(feature = "has_netdevs_tx_dropped_errors")]
        assert_eq!(net.tx_dropped_errors, $tx_dropped_errors);
        #[cfg(feature = "has_netdevs_tx_fifo_errors")]
        assert_eq!(net.tx_fifo_errors, $tx_fifo_errors);
        #[cfg(feature = "has_netdevs_tx_collisions")]
        assert_eq!(net.tx_collisions, $tx_collisions);
        #[cfg(feature = "has_netdevs_tx_carrier_errors")]
        assert_eq!(net.tx_carrier_errors, $tx_carrier_errors);
        #[cfg(feature = "has_netdevs_tx_compressed")]
        assert_eq!(net.tx_compressed, $tx_compressed);
    };
}

#[test]
fn test_netdevs_intel() {
    let mut sys = System::new(base_path_intel!());
    let netdevs = sys.get_netdevs();
    //
    assert_eq_net!(netdevs.nets[0] => ["lo" 1410863506 8884513 0 0 0 0 0 0 1410863506 8884513 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[1] => ["br0" 32062511411 26266110 0 0 0 0 0 0 2730076473 25361128 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[2] => ["virbr1-nic" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[3] => ["virbr0" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[4] => ["lxcbr0" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[5] => ["virbr1" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[6] => ["virbr0-nic" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[7] => ["eth1" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[8] => ["eth0" 33254794169 38759405 0 0 0 0 0 154405 2730077812 25361139 0 0 0 0 0 0]);
}

#[test]
fn test_netdevs_amd() {
    let mut sys = System::new(base_path_amd!());
    let netdevs = sys.get_netdevs();
    //
    assert_eq_net!(netdevs.nets[0] => ["vnet0" 237094338 1175908 0 0 0 0 0 0 382166977 1974065 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[1] => ["lo" 57680 924 0 0 0 0 0 0 57680 924 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[2] => ["br0" 574285212 3614734 0 0 0 0 0 0 629000686 3436567 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[3] => ["enp17s0" 2185776085 17650017 0 0 0 0 0 32114 12698558527 19270189 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[4] => ["virbr1-nic" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[5] => ["virbr0" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[6] => ["lxcbr0" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[7] => ["virbr1" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[8] => ["virbr0-nic" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[9] => ["vnet2" 9978677976 11548340 0 0 0 0 0 0 804727354 8619162 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[10] => ["vnet1" 1751040521 3108230 0 0 0 0 0 0 331196556 3661596 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[11] => ["vethEK7AW3" 295600 1288 0 0 0 0 0 0 9727206 76440 0 0 0 0 0 0]);
}

#[test]
fn test_netdevs_5_4() {
    let mut sys = System::new(base_path_5_4!());
    let netdevs = sys.get_netdevs();
    //
    assert_eq_net!(netdevs.nets[0] => ["ens5" 0 0 0 0 0 0 0 0 3142 37 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[1] => ["br1" 528985 7047 0 0 0 0 0 0 532473 5666 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[2] => ["vethzx7pqX" 623539 6991 0 0 0 0 0 0 541885 5786 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[3] => ["ens4" 9338539 11990 0 0 0 0 0 0 9330290 12337 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[4] => ["lo" 426143 4806 0 0 0 0 0 0 426143 4806 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[5] => ["vethPtSnYb" 2249784248 1085473 0 0 0 0 0 0 2266776422 1703322 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[6] => ["br0" 2234612926 1085902 0 0 0 0 0 0 2266826679 1703672 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[7] => ["vethYwZWTv" 16402 277 0 0 0 0 0 0 38442 333 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[8] => ["ens3" 2347912002 2970311 0 0 0 0 0 0 2251619405 1101250 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[9] => ["vpn-sk1" 32460 258 0 0 0 0 0 0 22384 352 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[10] => ["vethOn03qG" 31306 429 0 0 0 0 0 0 60638 488 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[11] => ["lxcbr0" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
}

#[test]
fn test_netdevs_5_4_intel() {
    let mut sys = System::new(base_path_5_4_intel!());
    let netdevs = sys.get_netdevs();
    //
    assert_eq_net!(netdevs.nets[0] => ["lxcbr0" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[1] => ["br0" 25841502897 21068674 0 0 0 0 0 0 20378207183 24125372 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[2] => ["enp4s2" 26219232808 22322740 0 0 0 0 0 14128 20378204067 24125363 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[3] => ["virbr1-nic" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[4] => ["enp4s0" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[5] => ["virbr0" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[6] => ["lo" 12857270   16213 0 0 0 0 0 0 12857270   16213 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[7] => ["virbr1" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
    assert_eq_net!(netdevs.nets[8] => ["virbr0-nic" 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]);
}
