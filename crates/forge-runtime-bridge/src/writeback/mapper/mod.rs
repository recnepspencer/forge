mod envelope;
mod mapped_input;
mod record;
mod witness;

pub use envelope::BridgeWritebackMapperEnvelope;
pub use mapped_input::BridgeMappedWritebackFamilyInput;
pub use record::BridgeWritebackMapperRecord;
pub use witness::BridgeWritebackMapperWitness;

use crate::identity::{
    BridgeIdentity, WritebackMappedFamilyInputIdentityTag, WritebackMapperEnvelopeIdentityTag,
    WritebackMapperRecordIdentityTag, WritebackMapperWitnessIdentityTag,
};

pub type BridgeMappedWritebackFamilyInputIdentity =
    BridgeIdentity<WritebackMappedFamilyInputIdentityTag>;
pub type BridgeWritebackMapperEnvelopeIdentity = BridgeIdentity<WritebackMapperEnvelopeIdentityTag>;
pub type BridgeWritebackMapperWitnessIdentity = BridgeIdentity<WritebackMapperWitnessIdentityTag>;
pub type BridgeWritebackMapperRecordIdentity = BridgeIdentity<WritebackMapperRecordIdentityTag>;
