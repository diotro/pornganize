pub trait Model {
    const TREE_NAME: &'static str;
    fn get_key(&self) -> &str;
    fn to_bytes(self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Self;
}

#[cfg(feature = "protobuf-db")]
pub mod protobuf;
