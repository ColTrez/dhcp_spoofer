use dhcproto::v4 as dhcp;
use dhcp::{DecodeResult, Message, Decoder, Decodable, EncodeResult, Encoder, Encodable};
use log::info;
use std::net::Ipv4Addr;


pub struct Spoofer {
    assign : Ipv4Addr,
    lease_duration: u32,
    subnet_mask: Ipv4Addr,
}

impl Spoofer {
    pub fn new(cli_args: dhcp_spoofer::Args) -> Spoofer {
        Spoofer { assign: cli_args.assign,
            lease_duration: cli_args.lease_duration,
            subnet_mask: cli_args.subnet }
    }

    pub fn decode_message(msg : &[u8]) -> DecodeResult<dhcp::Message> {
        Message::decode(&mut Decoder::new(&msg))
    }
}
#[test]
fn test_decode(){
    pretty_env_logger::init();

    let result = Spoofer::decode_message(&discover_message_bytes().unwrap()).unwrap();

    info!("result: {:?}", result);

}

fn discover_message_bytes() -> EncodeResult<Vec<u8>> {
    // hardware addr
    let chaddr = vec![
        29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
    ];

    let mut msg = dhcp::Message::default();
    msg.set_flags(dhcp::Flags::default().set_broadcast())
        .set_chaddr(&chaddr)
        .opts_mut()
        .insert(dhcp::DhcpOption::MessageType(dhcp::MessageType::Discover));

    msg.opts_mut()
        .insert(dhcp::DhcpOption::ParameterRequestList(vec![
            dhcp::OptionCode::SubnetMask,
            dhcp::OptionCode::Router,
            dhcp::OptionCode::DomainNameServer,
            dhcp::OptionCode::DomainName,
        ]));

    msg.opts_mut()
        .insert(dhcp::DhcpOption::ClientIdentifier(chaddr));

    // now encode to bytes
    let mut buf = Vec::new();
    let mut e = Encoder::new(&mut buf);
    msg.encode(&mut e)?;

    Ok(buf)
}
