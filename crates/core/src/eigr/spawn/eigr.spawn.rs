#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Registry {
    #[prost(map="string, message", tag="1")]
    pub actors: ::std::collections::HashMap<::prost::alloc::string::String, Actor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorSystem {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub registry: ::core::option::Option<Registry>,
}
/// A strategy for save state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorSnapshotStrategy {
    #[prost(oneof="actor_snapshot_strategy::Strategy", tags="1")]
    pub strategy: ::core::option::Option<actor_snapshot_strategy::Strategy>,
}
/// Nested message and enum types in `ActorSnapshotStrategy`.
pub mod actor_snapshot_strategy {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Strategy {
        /// the timeout strategy.
        #[prost(message, tag="1")]
        Timeout(super::TimeoutStrategy),
    }
}
/// A strategy which a user function's entity is passivated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorDeactivateStrategy {
    #[prost(oneof="actor_deactivate_strategy::Strategy", tags="1")]
    pub strategy: ::core::option::Option<actor_deactivate_strategy::Strategy>,
}
/// Nested message and enum types in `ActorDeactivateStrategy`.
pub mod actor_deactivate_strategy {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Strategy {
        /// the timeout strategy.
        #[prost(message, tag="1")]
        Timeout(super::TimeoutStrategy),
    }
}
/// A strategy based on a timeout. 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeoutStrategy {
    /// The timeout in millis
    #[prost(int64, tag="1")]
    pub timeout: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorState {
    #[prost(map="string, string", tag="1")]
    pub tags: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub state: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorSettings {
    /// Indicates if actor´s is abstract or non abstract.
    #[prost(bool, tag="1")]
    pub r#abstract: bool,
    /// Indicates whether an actor's state should be persisted in a definitive store.
    #[prost(bool, tag="2")]
    pub persistent: bool,
    /// Snapshot strategy
    #[prost(message, optional, tag="3")]
    pub snapshot_strategy: ::core::option::Option<ActorSnapshotStrategy>,
    /// Deactivate strategy
    #[prost(message, optional, tag="4")]
    pub deactivate_strategy: ::core::option::Option<ActorDeactivateStrategy>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorId {
    /// The name of a Actor Entity.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Name of a ActorSystem
    #[prost(string, tag="2")]
    pub system: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actor {
    /// Actor Identification
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<ActorId>,
    /// A Actor state.
    #[prost(message, optional, tag="2")]
    pub state: ::core::option::Option<ActorState>,
    /// Actor settings.
    #[prost(message, optional, tag="3")]
    pub settings: ::core::option::Option<ActorSettings>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpawnRequest {
    #[prost(message, optional, tag="2")]
    pub actor_system: ::core::option::Option<ActorSystem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpawnResponse {
    #[prost(message, optional, tag="1")]
    pub status: ::core::option::Option<RequestStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationRequest {
    #[prost(message, optional, tag="1")]
    pub service_info: ::core::option::Option<ServiceInfo>,
    #[prost(message, optional, tag="2")]
    pub actor_system: ::core::option::Option<ActorSystem>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceInfo {
    /// The name of the actor system, eg, "my-actor-system".
    #[prost(string, tag="1")]
    pub service_name: ::prost::alloc::string::String,
    /// The version of the service.
    #[prost(string, tag="2")]
    pub service_version: ::prost::alloc::string::String,
    /// A description of the runtime for the service. Can be anything, but examples might be:
    /// - node v10.15.2
    /// - OpenJDK Runtime Environment 1.8.0_192-b12
    #[prost(string, tag="3")]
    pub service_runtime: ::prost::alloc::string::String,
    /// If using a support library, the name of that library, eg "spawn-jvm"
    #[prost(string, tag="4")]
    pub support_library_name: ::prost::alloc::string::String,
    /// The version of the support library being used.
    #[prost(string, tag="5")]
    pub support_library_version: ::prost::alloc::string::String,
    /// Spawn protocol major version accepted by the support library.
    #[prost(int32, tag="6")]
    pub protocol_major_version: i32,
    /// Spawn protocol minor version accepted by the support library.
    #[prost(int32, tag="7")]
    pub protocol_minor_version: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyInfo {
    #[prost(int32, tag="1")]
    pub protocol_major_version: i32,
    #[prost(int32, tag="2")]
    pub protocol_minor_version: i32,
    #[prost(string, tag="3")]
    pub proxy_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub proxy_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationResponse {
    #[prost(message, optional, tag="1")]
    pub status: ::core::option::Option<RequestStatus>,
    #[prost(message, optional, tag="2")]
    pub proxy_info: ::core::option::Option<ProxyInfo>,
}
/// Context is where current and/or updated state is stored 
/// to be transmitted to/from proxy and user function
///
/// Params:
///    * state: Actor state passed back and forth between proxy and user function. 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<::prost_types::Any>,
}
/// The user function when it wants to send a message to an Actor uses the InvocationRequest message type.
///
/// Params:
///    * system: See ActorStstem message.
///    * actor: The target Actor, i.e. the one that the user function is calling to perform some computation.
///    * command_name: The function or method on the target Actor that will receive this request 
///      and perform some useful computation with the sent data.
///    * value: This is the value sent by the user function to be computed by the request's target Actor command.
///    * async: Indicates whether the command should be processed synchronously, where a response should be sent back to the user function, 
///             or whether the command should be processed asynchronously, i.e. no response sent to the caller and no waiting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationRequest {
    #[prost(message, optional, tag="1")]
    pub system: ::core::option::Option<ActorSystem>,
    #[prost(message, optional, tag="2")]
    pub actor: ::core::option::Option<Actor>,
    #[prost(string, tag="3")]
    pub command_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub value: ::core::option::Option<::prost_types::Any>,
    #[prost(bool, tag="5")]
    pub r#async: bool,
}
/// ActorInvocation is a translation message between a local invocation made via InvocationRequest 
/// and the real Actor that intends to respond to this invocation and that can be located anywhere in the cluster.
///
/// Params:
///    actor_name: The name of the Actor handling the InvocationRequest request, also called the target Actor.
///    actor_system: The name of ActorSystem registered in Registration step.
///    command_name: The function or method on the target Actor that will receive this request 
///                  and perform some useful computation with the sent data.
///    current_context: The current Context with current state value of the target Actor. 
///                     That is, the same as found via matching in %Actor{name: target_actor, state: %ActorState{state: value} = actor_state}. 
///                     In this case, the Context type will contain in the value attribute the same `value` as the matching above.
///    value: The value to be passed to the function or method corresponding to command_name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorInvocation {
    #[prost(string, tag="1")]
    pub actor_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub actor_system: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub command_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub current_context: ::core::option::Option<Context>,
    #[prost(message, optional, tag="5")]
    pub value: ::core::option::Option<::prost_types::Any>,
}
/// The user function's response after executing the action originated by the local proxy request via ActorInvocation.
///
/// Params:
///    actor_name: The name of the Actor handling the InvocationRequest request, also called the target Actor.
///    actor_system: The name of ActorSystem registered in Registration step.
///    updated_context: The Context with updated state value of the target Actor after user function has processed a request.
///    value: The value that the original request proxy will forward in response to the InvocationRequest type request. 
///           This is the final response from the point of view of the user who invoked the Actor call and its subsequent processing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorInvocationResponse {
    #[prost(string, tag="1")]
    pub actor_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub actor_system: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub updated_context: ::core::option::Option<Context>,
    #[prost(message, optional, tag="4")]
    pub value: ::core::option::Option<::prost_types::Any>,
}
/// InvocationResponse is the response that the proxy that received the InvocationRequest request will forward to the request's original user function.
///
/// Params:
///    status: Status of request. Could be one of [UNKNOWN, OK, ACTOR_NOT_FOUND, ERROR].
///    sytem: The original ActorSystem of the InvocationRequest request.
///    actor: The target Actor originally sent in the InvocationRequest message.
///    value: The value resulting from the request processing that the target Actor made.
///           This value must be passed by the user function to the one who requested the initial request in InvocationRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationResponse {
    #[prost(message, optional, tag="1")]
    pub status: ::core::option::Option<RequestStatus>,
    #[prost(message, optional, tag="2")]
    pub system: ::core::option::Option<ActorSystem>,
    #[prost(message, optional, tag="3")]
    pub actor: ::core::option::Option<Actor>,
    #[prost(message, optional, tag="4")]
    pub value: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestStatus {
    #[prost(enumeration="Status", tag="1")]
    pub status: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Unknown = 0,
    Ok = 1,
    ActorNotFound = 2,
    Error = 3,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Unknown => "UNKNOWN",
            Status::Ok => "OK",
            Status::ActorNotFound => "ACTOR_NOT_FOUND",
            Status::Error => "ERROR",
        }
    }
}
