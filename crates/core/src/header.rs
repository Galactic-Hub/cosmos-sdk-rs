//! Package header defines a generalized Header type that all consensus & networking layers must provide.//!
//! If modules need access to the current block header information, like height, hash, time, or chain ID
//! they should use the Header Service interface.

pub mod service {
    /// Service defines the interface in which you can get header information
    pub trait Service {
        // GetHeaderInfo returns the current header information
        fn get_header_info(&self) -> Info;
    }

    /// Info defines a struct that contains information about the header
    pub struct Info {
        /// Height returns the height of the block
        pub height: i64,
        /// Hash returns the hash of the block header
        pub hash: Vec<u8>,
        /// Time returns the time of the block
        pub time: time::Time,
        /// ChainId returns the chain ID of the block
        pub chain_id: String,
        /// AppHash used in the current block header
        pub app_hash: Vec<u8>,
    }
}
