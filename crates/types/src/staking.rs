//! staking module

/// Delay, in blocks, between when validator updates are returned to the
/// consensus-engine and when they are applied. For example, if
/// `VALIDATOR_UPDATE_DELAY` is set to X, and if a validator set update is
/// returned with new validators at the end of block 10, then the new
/// validators are expected to sign blocks beginning at block 11+X.
///
/// This value is constant as this should not change without a hard fork.
/// For CometBFT this should be set to 1 block, for more details see:
/// `https://github.com/cometbft/cometbft/blob/main/spec/abci/abci%2B%2B_basic_concepts.md#consensusblock-execution-methods`
pub const VALIDATOR_UPDATE_DELAY: i64 = 1;

/// `DEFAULT_BOND_DENOM` is the default bondable coin denomination (defaults to stake)
/// Overwriting this value has the side effect of changing the default denomination in genesis
pub const DEFAULT_BOND_DENOM: &str = "stake";

/// `DEFAULT_POWER_REDUCTION` is the default amount of staking tokens required for 1 unit of consensus-engine power
pub const DEFAULT_POWER_REDUCTION: i64 = 1000000;

/// `tokens_to_consensus_power` - convert input tokens to potential consensus-engine power
pub fn tokens_to_consensus_power(tokens: i64, power_reduction: i64) -> i64 {
    tokens / power_reduction
}

/// `tokens_from_consensus_power` - convert input power to tokens
pub fn tokens_from_consensus_power(power: i64, power_reduction: i64) -> i64 {
    power * power_reduction
}
