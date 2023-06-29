use crate::errors::Error;
use crate::Gas;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use ibc_proto::cosmos::base::abci::v1beta1::GasInfo as RawGasInfo;

/// [`GasInfo`] defines constraints for how much gas to use to execute a
/// transaction.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct GasInfo {
    /// GasWanted is the maximum units of work we allow this tx to perform.
    pub gas_wanted: Gas,

    /// GasUsed is the amount of gas actually consumed.
    pub gas_used: Gas,
}

impl TryFrom<RawGasInfo> for GasInfo {
    type Error = Error;

    fn try_from(proto: RawGasInfo) -> Result<GasInfo, Error> {
        Ok(Self {
            gas_wanted: proto.gas_wanted,
            gas_used: proto.gas_used,
        })
    }
}

impl From<GasInfo> for RawGasInfo {
    fn from(info: GasInfo) -> Self {
        Self {
            gas_wanted: info.gas_wanted,
            gas_used: info.gas_wanted,
        }
    }
}
