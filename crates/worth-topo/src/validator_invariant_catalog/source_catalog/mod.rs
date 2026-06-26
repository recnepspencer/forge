mod invariant_registration_rows;
mod source_proof;
mod validator_rule_rows;

use crate::validator_invariant_catalog::family_record::WorthTopologyLegalityFamilyRecordInput;
use crate::validator_invariant_catalog::{
    WorthTopologyInvariantFamilyIdentity, WorthTopologyValidatorFamilyIdentity,
};

pub(super) use invariant_registration_rows::current_invariant_family_inputs;
pub(in crate::validator_invariant_catalog) use source_proof::WorthTopologyLegalityFamilySourceProofInput;
pub use source_proof::{
    WorthTopologyLegalityFamilySourceAuthorityKind, WorthTopologyLegalityFamilySourceProof,
};
pub(super) use validator_rule_rows::current_validator_family_inputs;

pub(super) struct WorthTopologyValidatorFamilySourceRow {
    pub input: WorthTopologyLegalityFamilyRecordInput<WorthTopologyValidatorFamilyIdentity>,
    pub source_proof: WorthTopologyLegalityFamilySourceProof,
}

pub(super) struct WorthTopologyInvariantFamilySourceRow {
    pub input: WorthTopologyLegalityFamilyRecordInput<WorthTopologyInvariantFamilyIdentity>,
    pub source_proof: WorthTopologyLegalityFamilySourceProof,
}
