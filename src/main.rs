use std::net::{IpAddr, Ipv6Addr};
use std::time::Instant;

use pnet::packet::icmpv6::Icmpv6Types::{EchoReply, EchoRequest};
use pnet::packet::icmpv6::MutableIcmpv6Packet;
use pnet::packet::ip::IpNextHeaderProtocols::Icmpv6;
use pnet::transport::icmpv6_packet_iter;
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::{transport_channel, TransportProtocol::Ipv6};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    destination: String,
}

fn main() {
    let args = Args::parse();

    // get channel
    let (mut tx, mut rx) = transport_channel(5000, Layer4(Ipv6(Icmpv6))).unwrap();

    // build packet
    let mut icmp_buf = [0; 8];
    let mut icmp = MutableIcmpv6Packet::new(&mut icmp_buf).unwrap();

    icmp.set_icmpv6_type(EchoRequest);

    // create destination address
    let dest: Ipv6Addr = args.destination.parse().unwrap();
    let dest = IpAddr::from(dest);

    // start time measurment
    let start = Instant::now();

    // send packet
    let _ = tx.send_to(icmp, dest);

    // read response
    loop {
        if let Ok(p) = icmpv6_packet_iter(&mut rx).next() {
            if p.0.get_icmpv6_type() == EchoReply {
                break;
            }
        }
    }

    // measure duration
    let duration = start.elapsed();
    println!("{duration:?}");
}

