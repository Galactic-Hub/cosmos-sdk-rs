use crate::errors::Error;
use ibc_proto::cosmos::crypto::secp256k1::PrivKey as RawPrivKey;
use ibc_proto::cosmos::crypto::secp256k1::PubKey as RawPubKey;

/// PubKey defines a secp256k1 public key
/// Key is the compressed form of the pubkey. The first byte depends is a 0x02 byte
/// if the y-coordinate is the lexicographically largest of the two associated with
/// the x-coordinate. Otherwise the first byte is a 0x03.
/// This prefix is followed with the x-coordinate.
#[derive(Clone, PartialEq)]
pub struct PubKey {
    pub key: Vec<u8>,
}

impl TryFrom<RawPubKey> for PubKey {
    type Error = Error;

    fn try_from(raw: RawPubKey) -> Result<Self, Self::Error> {
        Ok(PubKey { key: raw.key })
    }
}

impl From<PubKey> for RawPubKey {
    fn from(pk: PubKey) -> Self {
        RawPubKey { key: pk.key }
    }
}

/// PrivKey defines a secp256k1 private key.
#[derive(Clone, PartialEq)]
pub struct PrivKey {
    pub key: Vec<u8>,
}

impl TryFrom<RawPrivKey> for PrivKey {
    type Error = Error;

    fn try_from(raw: RawPrivKey) -> Result<Self, Self::Error> {
        Ok(PrivKey { key: raw.key })
    }
}

impl From<PrivKey> for RawPrivKey {
    fn from(pk: PrivKey) -> Self {
        RawPrivKey { key: pk.key }
    }
}
