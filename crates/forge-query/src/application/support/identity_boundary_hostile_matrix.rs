use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy,
    ForgeQueryGraphCompositionDomainInvariantDenial,
    ForgeQueryGraphCompositionDomainInvariantSummary, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntimeError, ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryRuntimeFamilyTeachingPosture,
    ForgeQueryRuntimeSupportDenial, ForgeQueryStopClass,
};
use crate::session_label::ForgeQuerySessionLabel;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIdentityBoundaryHostileMatrixRow {
    name: &'static str,
    certified: bool,
    witness_digest: String,
}

impl ForgeQueryIdentityBoundaryHostileMatrixRow {
    fn new(name: &'static str, certified: bool, witness_digest: String) -> Self {
        Self {
            name,
            certified,
            witness_digest,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn certified(&self) -> bool {
        self.certified
    }

    pub fn witness_digest(&self) -> &str {
        &self.witness_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIdentityBoundaryHostileMatrixArtifact {
    suite_name: &'static str,
    certified: bool,
    canonical_rows: Vec<ForgeQueryIdentityBoundaryHostileMatrixRow>,
    rejection_rows: Vec<ForgeQueryIdentityBoundaryHostileMatrixRow>,
    artifact_digest: String,
}

impl ForgeQueryIdentityBoundaryHostileMatrixArtifact {
    pub fn suite_name(&self) -> &'static str {
        self.suite_name
    }

    pub fn certified(&self) -> bool {
        self.certified
    }

    pub fn canonical_rows(&self) -> &[ForgeQueryIdentityBoundaryHostileMatrixRow] {
        &self.canonical_rows
    }

    pub fn rejection_rows(&self) -> &[ForgeQueryIdentityBoundaryHostileMatrixRow] {
        &self.rejection_rows
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }
}

pub const MILESTONE_NINE_SIX_SUITE_NAME: &str =
    "Milestone 9.6 Identity And Stop-Class Hostile Certification Matrix";

pub const MILESTONE_NINE_SIX_REQUIRED_CANONICAL_ROW_NAMES: &[&str] = &[
    "evidence-identity-delimiter-collision-resistance",
    "family-admission-message-rewording-stability",
    "graph-domain-invariant-message-rewording-stability",
    "session-label-render-collision-distinctness",
    "session-label-same-family-replay-collision",
];

pub const MILESTONE_NINE_SIX_REQUIRED_REJECTION_ROW_NAMES: &[&str] = &[
    "joined-string-evidence-identity-collapses-distinct-fields",
    "consumer-message-substring-routing-drifts",
];

pub fn identity_boundary_hostile_matrix_artifact() -> ForgeQueryIdentityBoundaryHostileMatrixArtifact
{
    let canonical_rows = vec![
        evidence_identity_delimiter_collision_resistance_row(),
        family_admission_message_rewording_stability_row(),
        graph_domain_invariant_message_rewording_stability_row(),
        session_label_render_collision_distinctness_row(),
        session_label_same_family_replay_collision_row(),
    ];
    let rejection_rows = vec![
        joined_string_evidence_identity_collapse_row(),
        consumer_message_substring_routing_drift_row(),
    ];
    let certified = canonical_rows
        .iter()
        .chain(rejection_rows.iter())
        .all(ForgeQueryIdentityBoundaryHostileMatrixRow::certified);
    let artifact_digest =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact)
            .field_shape(
                ForgeQueryEvidenceTag::new("suite_name"),
                MILESTONE_NINE_SIX_SUITE_NAME,
            )
            .field_bool(ForgeQueryEvidenceTag::new("certified"), certified)
            .field_identity_sequence(
                ForgeQueryEvidenceTag::new("canonical_row"),
                canonical_rows
                    .iter()
                    .map(ForgeQueryIdentityBoundaryHostileMatrixRow::name),
            )
            .field_identity_sequence(
                ForgeQueryEvidenceTag::new("canonical_row_witness"),
                canonical_rows
                    .iter()
                    .map(ForgeQueryIdentityBoundaryHostileMatrixRow::witness_digest),
            )
            .field_identity_sequence(
                ForgeQueryEvidenceTag::new("rejection_row"),
                rejection_rows
                    .iter()
                    .map(ForgeQueryIdentityBoundaryHostileMatrixRow::name),
            )
            .field_identity_sequence(
                ForgeQueryEvidenceTag::new("rejection_row_witness"),
                rejection_rows
                    .iter()
                    .map(ForgeQueryIdentityBoundaryHostileMatrixRow::witness_digest),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("combined_drift_axes"),
                "digest-delimiter|message-reword|label-collision",
            )
            .seal()
            .as_str()
            .to_string();
    ForgeQueryIdentityBoundaryHostileMatrixArtifact {
        suite_name: MILESTONE_NINE_SIX_SUITE_NAME,
        certified,
        canonical_rows,
        rejection_rows,
        artifact_digest,
    }
}

pub fn identity_boundary_hostile_matrix_digest() -> String {
    identity_boundary_hostile_matrix_artifact()
        .artifact_digest()
        .to_string()
}

fn evidence_identity_delimiter_collision_resistance_row(
) -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let authority = ForgeQueryRuntimeEvidenceAuthority::new();
    let left = ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::ForgeQueryBasisAdmissionEvidenceRow::rows_from_values([
            "alpha",
            "beta|gamma",
        ]),
    );
    let right = ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::ForgeQueryBasisAdmissionEvidenceRow::rows_from_values([
            "basis|alpha",
            "beta|gamma",
        ]),
    );
    let certified = left.admission_digest() != right.admission_digest();
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
        "evidence-identity-delimiter-collision-resistance",
        certified,
        witness_digest(
            "evidence-identity-delimiter-collision-resistance",
            certified,
            [
                left.admission_digest().as_str(),
                right.admission_digest().as_str(),
            ],
        ),
    )
}

fn family_admission_message_rewording_stability_row() -> ForgeQueryIdentityBoundaryHostileMatrixRow
{
    let first_error =
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(ForgeQueryRuntimeSupportDenial::new(
            ForgeQueryRuntimeFacadeFamily::Temporal,
            ForgeQueryRuntimeFamilySupportStatus::Supported,
            Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            "first temporal wording",
        ));
    let second_error =
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(ForgeQueryRuntimeSupportDenial::new(
            ForgeQueryRuntimeFacadeFamily::Temporal,
            ForgeQueryRuntimeFamilySupportStatus::Supported,
            Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            "second temporal wording",
        ));
    let first_message = first_error.to_string();
    let second_message = second_error.to_string();
    let certified = matches!(
        first_error.stop_class(),
        ForgeQueryStopClass::FamilyAdmissionDenied {
            family: ForgeQueryRuntimeFacadeFamily::Temporal,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            ..
        }
    ) && matches!(
        second_error.stop_class(),
        ForgeQueryStopClass::FamilyAdmissionDenied {
            family: ForgeQueryRuntimeFacadeFamily::Temporal,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            ..
        }
    ) && first_message != second_message;
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
        "family-admission-message-rewording-stability",
        certified,
        witness_digest(
            "family-admission-message-rewording-stability",
            certified,
            [first_message.as_str(), second_message.as_str()],
        ),
    )
}

fn graph_domain_invariant_message_rewording_stability_row(
) -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let first_error = ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(
        graph_domain_invariant_denial("graph domain invariant failed first"),
    );
    let second_error = ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(
        graph_domain_invariant_denial("graph domain invariant failed second"),
    );
    let first_message = first_error.to_string();
    let second_message = second_error.to_string();
    let certified = matches!(
        first_error.stop_class(),
        ForgeQueryStopClass::GraphCompositionDomainInvariantDenied { .. }
    ) && matches!(
        second_error.stop_class(),
        ForgeQueryStopClass::GraphCompositionDomainInvariantDenied { .. }
    ) && first_message != second_message;
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
        "graph-domain-invariant-message-rewording-stability",
        certified,
        witness_digest(
            "graph-domain-invariant-message-rewording-stability",
            certified,
            [first_message.as_str(), second_message.as_str()],
        ),
    )
}

fn session_label_render_collision_distinctness_row() -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let left = ForgeQuerySessionLabel::scoped_strs("worth.kernel", ["preview"]).expect("label");
    let right = ForgeQuerySessionLabel::scoped_strs("worth", ["kernel", "preview"]).expect("label");
    let certified =
        left.display() == right.display() && left.identity_digest() != right.identity_digest();
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
        "session-label-render-collision-distinctness",
        certified,
        witness_digest(
            "session-label-render-collision-distinctness",
            certified,
            [
                left.identity_digest().as_str(),
                right.identity_digest().as_str(),
            ],
        ),
    )
}

fn session_label_same_family_replay_collision_row() -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let label = test_session_label("stop-class-collision");
    let error = ForgeQueryRuntimeError::SessionLabelCollision {
        authority_lane: ForgeQueryAuthorityLane::BranchLocalTruth,
        label: label.clone(),
    };
    let certified = matches!(
        error.stop_class(),
        ForgeQueryStopClass::SessionLabelCollision {
            authority_lane: ForgeQueryAuthorityLane::BranchLocalTruth,
            label: collision_label,
        } if collision_label == &label
    );
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
        "session-label-same-family-replay-collision",
        certified,
        witness_digest(
            "session-label-same-family-replay-collision",
            certified,
            [label.identity_digest().as_str()],
        ),
    )
}

fn joined_string_evidence_identity_collapse_row() -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let authority = ForgeQueryRuntimeEvidenceAuthority::new();
    let left = ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview|basis"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::ForgeQueryBasisAdmissionEvidenceRow::rows_from_values([
            "alpha",
            "beta|gamma",
        ]),
    );
    let right = ForgeQueryPreviewBasisAdmission::new(
        &authority,
        test_session_label("preview"),
        ForgeQueryEffectPolicy::SandboxedWriteIntent,
        crate::runtime::ForgeQueryBasisAdmissionEvidenceRow::rows_from_values([
            "basis|alpha",
            "beta|gamma",
        ]),
    );
    let naive_left = ["preview|basis", "alpha", "beta|gamma"].join("|");
    let naive_right = ["preview", "basis|alpha", "beta|gamma"].join("|");
    let certified =
        naive_left == naive_right && left.admission_digest() != right.admission_digest();
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
        "joined-string-evidence-identity-collapses-distinct-fields",
        certified,
        witness_digest(
            "joined-string-evidence-identity-collapses-distinct-fields",
            certified,
            [naive_left.as_str(), naive_right.as_str()],
        ),
    )
}

fn consumer_message_substring_routing_drift_row() -> ForgeQueryIdentityBoundaryHostileMatrixRow {
    let first_error =
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(ForgeQueryRuntimeSupportDenial::new(
            ForgeQueryRuntimeFacadeFamily::Temporal,
            ForgeQueryRuntimeFamilySupportStatus::Supported,
            Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            "first temporal wording",
        ));
    let second_error =
        ForgeQueryRuntimeError::UnsupportedFacadeFamily(ForgeQueryRuntimeSupportDenial::new(
            ForgeQueryRuntimeFacadeFamily::Temporal,
            ForgeQueryRuntimeFamilySupportStatus::Supported,
            Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            "public runtime DX remains support-gated",
        ));
    let first_message = first_error.to_string();
    let second_message = second_error.to_string();
    let first_contains_support_gated = first_message.contains("support-gated");
    let second_contains_support_gated = second_message.contains("support-gated");
    let certified = matches!(
        first_error.stop_class(),
        ForgeQueryStopClass::FamilyAdmissionDenied {
            family: ForgeQueryRuntimeFacadeFamily::Temporal,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            ..
        }
    ) && matches!(
        second_error.stop_class(),
        ForgeQueryStopClass::FamilyAdmissionDenied {
            family: ForgeQueryRuntimeFacadeFamily::Temporal,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
            ..
        }
    ) && first_contains_support_gated != second_contains_support_gated;
    ForgeQueryIdentityBoundaryHostileMatrixRow::new(
        "consumer-message-substring-routing-drifts",
        certified,
        witness_digest(
            "consumer-message-substring-routing-drifts",
            certified,
            [first_message.as_str(), second_message.as_str()],
        ),
    )
}

fn graph_domain_invariant_denial(message: &str) -> ForgeQueryGraphCompositionDomainInvariantDenial {
    ForgeQueryGraphCompositionDomainInvariantDenial::from_contributed(
        "graph.family",
        message,
        ForgeQueryGraphCompositionDomainInvariantSummary::from_parts(
            vec!["Task".to_string()],
            vec!["task_symbol".to_string()],
            vec!["same_batch_entity_relation_identity_edges".to_string()],
            vec!["mixed_existing_target_followup_mutation".to_string()],
            "program-digest".to_string(),
            "breadth-digest".to_string(),
            "components=1".to_string(),
        ),
    )
}

fn test_session_label(label: &str) -> ForgeQuerySessionLabel {
    ForgeQuerySessionLabel::scoped_strs("forge-query-identity-boundary", [label]).expect("label")
}

fn witness_digest<'a>(
    row_name: &'static str,
    certified: bool,
    evidence: impl IntoIterator<Item = &'a str>,
) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact)
        .field_shape(ForgeQueryEvidenceTag::new("row_name"), row_name)
        .field_bool(ForgeQueryEvidenceTag::new("certified"), certified)
        .field_identity_sequence(ForgeQueryEvidenceTag::new("evidence"), evidence)
        .seal()
        .as_str()
        .to_string()
}
