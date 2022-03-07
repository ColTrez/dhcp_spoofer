use std::net::Ipv4Addr;
use dhcp4r::options::DhcpOption as options;
use dhcp4r::options::MessageType as MessageType;
use dhcp4r::options::REQUESTED_IP_ADDRESS as REQUESTED_IP_ADDRESS;
use dhcp4r::server;

pub struct Spoofer {
    address_to_assign : Ipv4Addr,
    lease_duration: u32,
    subnet_mask: Ipv4Addr,
}

impl Spoofer {
    pub fn new(cli_args: dhcp_spoofer::Args) -> Spoofer {
        Spoofer { address_to_assign: cli_args.address,
            lease_duration: cli_args.lease_duration,
            subnet_mask: cli_args.subnet }
    }
}

impl server::Handler for Spoofer {
    fn handle_request(&mut self, server: &server::Server, in_packet: dhcp4r::packet::Packet) {
        match in_packet.message_type() {
            Ok(MessageType::Discover) => {
                //assign the ip to the client
                //reply(server, options::MessageType::Offer, in_packet, self.address_to_assign)
                let result = server.reply(
                    MessageType::Offer,
                    vec![
                        options::IpAddressLeaseTime(self.lease_duration),
                        options::SubnetMask(self.subnet_mask),
                    ],
                    self.address_to_assign,
                    in_packet,
                );
                if let Err(_) = result {println!("Error handling Discover message")};
            }
            Ok(MessageType::Request) => {
                let requested_ip = match in_packet.option(REQUESTED_IP_ADDRESS) {
                    Some(options::RequestedIpAddress(request)) => *request,
                    _ => in_packet.ciaddr,
                };
                if requested_ip == self.address_to_assign {
                    let result = server.reply(
                        MessageType::Ack,
                        vec![
                            options::IpAddressLeaseTime(self.lease_duration),
                            options::SubnetMask(self.subnet_mask),
                        ],
                        self.address_to_assign,
                        in_packet);
                        if let Err(_) = result {println!("Error handling Request message for assigned ip")};
                }
                else {
                    //client has requested something other than what we're trying to assign
                    print!("The client requested ip {}, which is not what we're trying to assign. Assignment failed.", self.address_to_assign);
                    let result = server.reply(MessageType::Nak,vec![options::Message("That IP is not available.".into())], Ipv4Addr::new(0, 0, 0, 0), in_packet);
                    if let Err(_) = result {println!("Error handling Request message for an ip other than the assigned")};
                }
            },
            Ok(MessageType::Decline) => println!("The client has declined the assigned IP. Try a different IP."),
            _ => {}//this isnt a real dhcp server, so ignore the other messages

        }
    }
}