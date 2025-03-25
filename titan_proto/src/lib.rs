use cosmos_sdk_proto::cosmos;

pub mod titan {
    pub mod nftmint {
        include!("prost/titan.nftmint.rs");
        #[cfg(feature = "grpc")]
        include!("prost/titan.nftmint.tonic.rs");
    }
    pub mod tokenfactory {
        pub mod v1beta1 {
            include!("prost/titan.tokenfactory.v1beta1.rs");
            #[cfg(feature = "grpc")]
            include!("prost/titan.tokenfactory.v1beta1.tonic.rs");
        }
    }
    pub mod validatorreward {
        include!("prost/titan.validatorreward.rs");
        #[cfg(feature = "grpc")]
        include!("prost/titan.validatorreward.tonic.rs");
    }
}
