use clap::{App, Arg};
use std::sync::mpsc::{channel, Receiver, Sender};

extern crate serde_json;

mod photon_decode;
mod packet_sniffer;
use packet_sniffer::UdpPacket;

use aoaddons::game::World;
use aoaddons::photon_decode::Photon;
use aoaddons::translate::raw_to_photon_messages;

fn main() {
    let matches = App::new("Testing utility")
        .about("Debug and develop new features")
        .subcommand(
            App::new("sniff")
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
        )
        .subcommand(
            App::new("test")
            .about("Returns list of decoded game events from given payload")
            .arg(
                Arg::with_name("payload")
                    .help("Byte array payload")
                    .index(1)
                    .default_value("[[58, 3, 0, 1, 0, 78, 158, 217, 96, 40, 29, 110, 6, 0, 1, 4, 0, 0, 0, 28, 0, 0, 0, 89, 243, 2, 1, 0, 2, 0, 115, 0, 3, 97, 97, 97, 253, 107, 0, 188]]")
                    .required(true)
            )
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("sniff") {
        sniff(
            matches.value_of("MODE").unwrap(), 
            if let Some(port) = matches.value_of("port") {Some(port.parse::<u16>().unwrap())} else { None });
    }

    if let Some(ref matches) = matches.subcommand_matches("test") {
        let payloads : Vec<Vec<u8>> = serde_json::from_str(matches.value_of("payload").unwrap()).unwrap();
        game_events(&payloads);
    }
}

fn sniff(mode: &str, port: Option<u16>)
{
    if let Ok(interfaces) = packet_sniffer::network_interfaces() {
        let (tx, rx): (Sender<UdpPacket>, Receiver<UdpPacket>) = channel();
        packet_sniffer::receive(interfaces, tx);

        loop {
            if let Ok(packet) = rx.recv() {

                if let Some(port) = port {
                    if packet.destination_port != port && packet.source_port != port {
                        continue;
                    }
                };

                match mode {
                    "raw" => {
                        println!("[RAW] {:?}", packet);
                    }
                    "decoding" => {
                        let mut photon = photon_decode::Photon::new();
                        println!("[RAW] {:?}", packet);
                        photon
                            .try_decode(&packet.payload)
                            .into_iter()
                            .for_each(|p| println!("[DECODING] {:?}", p));
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn game_events(payloads: &Vec<Vec<u8>>) {
    let mut photon = Photon::new();
    let mut world = World::new();

    for payload in payloads {
        println!("[PAYLOAD] {:?}", payload);

        photon
            .try_decode(&payload)
            .into_iter()
            .for_each(|p| println!("[DECODING] {:?}", p));

        raw_to_photon_messages(&mut photon, &payload)
        .into_iter()
        .inspect(|i| println!("[GameMessage] {:?}", i))
        .map(|message| world.transform(message))
        .flatten()
        .flatten()
        .for_each(|e| println!("[GameEvent] {:?}", e));
    }
}