use crate::errors::Error;
use ibc_proto::cosmos::base::abci::v1beta1::Attribute as RawAttribute;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// Attribute defines an attribute wrapper where the key and value are
/// strings instead of raw bytes.
#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

impl TryFrom<RawAttribute> for Attribute {
    type Error = Error;

    fn try_from(proto: RawAttribute) -> Result<Attribute, Error> {
        Ok(Self {
            key: proto.key,
            value: proto.value,
        })
    }
}

impl From<Attribute> for RawAttribute {
    fn from(info: Attribute) -> Self {
        Self {
            key: info.key,
            value: info.value,
        }
    }
}
