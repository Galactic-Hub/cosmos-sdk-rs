pub mod v1 {
    use crate::errors::Error;
    use ibc_proto::cosmos::crypto::hd::v1::Bip44Params as RawBip44Params;

    /// BIP44Params is used as path field in ledger item in Record.
    #[derive(Clone, PartialEq)]
    pub struct Bip44Params {
        /// purpose is a constant set to 44' (or 0x8000002C) following the BIP43 recommendation
        pub purpose: u32,
        /// coin_type is a constant that improves privacy
        pub coin_type: u32,
        /// account splits the key space into independent user identities
        pub account: u32,
        /// change is a constant used for public derivation. Constant 0 is used for external chain and constant 1 for internal
        /// chain.
        pub change: bool,
        /// address_index is used as child index in BIP32 derivation
        pub address_index: u32,
    }

    impl TryFrom<RawBip44Params> for Bip44Params {
        type Error = Error;

        fn try_from(raw: RawBip44Params) -> Result<Self, Self::Error> {
            Ok(Bip44Params {
                purpose: raw.purpose,
                coin_type: raw.coin_type,
                account: raw.account,
                change: raw.change,
                address_index: raw.address_index,
            })
        }
    }

    impl From<Bip44Params> for RawBip44Params {
        fn from(bip44: Bip44Params) -> Self {
            RawBip44Params {
                purpose: bip44.purpose,
                coin_type: bip44.coin_type,
                account: bip44.account,
                change: bip44.change,
                address_index: bip44.address_index,
            }
        }
    }
}
