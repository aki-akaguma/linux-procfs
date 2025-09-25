use linux_procfs::System;

macro_rules! base_path_netdevs_zero {
    () => {
        "fixtures/test-netdevs-zero"
    };
}

macro_rules! base_path_netdevs_large {
    () => {
        "fixtures/test-netdevs-large"
    };
}

macro_rules! base_path_netdevs_no_stats {
    () => {
        "fixtures/test-netdevs-no-stats"
    };
}

#[test]
fn test_netdevs_zero() {
    let mut sys = System::new(base_path_netdevs_zero!());
    let netdevs = sys.get_netdevs();

    assert_eq!(netdevs.nets.len(), 1);
    let net = &netdevs.nets[0];

    assert_eq!(net.name, "lo");
    assert_eq!(net.rx_bytes, 0);
    assert_eq!(net.rx_packets, 0);
    #[cfg(feature = "has_netdevs_rx_errors")]
    assert_eq!(net.rx_errors, 0);
    #[cfg(feature = "has_netdevs_rx_dropped_errors")]
    assert_eq!(net.rx_dropped_errors, 0);
    #[cfg(feature = "has_netdevs_rx_fifo_errors")]
    assert_eq!(net.rx_fifo_errors, 0);
    #[cfg(feature = "has_netdevs_rx_frame_errors")]
    assert_eq!(net.rx_frame_errors, 0);
    #[cfg(feature = "has_netdevs_rx_compressed")]
    assert_eq!(net.rx_compressed, 0);
    #[cfg(feature = "has_netdevs_rx_multicast")]
    assert_eq!(net.rx_multicast, 0);
    assert_eq!(net.tx_bytes, 0);
    assert_eq!(net.tx_packets, 0);
    #[cfg(feature = "has_netdevs_tx_errors")]
    assert_eq!(net.tx_errors, 0);
    #[cfg(feature = "has_netdevs_tx_dropped_errors")]
    assert_eq!(net.tx_dropped_errors, 0);
    #[cfg(feature = "has_netdevs_tx_fifo_errors")]
    assert_eq!(net.tx_fifo_errors, 0);
    #[cfg(feature = "has_netdevs_tx_collisions")]
    assert_eq!(net.tx_collisions, 0);
    #[cfg(feature = "has_netdevs_tx_carrier_errors")]
    assert_eq!(net.tx_carrier_errors, 0);
    #[cfg(feature = "has_netdevs_tx_compressed")]
    assert_eq!(net.tx_compressed, 0);
}

#[test]
fn test_netdevs_large() {
    let mut sys = System::new(base_path_netdevs_large!());
    let netdevs = sys.get_netdevs();

    assert_eq!(netdevs.nets.len(), 1);
    let net = &netdevs.nets[0];

    assert_eq!(net.name, "large");
    assert_eq!(net.rx_bytes, 999999999999999999);
    assert_eq!(net.rx_packets, 999999999999999999);
    #[cfg(feature = "has_netdevs_rx_errors")]
    assert_eq!(net.rx_errors, 999999999999999999);
    #[cfg(feature = "has_netdevs_rx_dropped_errors")]
    assert_eq!(net.rx_dropped_errors, 999999999999999999);
    #[cfg(feature = "has_netdevs_rx_fifo_errors")]
    assert_eq!(net.rx_fifo_errors, 999999999999999999);
    #[cfg(feature = "has_netdevs_rx_frame_errors")]
    assert_eq!(net.rx_frame_errors, 999999999999999999);
    #[cfg(feature = "has_netdevs_rx_compressed")]
    assert_eq!(net.rx_compressed, 999999999999999999);
    #[cfg(feature = "has_netdevs_rx_multicast")]
    assert_eq!(net.rx_multicast, 999999999999999999);
    assert_eq!(net.tx_bytes, 999999999999999999);
    assert_eq!(net.tx_packets, 999999999999999999);
    #[cfg(feature = "has_netdevs_tx_errors")]
    assert_eq!(net.tx_errors, 999999999999999999);
    #[cfg(feature = "has_netdevs_tx_dropped_errors")]
    assert_eq!(net.tx_dropped_errors, 999999999999999999);
    #[cfg(feature = "has_netdevs_tx_fifo_errors")]
    assert_eq!(net.tx_fifo_errors, 999999999999999999);
    #[cfg(feature = "has_netdevs_tx_collisions")]
    assert_eq!(net.tx_collisions, 999999999999999999);
    #[cfg(feature = "has_netdevs_tx_carrier_errors")]
    assert_eq!(net.tx_carrier_errors, 999999999999999999);
    #[cfg(feature = "has_netdevs_tx_compressed")]
    assert_eq!(net.tx_compressed, 999999999999999999);
}

#[test]
fn test_netdevs_no_stats() {
    let mut sys = System::new(base_path_netdevs_no_stats!());
    let netdevs = sys.get_netdevs();

    assert_eq!(netdevs.nets.len(), 1);
    let net = &netdevs.nets[0];

    assert_eq!(net.name, "none");
    assert_eq!(net.rx_bytes, 0);
    assert_eq!(net.rx_packets, 0);
    #[cfg(feature = "has_netdevs_rx_errors")]
    assert_eq!(net.rx_errors, 0);
    #[cfg(feature = "has_netdevs_rx_dropped_errors")]
    assert_eq!(net.rx_dropped_errors, 0);
    #[cfg(feature = "has_netdevs_rx_fifo_errors")]
    assert_eq!(net.rx_fifo_errors, 0);
    #[cfg(feature = "has_netdevs_rx_frame_errors")]
    assert_eq!(net.rx_frame_errors, 0);
    #[cfg(feature = "has_netdevs_rx_compressed")]
    assert_eq!(net.rx_compressed, 0);
    #[cfg(feature = "has_netdevs_rx_multicast")]
    assert_eq!(net.rx_multicast, 0);
    assert_eq!(net.tx_bytes, 0);
    assert_eq!(net.tx_packets, 0);
    #[cfg(feature = "has_netdevs_tx_errors")]
    assert_eq!(net.tx_errors, 0);
    #[cfg(feature = "has_netdevs_tx_dropped_errors")]
    assert_eq!(net.tx_dropped_errors, 0);
    #[cfg(feature = "has_netdevs_tx_fifo_errors")]
    assert_eq!(net.tx_fifo_errors, 0);
    #[cfg(feature = "has_netdevs_tx_collisions")]
    assert_eq!(net.tx_collisions, 0);
    #[cfg(feature = "has_netdevs_tx_carrier_errors")]
    assert_eq!(net.tx_carrier_errors, 0);
    #[cfg(feature = "has_netdevs_tx_compressed")]
    assert_eq!(net.tx_compressed, 0);
}
