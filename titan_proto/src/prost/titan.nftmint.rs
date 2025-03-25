// @generated
/// EventCreateClass is emitted on CreateClass.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventCreateClass {
    /// id is a unique identifier of the class.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// owner is the owner address of the class.
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
/// EventUpdateClass is emitted on UpdateClass.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventUpdateClass {
    /// id is a unique identifier of the class.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/// EventTransferClass is emitted on TransferClass.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTransferClass {
    /// id is a unique identifier of the class.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// old_owner is the old owner address of the class.
    #[prost(string, tag="2")]
    pub old_owner: ::prost::alloc::string::String,
    /// new_owner is the new owner address of the class.
    #[prost(string, tag="3")]
    pub new_owner: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
}
/// SystemInfo defines the system info of this module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemInfo {
    /// next_class_id is the unique identifier of the next class that will be
    /// created.
    #[prost(uint64, tag="1")]
    pub next_class_id: u64,
}
/// MintingInfo defines the minting info for a class.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintingInfo {
    /// class_id is a unique identifier of the class.
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
    /// owner is the owner address of the class.
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    /// next_token_id is the unique identifier of the next token that will be
    /// minted under this class.
    #[prost(uint64, tag="3")]
    pub next_token_id: u64,
}
/// GenesisState defines the nftmint module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, optional, tag="2")]
    pub system_info: ::core::option::Option<SystemInfo>,
    #[prost(message, repeated, tag="3")]
    pub minting_info_list: ::prost::alloc::vec::Vec<MintingInfo>,
}
/// Metadata represents class's and token's metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    #[prost(string, tag="1")]
    pub data: ::prost::alloc::string::String,
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
/// QuerySystemInfoRequest is request type for the Query/QuerySystemInfo RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySystemInfoRequest {
}
/// QuerySystemInfoResponse is response type for the Query/QuerySystemInfo
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySystemInfoResponse {
    #[prost(message, optional, tag="1")]
    pub system_info: ::core::option::Option<SystemInfo>,
}
/// QueryMintingInfoRequest is request type for the Query/QueryMintingInfo RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMintingInfoRequest {
    #[prost(string, tag="1")]
    pub class_id: ::prost::alloc::string::String,
}
/// QueryMintingInfoResponse is response type for the Query/QueryMintingInfo RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMintingInfoResponse {
    #[prost(message, optional, tag="1")]
    pub minting_info: ::core::option::Option<MintingInfo>,
}
/// QueryMintingInfosRequest is request type for the Query/QueryMintingInfos RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMintingInfosRequest {
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryMintingInfosResponse is response type for the Query/QueryMintingInfos
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMintingInfosResponse {
    #[prost(message, repeated, tag="1")]
    pub minting_info: ::prost::alloc::vec::Vec<MintingInfo>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgCreateClass represents a message to create new class.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClass {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub data: ::prost::alloc::string::String,
}
/// MsgCreateClassResponse defines the Msg/CreateClass response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateClassResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/// MsgUpdateClass represents a message to update a class.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClass {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub data: ::prost::alloc::string::String,
}
/// MsgUpdateClassResponse defines the Msg/UpdateClass response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateClassResponse {
}
/// MsgTransferClass represents a message to transfer a class to new owner.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferClass {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub receiver: ::prost::alloc::string::String,
}
/// MsgTransferClassResponse defines the Msg/TransferClass response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferClassResponse {
}
/// MsgMint represents a message to mint new NFT.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMint {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub class_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub uri_hash: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub data: ::prost::alloc::string::String,
}
/// MsgMintResponse defines the Msg/Mint response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
