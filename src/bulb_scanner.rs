use std::time;
use std::net;
use std::net::{Ipv4Addr};

mod bulb_scanner{
    use super::*;

    struct BulbScanner {
        found_bulbs: Vec<ipaddress::IPAddress>

    }

    impl BulbScanner {

        pub fn scan(timeout: time::Duration) -> Vec<Ipv4Addr>{

            //Sets discovery socket address to Broadcast IP and to the port from flux_LED python API
            let discoveryAdd = Ipv4Addr::BROADCAST.to_string() + "48899";

            let sock = net::UdpSocket::bind(discoveryAdd)
                .expect("Broadcast socket did not bind");

            let now = time::Instant::now();
            let mut found_bulbs: Vec<Ipv4Addr> = [].to_vec();

            let msg: str = HF-A11ASSISTHREAD;
            let b_msg = msg.as_bytes();


            while now.elapsed() < timeout{
                sock.send_to(b_msg, &discoveryAdd);

                loop{
                    let mut buffer = [0; 64];
                    let (bytes_received, src_addr) = sock.recv_from(&mut buffer);
                }
            }
            found_bulbs
        }

    }
}
#[cfg(test)]
mod tests{
    use super::*;

}
