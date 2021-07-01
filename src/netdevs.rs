#[derive(Debug, Default, Clone)]
pub struct NetDev {
    pub name: String,
    //
    pub rx_bytes: u64,
    pub rx_packets: u64,
    #[cfg(feature = "has_netdevs_rx_errors")]
    pub rx_errors: u64,
    #[cfg(feature = "has_netdevs_rx_dropped_errors")]
    pub rx_dropped_errors: u64,
    #[cfg(feature = "has_netdevs_rx_fifo_errors")]
    pub rx_fifo_errors: u64,
    #[cfg(feature = "has_netdevs_rx_frame_errors")]
    pub rx_frame_errors: u64,
    #[cfg(feature = "has_netdevs_rx_compressed")]
    pub rx_compressed: u64,
    #[cfg(feature = "has_netdevs_rx_multicast")]
    pub rx_multicast: u64,
    //
    pub tx_bytes: u64,
    pub tx_packets: u64,
    #[cfg(feature = "has_netdevs_tx_errors")]
    pub tx_errors: u64,
    #[cfg(feature = "has_netdevs_tx_dropped_errors")]
    pub tx_dropped_errors: u64,
    #[cfg(feature = "has_netdevs_tx_fifo_errors")]
    pub tx_fifo_errors: u64,
    #[cfg(feature = "has_netdevs_tx_collisions")]
    pub tx_collisions: u64,
    #[cfg(feature = "has_netdevs_tx_carrier_errors")]
    pub tx_carrier_errors: u64,
    #[cfg(feature = "has_netdevs_tx_compressed")]
    pub tx_compressed: u64,
}

#[derive(Debug, Default, Clone)]
pub struct NetDevs {
    pub nets: Vec<NetDev>,
}
