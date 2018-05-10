extern crate pcap;


fn main() {
    for device in pcap::Device::list().unwrap() {
        println!("Interface: {:?}", device);

        // now you can create a Capture with this Device if you want.
        // let mut cap = device.open().unwrap();

        // get a packet from this capture
        // let packet = cap.next();

        // println!("got a packet! {:?}", packet);
    }

    // get the default Device
    // let mut cap = pcap::Device::lookup().unwrap().open().unwrap();
    let mut cap = pcap::Capture::from_device("lo").unwrap().open().unwrap();
    cap.filter("port 8000").unwrap();

    // get 10 packets
    for _ in 0..100 {
        let packet = cap.next().ok();
        let data = String::from_utf8_lossy(packet.unwrap().data);
        println!("{}", data);
    }
    let stats = cap.stats().unwrap();
    println!("Received: {}, dropped: {}, if_dropped: {}", stats.received, stats.dropped, stats.if_dropped);
}
