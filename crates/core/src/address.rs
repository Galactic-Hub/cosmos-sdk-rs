pub mod codec {
    // Codec defines an interface to convert addresses from and to string/bytes.
    pub trait Codec {
        /// StringToBytes decodes text to bytes
        fn string_to_bytes(&self, text: &str) -> Vec<u8>;
        /// BytesToString encodes bytes to text
        fn bytes_to_string(&self, bytes: &[u8]) -> String;
    }
}
