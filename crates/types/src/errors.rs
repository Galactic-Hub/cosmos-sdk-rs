use ibc::core::ics24_host::identifier::IdentifierError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid coin: `{coin}`")]
    InvalidCoin { coin: String },

    #[error("invalid denom: {0}")]
    InvalidDenom(String),

    #[error("invalid amount: {0}")]
    InvalidAmount(String),

    #[error("amount is nil")]
    AmountIsNil,

    #[error("invalid utf8: {0}")]
    Utf8Decode(std::str::Utf8Error),

    #[error("invalid trace channel id at position: `{pos}`, error(`{validation_error}`)")]
    InvalidTraceChannelId {
        pos: usize,
        validation_error: IdentifierError,
    },

    #[error("invalid trace port id at position: `{pos}`, error(`{validation_error}`)")]
    InvalidTracePortId {
        pos: usize,
        validation_error: IdentifierError,
    },

    #[error("invalid trace length: `{len}`")]
    InvalidTraceLength { len: usize },

    #[error("empty base denom")]
    EmptyBaseDenom,

    #[error("infalible")]
    Infalible(std::convert::Infallible),

    #[error("invalid coin denom: `{left}`, `{right}`")]
    InvalidCoinDenom { left: String, right: String },

    #[error("empty bech32 address")]
    EmptyBech32Address,

    #[error("invalid bech32 prefix")]
    InvalidBech32Prefix,

    #[error("decoding Bech32 address failed: must provide a non empty address")]
    Bech32EmptyAddress,

    #[error("decoding address from hex string failed: empty address")]
    EmptyHexAddress,
}

impl From<std::convert::Infallible> for Error {
    fn from(e: std::convert::Infallible) -> Self {
        Self::Infalible(e)
    }
}
