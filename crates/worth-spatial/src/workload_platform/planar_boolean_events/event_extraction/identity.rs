use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::{PlanarBooleanEventExtractionDenialKind, PlanarBooleanEventExtractionPolicyExitKind};

pub(crate) struct EventExtractionIdentityBasis<'a> {
    pub(crate) label: &'static str,
    pub(crate) kind_key: &'static str,
    pub(crate) reduced_pair_identity: &'a str,
    pub(crate) carrier_identity: Option<&'a str>,
    pub(crate) segment_pair_identity: Option<&'a str>,
    pub(crate) predicate_binding_identity: Option<&'a str>,
    pub(crate) precision_basis_identity: Option<&'a str>,
    pub(crate) workload_evidence_stage: WorkloadEvidenceStage,
}

pub(crate) fn denial_identity(
    kind: PlanarBooleanEventExtractionDenialKind,
    basis: &EventExtractionIdentityBasis<'_>,
) -> String {
    artifact_identity(kind.query_key(), basis)
}

pub(crate) fn policy_exit_identity(
    kind: PlanarBooleanEventExtractionPolicyExitKind,
    basis: &EventExtractionIdentityBasis<'_>,
) -> String {
    artifact_identity(kind.query_key(), basis)
}

fn artifact_identity(kind_key: &'static str, basis: &EventExtractionIdentityBasis<'_>) -> String {
    let parts = [
        basis.label.to_string(),
        kind_key.to_string(),
        basis.kind_key.to_string(),
        basis.reduced_pair_identity.to_string(),
        basis.carrier_identity.unwrap_or("carrier:none").to_string(),
        basis
            .segment_pair_identity
            .unwrap_or("segment-pair:none")
            .to_string(),
        basis
            .predicate_binding_identity
            .unwrap_or("predicate-binding:none")
            .to_string(),
        basis
            .precision_basis_identity
            .unwrap_or("precision:none")
            .to_string(),
        basis.workload_evidence_stage.human_name().to_string(),
    ];
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
