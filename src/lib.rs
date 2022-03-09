use clap::Parser;
use std::net::Ipv4Addr;

use std::io;
use std::error::Error;
use dhcproto::v4::DecodeError;

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

#[derive(Debug)]
pub enum SpooferError {
    IO(io::Error),
    Decode(dhcproto::v4::DecodeError),
    NoDhcpType,
    OtherError(),
}

impl From<io::Error> for SpooferError {
    fn from(err: io::Error) -> SpooferError {
        SpooferError::IO(err)
    }
}

impl From<dhcproto::v4::DecodeError> for SpooferError {
    fn from(err: dhcproto::v4::DecodeError) -> SpooferError {
        SpooferError::Decode(err)
    }
}