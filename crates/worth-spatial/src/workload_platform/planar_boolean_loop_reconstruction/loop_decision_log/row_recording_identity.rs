use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanDegenerateLoopOutcomeKind, PlanarBooleanLoopRoleOutcomeKind,
};

use super::counters::PlanarBooleanLoopDecisionLogCounters;
use super::denial::PlanarBooleanLoopDecisionLogDenial;
use super::identity::decision_identity;
use super::input::PlanarBooleanLoopDecisionLogInput;
use super::row::PlanarBooleanLoopDecisionRow;
use super::row_recording::push_row;
use super::vocabulary::{
    PlanarBooleanLoopDecisionAffectedArtifact as Artifact,
    PlanarBooleanLoopDecisionKind as KindRow, PlanarBooleanLoopDecisionPhase as Phase,
    PlanarBooleanLoopDecisionReason as Reason,
};

pub(super) fn record_identity_rows(
    input: PlanarBooleanLoopDecisionLogInput<'_>,
    rows: &mut Vec<PlanarBooleanLoopDecisionRow>,
    seen_decision_identities: &mut std::collections::BTreeSet<String>,
    counters: &mut PlanarBooleanLoopDecisionLogCounters,
) -> Result<(), PlanarBooleanLoopDecisionLogDenial> {
    for row in input.role_outcomes().rows() {
        counters.consumed_role_row();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::RoleClassification.as_str(),
                    Artifact::RoleOutcome.as_str(),
                    row.role_outcome_identity(),
                    row.loop_identity(),
                ),
                Phase::RoleClassification,
                role_kind(row.kind()),
                Artifact::RoleOutcome,
                row.role_outcome_identity().to_string(),
                row.source_loop_identities().to_vec(),
                Vec::new(),
                Vec::new(),
                vec![row.loop_identity().to_string()],
                Some(format!("{:?}", row.kind())),
                Reason::ClassifiedLoopRole,
                "classified the reconstructed loop role from source loop evidence".to_string(),
            ),
        )?;
    }

    for row in input.degenerate_outcomes().rows() {
        counters.consumed_degenerate_row();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::DegeneracyClassification.as_str(),
                    Artifact::DegenerateOutcome.as_str(),
                    row.degenerate_loop_outcome_identity(),
                    row.loop_identity(),
                ),
                Phase::DegeneracyClassification,
                degeneracy_kind(row.kind()),
                Artifact::DegenerateOutcome,
                row.degenerate_loop_outcome_identity().to_string(),
                row.source_loop_identities().to_vec(),
                row.fragment_identities().to_vec(),
                row.split_vertex_identities().to_vec(),
                row.role_outcome_identity()
                    .into_iter()
                    .map(str::to_string)
                    .chain(
                        row.containment_posture_identity()
                            .into_iter()
                            .map(str::to_string),
                    )
                    .collect(),
                Some(row.kind().as_str().to_string()),
                Reason::ClassifiedDegenerateLoop,
                row.human_reason().to_string(),
            ),
        )?;
    }

    for row in input.loop_identity_map().rows() {
        counters.consumed_identity_row();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::IdentityMinting.as_str(),
                    Artifact::LoopIdentityRow.as_str(),
                    row.row_identity(),
                    row.canonical_loop_identity(),
                ),
                Phase::IdentityMinting,
                KindRow::Admitted,
                Artifact::LoopIdentityRow,
                row.row_identity().to_string(),
                row.source_loop_identities().to_vec(),
                row.fragment_identities().to_vec(),
                row.split_vertex_identities().to_vec(),
                vec![
                    row.tracked_loop_identity().to_string(),
                    row.canonical_loop_identity().to_string(),
                    row.role_outcome_identity().to_string(),
                    row.degenerate_outcome_identity().to_string(),
                ],
                Some(format!("{:?}", row.loop_kind())),
                Reason::MintedCanonicalLoopIdentity,
                "minted the canonical loop identity boundary row".to_string(),
            ),
        )?;
    }
    for row in input.persistent_name_map().rows() {
        counters.consumed_propagated_name_row();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::IdentityMinting.as_str(),
                    Artifact::PersistentNameRow.as_str(),
                    row.row_identity(),
                    row.canonical_loop_identity(),
                ),
                Phase::IdentityMinting,
                KindRow::Preserved,
                Artifact::PersistentNameRow,
                row.row_identity().to_string(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                vec![
                    row.canonical_loop_identity().to_string(),
                    row.tracked_loop_identity().to_string(),
                    row.upstream_artifact_identity().to_string(),
                    row.upstream_persistent_name_identity().to_string(),
                ],
                Some(format!("{:?}", row.loop_kind())),
                Reason::PropagatedPersistentName,
                "propagated persistent naming through the canonical loop boundary".to_string(),
            ),
        )?;
    }
    for row in input.subshape_signature_map().rows() {
        counters.consumed_propagated_signature_row();
        push_row(
            rows,
            seen_decision_identities,
            counters,
            PlanarBooleanLoopDecisionRow::new(
                decision_identity(
                    Phase::IdentityMinting.as_str(),
                    Artifact::SubshapeSignatureRow.as_str(),
                    row.row_identity(),
                    row.canonical_loop_identity(),
                ),
                Phase::IdentityMinting,
                KindRow::Derived,
                Artifact::SubshapeSignatureRow,
                row.row_identity().to_string(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                vec![
                    row.canonical_loop_identity().to_string(),
                    row.tracked_loop_identity().to_string(),
                    row.upstream_artifact_identity().to_string(),
                    row.signature_basis_identity().to_string(),
                ],
                Some(format!("{:?}", row.loop_kind())),
                Reason::PropagatedSubshapeSignature,
                "derived loop subshape signature evidence from admitted lineage".to_string(),
            ),
        )?;
    }
    Ok(())
}

fn role_kind(kind: PlanarBooleanLoopRoleOutcomeKind) -> KindRow {
    match kind {
        PlanarBooleanLoopRoleOutcomeKind::PreservedSourceRole => KindRow::Preserved,
        PlanarBooleanLoopRoleOutcomeKind::SingleSourceBornLoopRoleDerivedFromEvidence => {
            KindRow::Derived
        }
        PlanarBooleanLoopRoleOutcomeKind::BornLoopRoleAmbiguous
        | PlanarBooleanLoopRoleOutcomeKind::ContradictorySourceRoleEvidence
        | PlanarBooleanLoopRoleOutcomeKind::MissingSourceRoleEvidence => KindRow::Denied,
    }
}

fn degeneracy_kind(kind: PlanarBooleanDegenerateLoopOutcomeKind) -> KindRow {
    match kind {
        PlanarBooleanDegenerateLoopOutcomeKind::AdmittedForIdentityMinting => KindRow::Admitted,
        PlanarBooleanDegenerateLoopOutcomeKind::DeniedTinyCardinality
        | PlanarBooleanDegenerateLoopOutcomeKind::DeniedSelfTouching
        | PlanarBooleanDegenerateLoopOutcomeKind::DeniedZeroArea => KindRow::Denied,
        PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredGeometryEvidence
        | PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredRoleEvidence
        | PlanarBooleanDegenerateLoopOutcomeKind::PolicyRequiredContainmentEvidence => {
            KindRow::PolicyRequired
        }
    }
}
