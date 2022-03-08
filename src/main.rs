use std::net::{UdpSocket, Ipv4Addr};
use clap::Parser;

use crate::spoofer::Spoofer;

mod spoofer;

fn main() -> std::io::Result<()> {
    let args = dhcp_spoofer::Args::parse();

    println!("Spinning up phoney DHCP service to assign IP : {}", args.assign);
    println!("Opening socket to listen for DHCP Discover messages...");

    let socket = UdpSocket::bind("0.0.0.0:67")?;
    socket.set_broadcast(true)?;

    let spoofer = spoofer::Spoofer::new(args);

    let mut buffer = [0; 1000];
    let (num_bytes, src_addr) = socket.recv_from(&mut buffer).expect("couldnt read");
    let msg = Spoofer::decode_message(&buffer);
    
    Ok(())
}