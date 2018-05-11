extern crate pcap;
extern crate pnet_packet;

use pnet_packet::ethernet::EthernetPacket;
use pnet_packet::ipv4::Ipv4Packet;
use pnet_packet::tcp::TcpPacket;
use pnet_packet::Packet;


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
    // cap.filter("port 8000").unwrap();
    cap.filter("tcp port 8000 and (((ip[2:2] - ((ip[0]&0xf)<<2)) - ((tcp[12]&0xf0)>>2)) != 0)").unwrap();

    // get 10 packets
    for _ in 0..100 {
        let packet = cap.next().ok().unwrap();
        let frame = EthernetPacket::new(packet.data).unwrap();
        let ip = Ipv4Packet::new(frame.payload()).unwrap(); 
        let tcp = TcpPacket::new(ip.payload()).unwrap();
        let payload = tcp.payload();
        let data = String::from_utf8_lossy(payload);
        print!("{}\n", data);
        println!("=============================")
    }
    let stats = cap.stats().unwrap();
    println!("Received: {}, dropped: {}, if_dropped: {}", stats.received, stats.dropped, stats.if_dropped);
}
