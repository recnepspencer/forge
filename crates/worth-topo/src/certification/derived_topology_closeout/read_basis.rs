use super::traced_reports::{traced_milestone_two_envelope, traced_milestone_two_failure};
use super::*;
use crate::committed_artifact::TopologyCommittedArtifact;

pub(crate) fn certify_milestone_two_read_basis_runtime_traced_impl(
    runtime: &mut RelationalRuntime,
    read_basis: DerivedTopologyReadBasis,
) -> Result<TracedMilestoneTwoDerivedReadReport, BoundaryFailure<MilestoneOneCertificationError>> {
    let certified = certify_milestone_two_query_read_basis(runtime, read_basis.clone())
        .map_err(|error| traced_milestone_two_failure(error, &read_basis, None, 0))?;
    Ok(traced_milestone_two_envelope(
        certified.report,
        certified.query_evidence,
        &read_basis,
        None,
        0,
    ))
}

pub(crate) fn certify_milestone_two_verified_commit_traced_impl(
    runtime: &mut RelationalRuntime,
    verified: &TopologyCommittedArtifact,
) -> Result<TracedMilestoneTwoDerivedReadReport, BoundaryFailure<MilestoneOneCertificationError>> {
    let mut certified =
        certify_milestone_two_query_read_basis(runtime, verified.read_basis().clone()).map_err(
            |error| {
                traced_milestone_two_failure(
                    error,
                    &verified.read_basis(),
                    Some(verified.commits()),
                    verified.commits().len(),
                )
            },
        )?;
    if let Some(replay_commit_id) = verified
        .commits
        .last()
        .map(|commit| commit.outcome.commit.commit_id.clone())
    {
        let replay = runtime
            .replay_authority()
            .replay_commit(forge_relational::facade::replay::RelationalReplayRequest {
                branch_id: verified.branch_id().clone(),
                commit_id: replay_commit_id,
                execution_mode:
                    forge_relational::facade::replay::ReplayExecutionMode::SerialDeterministic,
                verification_mode:
                    forge_relational::facade::replay::ReplayVerificationMode::NormalRecoveryVerification,
            });
        certified
            .report
            .derived_replay_parity_report
            .relational_replay_checked = true;
        certified
            .report
            .derived_replay_parity_report
            .replayed_commit_id = replay
            .commit
            .as_ref()
            .map(|commit| commit.commit_id.0.to_string());
        certified
            .report
            .derived_replay_parity_report
            .compared_surfaces = replay.compared_surfaces.clone();
        if runtime.replay().compare_outcome(&replay) {
            certified
                .report
                .derived_replay_parity_report
                .relational_replay_verified = true;
            certified.report.derived_replay_parity_report.parity_status = if certified
                .report
                .derived_replay_parity_report
                .interpretation_digest_match
                && certified
                    .report
                    .derived_replay_parity_report
                    .truth_digest_match
                && certified
                    .report
                    .derived_replay_parity_report
                    .validation_digest_match
            {
                ReplayParityStatus::Match
            } else {
                ReplayParityStatus::Mismatch
            };
        } else {
            certified.report.derived_replay_parity_report.replay_failure = replay.failure;
            certified.report.derived_replay_parity_report.mismatch_count = replay.mismatches.len();
            certified.report.derived_replay_parity_report.parity_status =
                ReplayParityStatus::Mismatch;
        }
        certified
            .report
            .milestone_2_counter_report
            .replay_checked_count = 1;
    }
    Ok(traced_milestone_two_envelope(
        certified.report,
        certified.query_evidence,
        &verified.read_basis(),
        Some(verified.commits()),
        verified.commits().len(),
    ))
}

#[derive(Debug, Clone)]
struct MilestoneTwoQueryCertification {
    report: MilestoneTwoDerivedReadReport,
    query_evidence: MilestoneTwoQueryEvidence,
}

fn certify_milestone_two_query_read_basis(
    runtime: &mut RelationalRuntime,
    read_basis: DerivedTopologyReadBasis,
) -> Result<MilestoneTwoQueryCertification, MilestoneOneCertificationError> {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .ok_or_else(|| {
            MilestoneOneCertificationError::ReadView(format!(
                " certification could not open snapshot {:?}",
                read_basis.snapshot()
            ))
        })?;
    validate_named_topology_truth(&read_view)?;

    let adapters =
        TopologyRuntimeAdapters::snapshot_read_only(read_view, read_basis.snapshot().clone());
    let mut workspace = topology_runtime(adapters, ".milestone-two.certification")
        .map_err(|error| MilestoneOneCertificationError::Query(error.to_string()))?;
    let assembly = TopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| MilestoneOneCertificationError::Query(error.to_string()))?;
    let validation_state = workspace
        .state(assembly.validation())
        .map_err(|error| MilestoneOneCertificationError::Query(error.to_string()))?;
    ensure_query_surface_ready(".topology.validation", &validation_state)?;
    let equivalence_state = workspace
        .state(assembly.equivalence_contract())
        .map_err(|error| MilestoneOneCertificationError::Query(error.to_string()))?;
    ensure_query_surface_ready(".topology.equivalence_contract", &equivalence_state)?;
    let validation_inspection = derived_query_inspection(
        &mut workspace,
        assembly.validation(),
        ".topology.validation",
    )?;
    let equivalence_inspection = derived_query_inspection(
        &mut workspace,
        assembly.equivalence_contract(),
        ".topology.equivalence_contract",
    )?;
    let snapshot = assembly.snapshot_for_read_basis(&mut workspace, &read_basis)?;
    let read_artifact = build_topology_read_artifact(&read_basis, &snapshot.interpreted);
    let certified_interpretation = certify_topology_view(read_basis.clone(), &snapshot.interpreted);
    let replay_basis = read_basis.replay_of();
    let replay_equivalence_contract = build_derived_equivalence_contract_report(
        replay_basis.snapshot().snapshot_id.0,
        replay_basis.branch_id().0.clone(),
        replay_basis.authoritative_mutation_origin(),
        replay_basis.derivation_origin(),
        replay_basis
            .authority
            .truth_basis_identity
            .mutation_batch_digest_hex
            .clone(),
        replay_basis
            .authority
            .truth_basis_identity
            .touched_aspect_count,
        crate::projection::diagnostic_surfaces::triggered_invalidation_targets(&replay_basis),
        replay_basis.precision_fallbacks.len(),
        replay_basis.precision_budget_fallbacks.len(),
        &snapshot.materialized,
        &snapshot.interpreted,
        &snapshot.validation,
    );
    let replay_comparison = compare_derived_equivalence_contracts(
        &snapshot.equivalence_contract,
        &replay_equivalence_contract,
    );
    let branch_local_report = crate::certification::support::reporting::BranchLocalTopologyReport {
        mutation_origin: read_basis.derivation_origin(),
        branch_local: matches!(
            read_basis.derivation_origin(),
            schema::facade::platform::authority::MutationOrigin::BranchLocalApplication
        ),
        branch_id: read_basis.branch_id().clone(),
        snapshot_id: read_basis.snapshot().snapshot_id.0,
        touched_aspect_count: read_basis.touched_aspects().len(),
    };
    let replay_report = crate::certification::support::reporting::ReplayParityReport {
        mutation_origin: read_basis.derivation_origin(),
        replay_origin: matches!(
            read_basis.derivation_origin(),
            schema::facade::platform::authority::MutationOrigin::Replay
        ),
        branch_id: read_basis.branch_id().clone(),
        parity_status: ReplayParityStatus::NotChecked,
        equivalence_contract: snapshot.equivalence_contract.clone(),
        replay_equivalence_contract: Some(replay_equivalence_contract),
        relational_replay_checked: false,
        relational_replay_verified: false,
        replayed_commit_id: None,
        compared_surfaces: Vec::new(),
        mismatch_count: 0,
        replay_failure: None,
        interpretation_digest_match: replay_comparison.interpreted_topology_digest_match,
        truth_digest_match: read_basis
            .authority
            .truth_basis_identity
            .mutation_batch_digest_hex
            == replay_basis
                .authority
                .truth_basis_identity
                .mutation_batch_digest_hex,
        validation_digest_match: replay_comparison.derived_validation_digest_match,
    };
    let report = MilestoneTwoDerivedReadReport {
        materialized_topology_digest: snapshot
            .equivalence_contract
            .materialized_topology_digest
            .clone(),
        interpreted_topology_digest: snapshot
            .equivalence_contract
            .interpreted_topology_digest
            .clone(),
        derived_validation_digest: snapshot
            .equivalence_contract
            .derived_validation_digest
            .clone(),
        derived_invalidation_report: snapshot.diagnostics.invalidation_report.clone(),
        derived_rebuild_report: snapshot.diagnostics.rebuild_report.clone(),
        derived_fallback_report: snapshot.diagnostics.fallback_report.clone(),
        derived_equivalence_contract_report: snapshot.equivalence_contract.clone(),
        derived_branch_local_parity_report: branch_local_report,
        derived_replay_parity_report: replay_report,
        milestone_2_counter_report: MilestoneTwoCounters {
            derived_read_count: 1,
            touched_aspect_count: snapshot.equivalence_contract.touched_aspect_count,
            triggered_invalidation_target_count: snapshot
                .equivalence_contract
                .triggered_invalidation_targets
                .len(),
            validation_row_count: snapshot.validation.rows.len(),
            whole_view_rebuild_count: usize::from(
                snapshot.diagnostics.rebuild_report.whole_view_rebuild,
            ),
            explicit_fallback_count: snapshot.diagnostics.fallback_report.explicit_fallback_count,
            replay_checked_count: 0,
            branch_local_case_count: usize::from(matches!(
                read_basis.derivation_origin(),
                schema::facade::platform::authority::MutationOrigin::BranchLocalApplication
            )),
        },
        read_artifact,
        certified_interpretation,
    };
    Ok(MilestoneTwoQueryCertification {
        report,
        query_evidence: MilestoneTwoQueryEvidence {
            affected_live_view_count: 0,
            affected_derived_view_count: 0,
            considered_computed_view_count: 0,
            validation_materialized_row_count: validation_inspection.materialized_row_count(),
            equivalence_materialized_row_count: equivalence_inspection.materialized_row_count(),
            validation_pending_refresh_fallback_count: validation_inspection
                .pending_refresh_fallback_count(),
            equivalence_pending_refresh_fallback_count: equivalence_inspection
                .pending_refresh_fallback_count(),
            declared_aspect_operation_count: 0,
            mutation_metadata_key_count: 0,
        },
    })
}

fn ensure_query_surface_ready(
    surface_name: &str,
    state: &forge_query::facade::ForgeQueryRuntimeStateSnapshot,
) -> Result<(), MilestoneOneCertificationError> {
    if state.kind() != ForgeQueryRuntimeStateKind::Ready {
        return Err(MilestoneOneCertificationError::Query(format!(
            "query certification surface `{surface_name}` is `{}` instead of `ready`: {}",
            state.kind(),
            state.explanation()
        )));
    }
    Ok(())
}

fn derived_query_inspection<T>(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    view: &forge_query::facade::ForgeQueryDerivedViewHandle<T>,
    expected_name: &str,
) -> Result<ForgeQueryComputedInspectionEvidence, MilestoneOneCertificationError> {
    match workspace
        .inspect(view)
        .map_err(|error| MilestoneOneCertificationError::Query(error.to_string()))?
    {
        ForgeQueryInspection::DerivedView(inspection) => {
            if inspection.name() != expected_name {
                return Err(MilestoneOneCertificationError::Query(format!(
                    "query inspection returned derived surface `{}` while `{expected_name}` was expected",
                    inspection.name()
                )));
            }
            Ok(inspection)
        }
        other => Err(MilestoneOneCertificationError::Query(format!(
            "query inspection for `{expected_name}` returned wrong artifact family: {other:?}"
        ))),
    }
}
