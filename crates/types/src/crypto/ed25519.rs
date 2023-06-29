use crate::errors::Error;
use ibc_proto::cosmos::crypto::ed25519::PrivKey as RawPrivKey;
use ibc_proto::cosmos::crypto::ed25519::PubKey as RawPubKey;

/// PubKey is an ed25519 public key for handling Tendermint keys in SDK.
/// It's needed for Any serialization and SDK compatibility.
/// It must not be used in a non Tendermint key context because it doesn't implement
/// ADR-28. Nevertheless, you will like to use ed25519 in app user level
/// then you must create a new proto message and follow ADR-28 for Address construction.
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

/// Deprecated: PrivKey defines a ed25519 private key.
/// NOTE: ed25519 keys must not be used in SDK apps except in a tendermint validator context.
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
