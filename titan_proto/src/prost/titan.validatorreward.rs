// @generated
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag="1")]
    pub rate: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authority: ::prost::alloc::string::String,
}
/// GenesisState defines the validatorreward module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    /// last_distribute_time is the last block time when the validator reward is
    /// distributed.
    #[prost(message, optional, tag="2")]
    pub last_distribute_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// TimestampProto is a wrapper around google.protobuf.Timestamp.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampProto {
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryRewardPoolRequest is request type for the Query/RewardPool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardPoolRequest {
}
/// QueryRewardPoolResponse is response type for the Query/RewardPool RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardPoolResponse {
    #[prost(message, repeated, tag="1")]
    pub pool: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSetRate allow authority config `rate`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetRate {
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rate: ::prost::alloc::string::String,
}
/// MsgSetRateResponse defines the Msg/SetRate response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetRateResponse {
}
/// MsgSetAuthority allow authority config `authority`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetAuthority {
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_authority: ::prost::alloc::string::String,
}
/// MsgSetAuthorityResponse defines the Msg/SetAuthority response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetAuthorityResponse {
}
/// MsgFundRewardPool allow anyone to fund the reward pool
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundRewardPool {
    #[prost(string, tag="1")]
    pub depositor: ::prost::alloc::string::String,
    /// repeated cosmos.base.v1beta1.Coin amount = 2 [(gogoproto.nullable) = false]
    #[prost(message, repeated, tag="2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgFundRewardPoolResponse defines the Msg/FundRewardPool response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundRewardPoolResponse {
}
// @@protoc_insertion_point(module)
