/// Context is where current and/or updated state is stored
/// to be transmitted to/from proxy and user function
///
/// Params:
///    * state: Actor state passed back and forth between proxy and user function.
///    * metadata: Meta information that comes in invocations
///    * tags: Meta information stored in the actor
///    * caller: ActorId of who is calling target actor
///    * self: ActorId of itself
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<::prost_types::Any>,
    #[prost(map = "string, string", tag = "4")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(map = "string, string", tag = "5")]
    pub tags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Who is calling target actor
    #[prost(message, optional, tag = "2")]
    pub caller: ::core::option::Option<actors::ActorId>,
    /// The target actor itself
    #[prost(message, optional, tag = "3")]
    pub self_: ::core::option::Option<actors::ActorId>,
}
/// Noop is used when the input or output value of a function or method
/// does not matter to the caller of a Workflow or when the user just wants to receive
/// the Context in the request, that is,
/// he does not care about the input value only with the state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Noop {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationRequest {
    #[prost(message, optional, tag = "1")]
    pub service_info: ::core::option::Option<ServiceInfo>,
    #[prost(message, optional, tag = "2")]
    pub actor_system: ::core::option::Option<actors::ActorSystem>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<RequestStatus>,
    #[prost(message, optional, tag = "2")]
    pub proxy_info: ::core::option::Option<ProxyInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceInfo {
    /// The name of the actor system, eg, "my-actor-system".
    #[prost(string, tag = "1")]
    pub service_name: ::prost::alloc::string::String,
    /// The version of the service.
    #[prost(string, tag = "2")]
    pub service_version: ::prost::alloc::string::String,
    /// A description of the runtime for the service. Can be anything, but examples might be:
    /// - node v10.15.2
    /// - OpenJDK Runtime Environment 1.8.0_192-b12
    #[prost(string, tag = "3")]
    pub service_runtime: ::prost::alloc::string::String,
    /// If using a support library, the name of that library, eg "spawn-jvm"
    #[prost(string, tag = "4")]
    pub support_library_name: ::prost::alloc::string::String,
    /// The version of the support library being used.
    #[prost(string, tag = "5")]
    pub support_library_version: ::prost::alloc::string::String,
    /// Spawn protocol major version accepted by the support library.
    #[prost(int32, tag = "6")]
    pub protocol_major_version: i32,
    /// Spawn protocol minor version accepted by the support library.
    #[prost(int32, tag = "7")]
    pub protocol_minor_version: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpawnRequest {
    #[prost(message, repeated, tag = "1")]
    pub actors: ::prost::alloc::vec::Vec<actors::ActorId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpawnResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<RequestStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProxyInfo {
    #[prost(int32, tag = "1")]
    pub protocol_major_version: i32,
    #[prost(int32, tag = "2")]
    pub protocol_minor_version: i32,
    #[prost(string, tag = "3")]
    pub proxy_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub proxy_version: ::prost::alloc::string::String,
}
/// When a Host Function is invoked it returns the updated state and return value to the call.
/// It can also return a number of side effects to other Actors as a result of its computation.
/// These side effects will be forwarded to the respective Actors asynchronously and should not affect the Host Function's response to its caller.
/// Internally side effects is just a special kind of InvocationRequest.
/// Useful for handle handle `recipient list` and `Composed Message Processor` patterns:
/// <https://www.enterpriseintegrationpatterns.com/patterns/messaging/RecipientList.html>
/// <https://www.enterpriseintegrationpatterns.com/patterns/messaging/DistributionAggregate.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SideEffect {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<InvocationRequest>,
}
/// Broadcast a message to many Actors
/// Useful for handle `recipient list`, `publish-subscribe channel`, and `scatter-gatther` patterns:
/// <https://www.enterpriseintegrationpatterns.com/patterns/messaging/RecipientList.html>
/// <https://www.enterpriseintegrationpatterns.com/patterns/messaging/PublishSubscribeChannel.html>
/// <https://www.enterpriseintegrationpatterns.com/patterns/messaging/BroadcastAggregate.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Broadcast {
    /// Channel of target Actors
    #[prost(string, tag = "1")]
    pub channel_group: ::prost::alloc::string::String,
    /// Command. Only Actors that have this command will run successfully
    #[prost(string, tag = "2")]
    pub command_name: ::prost::alloc::string::String,
    /// Payload
    #[prost(oneof = "broadcast::Payload", tags = "3, 4")]
    pub payload: ::core::option::Option<broadcast::Payload>,
}
/// Nested message and enum types in `Broadcast`.
pub mod broadcast {
    /// Payload
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "3")]
        Value(::prost_types::Any),
        #[prost(message, tag = "4")]
        Noop(super::Noop),
    }
}
/// Sends the output of a command of an Actor to the input of another command of an Actor
/// Useful for handle `pipes` pattern:
/// <https://www.enterpriseintegrationpatterns.com/patterns/messaging/PipesAndFilters.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pipe {
    /// Target Actor
    #[prost(string, tag = "1")]
    pub actor: ::prost::alloc::string::String,
    /// Command.
    #[prost(string, tag = "2")]
    pub command_name: ::prost::alloc::string::String,
}
/// Sends the input of a command of an Actor to the input of another command of an Actor
/// Useful for handle `content-basead router` pattern
/// <https://www.enterpriseintegrationpatterns.com/patterns/messaging/ContentBasedRouter.html>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Forward {
    /// Target Actor
    #[prost(string, tag = "1")]
    pub actor: ::prost::alloc::string::String,
    /// Command.
    #[prost(string, tag = "2")]
    pub command_name: ::prost::alloc::string::String,
}
/// Container for archicetural message patterns
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workflow {
    #[prost(message, optional, tag = "2")]
    pub broadcast: ::core::option::Option<Broadcast>,
    #[prost(message, repeated, tag = "1")]
    pub effects: ::prost::alloc::vec::Vec<SideEffect>,
    #[prost(oneof = "workflow::Routing", tags = "3, 4")]
    pub routing: ::core::option::Option<workflow::Routing>,
}
/// Nested message and enum types in `Workflow`.
pub mod workflow {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Routing {
        #[prost(message, tag = "3")]
        Pipe(super::Pipe),
        #[prost(message, tag = "4")]
        Forward(super::Forward),
    }
}
/// The user function when it wants to send a message to an Actor uses the InvocationRequest message type.
///
/// Params:
///    * system: See ActorSystem message.
///    * actor: The target Actor, i.e. the one that the user function is calling to perform some computation.
///    * caller: The caller Actor
///    * command_name: The function or method on the target Actor that will receive this request
///      and perform some useful computation with the sent data.
///    * value: This is the value sent by the user function to be computed by the request's target Actor command.
///    * async: Indicates whether the command should be processed synchronously, where a response should be sent back to the user function,
///             or whether the command should be processed asynchronously, i.e. no response sent to the caller and no waiting.
///    * metadata: Meta information or headers
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationRequest {
    #[prost(message, optional, tag = "1")]
    pub system: ::core::option::Option<actors::ActorSystem>,
    #[prost(message, optional, tag = "2")]
    pub actor: ::core::option::Option<actors::Actor>,
    #[prost(string, tag = "3")]
    pub command_name: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub r#async: bool,
    #[prost(message, optional, tag = "6")]
    pub caller: ::core::option::Option<actors::ActorId>,
    #[prost(map = "string, string", tag = "8")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(int64, tag = "9")]
    pub scheduled_to: i64,
    #[prost(bool, tag = "10")]
    pub pooled: bool,
    #[prost(oneof = "invocation_request::Payload", tags = "4, 7")]
    pub payload: ::core::option::Option<invocation_request::Payload>,
}
/// Nested message and enum types in `InvocationRequest`.
pub mod invocation_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "4")]
        Value(::prost_types::Any),
        #[prost(message, tag = "7")]
        Noop(super::Noop),
    }
}
/// ActorInvocation is a translation message between a local invocation made via InvocationRequest
/// and the real Actor that intends to respond to this invocation and that can be located anywhere in the cluster.
///
/// Params:
///    * actor: The ActorId handling the InvocationRequest request, also called the target Actor.
///    * command_name: The function or method on the target Actor that will receive this request
///                  and perform some useful computation with the sent data.
///    * current_context: The current Context with current state value of the target Actor.
///                     That is, the same as found via matching in %Actor{name: target_actor, state: %ActorState{state: value} = actor_state}.
///                     In this case, the Context type will contain in the value attribute the same `value` as the matching above.
///    * payload: The value to be passed to the function or method corresponding to command_name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorInvocation {
    #[prost(message, optional, tag = "1")]
    pub actor: ::core::option::Option<actors::ActorId>,
    #[prost(string, tag = "2")]
    pub command_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub current_context: ::core::option::Option<Context>,
    #[prost(message, optional, tag = "6")]
    pub caller: ::core::option::Option<actors::ActorId>,
    #[prost(oneof = "actor_invocation::Payload", tags = "4, 5")]
    pub payload: ::core::option::Option<actor_invocation::Payload>,
}
/// Nested message and enum types in `ActorInvocation`.
pub mod actor_invocation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "4")]
        Value(::prost_types::Any),
        #[prost(message, tag = "5")]
        Noop(super::Noop),
    }
}
/// The user function's response after executing the action originated by the local proxy request via ActorInvocation.
///
/// Params:
///    actor_name: The name of the Actor handling the InvocationRequest request, also called the target Actor.
///    actor_system: The name of ActorSystem registered in Registration step.
///    updated_context: The Context with updated state value of the target Actor after user function has processed a request.
///    value: The value that the original request proxy will forward in response to the InvocationRequest type request.
///           This is the final response from the point of view of the user who invoked the Actor call and its subsequent processing.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorInvocationResponse {
    #[prost(string, tag = "1")]
    pub actor_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub actor_system: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub updated_context: ::core::option::Option<Context>,
    #[prost(message, optional, tag = "5")]
    pub workflow: ::core::option::Option<Workflow>,
    #[prost(oneof = "actor_invocation_response::Payload", tags = "4, 6")]
    pub payload: ::core::option::Option<actor_invocation_response::Payload>,
}
/// Nested message and enum types in `ActorInvocationResponse`.
pub mod actor_invocation_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "4")]
        Value(::prost_types::Any),
        #[prost(message, tag = "6")]
        Noop(super::Noop),
    }
}
/// InvocationResponse is the response that the proxy that received the InvocationRequest request will forward to the request's original user function.
///
/// Params:
///    status: Status of request. Could be one of [UNKNOWN, OK, ACTOR_NOT_FOUND, ERROR].
///    system: The original ActorSystem of the InvocationRequest request.
///    actor: The target Actor originally sent in the InvocationRequest message.
///    value: The value resulting from the request processing that the target Actor made.
///           This value must be passed by the user function to the one who requested the initial request in InvocationRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvocationResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<RequestStatus>,
    #[prost(message, optional, tag = "2")]
    pub system: ::core::option::Option<actors::ActorSystem>,
    #[prost(message, optional, tag = "3")]
    pub actor: ::core::option::Option<actors::Actor>,
    #[prost(oneof = "invocation_response::Payload", tags = "4, 5")]
    pub payload: ::core::option::Option<invocation_response::Payload>,
}
/// Nested message and enum types in `InvocationResponse`.
pub mod invocation_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "4")]
        Value(::prost_types::Any),
        #[prost(message, tag = "5")]
        Noop(super::Noop),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestStatus {
    #[prost(enumeration = "Status", tag = "1")]
    pub status: i32,
    #[prost(string, tag = "2")]
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
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "OK" => Some(Self::Ok),
            "ACTOR_NOT_FOUND" => Some(Self::ActorNotFound),
            "ERROR" => Some(Self::Error),
            _ => None,
        }
    }
}
