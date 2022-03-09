use std::net::{UdpSocket, Ipv4Addr};
use clap::Parser;

use crate::spoofer::Spoofer;
use dhcp_spoofer::SpooferError;

mod spoofer;

fn main() ->  Result<(), SpooferError>{
    let args = dhcp_spoofer::Args::parse();

    println!("Spinning up phoney DHCP service to assign IP : {}", args.assign);
    println!("Opening socket to listen for DHCP Discover messages...");

    let spoofer = spoofer::Spoofer::new(args);

    spoofer.spoof()
}