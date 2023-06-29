use crate::base::abci::msg_data::MsgData;
use crate::base::abci::Data;
use crate::errors::Error;
use anyhow::Result;
use ibc_proto::cosmos::base::abci::v1beta1::TxMsgData as RawTxMsgData;
use ibc_proto::google::protobuf::Any;

use prost::Message;
use std::convert::TryFrom;

/// TxMsgData defines a list of MsgData. A transaction will have a MsgData object for each message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TxMsgData {
    /// Data emitted by the messages in a particular transaction.
    // Note: this field will be deprecated and not populated as of cosmos-sdk 0.46.
    // It will be superseded by `msg_responses` field of type Vec<Any>
    pub data: Vec<MsgData>,
    /// msg_responses contains the Msg handler responses packed into Anys.
    ///
    /// Since: cosmos-sdk 0.46.
    pub msg_responses: Vec<Any>,
}

impl TryFrom<Data> for TxMsgData {
    type Error = Error;

    fn try_from(data: Data) -> Result<TxMsgData, Error> {
        RawTxMsgData::decode(data.as_ref())
            .map_err(Error::DecodeError)?
            .try_into()
    }
}

impl TryFrom<RawTxMsgData> for TxMsgData {
    type Error = Error;

    #[allow(deprecated)]
    fn try_from(proto: RawTxMsgData) -> Result<TxMsgData, Error> {
        // TODO(tarcieri): parse `msg_responses`
        if !proto.msg_responses.is_empty() {
            return Err(Error::TxMsgDataUnsupported);
        }

        Ok(Self {
            data: proto
                .data
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<_, _>>()?,
            msg_responses: proto.msg_responses,
        })
    }
}

impl From<TxMsgData> for RawTxMsgData {
    #[allow(deprecated)]
    fn from(tx_msg_data: TxMsgData) -> Self {
        Self {
            data: tx_msg_data.data.into_iter().map(Into::into).collect(),
            msg_responses: tx_msg_data.msg_responses, // TODO(tarcieri): serialize responses
        }
    }
}
