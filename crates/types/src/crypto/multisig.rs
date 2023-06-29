use crate::errors::Error;
use ibc_proto::cosmos::crypto::multisig::LegacyAminoPubKey as RawLegacyAminoPubKey;
use ibc_proto::google::protobuf::Any;

/// LegacyAminoPubKey specifies a public key type
/// which nests multiple public keys and a threshold,
/// it uses legacy amino address rules.
#[derive(Clone, PartialEq)]
pub struct LegacyAminoPubKey {
    pub threshold: u32,
    pub public_keys: Vec<Any>,
}

impl TryFrom<RawLegacyAminoPubKey> for LegacyAminoPubKey {
    type Error = Error;

    fn try_from(value: RawLegacyAminoPubKey) -> Result<Self, Self::Error> {
        let threshold = value.threshold;
        let public_keys = value.public_keys;

        Ok(Self {
            threshold,
            public_keys,
        })
    }
}

impl From<LegacyAminoPubKey> for RawLegacyAminoPubKey {
    fn from(value: LegacyAminoPubKey) -> Self {
        Self {
            threshold: value.threshold,
            public_keys: value.public_keys,
        }
    }
}

pub mod v1beta1 {
    use crate::errors::Error;
    use ibc_proto::cosmos::crypto::multisig::v1beta1::CompactBitArray as RawCompactBitArray;
    use ibc_proto::cosmos::crypto::multisig::v1beta1::MultiSignature as RawMultiSignature;

    /// MultiSignature wraps the signatures from a multisig.LegacyAminoPubKey.
    /// See cosmos.tx.v1betata1.ModeInfo.Multi for how to specify which signers
    /// signed and with which modes.
    #[derive(Clone, PartialEq)]
    pub struct MultiSignature {
        pub signatures: Vec<Vec<u8>>,
    }

    impl TryFrom<RawMultiSignature> for MultiSignature {
        type Error = Error;

        fn try_from(value: RawMultiSignature) -> Result<Self, Self::Error> {
            let signatures = value.signatures;

            Ok(Self { signatures })
        }
    }

    impl From<MultiSignature> for RawMultiSignature {
        fn from(value: MultiSignature) -> Self {
            Self {
                signatures: value.signatures,
            }
        }
    }

    /// CompactBitArray is an implementation of a space efficient bit array.
    /// This is used to ensure that the encoded data takes up a minimal amount of
    /// space after proto encoding.
    /// This is not thread safe, and is not intended for concurrent usage.
    #[derive(Clone, PartialEq)]
    pub struct CompactBitArray {
        pub extra_bits_stored: u32,
        pub elems: Vec<u8>,
    }

    impl TryFrom<RawCompactBitArray> for CompactBitArray {
        type Error = Error;

        fn try_from(value: RawCompactBitArray) -> Result<Self, Self::Error> {
            let extra_bits_stored = value.extra_bits_stored;
            let elems = value.elems;

            Ok(Self {
                extra_bits_stored,
                elems,
            })
        }
    }

    impl From<CompactBitArray> for RawCompactBitArray {
        fn from(value: CompactBitArray) -> Self {
            Self {
                extra_bits_stored: value.extra_bits_stored,
                elems: value.elems,
            }
        }
    }
}
