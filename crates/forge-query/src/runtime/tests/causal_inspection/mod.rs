use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::inspection::CausalEvidenceReferenceDigest;

mod admission;
mod adversarial;
mod anchor_reference;
pub(in crate::runtime::tests) mod certification;
mod dx;
mod materialization;
mod reference_index;

pub(in crate::runtime::tests) fn causal_test_reference_digest(
    reference_label: impl AsRef<str>,
) -> CausalEvidenceReferenceDigest {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalEvidenceReference)
        .field_value(
            ForgeQueryEvidenceTag::new("fixture_reference"),
            reference_label.as_ref(),
        )
        .seal()
        .into()
}
