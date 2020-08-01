
use std::sync::mpsc::{channel, Receiver, Sender};

mod packet_sniffer;

use packet_sniffer::UdpPacket;

fn main() {
    if let Ok(interfaces) = packet_sniffer::network_interfaces() {
        let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = channel();
        packet_sniffer::receive(interfaces, tx);


        loop {
            if let Ok(packet) = rx.recv() {
                println!("Received packet: {:?}", &packet);
            }
        }
    }

}