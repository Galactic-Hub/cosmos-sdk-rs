use crate::base::abci::tx_response::TxResponse;
use crate::errors::Error;
use ibc_proto::cosmos::base::abci::v1beta1::SearchTxsResult as RawSearchTxsResult;
use std::convert::TryFrom;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
/// SearchTxsResult defines a structure for querying txs pageable
pub struct SearchTxsResult {
    /// Count of all txs
    pub total_count: u64,
    /// Count of txs in current page
    pub count: u64,
    /// Index of current page, start from 1
    pub page_number: u64,
    /// Count of total pages
    pub page_total: u64,
    /// Max count txs per page
    pub limit: u64,
    /// List of txs in current page
    pub txs: Vec<TxResponse>,
}

impl TryFrom<RawSearchTxsResult> for SearchTxsResult {
    type Error = Error;

    fn try_from(proto: RawSearchTxsResult) -> Result<SearchTxsResult, Error> {
        Ok(Self {
            total_count: proto.total_count,
            count: proto.count,
            page_number: proto.page_number,
            page_total: proto.page_total,
            limit: proto.limit,
            txs: proto
                .txs
                .into_iter()
                .map(TxResponse::try_from)
                .collect::<Result<Vec<TxResponse>, Error>>()?,
        })
    }
}

impl From<SearchTxsResult> for RawSearchTxsResult {
    fn from(info: SearchTxsResult) -> Self {
        Self {
            total_count: info.total_count,
            count: info.count,
            page_number: info.page_number,
            page_total: info.page_total,
            limit: info.limit,
            txs: info.txs.into_iter().map(Into::into).collect(),
        }
    }
}
