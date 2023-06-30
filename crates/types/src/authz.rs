use crate::errors::Error;
use ibc::core::timestamp::Timestamp;
use ibc_proto::cosmos::authz::v1beta1::{
    GenericAuthorization as RawGenericAuthorization, Grant as RawGrant,
    GrantAuthorization as RawGrantAuthorization, GrantQueueItem as RawGrantQueueItem,
    MsgExec as RawMsgExec, MsgExecResponse as RawMsgExecResponse, MsgGrant as RawMsgGrant,
    MsgGrantResponse as RawMsgGrantResponse, MsgRevoke as RawMsgRevoke,
    MsgRevokeResponse as RawMsgRevokeResponse,
};
use ibc_proto::google::protobuf::Any;
use std::convert::TryFrom;

/// GenericAuthorization gives the grantee unrestricted permissions to execute
/// the provided method on behalf of the granter's account.
#[derive(Clone, PartialEq)]
pub struct GenericAuthorization {
    /// Msg, identified by it's type URL, to grant unrestricted permissions to execute
    pub msg: String,
}

impl TryFrom<RawGenericAuthorization> for GenericAuthorization {
    type Error = Error;

    fn try_from(value: RawGenericAuthorization) -> Result<Self, Self::Error> {
        Ok(Self { msg: value.msg })
    }
}

impl From<GenericAuthorization> for RawGenericAuthorization {
    fn from(value: GenericAuthorization) -> Self {
        Self { msg: value.msg }
    }
}

/// Grant gives permissions to execute
/// the provide method with expiration time.
#[derive(Clone, PartialEq)]
pub struct Grant {
    pub authorization: Option<Any>,
    /// time when the grant will expire and will be pruned. If null, then the grant
    /// doesn't have a time expiration (other conditions  in `authorization`
    /// may apply to invalidate the grant)
    pub expiration: Option<Timestamp>,
}

impl TryFrom<RawGrant> for Grant {
    type Error = Error;

    fn try_from(_value: RawGrant) -> Result<Self, Self::Error> {
        // Ok(Self {
        //     authorization: value.authorization,
        //     expiration: value.expiration.map(TryFrom::try_from).transpose()?,
        // })
        todo!()
    }
}

impl From<Grant> for RawGrant {
    fn from(_value: Grant) -> Self {
        // Self {
        //     authorization: value.authorization,
        //     expiration: value.expiration.map(Into::into),
        // }
        todo!()
    }
}

/// GrantAuthorization extends a grant with both the addresses of the grantee and granter.
/// It is used in genesis.proto and query.proto
#[derive(Clone, PartialEq)]
pub struct GrantAuthorization {
    pub granter: String,
    pub grantee: String,
    pub authorization: Option<Any>,
    pub expiration: Option<Timestamp>,
}

impl TryFrom<RawGrantAuthorization> for GrantAuthorization {
    type Error = Error;

    fn try_from(_value: RawGrantAuthorization) -> Result<Self, Self::Error> {
        // Ok(Self {
        //     granter: value.granter,
        //     grantee: value.grantee,
        //     authorization: value.authorization,
        //     expiration: value.expiration.map(TryFrom::try_from).transpose()?,
        // })
        todo!()
    }
}

impl From<GrantAuthorization> for RawGrantAuthorization {
    fn from(_value: GrantAuthorization) -> Self {
        // Self {
        //     granter: value.granter,
        //     grantee: value.grantee,
        //     authorization: value.authorization,
        //     expiration: value.expiration.map(Into::into),
        // }
        todo!()
    }
}

/// GrantQueueItem contains the list of TypeURL of a sdk.Msg.
#[derive(Clone, PartialEq)]
pub struct GrantQueueItem {
    /// msg_type_urls contains the list of TypeURL of a sdk.Msg.
    pub msg_type_urls: Vec<String>,
}

impl TryFrom<RawGrantQueueItem> for GrantQueueItem {
    type Error = Error;

    fn try_from(value: RawGrantQueueItem) -> Result<Self, Self::Error> {
        Ok(Self {
            msg_type_urls: value.msg_type_urls,
        })
    }
}

impl From<GrantQueueItem> for RawGrantQueueItem {
    fn from(value: GrantQueueItem) -> Self {
        Self {
            msg_type_urls: value.msg_type_urls,
        }
    }
}

/// MsgGrant is a request type for Grant method. It declares authorization to the grantee
/// on behalf of the granter with the provided expiration time.
#[derive(Clone, PartialEq)]
pub struct MsgGrant {
    pub granter: String,
    pub grantee: String,
    pub grant: Option<Grant>,
}

impl TryFrom<RawMsgGrant> for MsgGrant {
    type Error = Error;

    fn try_from(value: RawMsgGrant) -> Result<Self, Self::Error> {
        Ok(Self {
            granter: value.granter,
            grantee: value.grantee,
            grant: value.grant.map(TryFrom::try_from).transpose()?,
        })
    }
}

impl From<MsgGrant> for RawMsgGrant {
    fn from(value: MsgGrant) -> Self {
        Self {
            granter: value.granter,
            grantee: value.grantee,
            grant: value.grant.map(Into::into),
        }
    }
}

/// MsgExecResponse defines the Msg/MsgExecResponse response type.
#[derive(Clone, PartialEq)]
pub struct MsgExecResponse {
    pub results: Vec<Vec<u8>>,
}

impl TryFrom<RawMsgExecResponse> for MsgExecResponse {
    type Error = Error;

    fn try_from(value: RawMsgExecResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            results: value.results,
        })
    }
}

impl From<MsgExecResponse> for RawMsgExecResponse {
    fn from(value: MsgExecResponse) -> Self {
        Self {
            results: value.results,
        }
    }
}

/// MsgExec attempts to execute the provided messages using
/// authorizations granted to the grantee. Each message should have only
/// one signer corresponding to the granter of the authorization.
#[derive(Clone, PartialEq)]
pub struct MsgExec {
    pub grantee: String,
    /// Authorization Msg requests to execute. Each msg must implement Authorization interface
    /// The x/authz will try to find a grant matching (msg.signers\[0\], grantee, MsgTypeURL(msg))
    /// triple and validate it.
    pub msgs: Vec<Any>,
}

impl TryFrom<RawMsgExec> for MsgExec {
    type Error = Error;

    fn try_from(value: RawMsgExec) -> Result<Self, Self::Error> {
        Ok(Self {
            grantee: value.grantee,
            msgs: value.msgs,
        })
    }
}

impl From<MsgExec> for RawMsgExec {
    fn from(value: MsgExec) -> Self {
        Self {
            grantee: value.grantee,
            msgs: value.msgs,
        }
    }
}

/// MsgGrantResponse defines the Msg/MsgGrant response type.
#[derive(Clone, PartialEq)]
pub struct MsgGrantResponse {}

impl TryFrom<RawMsgGrantResponse> for MsgGrantResponse {
    type Error = Error;

    fn try_from(_value: RawMsgGrantResponse) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl From<MsgGrantResponse> for RawMsgGrantResponse {
    fn from(_value: MsgGrantResponse) -> Self {
        Self {}
    }
}

/// MsgRevoke revokes any authorization with the provided sdk.Msg type on the
/// granter's account with that has been granted to the grantee.
#[derive(Clone, PartialEq)]
pub struct MsgRevoke {
    pub granter: String,
    pub grantee: String,
    pub msg_type_url: String,
}

impl TryFrom<RawMsgRevoke> for MsgRevoke {
    type Error = Error;

    fn try_from(value: RawMsgRevoke) -> Result<Self, Self::Error> {
        Ok(Self {
            granter: value.granter,
            grantee: value.grantee,
            msg_type_url: value.msg_type_url,
        })
    }
}

impl From<MsgRevoke> for RawMsgRevoke {
    fn from(value: MsgRevoke) -> Self {
        Self {
            granter: value.granter,
            grantee: value.grantee,
            msg_type_url: value.msg_type_url,
        }
    }
}

/// MsgRevokeResponse defines the Msg/MsgRevokeResponse response type.
#[derive(Clone, PartialEq)]
pub struct MsgRevokeResponse {}

impl TryFrom<RawMsgRevokeResponse> for MsgRevokeResponse {
    type Error = Error;

    fn try_from(_value: RawMsgRevokeResponse) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl From<MsgRevokeResponse> for RawMsgRevokeResponse {
    fn from(_value: MsgRevokeResponse) -> Self {
        Self {}
    }
}
