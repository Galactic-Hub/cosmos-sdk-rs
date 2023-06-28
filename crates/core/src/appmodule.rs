//! Package appmodule defines the functionality for registering Cosmos SDK app
//! modules that are assembled using the cosmossdk.io/depinject
//! dependency injection system and the declarative app configuration format
//! handled by the appconfig package.

pub mod event;
pub mod genesis;
pub mod module;
pub mod option;
pub mod register;
