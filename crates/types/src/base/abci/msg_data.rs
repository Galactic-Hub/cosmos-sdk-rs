use crate::errors::Error;
use anyhow::Result;
use ibc_proto::cosmos::base::abci::v1beta1::MsgData as RawMsgData;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

/// MsgData defines the data returned in a Result object during message execution.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct MsgData {
    /// Incoming message type that emitted this result data, for example `"/cosmos.bank.v1beta1.MsgSend"`.
    pub msg_type: String,

    /// Binary data emitted by this message.
    // Do note that usually the data has to be decoded into the corresponding protobuf `Response` type.
    // For example, if the data was emitted as a result of a `MsgSend`, i.e. `msg.msg_type == "/cosmos.bank.v1beta1.MsgSend"`,
    // then you should decode it into `"/cosmos.bank.v1beta1.MsgSendResponse"
    pub data: Vec<u8>,
}

impl TryFrom<RawMsgData> for MsgData {
    type Error = Error;

    fn try_from(proto: RawMsgData) -> Result<MsgData, Error> {
        Ok(Self {
            msg_type: proto.msg_type,
            data: proto.data,
        })
    }
}

impl From<MsgData> for RawMsgData {
    fn from(msg_data: MsgData) -> Self {
        Self {
            msg_type: msg_data.msg_type,
            data: msg_data.data,
        }
    }
}
