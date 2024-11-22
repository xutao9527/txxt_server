use crate::protocol::definition::packet_request::PacketRequest;
use crate::protocol::definition::packet_type::PacketRequestType;

pub fn process(data: PacketRequest) {
    match data.packet_type {
        PacketRequestType::TestOne => {
            println!("{:?}", data.packet_type);
        }
        PacketRequestType::TestTwo => {
            println!("{:?}", data.packet_type);
        }
    }

}

