use serde::Deserialize;

pub trait PacketHandler {
    type Payload: for<'de> Deserialize<'de> + Send + Sync;
    fn handle(payload: Self::Payload);
}