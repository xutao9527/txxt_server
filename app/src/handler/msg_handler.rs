use serde::Deserialize;

pub trait MsgHandler {
    type Payload: for<'de> Deserialize<'de> + Send + Sync; // Allow `Payload` to be deserialized for any lifetime `'de`
    fn handle(payload: Self::Payload);
}