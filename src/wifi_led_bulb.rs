mod wifi_led_bulb {
    struct WifiLEDBulb {
        //unknown IP implementation, setting as str for placehold
        ip_addr: String,
        port: String,
        timeout: u16,

        protocol: String,
        rgb_capable: bool,
        rgb_protocol: bool,


        raw_state: Vec<u16>,
        is_on: bool,
        mode: String,
        socket: String,
    }

    impl WifiLEDBulb {
        ///Returns new instance of a bulb

        //pub fn new(ip_addr: String) -> WifiLEDBulb{
        // }

        pub fn is_on(&self) -> bool{
            self.is_on
        }

        pub fn mode(&self) -> String{
            self.mode.to_string()
        }
    }
}