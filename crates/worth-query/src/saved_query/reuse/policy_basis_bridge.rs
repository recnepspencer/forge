use crate::policy_basis::{
    saved_query_policy_reuse_artifact_digest, saved_query_policy_reuse_disposition,
    saved_query_policy_reuse_surface_posture, SavedQueryPolicyReuseDisposition,
};
use crate::saved_query::SavedQueryTemporalAsyncSurfacePosture;
use crate::saved_query::{SavedQueryArtifact, SavedQueryReuseDescriptor};

use super::{binding_row, SavedQueryBindingMatrixRow, SavedQueryRebindingDimension};

pub(super) fn evaluate_policy_basis_reuse(
    artifact: &SavedQueryArtifact,
    descriptor: &SavedQueryReuseDescriptor,
) -> SavedQueryBindingMatrixRow {
    let source_posture = artifact.metadata().temporal_async_surface_posture();
    let target_posture = descriptor.temporal_async_surface_posture();

    match (source_posture, target_posture) {
        (
            SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly,
            SavedQueryTemporalAsyncSurfacePosture::OrdinaryOnly,
        ) => binding_row(
            SavedQueryRebindingDimension::PolicyBasisReuse,
            super::SavedQueryRebindingLegality::LegalNoSemanticChange,
            "ordinary-only saved-query reuse does not require preserved policy-basis temporal/async evidence",
        ),
        (
            SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
            SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked,
        ) => match descriptor.policy_basis_reuse_evaluation() {
            Some(evaluation)
                if saved_query_policy_reuse_artifact_digest(evaluation)
                    != artifact.digest().as_str() =>
            {
                binding_row(
                    SavedQueryRebindingDimension::PolicyBasisReuse,
                    super::SavedQueryRebindingLegality::IllegalSemanticDrift,
                    "policy-basis preserved reuse evidence was computed for a different saved query artifact",
                )
            }
            Some(evaluation)
                if saved_query_policy_reuse_surface_posture(evaluation)
                    != SavedQueryTemporalAsyncSurfacePosture::FuturePreservingRuntimeBacked =>
            {
                binding_row(
                    SavedQueryRebindingDimension::PolicyBasisReuse,
                    super::SavedQueryRebindingLegality::IllegalSemanticDrift,
                    "policy-basis preserved reuse evidence was not minted for the runtime-backed temporal/async surface",
                )
            }
            Some(evaluation)
                if saved_query_policy_reuse_disposition(evaluation)
                    == SavedQueryPolicyReuseDisposition::LegalNoSemanticChange =>
            {
                binding_row(
                    SavedQueryRebindingDimension::PolicyBasisReuse,
                    super::SavedQueryRebindingLegality::LegalNoSemanticChange,
                    "policy-basis preserved reuse keeps the runtime-backed temporal/async surface exact",
                )
            }
            Some(evaluation)
                if saved_query_policy_reuse_disposition(evaluation)
                    == SavedQueryPolicyReuseDisposition::LegalRequiresFreshFreeze =>
            {
                binding_row(
                    SavedQueryRebindingDimension::PolicyBasisReuse,
                    super::SavedQueryRebindingLegality::LegalRequiresFreshFreeze,
                    "policy-basis preserved reuse keeps runtime-backed temporal/async meaning but requires a fresh freeze artifact",
                )
            }
            Some(_) | None => binding_row(
                SavedQueryRebindingDimension::PolicyBasisReuse,
                super::SavedQueryRebindingLegality::IllegalSemanticDrift,
                "runtime-backed temporal/async saved-query reuse lacks preserved policy-basis equivalence",
            ),
        },
        (
            SavedQueryTemporalAsyncSurfacePosture::VisibleButDeferred,
            _,
        )
        | (
            _,
            SavedQueryTemporalAsyncSurfacePosture::VisibleButDeferred,
        ) => binding_row(
            SavedQueryRebindingDimension::PolicyBasisReuse,
            super::SavedQueryRebindingLegality::IllegalSemanticDrift,
            "visible-but-deferred temporal/async neighbors remain unavailable for preserved saved-query reuse",
        ),
        _ => binding_row(
            SavedQueryRebindingDimension::PolicyBasisReuse,
            super::SavedQueryRebindingLegality::IllegalSemanticDrift,
            "policy-basis preserved reuse changed the temporal/async saved-query surface",
        ),
    }
}
