// linux source:
//   block/genhd.c >= v2.6.18
// https://elixir.bootlin.com/linux/v2.6.18/source/net/core/dev.c#L2108

use crate::netdevs::NetDev;
use crate::netdevs::NetDevs;
use crate::scanner::ProcScanner;
use crate::ProcResult;
use cfg_iif::cfg_iif;

#[derive(Debug, Default, Clone)]
pub struct NetDevsParser();

impl NetDevsParser {
    pub fn parse(&mut self, sl: &[u8]) -> ProcResult<NetDevs> {
        let mut netdevs = NetDevs::default();
        if sl.is_empty() {
            return Ok(netdevs);
        }
        //
        let mut sc = ProcScanner::new(sl);
        let mut idx: usize = 0;
        //
        // skip header 1
        sc.scan_until(b'\n')?;
        // skip header 2
        sc.scan_until(b'\n')?;
        //
        while !sc.is_empty() {
            //
            if idx >= netdevs.nets.len() {
                netdevs.nets.resize(idx + 1, NetDev::default());
            }
            let net_ref: &mut NetDev = match netdevs.nets.get_mut(idx) {
                Some(net) => net,
                None => return Err(crate::ProcError::InternalError),
            };
            {
                // content of /proc/net/dev
                //   "   br0: 15826243374 23367319    0    0    0     0          0         0 3034115058 22744337    0    0    0     0       0          0\n"
                //   "virbr1-nic:       0       0    0    0    0     0          0         0        0       0    0    0    0     0       0          0\n"
                // on linux:
                //   "%6s:%8lu %7lu %4lu %4lu %4lu %5lu %10lu %9lu "
                //   "%8lu %7lu %4lu %4lu %4lu %5lu %7lu %10lu\n"
                //
                // from https://elixir.bootlin.com/linux/v2.6.18/source/net/core/dev.c
                //
                sc.skip_spaces();
                net_ref.name = sc.next(b':')?;
                //
                if sc.check(b'N') {
                    sc.scan_until(b'\n')?;
                } else {
                    sc.skip_spaces();
                    {
                        net_ref.rx_bytes = sc.next(b' ')?;
                    }
                    sc.skip_spaces();
                    {
                        net_ref.rx_packets = sc.next(b' ')?;
                    }
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_rx_errors" {
                        net_ref.rx_errors = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_rx_dropped_errors" {
                        net_ref.rx_dropped_errors = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_rx_fifo_errors" {
                        net_ref.rx_fifo_errors = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_rx_frame_errors" {
                        net_ref.rx_frame_errors = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_rx_compressed" {
                        net_ref.rx_compressed = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_rx_multicast" {
                        net_ref.rx_multicast = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    //
                    sc.skip_spaces();
                    {
                        net_ref.tx_bytes = sc.next(b' ')?;
                    }
                    sc.skip_spaces();
                    {
                        net_ref.tx_packets = sc.next(b' ')?;
                    }
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_tx_errors" {
                        net_ref.tx_errors = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_tx_dropped_errors" {
                        net_ref.tx_dropped_errors = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_tx_fifo_errors" {
                        net_ref.tx_fifo_errors = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_tx_collisions" {
                        net_ref.tx_collisions = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_tx_carrier_errors" {
                        net_ref.tx_carrier_errors = sc.next(b' ')?;
                    } else {
                        sc.scan_until(b' ')?;
                    });
                    sc.skip_spaces();
                    cfg_iif!(feature = "has_netdevs_tx_compressed" {
                        net_ref.tx_compressed = sc.next(b'\n')?;
                    } else {
                        sc.scan_until(b'\n')?;
                    });
                }
            }
            //
            idx += 1;
        }
        netdevs.nets.resize(idx, NetDev::default());
        //
        Ok(netdevs)
    }
}
