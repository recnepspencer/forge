use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::inspection::CausalEvidenceReferenceDigest;
use forge_runtime_bridge::facade::{
    BridgeCausalInspectionAdmissionSummary, BridgeIdentityEvidence,
};

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

pub(in crate::runtime::tests::causal_inspection) fn bridge_external_evidence(
    value: impl AsRef<str>,
) -> BridgeIdentityEvidence {
    BridgeIdentityEvidence::from_external_authority(value)
}

pub(in crate::runtime::tests::causal_inspection) fn bridge_query_evidence(
    scope: &str,
    token: &str,
) -> BridgeIdentityEvidence {
    BridgeIdentityEvidence::from_query_evidence_identity(scope, token)
}

pub(in crate::runtime::tests::causal_inspection) fn bridge_admitted_summary(
    admitted: &crate::runtime::AdmittedCausalInspection,
) -> BridgeCausalInspectionAdmissionSummary {
    BridgeCausalInspectionAdmissionSummary::admitted(
        bridge_query_evidence(
            "causal-inspection-outcome",
            admitted.admitted_inspection_digest(),
        ),
        bridge_query_evidence(
            "causal-observation-anchor",
            admitted.subject().anchor_for_reporting(),
        ),
    )
    .expect("query admission summary should be valid")
}

pub(in crate::runtime::tests::causal_inspection) fn bridge_advisory_summary(
    advisory: &crate::runtime::AdvisoryCausalInspection,
) -> BridgeCausalInspectionAdmissionSummary {
    BridgeCausalInspectionAdmissionSummary::advisory(
        bridge_query_evidence(
            "causal-inspection-outcome",
            advisory.advisory_inspection_digest(),
        ),
        bridge_query_evidence(
            "causal-observation-anchor",
            advisory.subject().anchor_for_reporting(),
        ),
    )
    .expect("query advisory summary should be valid")
}
