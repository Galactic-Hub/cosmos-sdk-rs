use crate::errors::Error;
use ibc_proto::cosmos::auth::v1beta1::BaseAccount as RawBaseAccount;
use ibc_proto::google::protobuf::Any;
use serde::{Deserialize, Serialize};

/// BaseAccount defines a base account type. It contains all the necessary fields
/// for basic account functionality. Any custom account type should extend this
/// type for additional functionality (e.g. vesting).
#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct BaseAccount {
    pub address: ::prost::alloc::string::String,
    pub pub_key: Option<Any>,
    pub account_number: u64,
    pub sequence: u64,
}

impl BaseAccount {
    pub fn new(
        address: ::prost::alloc::string::String,
        pub_key: Option<Any>,
        account_number: u64,
        sequence: u64,
    ) -> Self {
        BaseAccount {
            address,
            pub_key,
            account_number,
            sequence,
        }
    }
}

impl TryFrom<RawBaseAccount> for BaseAccount {
    type Error = Error;

    fn try_from(proto: RawBaseAccount) -> Result<BaseAccount, Error> {
        Ok(Self {
            address: proto.address,
            pub_key: proto.pub_key,
            account_number: proto.account_number,
            sequence: proto.sequence,
        })
    }
}

impl From<BaseAccount> for RawBaseAccount {
    fn from(info: BaseAccount) -> Self {
        Self {
            address: info.address,
            pub_key: info.pub_key,
            account_number: info.account_number,
            sequence: info.sequence,
        }
    }
}
