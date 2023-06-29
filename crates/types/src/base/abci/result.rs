use crate::errors::Error;
use ibc_proto::cosmos::base::abci::v1beta1::Result as RawResult;
use ibc_proto::google::protobuf::Any;
use std::convert::TryFrom;
use tendermint::abci::Event as TendermintAbciEvent;

/// Result is the union of ResponseFormat and ResponseCheckTx.
#[derive(Clone, PartialEq)]
pub struct AbciResult {
    /// Data is any data returned from message or handler execution. It MUST be
    /// length prefixed in order to separate data from multiple message executions.
    /// Deprecated. This field is still populated, but prefer msg_response instead
    /// because it also contains the Msg response typeURL.
    #[deprecated]
    pub data: Vec<u8>,
    /// Log contains the log information from message or handler execution.
    pub log: String,
    /// Events contains a slice of Event objects that were emitted during message
    /// or handler execution.
    pub events: Vec<TendermintAbciEvent>,
    /// msg_responses contains the Msg handler responses type packed in Anys.
    ///
    /// Since: cosmos-sdk 0.46
    pub msg_responses: Vec<Any>,
}

impl TryFrom<RawResult> for AbciResult {
    type Error = Error;

    fn try_from(proto: RawResult) -> Result<AbciResult, Error> {
        Ok(Self {
            data: proto.data,
            log: proto.log,
            events: proto
                .events
                .into_iter()
                .map(TendermintAbciEvent::try_from)
                .collect::<Result<Vec<TendermintAbciEvent>, tendermint::Error>>()
                .map_err(|e| Error::Custom(e.to_string()))?,
            msg_responses: proto.msg_responses,
        })
    }
}

impl From<AbciResult> for RawResult {
    fn from(info: AbciResult) -> Self {
        Self {
            data: info.data,
            log: info.log,
            events: info.events.into_iter().map(Into::into).collect(),
            msg_responses: info.msg_responses,
        }
    }
}
