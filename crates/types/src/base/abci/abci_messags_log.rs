use crate::base::abci::string_event::StringEvent;
use crate::errors::Error;
use ibc_proto::cosmos::base::abci::v1beta1::AbciMessageLog as RawAbciMessageLog;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// ABCIMessageLog defines a structure containing an indexed tx ABCI message log.
#[derive(Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct AbciMessageLog {
    pub msg_index: u32,
    pub log: ::prost::alloc::string::String,
    /// Events contains a slice of Event objects that were emitted during some
    /// execution.
    pub events: Vec<StringEvent>,
}

impl TryFrom<RawAbciMessageLog> for AbciMessageLog {
    type Error = Error;

    fn try_from(proto: RawAbciMessageLog) -> Result<AbciMessageLog, Error> {
        Ok(Self {
            msg_index: proto.msg_index,
            log: proto.log,
            events: proto
                .events
                .into_iter()
                .map(StringEvent::try_from)
                .collect::<Result<Vec<StringEvent>, Error>>()?,
        })
    }
}

impl From<AbciMessageLog> for RawAbciMessageLog {
    fn from(info: AbciMessageLog) -> Self {
        Self {
            msg_index: info.msg_index,
            log: info.log,
            events: info.events.into_iter().map(Into::into).collect(),
        }
    }
}
