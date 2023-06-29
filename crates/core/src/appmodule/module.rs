// use std::context::Context;

// use anyhow::Result;
// use tonic::server::Grpc;
// use tonic::transport::NamedService;

// todo
pub trait OnePerModuleType {}

/// AppModule is a tag interface for app module implementations to use as a basis
/// for extension interfaces. It provides no functionality itself, but is the
/// type that all valid app modules should provide so that they can be identified
/// by other modules (usually via depinject) as app modules.
pub trait AppModule: OnePerModuleType {
    // IsAppModule is a dummy method to tag a struct as implementing an AppModule.
    // 在 Rust 中不需要 IsAppModule 方法
}

/// HasServices is the extension interface that modules should implement to register
/// implementations of services defined in .proto files.
pub trait HasServices: AppModule {
    // RegisterServices registers the module's services with the app's service
    // registrar.
    //
    // Two types of services are currently supported:
    // - read-only gRPC query services, which are the default.
    // - transaction message services, which must have the protobuf service
    //   option "cosmos.msg.v1.service" (defined in "cosmos/msg/v1/service.proto")
    //   set to true.
    //
    // The service registrar will figure out which type of service you are
    // implementing based on the presence (or absence) of protobuf options. You
    // do not need to specify this in golang code.
    // todo(davirain)
    // fn register_services(&self, registrar: &mut Box<dyn NamedService>) -> Result<()>;
}

// todo
/// HasPrepareCheckState is an extension interface that contains information about the AppModule
/// and PrepareCheckState.
pub trait HasPrepareCheckState: AppModule {
    // PrepareCheckState is called to prepare the state for a new block and perform
    // any necessary checks.

    // fn prepare_check_state(&self, ctx: Context) -> Result<(), Box<dyn std::error::Error>>;
}

// todo
/// HasPrecommit is an extension interface that contains information about the AppModule and Precommit.
pub trait HasPrecommit: AppModule {
    // Precommit is called to perform any logic that should run before committing
    // the block.
    // fn precommit(&self, ctx: Context) -> Result<(), Box<dyn std::error::Error>>;
}

// todo
/// HasBeginBlocker is the extension interface that modules should implement to run
/// custom logic before transaction processing in a block.
pub trait HasBeginBlocker: AppModule {
    // BeginBlock is called before transaction processing in a block.
    // fn begin_block(&self, ctx: Context) -> Result<(), Box<dyn std::error::Error>>;
}

// todo
/// HasEndBlocker is the extension interface that modules should implement to run
/// custom logic after transaction processing in a block.
pub trait HasEndBlocker: AppModule {
    // EndBlock is called after transaction processing in a block.
    // fn end_block(&self, ctx: Context) -> Result<(), Box<dyn std::error::Error>>;
}
