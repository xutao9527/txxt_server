pub mod packet_processor;


#[cfg(test)]
mod tests {
    use crate::protocol::definition::packet_request::PacketRequest;
    use crate::protocol::definition::packet_type::PacketRequestType;
    use crate::protocol::processor::packet_processor::process;

    #[test]
    fn test_process() {
        let pkg_req1 = PacketRequest {
            packet_type: PacketRequestType::TestOne
        };
        let pkg_req2 = PacketRequest {
            packet_type: PacketRequestType::TestTwo
        };
        //let pkg_str = serde_json::to_string(&pkg_req).expect("Failed to serialize to JSON");

        process(pkg_req1);
        process(pkg_req2);

    }
}