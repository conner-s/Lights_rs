use std::time;
use std::net;
use std::net::{Ipv4Addr};
use ipaddress::IPAddress;

mod bulb_scanner {
    use super::*;
    use std::str;


    struct BulbScanner {
        found_bulbs: Vec<ipaddress::IPAddress>

    }

    impl BulbScanner {

        pub fn scan(timeout: time::Duration) -> Vec<IPAddress> {
            let mut found_bulbs: Vec<IPAddress> = [].to_vec();

            //Sets discovery socket address to Broadcast IP and to the port from flux_LED python API
            let discovery_add = Ipv4Addr::BROADCAST.to_string() + "48899";

            let sock = net::UdpSocket::bind(&discovery_add)
                .expect("Broadcast socket did not bind");

            let now = time::Instant::now();


            let msg: String = "HF-A11ASSISTHREAD".to_string();

            let b_msg = msg.as_bytes();

            let mut post_scan_vec: Vec<String> = Vec::new();
            let mut buffer = [0; 64];
            while now.elapsed() < timeout {
                sock.send_to(b_msg, &discovery_add).expect("Could not send packet");
                let (bytes_received, src_addr) = sock.recv_from(&mut buffer)
                    .expect("Did not receive data gram");
                let filled_buf = &mut buffer[..bytes_received];
                let buff_to_ascii = filled_buf.to_ascii_lowercase();
                let data_split = str::from_utf8(&buff_to_ascii).unwrap().split(',');
                let data_vec = data_split.collect();
                post_scan_vec.push(data_vec);
            }

            for addr in post_scan_vec {
                let push_address= ipaddress::ipv4::new(addr)
                    .expect("could not parse address");
                found_bulbs.push(push_address);
            }
            found_bulbs
        }

    }
}
#[cfg(test)]
mod tests {
    use super::*;

}
