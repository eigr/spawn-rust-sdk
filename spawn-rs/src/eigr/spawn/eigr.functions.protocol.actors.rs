#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Registry {
    #[prost(map = "string, message", tag = "1")]
    pub actors: ::std::collections::HashMap<::prost::alloc::string::String, Actor>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorSystem {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub registry: ::core::option::Option<Registry>,
}
/// A strategy for save state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorSnapshotStrategy {
    #[prost(oneof = "actor_snapshot_strategy::Strategy", tags = "1")]
    pub strategy: ::core::option::Option<actor_snapshot_strategy::Strategy>,
}
/// Nested message and enum types in `ActorSnapshotStrategy`.
pub mod actor_snapshot_strategy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Strategy {
        /// the timeout strategy.
        #[prost(message, tag = "1")]
        Timeout(super::TimeoutStrategy),
    }
}
/// A strategy which a user function's entity is passivated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorDeactivationStrategy {
    #[prost(oneof = "actor_deactivation_strategy::Strategy", tags = "1")]
    pub strategy: ::core::option::Option<actor_deactivation_strategy::Strategy>,
}
/// Nested message and enum types in `ActorDeactivationStrategy`.
pub mod actor_deactivation_strategy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Strategy {
        /// the timeout strategy.
        #[prost(message, tag = "1")]
        Timeout(super::TimeoutStrategy),
    }
}
/// A strategy based on a timeout.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeoutStrategy {
    /// The timeout in millis
    #[prost(int64, tag = "1")]
    pub timeout: i64,
}
/// A command represents an action that the user can perform on an Actor.
/// Commands in supporting languages are represented by functions or methods.
/// An Actor command has nothing to do with the semantics of Commands in a CQRS/EventSourced system.
/// It just represents an action that supporting languages can invoke.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    /// The name of the function or method in the supporting language that has been registered in Ator.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A FixedTimerCommand is similar to a regular Command, its main differences are that it is scheduled to run at regular intervals
/// and only takes the actor's state as an argument.
/// Timer Commands are good for executing loops that manipulate the actor's own state.
/// In Elixir or other languages in BEAM it would be similar to invoking Process.send_after(self(), atom, msg, timeout)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedTimerCommand {
    /// The time to wait until the command is triggered
    #[prost(int32, tag = "1")]
    pub seconds: i32,
    /// See Command description Above
    #[prost(message, optional, tag = "2")]
    pub command: ::core::option::Option<Command>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorState {
    #[prost(map = "string, string", tag = "1")]
    pub tags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<::prost_types::Any>,
}
/// TODO doc here
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// A channel group represents a way to send commands to various actors
    /// that belong to a certain semantic group.
    #[prost(string, tag = "1")]
    pub channel_group: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub tags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorSettings {
    /// Indicates the type of Actor to be configured.
    #[prost(enumeration = "Kind", tag = "1")]
    pub kind: i32,
    /// Indicates whether an actor's state should be persisted in a definitive store.
    #[prost(bool, tag = "2")]
    pub stateful: bool,
    /// Snapshot strategy
    #[prost(message, optional, tag = "3")]
    pub snapshot_strategy: ::core::option::Option<ActorSnapshotStrategy>,
    /// Deactivate strategy
    #[prost(message, optional, tag = "4")]
    pub deactivation_strategy: ::core::option::Option<ActorDeactivationStrategy>,
    /// When kind is POOLED this is used to define minimun actor instances
    #[prost(int32, tag = "5")]
    pub min_pool_size: i32,
    /// When kind is POOLED this is used to define maximum actor instances
    #[prost(int32, tag = "6")]
    pub max_pool_size: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorId {
    /// The name of a Actor Entity.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Name of a ActorSystem
    #[prost(string, tag = "2")]
    pub system: ::prost::alloc::string::String,
    /// When the Actor is of the Abstract type,
    /// the name of the parent Actor must be informed here.
    #[prost(string, tag = "3")]
    pub parent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actor {
    /// Actor Identification
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<ActorId>,
    /// A Actor state.
    #[prost(message, optional, tag = "2")]
    pub state: ::core::option::Option<ActorState>,
    /// Actor metadata
    #[prost(message, optional, tag = "6")]
    pub metadata: ::core::option::Option<Metadata>,
    /// Actor settings.
    #[prost(message, optional, tag = "3")]
    pub settings: ::core::option::Option<ActorSettings>,
    /// The commands registered for an actor
    #[prost(message, repeated, tag = "4")]
    pub commands: ::prost::alloc::vec::Vec<Command>,
    /// The registered timer commands for an actor.
    #[prost(message, repeated, tag = "5")]
    pub timer_commands: ::prost::alloc::vec::Vec<FixedTimerCommand>,
}
/// The type that defines the runtime characteristics of the Actor.
/// Regardless of the type of actor it is important that
/// all actors are registered during the proxy and host initialization phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Kind {
    /// When no type is informed, the default to be assumed will be the Singleton pattern.
    UnknowKind = 0,
    /// Abstract actors are used to create children of this based actor at runtime
    Abstract = 1,
    /// Singleton actors as the name suggests have only one real instance of themselves running
    /// during their entire lifecycle. That is, they are the opposite of the Abstract type Actors.
    Singleton = 2,
    /// Pooled Actors are similar to abstract actors, but unlike them,
    /// their identifying name will always be the one registered at the system initialization stage.
    /// The great advantage of Pooled actors is that they have multiple instances of themselves
    /// acting as a request service pool.
    /// Pooled actors are also stateless actors, that is, they will not have their
    /// in-memory state persisted via Statesstore. This is done to avoid problems
    /// with the correctness of the stored state.
    /// Pooled Actors are generally used for tasks where the Actor Model would perform worse
    /// than other concurrency models and for tasks that do not require state concerns.
    /// Integration flows, data caching, proxies are good examples of use cases
    /// for this type of Actor.
    Pooled = 3,
    /// Reserved for future use
    Proxy = 4,
}
impl Kind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Kind::UnknowKind => "UNKNOW_KIND",
            Kind::Abstract => "ABSTRACT",
            Kind::Singleton => "SINGLETON",
            Kind::Pooled => "POOLED",
            Kind::Proxy => "PROXY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOW_KIND" => Some(Self::UnknowKind),
            "ABSTRACT" => Some(Self::Abstract),
            "SINGLETON" => Some(Self::Singleton),
            "POOLED" => Some(Self::Pooled),
            "PROXY" => Some(Self::Proxy),
            _ => None,
        }
    }
}
