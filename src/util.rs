pub trait DisplayBinary {
    fn display_bin(&self) -> Vec<String>;
}

pub trait Serializable: Sized {
    type Error;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(bytes: &[u8]) -> Result<Self, Self::Error>;
}