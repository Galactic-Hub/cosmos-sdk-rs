use crate::base::abci::abci_messags_log::AbciMessageLog;
use crate::errors::Error;
use ibc_proto::cosmos::base::abci::v1beta1::TxResponse as RawTxResponse;
use ibc_proto::google::protobuf::Any;
use tendermint::abci::Event as TendermintAbciEvent;

/// TxResponse defines a structure containing relevant tx data and metadata. The
/// tags are stringified and the log is JSON decoded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TxResponse {
    /// The block height
    pub height: i64,
    /// The transaction hash.
    pub txhash: String,
    /// Namespace for the Code
    pub codespace: String,
    /// Response code.
    pub code: u32,
    /// Result bytes, if any.
    pub data: String,
    /// The output of the application's logger (raw string). May be
    /// non-deterministic.
    pub raw_log: ::prost::alloc::string::String,
    /// The output of the application's logger (typed). May be non-deterministic.
    pub logs: Vec<AbciMessageLog>,
    /// Additional information. May be non-deterministic.
    pub info: ::prost::alloc::string::String,
    /// Amount of gas requested for transaction.
    pub gas_wanted: i64,
    /// Amount of gas consumed by transaction.
    pub gas_used: i64,
    /// The request transaction bytes.
    pub tx: Option<Any>,
    /// Time of the previous block. For heights > 1, it's the weighted median of
    /// the timestamps of the valid votes in the block.LastCommit. For height == 1,
    /// it's genesis time.
    pub timestamp: String,
    /// Events defines all the events emitted by processing a transaction. Note,
    /// these events include those emitted by processing all the messages and those
    /// emitted from the ante. Whereas Logs contains the events, with
    /// additional metadata, emitted only by processing the messages.
    ///
    /// Since: cosmos-sdk 0.42.11, 0.44.5, 0.45
    pub events: Vec<TendermintAbciEvent>,
}

impl TryFrom<RawTxResponse> for TxResponse {
    type Error = Error;

    fn try_from(proto: RawTxResponse) -> Result<TxResponse, Error> {
        Ok(Self {
            height: proto.height,
            txhash: proto.txhash,
            codespace: proto.codespace,
            code: proto.code,
            data: proto.data,
            raw_log: proto.raw_log,
            logs: proto
                .logs
                .into_iter()
                .map(AbciMessageLog::try_from)
                .collect::<Result<Vec<AbciMessageLog>, Error>>()?,
            info: proto.info,
            gas_wanted: proto.gas_wanted,
            gas_used: proto.gas_used,
            tx: proto.tx,
            timestamp: proto.timestamp,
            events: proto
                .events
                .into_iter()
                .map(TryFrom::try_from)
                .collect::<Result<Vec<TendermintAbciEvent>, tendermint::Error>>()
                .map_err(|e| Error::Custom(e.to_string()))?,
        })
    }
}

impl From<TxResponse> for RawTxResponse {
    fn from(info: TxResponse) -> Self {
        Self {
            height: info.height,
            txhash: info.txhash,
            codespace: info.codespace,
            code: info.code,
            data: info.data,
            raw_log: info.raw_log,
            logs: info.logs.into_iter().map(Into::into).collect(),
            info: info.info,
            gas_wanted: info.gas_wanted,
            gas_used: info.gas_used,
            tx: info.tx,
            timestamp: info.timestamp,
            events: info.events.into_iter().map(Into::into).collect(),
        }
    }
}
