// @generated
/// DenomAuthorityMetadata specifies metadata for addresses that have specific
/// capabilities over a token factory denom. Right now there is only one Admin
/// permission, but is planned to be extended to the future.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomAuthorityMetadata {
    /// Can be empty for no admin, or a valid titan address
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
}
/// Params defines the parameters for the tokenfactory module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// DenomCreationFee defines the fee to be charged on the creation of a new
    /// denom. The fee is drawn from the MsgCreateDenom's sender account, and
    /// transferred to the community pool.
    #[prost(message, repeated, tag="1")]
    pub denom_creation_fee: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// DenomCreationGasConsume defines the gas cost for creating a new denom.
    /// This is intended as a spam deterrence mechanism.
    ///
    /// See: <https://github.com/CosmWasm/token-factory/issues/11>
    #[prost(uint64, tag="2")]
    pub denom_creation_gas_consume: u64,
}
/// GenesisState defines the tokenfactory module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub factory_denoms: ::prost::alloc::vec::Vec<GenesisDenom>,
}
/// GenesisDenom defines a tokenfactory denom that is defined within genesis
/// state. The structure contains DenomAuthorityMetadata which defines the
/// denom's admin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisDenom {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryDenomAuthorityMetadataRequest is request type for the
/// Query/DenomAuthorityMetadata RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomAuthorityMetadataRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryDenomAuthorityMetadataResponse is response type for the
/// Query/DenomAuthorityMetadata RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomAuthorityMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
/// QueryDenomsFromCreatorRequest is request type for the
/// Query/DenomsFromCreator RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomsFromCreatorRequest {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
}
/// QueryDenomsFromCreatorResponse is response type for the
/// Query/DenomsFromCreator RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomsFromCreatorResponse {
    #[prost(string, repeated, tag="1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgCreateDenom defines the message structure for the CreateDenom gRPC service
/// method. It allows an account to create a new denom. It requires a sender
/// address and a sub denomination. The (sender_address, sub_denomination) tuple
/// must be unique and cannot be re-used.
///
/// The resulting denom created is defined as
/// <factory/{creatorAddress}/{subdenom}>. The resulting denom's admin is
/// originally set to be the creator, but this can be changed later. The token
/// denom does not indicate the current admin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDenom {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag="2")]
    pub subdenom: ::prost::alloc::string::String,
}
/// MsgMintResponse defines the Msg/CreateDenom response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDenomResponse {
    #[prost(string, tag="1")]
    pub new_token_denom: ::prost::alloc::string::String,
}
/// MsgMint is the sdk.Msg type for allowing an admin account to mint more of a
/// token. Only the admin of the token factory denom has permission to mint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMint {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub mint_to_address: ::prost::alloc::string::String,
}
/// MsgMintResponse defines the Msg/Mint response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintResponse {
}
/// MsgBurn is the sdk.Msg type for allowing an admin account to burn a token.
/// Only the admin of the token factory denom has permission to burn the denom.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgMintResponse defines the Msg/Burn response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnResponse {
}
/// MsgChangeAdmin is the sdk.Msg type for allowing an admin account to reassign
/// adminship of a denom to a new account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChangeAdmin {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_admin: ::prost::alloc::string::String,
}
/// MsgMintResponse defines the Msg/ChangeAdmin response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChangeAdminResponse {
}
/// MsgSetDenomMetadata is the sdk.Msg type for allowing an admin account to set
/// the denom's bank metadata
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDenomMetadata {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// MsgMintResponse defines the Msg/SetDenomMetadata response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDenomMetadataResponse {
}
/// MsgUpdateParamsResponse defines the Msg/UpdateParams request type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless
    /// overwritten).
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/tokenfactory parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the Msg/UpdateParams response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
// @@protoc_insertion_point(module)
