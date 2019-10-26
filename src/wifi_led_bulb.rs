mod wifi_led_bulb {
    use std::net::Ipv4Addr;
    use std::net;
    use std::ops::Add;

    struct WifiLEDBulb {
        ip_addr: Ipv4Addr,
        port: u16,
        timeout: u16,

        protocol: String,
        rgb_capable: bool,
        rgb_protocol: bool,


        raw_state: Vec< u16>,
        is_on: bool,
        mode: String,
        socket: net::UdpSocket,

        query_len: u8,
        use_checksum: bool,

    }

    impl WifiLEDBulb {


        ///Returns new instance of a bulb
        pub fn new(ip_addr: Ipv4Addr, port: u16) -> WifiLEDBulb{
            let socket_addr: String = ip_addr.to_string().add(String::from(port).as_str());
            let sock = net::UdpSocket::bind(socket_addr).unwrap();

            let mut new_bulb = WifiLEDBulb {ip_addr, port, timeout: 5, protocol: "".to_string(), rgb_capable: false, rgb_protocol: false, raw_state: vec![], is_on: false, mode: "".to_string(), socket: sock, query_len: 0, use_checksum: false };

            //Updates bulb after it is created
            new_bulb.update_state();

            return new_bulb
         }

        pub fn is_on(&self) -> bool{
            self.is_on
        }

        pub fn mode(&self) -> String{
            self.mode.to_string()
        }



        ///TODO Implement state update
        pub fn update_state(&self) {

        }
    }
}