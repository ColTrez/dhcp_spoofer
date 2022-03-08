use clap::Parser;
use std::net::Ipv4Addr;

#[derive(Parser, Debug)]
#[clap(author)]
pub struct Args {
    #[clap(short, long)]
    pub my_address: Ipv4Addr,
    #[clap(short, long)]
    pub assign: Ipv4Addr,
    #[clap(default_value_t = Ipv4Addr::new(255, 255, 255, 0), short, long)]
    pub subnet: Ipv4Addr,
    #[clap(default_value_t = u32::MAX, short, long)]
    pub lease_duration: u32,
}