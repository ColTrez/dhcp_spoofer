use std::net::{UdpSocket, Ipv4Addr};
use clap::Parser;
use dhcp4r::server;

mod spoofer;

const SERVER_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 0, 76);//this is the actual ip the server will to

fn main() -> std::io::Result<()> {
    let args = dhcp_spoofer::Args::parse();

    println!("Spinning up phoney DHCP service to assign IP : {}", args.address);
    println!("Opening socket to listen for DHCP Discover messages...");

    let socket = UdpSocket::bind("0.0.0.0:67")?;
    socket.set_broadcast(true)?;

    let spoofer = spoofer::Spoofer::new(args);

    server::Server::serve(socket, SERVER_IP, spoofer);

    Ok(())
}