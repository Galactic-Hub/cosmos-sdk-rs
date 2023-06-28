use anyhow::Result;

// AccountI is an interface used to store coins at a given address within state.
// It presumes a notion of sequence numbers for replay protection,
// a notion of account numbers for replay protection for previously pruned accounts,
// and a pubkey for authentication purposes.
//
// Many complex conditions can be used in the concrete struct which implements AccountI.
pub trait AccountI: prost::Message + std::fmt::Display {
    type AccAddress;
    type PubKey;

    fn get_address() -> Self::AccAddress;
    fn set_address() -> Result<()>;
    fn get_pub_key() -> Self::PubKey;
    fn set_pub_key() -> Result<()>;
    fn get_account_number() -> u64;
    fn set_account_number() -> Result<()>;
    fn get_sequence() -> u64;
    fn set_sequence() -> Result<()>;
}

// ModuleAccountI defines an account interface for modules that hold tokens in
// an escrow.
pub trait ModuleAccountI: AccountI {
    fn get_name() -> String;
    fn get_permissions() -> Vec<String>;
    fn has_permission() -> bool;
}
