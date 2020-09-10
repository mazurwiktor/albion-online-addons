use clap::{App, Arg};
use std::sync::mpsc::{channel, Receiver, Sender};

mod photon_decode;

mod packet_sniffer;

use packet_sniffer::UdpPacket;

fn main() {
    let matches = App::new("Testing utility")
        .about("Debug and develop new features")
        .arg(
            Arg::with_name("MODE")
                .help("What mode to run the program in")
                .index(1)
                .possible_values(&["raw", "decoding"])
                .default_value("raw")
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .help("UDP packets port")
        )
        .get_matches();

    if let Ok(interfaces) = packet_sniffer::network_interfaces() {
        let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = channel();
        packet_sniffer::receive(interfaces, tx);

        loop {
            if let Ok(packet) = rx.recv() {

                if let Some(port) = matches.value_of("port") {
                    let port_number = port.parse::<u16>().unwrap();

                    if packet.destination_port != port_number && packet.source_port != port_number {
                        continue;
                    }
                };

                match matches.value_of("MODE").unwrap() {
                    "raw" => {
                        println!("{:?}", packet);
                    }
                    "decoding" => {
                        let mut photon = photon_decode::Photon::new();

                        photon
                            .try_decode(&packet.payload)
                            .into_iter()
                            .for_each(|p| println!("{:?}", p));
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
