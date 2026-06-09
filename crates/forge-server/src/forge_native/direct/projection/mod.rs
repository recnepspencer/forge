mod artifact;
mod fact_receipt;
mod materialization_digest;
mod request;

pub use artifact::ForgeServerDirectProjection;
pub use fact_receipt::ForgeServerDirectProjectionFactReceipt;
pub use materialization_digest::ForgeServerDirectMaterializationDigest;
pub use request::ForgeServerDirectProjectionRequest;

pub type ForgeServerDirectProjectionConsumption = ForgeServerDirectProjection;
pub type ForgeServerDirectFactReceipt = ForgeServerDirectProjectionFactReceipt;
