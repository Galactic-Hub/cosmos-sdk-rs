use crate::errors::Error;
use ibc_proto::cosmos::crypto::secp256r1::PrivKey as RawPrivKey;
use ibc_proto::cosmos::crypto::secp256r1::PubKey as RawPubKey;

/// PubKey defines a secp256r1 ECDSA public key.
#[derive(Clone, PartialEq)]
pub struct PubKey {
    /// Point on secp256r1 curve in a compressed representation as specified in section
    /// 4.3.6 of ANSI X9.62: <https://webstore.ansi.org/standards/ascx9/ansix9621998>
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

/// PrivKey defines a secp256r1 ECDSA private key.
#[derive(Clone, PartialEq)]
pub struct PrivKey {
    /// secret number serialized using big-endian encoding
    pub secret: Vec<u8>,
}

impl TryFrom<RawPrivKey> for PrivKey {
    type Error = Error;

    fn try_from(raw: RawPrivKey) -> Result<Self, Self::Error> {
        Ok(PrivKey { secret: raw.secret })
    }
}

impl From<PrivKey> for RawPrivKey {
    fn from(pk: PrivKey) -> Self {
        RawPrivKey { secret: pk.secret }
    }
}
