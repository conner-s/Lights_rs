use std::time;
use std::net;
use std::str;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn scan(timeout: time::Duration) -> net::Ipv4Addr {
    //Declaring return value to be able to keep it in scoop in the loop
    let mut ret_addr: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);


    //Sets discovery socket address to Broadcast IP and to the port from flux_LED python API
    let sock = net::UdpSocket::bind("0.0.0.0:48899")
        .expect("Socket did not bind");

    //Takes current time for timeout purposes
    let now = time::Instant::now();


    //Declares message to send to broadcast to LED controller
    let msg: String = "HF-A11ASSISTHREAD".to_string();
    let b_msg = msg.as_bytes();
    let mut buffer = [0; 64];

    //Broadcasts
    sock.send_to(b_msg, "192.168.0.255:48899")
        .expect("Could not send packet");

    //Setting timout for socket method
    sock.set_read_timeout(Option::from(timeout))
        .expect("Could not set timout");


    loop {
        //Timeout Break case
        if now.elapsed().as_secs() > timeout.as_secs() {
            break;
        }

        //Checking for packets to receive
        let (bytes_received, src_addr) = sock.recv_from(&mut buffer)
            .expect("Did not receive data gram");
        //unwrapping pack info to text
        let filled_buf = &mut buffer[..bytes_received];
        let buff_to_ascii = filled_buf.to_ascii_lowercase();
        let bulb_data = str::from_utf8(&buff_to_ascii).unwrap();

        //Checks to see if the packet is from the LED controller
        if !bulb_data.contains("hf-a11assisthread") {
            let chop = src_addr.ip().to_string();
            ret_addr = Ipv4Addr::from_str(&chop[0..])
                .expect("Could not convert to ipv4");
            break;
        }
    }
    ret_addr
}

#[cfg(test)]
mod tests {
    use super::*;
}
