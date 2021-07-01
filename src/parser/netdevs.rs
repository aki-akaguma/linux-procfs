// linux source:
//   block/genhd.c >= v2.6.18
// https://elixir.bootlin.com/linux/v2.6.18/source/net/core/dev.c#L2108

use crate::netdevs::NetDev;
use crate::netdevs::NetDevs;
use crate::util::{find_to_pos, skip_to_pos};
use cfg_iif::cfg_iif;

#[allow(unused_imports)]
use crate::util::find_to_opt;

#[derive(Debug, Default, Clone)]
pub struct NetDevsParser();

impl NetDevsParser {
    pub fn parse(&mut self, sl: &[u8]) -> NetDevs {
        let mut netdevs = NetDevs::default();
        if sl.is_empty() {
            return netdevs;
        }
        //
        let mut pos1: usize = 0;
        let mut pos2: usize;
        let mut pos_end: usize;
        let mut idx: usize = 0;
        //
        // skip header 1
        let haystack = &sl[pos1..];
        let needle = b"\n";
        pos2 = pos1 + find_to_pos(haystack, needle);
        pos1 = pos2 + 1;
        // skip header 2
        let haystack = &sl[pos1..];
        let needle = b"\n";
        pos2 = pos1 + find_to_pos(haystack, needle);
        pos1 = pos2 + 1;
        //
        'net_loop: loop {
            if pos1 >= sl.len() {
                break 'net_loop;
            }
            let haystack = &sl[pos1..];
            let needle = b"\n";
            pos_end = pos1
                + 1
                + match find_to_opt(haystack, needle) {
                    Some(pos) => pos,
                    None => break 'net_loop,
                };
            //
            if idx >= netdevs.nets.len() {
                netdevs.nets.resize(idx + 1, NetDev::default());
            }
            let net_ref: &mut NetDev = match netdevs.nets.get_mut(idx) {
                Some(net) => net,
                None => unreachable!(),
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
                macro_rules! myscan {
                    (skip_spaces) => {{
                        pos1 += skip_to_pos(&sl[pos1..pos_end], b' ');
                    }};
                    (skip, $needle:expr) => {{
                        pos2 = {
                            let haystack = &sl[pos1..pos_end];
                            let needle = $needle;
                            pos1 + find_to_pos(haystack, needle)
                        };
                        let s = &sl[pos1..pos2];
                        pos1 = pos2 + 1;
                        s
                    }};
                    ($needle:expr) => {{
                        let s = myscan!(skip, $needle);
                        let input = String::from_utf8_lossy(s);
                        input.as_ref().parse().unwrap()
                    }};
                }
                //
                myscan!(skip_spaces);
                let s = myscan!(skip, b":");
                net_ref.name = String::from_utf8_lossy(s).into_owned();
                //
                if sl[pos1..pos_end].starts_with(b" No statistics available.") {
                    // nothing todo
                } else {
                    myscan!(skip_spaces);
                    {
                        net_ref.rx_bytes = myscan!(b" ");
                    }
                    myscan!(skip_spaces);
                    {
                        net_ref.rx_packets = myscan!(b" ");
                    }
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_rx_errors" {
                        net_ref.rx_errors = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_rx_dropped_errors" {
                        net_ref.rx_dropped_errors = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_rx_fifo_errors" {
                        net_ref.rx_fifo_errors = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_rx_frame_errors" {
                        net_ref.rx_frame_errors = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_rx_compressed" {
                        net_ref.rx_compressed = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_rx_multicast" {
                        net_ref.rx_multicast = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    //
                    myscan!(skip_spaces);
                    {
                        net_ref.tx_bytes = myscan!(b" ");
                    }
                    myscan!(skip_spaces);
                    {
                        net_ref.tx_packets = myscan!(b" ");
                    }
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_tx_errors" {
                        net_ref.tx_errors = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_tx_dropped_errors" {
                        net_ref.tx_dropped_errors = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_tx_fifo_errors" {
                        net_ref.tx_fifo_errors = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_tx_collisions" {
                        net_ref.tx_collisions = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_tx_carrier_errors" {
                        net_ref.tx_carrier_errors = myscan!(b" ");
                    } else {
                        myscan!(skip, b" ");
                    });
                    myscan!(skip_spaces);
                    cfg_iif!(feature = "has_netdevs_tx_compressed" {
                        net_ref.tx_compressed = myscan!(b"\n");
                    } else {
                        myscan!(skip, b"\n");
                    });
                    let _ = pos1;
                }
            }
            //
            idx += 1;
            pos1 = pos2 + 1;
        }
        netdevs.nets.resize(idx, NetDev::default());
        //
        netdevs
    }
}
