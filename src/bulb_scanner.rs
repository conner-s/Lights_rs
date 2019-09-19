mod bulb_scanner{
    use std::time;
    use ipaddress;
    use std::ops::Add;

    struct BulbScanner {
        found_bulbs: Vec<ipaddress::IPAddress>

    }

    impl BulbScanner {

        pub fn scan(timeout: u64) -> Vec<ipaddress::IPAddress>{
            let now = time::Instant::now();
            let mut found_bulbs: Vec<ipaddress::IPAddress> = [];

            while now < now.elapsed().as_secs().add(timeout){

            }
            found_bulbs
        }

    }
}
