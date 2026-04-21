use crate::{
    backend::engine::{StateBackedStoreBackend, StatePersistence},
    backend::integrity::stable_structural_digest,
    evidence::StoreCounterSnapshot,
};
use serde::Serialize;

pub(crate) fn milestone_13_counter_contract<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone13CounterContract {
    crate::Milestone13CounterContract::from_snapshot(&backend.counters().snapshot())
}

pub(crate) fn milestone_13_artifact_report<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> Result<crate::Milestone13ArtifactReport, crate::StoreError> {
    let state = backend.state();
    let artifact_digest = stable_structural_digest(&Milestone13ArtifactDigestBasis {
        residency_identity_records: tiering_identity_records(state)?,
    })?;
    let diagnostics_digest = stable_structural_digest(&Milestone13DiagnosticsDigestBasis {
        manifest: super::execution::shared::manifest_from_state(state),
        complexity_surface: milestone_13_complexity_surface(backend),
        counter_contract: milestone_13_counter_contract(backend),
    })?;
    let residual_residency_ambiguity_count = state
        .tier_transfer_records
        .values()
        .filter(|record| {
            if !record.cutover_completed {
                return false;
            }
            let Some(residency) = state.tier_residency_records.get(&record.artifact_key) else {
                return true;
            };
            residency.artifact_family != record.artifact_family
                || residency.canonical_residence != record.target_residence
                || residency.verification_label
                    != record.verification_label.clone().unwrap_or_default()
        })
        .count();

    Ok(crate::Milestone13ArtifactReport {
        artifact_digest,
        diagnostics_digest,
        resident_artifact_count: state.tier_residency_records.len(),
        in_flight_transfer_count: state.tier_transfer_records.len(),
        residual_residency_ambiguity_count,
    })
}

pub(crate) fn milestone_13_complexity_surface<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
) -> crate::Milestone13ComplexitySurface {
    complexity_surface_from_snapshot(&backend.counters().snapshot())
}

fn complexity_surface_from_snapshot(
    snapshot: &StoreCounterSnapshot,
) -> crate::Milestone13ComplexitySurface {
    let mut surface = crate::Milestone13ComplexitySurface::phase_1_default();
    let phase_3_debt =
        "Phase 3 execution has not yet recorded a real bounded move/recall proof path";

    surface.placement_state_reconstruction = crate::Milestone13ComplexityPathStatus::verified(
        "phase 1 and 2 expose manifest-bounded placement-state vocabulary without inventory scans",
    );
    surface.working_set_classification = if snapshot.working_set_debt_count > 0 {
        crate::Milestone13ComplexityPathStatus::debt(
            "working-set classification has recorded explicit unsupported heuristic debt",
        )
    } else {
        crate::Milestone13ComplexityPathStatus::verified(
            "working-set observation and classification are lowered through scope-typed windows",
        )
    };
    surface.tier_move_planning = if snapshot.placement_debt_count > 0 {
        crate::Milestone13ComplexityPathStatus::debt(
            "tier move planning has recorded unsupported placement ambitions or illegal move posture",
        )
    } else {
        crate::Milestone13ComplexityPathStatus::verified(
            "tier move planning lowers conservative-policy placement into typed authoritative and derived plans",
        )
    };
    surface.tier_move_cutover = if snapshot.tier_move_cutover_count > 0 {
        crate::Milestone13ComplexityPathStatus::verified(
            "tier cutover persists canonical residency through typed verified-replica witnesses",
        )
    } else {
        crate::Milestone13ComplexityPathStatus::debt(phase_3_debt)
    };
    surface.tier_move_execution =
        if snapshot.authoritative_tier_move_count > 0 || snapshot.derived_tier_move_count > 0 {
            crate::Milestone13ComplexityPathStatus::verified(
                "tier execution consumes lowered move plans and persists in-flight transfer state",
            )
        } else {
            crate::Milestone13ComplexityPathStatus::debt(phase_3_debt)
        };
    surface.cold_recall_execution = if snapshot.cold_tier_recall_count > 0 {
        crate::Milestone13ComplexityPathStatus::verified(
            "cold recall executes through explicit leases and eligibility witnesses",
        )
    } else {
        crate::Milestone13ComplexityPathStatus::debt(phase_3_debt)
    };
    surface.recall_coalescing = if snapshot.recall_coalesced_request_count > 0
        && snapshot.recall_duplicate_suppression_count > 0
    {
        crate::Milestone13ComplexityPathStatus::verified(
                "recall execution shares work by coalescing key and records duplicate suppression exactly",
            )
    } else {
        crate::Milestone13ComplexityPathStatus::debt(
                "recall coalescing vocabulary exists, but no executed shared recall lane has been observed yet",
            )
    };

    surface
}

#[derive(Serialize)]
struct Milestone13ArtifactDigestBasis {
    residency_identity_records: Vec<Milestone13ResidencyIdentityOwned>,
}

#[derive(Serialize)]
struct Milestone13DiagnosticsDigestBasis {
    manifest: crate::CanonicalResidencyManifest,
    complexity_surface: crate::Milestone13ComplexitySurface,
    counter_contract: crate::Milestone13CounterContract,
}

fn tiering_identity_records(
    state: &crate::backend::records::StoreState,
) -> Result<Vec<Milestone13ResidencyIdentityOwned>, crate::StoreError> {
    let mut artifact_keys = Vec::new();
    artifact_keys.extend(
        state
            .branch_head_records
            .values()
            .filter(|record| record.head_commit_id.is_some())
            .map(|record| format!("authoritative_branch_head:{}", record.branch_id.0)),
    );
    artifact_keys.extend(
        state
            .snapshot_basis_records
            .keys()
            .map(|snapshot_id| format!("retained_authority:snapshot:{snapshot_id}")),
    );
    artifact_keys.extend(
        state
            .stable_basis_records
            .keys()
            .map(|artifact_id| format!("stable_basis:{artifact_id}")),
    );
    artifact_keys.extend(
        state
            .snapshot_basis_records
            .keys()
            .map(|snapshot_id| format!("snapshot:{snapshot_id}")),
    );
    artifact_keys.extend(
        state
            .branch_delta_layer_records
            .keys()
            .map(|layer_id| format!("branch_delta:{layer_id}")),
    );
    artifact_keys.extend(
        state
            .milestone_6_layout_materialization_records
            .keys()
            .map(|artifact_id| format!("milestone6_layout:{artifact_id}")),
    );
    artifact_keys.sort();
    artifact_keys.dedup();

    artifact_keys
        .into_iter()
        .map(|artifact_key| {
            let artifact_family =
                crate::backend::tiering::execution::shared::placement_family_for_artifact_key(
                    &artifact_key,
                )?;
            let verification_label =
                crate::backend::tiering::execution::shared::expected_verification_label(
                    state,
                    &artifact_key,
                )?;
            Ok(Milestone13ResidencyIdentityOwned {
                artifact_key,
                artifact_family: artifact_family.label().to_string(),
                verification_label,
            })
        })
        .collect()
}

#[derive(Serialize)]
struct Milestone13ResidencyIdentityOwned {
    artifact_key: String,
    artifact_family: String,
    verification_label: String,
}
