use crate::base::abci::attribute::Attribute;
use crate::errors::Error;
use ibc_proto::cosmos::base::abci::v1beta1::StringEvent as RawStringEvent;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// StringEvent defines en Event object wrapper where all the attributes
/// contain key/value pairs that are strings instead of raw bytes.
#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct StringEvent {
    pub r#type: String,
    pub attributes: Vec<Attribute>,
}

impl TryFrom<RawStringEvent> for StringEvent {
    type Error = Error;

    fn try_from(proto: RawStringEvent) -> Result<StringEvent, Error> {
        Ok(Self {
            r#type: proto.r#type,
            attributes: proto
                .attributes
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl From<StringEvent> for RawStringEvent {
    fn from(info: StringEvent) -> Self {
        Self {
            r#type: info.r#type,
            attributes: info
                .attributes
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
        }
    }
}
