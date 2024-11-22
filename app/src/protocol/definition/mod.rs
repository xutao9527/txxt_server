pub mod packet_request;
pub mod packet_response;



#[cfg(test)]
mod tests {
    use crate::protocol::definition::packet_request::PacketRequest;
    use crate::protocol::definition::packet_type::PacketRequestType;

    #[test]
    fn test_json() {
        let pkg_req = PacketRequest {
            packet_type: PacketRequestType::TestOne
        };
        let pkg_req = PacketRequest {
            packet_type: PacketRequestType::TestTwo
        };
        let pkg_str = serde_json::to_string(&pkg_req).expect("Failed to serialize to JSON");
        println!("{}",pkg_str);
    }
}