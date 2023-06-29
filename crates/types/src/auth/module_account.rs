use crate::auth::base_account::BaseAccount;
use crate::errors::Error;
use ibc_proto::cosmos::auth::v1beta1::ModuleAccount as RawModuleAccount;
use serde::{Deserialize, Serialize};

/// ModuleAccount defines an account for modules that holds coins on a pool.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ModuleAccount {
    pub base_account: Option<BaseAccount>,
    pub name: String,
    pub permissions: Vec<String>,
}

impl TryFrom<RawModuleAccount> for ModuleAccount {
    type Error = Error;

    fn try_from(proto: RawModuleAccount) -> Result<ModuleAccount, Error> {
        Ok(Self {
            base_account: proto.base_account.map(BaseAccount::try_from).transpose()?,
            name: proto.name,
            permissions: proto.permissions,
        })
    }
}

impl From<ModuleAccount> for RawModuleAccount {
    fn from(info: ModuleAccount) -> Self {
        Self {
            base_account: info.base_account.map(Into::into),
            name: info.name,
            permissions: info.permissions,
        }
    }
}
