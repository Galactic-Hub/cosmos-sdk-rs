use crate::base::abci::gas_info::GasInfo;
use crate::base::abci::result::AbciResult;
use crate::errors::Error;
use ibc_proto::cosmos::base::abci::v1beta1::SimulationResponse as RawSimulationResponse;
use std::convert::TryFrom;

/// SimulationResponse defines the response generated when a transaction is
/// successfully simulated.
#[derive(Clone, PartialEq)]
pub struct SimulationResponse {
    pub gas_info: Option<GasInfo>,
    pub result: Option<AbciResult>,
}

impl TryFrom<RawSimulationResponse> for SimulationResponse {
    type Error = Error;

    fn try_from(proto: RawSimulationResponse) -> Result<SimulationResponse, Error> {
        Ok(Self {
            gas_info: proto.gas_info.map(TryFrom::try_from).transpose()?,
            result: proto.result.map(TryFrom::try_from).transpose()?,
        })
    }
}

impl From<SimulationResponse> for RawSimulationResponse {
    fn from(info: SimulationResponse) -> Self {
        Self {
            gas_info: info.gas_info.map(Into::into),
            result: info.result.map(Into::into),
        }
    }
}
