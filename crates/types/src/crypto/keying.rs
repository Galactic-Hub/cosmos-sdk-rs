use crate::errors::Error;
use ibc_proto::cosmos::crypto::keyring::v1::Record as RawRecord;
use ibc_proto::google::protobuf::Any;

/// Record is used for representing a key in the keyring.
#[derive(Clone, PartialEq)]
pub struct Record {
    /// name represents a name of Record
    pub name: String,
    /// pub_key represents a public key in any format
    pub pub_key: Option<Any>,
    /// Record contains one of the following items
    pub item: Option<record::Item>,
}

impl TryFrom<RawRecord> for Record {
    type Error = Error;
    fn try_from(value: RawRecord) -> Result<Self, Self::Error> {
        let name = value.name;
        let pub_key = value.pub_key;
        let item = value.item.map(record::Item::try_from).transpose()?;
        Ok(Record {
            name,
            pub_key,
            item,
        })
    }
}

impl From<Record> for RawRecord {
    fn from(value: Record) -> Self {
        RawRecord {
            name: value.name,
            pub_key: value.pub_key,
            item: value.item.map(Into::into),
        }
    }
}

pub mod record {
    use crate::crypto::hd::v1::Bip44Params;
    use crate::errors::Error;
    use ibc_proto::cosmos::crypto::keyring::v1::record::{
        Item as RawItem, Ledger as RawLedger, Local as RawLocal, Multi as RawMulti,
        Offline as RawOffline,
    };
    use ibc_proto::google::protobuf::Any;

    /// Item is a keyring item stored in a keyring backend.
    /// Local item
    #[derive(Clone, PartialEq)]
    pub struct Local {
        pub priv_key: Option<Any>,
    }

    impl TryFrom<RawLocal> for Local {
        type Error = Error;
        fn try_from(value: RawLocal) -> Result<Self, Self::Error> {
            Ok(Local {
                priv_key: value.priv_key,
            })
        }
    }

    impl From<Local> for RawLocal {
        fn from(value: Local) -> Self {
            RawLocal {
                priv_key: value.priv_key,
            }
        }
    }

    /// Ledger item
    #[derive(Clone, PartialEq)]
    pub struct Ledger {
        pub path: Option<Bip44Params>,
    }

    impl TryFrom<RawLedger> for Ledger {
        type Error = Error;
        fn try_from(value: RawLedger) -> Result<Self, Self::Error> {
            let path = value.path.map(Bip44Params::try_from).transpose()?;
            Ok(Ledger { path })
        }
    }

    impl From<Ledger> for RawLedger {
        fn from(value: Ledger) -> Self {
            RawLedger {
                path: value.path.map(Into::into),
            }
        }
    }

    /// Multi item
    #[derive(Clone, PartialEq)]
    pub struct Multi {}

    impl TryFrom<RawMulti> for Multi {
        type Error = Error;
        fn try_from(_: RawMulti) -> Result<Self, Self::Error> {
            Ok(Multi {})
        }
    }

    impl From<Multi> for RawMulti {
        fn from(_: Multi) -> Self {
            RawMulti {}
        }
    }

    /// Offline item
    #[derive(Clone, PartialEq)]
    pub struct Offline {}

    impl TryFrom<RawOffline> for Offline {
        type Error = Error;
        fn try_from(_: RawOffline) -> Result<Self, Self::Error> {
            Ok(Offline {})
        }
    }

    impl From<Offline> for RawOffline {
        fn from(_: Offline) -> Self {
            RawOffline {}
        }
    }

    /// Record contains one of the following items
    #[derive(Clone, PartialEq)]
    pub enum Item {
        /// local stores the private key locally.
        Local(Local),
        /// ledger stores the information about a Ledger key.
        Ledger(Ledger),
        /// Multi does not store any other information.
        Multi(Multi),
        /// Offline does not store any other information.
        Offline(Offline),
    }

    impl TryFrom<RawItem> for Item {
        type Error = Error;
        fn try_from(value: RawItem) -> Result<Self, Self::Error> {
            match value {
                RawItem::Local(local) => Ok(Item::Local(Local::try_from(local)?)),
                RawItem::Ledger(ledger) => Ok(Item::Ledger(Ledger::try_from(ledger)?)),
                RawItem::Multi(multi) => Ok(Item::Multi(Multi::try_from(multi)?)),
                RawItem::Offline(offline) => Ok(Item::Offline(Offline::try_from(offline)?)),
            }
        }
    }

    impl From<Item> for RawItem {
        fn from(value: Item) -> Self {
            match value {
                Item::Local(local) => RawItem::Local(local.into()),
                Item::Ledger(ledger) => RawItem::Ledger(ledger.into()),
                Item::Multi(multi) => RawItem::Multi(multi.into()),
                Item::Offline(offline) => RawItem::Offline(offline.into()),
            }
        }
    }
}
