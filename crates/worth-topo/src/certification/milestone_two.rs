use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryInspection, ForgeQueryRuntimeStateKind,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::transactions::CommitResult;
use worth_schema::facade::{
    DerivedTopologyReadBasis, VerifiedTopologyCommit, WorthAuthorityTraceAnchor,
    WorthAuthorityTraceEvidence, WorthBoundaryEnvelope, WorthBoundaryFailure, WorthDecisionTrace,
    WorthDerivedTraceAnchor, WorthDerivedTraceEvidence, WorthNamedCounter,
    WorthPerformanceAccounting, WorthTraceAvailability,
};

use crate::certification::bridge::certify_milestone_one_bridge_proof;
use crate::certification::corpus::certify_milestone_one_default_primitive_corpus_impl;
use crate::certification::error::WorthMilestoneOneCertificationError;
use crate::certification::read_view::certification_integrity_markers;
use crate::certification::report::{
    WorthDerivedEquivalenceContractAggregateReport, WorthDerivedEquivalenceContractAggregateRow,
    WorthDerivedFallbackAggregateReport, WorthDerivedFallbackAggregateRow,
    WorthDerivedFamilyCoverageMatrix, WorthDerivedFamilyCoverageRow,
    WorthDerivedFamilyParityMatrix, WorthDerivedFamilyParityRow,
    WorthDerivedInvalidationAggregateReport, WorthDerivedInvalidationAggregateRow,
    WorthDerivedRebuildAggregateReport, WorthDerivedRebuildAggregateRow,
    WorthDerivedValidatorCoverageReport, WorthDerivedValidatorCoverageRow,
    WorthDeterministicDigest, WorthFailureLocalityReport, WorthMilestoneOneCertificationReport,
    WorthMilestoneTwoBranchLocalParityReport, WorthMilestoneTwoCloseoutReport,
    WorthMilestoneTwoCounters, WorthMilestoneTwoDerivedCorpusReport,
    WorthMilestoneTwoDerivedReadReport, WorthMilestoneTwoReplayParityReport,
    WorthPrimitiveCorpusParityReport, WorthPrimitiveCorpusReport,
};
use crate::certification::requirements::milestone_two_closeout_requirements;
use crate::certification::shared::digest_rows;
use crate::facade::{
    build_topology_read_artifact, certify_topology_view, compare_derived_equivalence_contracts,
    validate_named_topology_truth, WorthReplayParityStatus, WorthTopologyQueryAssembly,
};
use crate::parity::build_derived_equivalence_contract_report;
use crate::query::{worth_topology_runtime, WorthTopologyRuntimeAdapters};

pub type WorthTracedMilestoneTwoDerivedReadReport =
    WorthBoundaryEnvelope<WorthMilestoneTwoDerivedReadReport>;

#[derive(Debug, Clone, Copy)]
struct WorthMilestoneTwoQueryEvidence {
    affected_live_view_count: usize,
    affected_derived_view_count: usize,
    considered_computed_view_count: usize,
    validation_materialized_row_count: usize,
    equivalence_materialized_row_count: usize,
    validation_pending_refresh_fallback_count: usize,
    equivalence_pending_refresh_fallback_count: usize,
    declared_aspect_operation_count: usize,
    mutation_metadata_key_count: usize,
}

pub fn certify_milestone_two_read_basis_runtime_traced_impl(
    runtime: &mut RelationalRuntime,
    read_basis: DerivedTopologyReadBasis,
) -> Result<
    WorthTracedMilestoneTwoDerivedReadReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
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

pub fn certify_milestone_two_verified_commit_traced_impl(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<
    WorthTracedMilestoneTwoDerivedReadReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    let mut certified =
        certify_milestone_two_query_read_basis(runtime, verified.read_basis.clone()).map_err(
            |error| {
                traced_milestone_two_failure(
                    error,
                    &verified.read_basis,
                    Some(&verified.commits),
                    verified.commits.len(),
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
                branch_id: verified.branch_id.clone(),
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
                WorthReplayParityStatus::Match
            } else {
                WorthReplayParityStatus::Mismatch
            };
        } else {
            certified.report.derived_replay_parity_report.replay_failure = replay.failure;
            certified.report.derived_replay_parity_report.mismatch_count = replay.mismatches.len();
            certified.report.derived_replay_parity_report.parity_status =
                WorthReplayParityStatus::Mismatch;
        }
        certified
            .report
            .milestone_2_counter_report
            .replay_checked_count = 1;
    }
    Ok(traced_milestone_two_envelope(
        certified.report,
        certified.query_evidence,
        &verified.read_basis,
        Some(&verified.commits),
        verified.commits.len(),
    ))
}

#[derive(Debug, Clone)]
struct WorthMilestoneTwoQueryCertification {
    report: WorthMilestoneTwoDerivedReadReport,
    query_evidence: WorthMilestoneTwoQueryEvidence,
}

fn certify_milestone_two_query_read_basis(
    runtime: &mut RelationalRuntime,
    read_basis: DerivedTopologyReadBasis,
) -> Result<WorthMilestoneTwoQueryCertification, WorthMilestoneOneCertificationError> {
    let read_view = runtime
        .read_truth()
        .read_snapshot(read_basis.snapshot())
        .ok_or_else(|| {
            WorthMilestoneOneCertificationError::ReadView(format!(
                "worth certification could not open snapshot {:?}",
                read_basis.snapshot()
            ))
        })?;
    validate_named_topology_truth(&read_view)?;

    let adapters =
        WorthTopologyRuntimeAdapters::snapshot_read_only(read_view, read_basis.snapshot().clone());
    let mut workspace = worth_topology_runtime(adapters, "worth.milestone-two.certification")
        .map_err(|error| WorthMilestoneOneCertificationError::Query(error.to_string()))?;
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace)
        .map_err(|error| WorthMilestoneOneCertificationError::Query(error.to_string()))?;
    let validation_state = workspace
        .state(assembly.validation())
        .map_err(|error| WorthMilestoneOneCertificationError::Query(error.to_string()))?;
    ensure_query_surface_ready("worth.topology.validation", &validation_state)?;
    let equivalence_state = workspace
        .state(assembly.equivalence_contract())
        .map_err(|error| WorthMilestoneOneCertificationError::Query(error.to_string()))?;
    ensure_query_surface_ready("worth.topology.equivalence_contract", &equivalence_state)?;
    let validation_inspection = derived_query_inspection(
        &mut workspace,
        assembly.validation(),
        "worth.topology.validation",
    )?;
    let equivalence_inspection = derived_query_inspection(
        &mut workspace,
        assembly.equivalence_contract(),
        "worth.topology.equivalence_contract",
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
        crate::diagnostics::triggered_invalidation_targets(&replay_basis),
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
    let branch_local_report = crate::certification::report::WorthBranchLocalTopologyReport {
        mutation_origin: read_basis.derivation_origin(),
        branch_local: matches!(
            read_basis.derivation_origin(),
            worth_schema::facade::WorthMutationOrigin::BranchLocalApplication
        ),
        branch_id: read_basis.branch_id().clone(),
        snapshot_id: read_basis.snapshot().snapshot_id.0,
        touched_aspect_count: read_basis.touched_aspects().len(),
    };
    let replay_report = crate::certification::report::WorthReplayParityReport {
        mutation_origin: read_basis.derivation_origin(),
        replay_origin: matches!(
            read_basis.derivation_origin(),
            worth_schema::facade::WorthMutationOrigin::Replay
        ),
        branch_id: read_basis.branch_id().clone(),
        parity_status: WorthReplayParityStatus::NotChecked,
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
    let report = WorthMilestoneTwoDerivedReadReport {
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
        milestone_2_counter_report: WorthMilestoneTwoCounters {
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
                worth_schema::facade::WorthMutationOrigin::BranchLocalApplication
            )),
        },
        read_artifact,
        certified_interpretation,
    };
    Ok(WorthMilestoneTwoQueryCertification {
        report,
        query_evidence: WorthMilestoneTwoQueryEvidence {
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
) -> Result<(), WorthMilestoneOneCertificationError> {
    if state.kind() != ForgeQueryRuntimeStateKind::Ready {
        return Err(WorthMilestoneOneCertificationError::Query(format!(
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
) -> Result<ForgeQueryComputedInspectionEvidence, WorthMilestoneOneCertificationError> {
    match workspace
        .inspect(view)
        .map_err(|error| WorthMilestoneOneCertificationError::Query(error.to_string()))?
    {
        ForgeQueryInspection::DerivedView(inspection) => {
            if inspection.name() != expected_name {
                return Err(WorthMilestoneOneCertificationError::Query(format!(
                    "query inspection returned derived surface `{}` while `{expected_name}` was expected",
                    inspection.name()
                )));
            }
            Ok(inspection)
        }
        other => Err(WorthMilestoneOneCertificationError::Query(format!(
            "query inspection for `{expected_name}` returned wrong artifact family: {other:?}"
        ))),
    }
}

fn traced_milestone_two_envelope(
    report: WorthMilestoneTwoDerivedReadReport,
    query_evidence: WorthMilestoneTwoQueryEvidence,
    read_basis: &DerivedTopologyReadBasis,
    commit_results: Option<&[CommitResult]>,
    replay_history_length: usize,
) -> WorthTracedMilestoneTwoDerivedReadReport {
    WorthBoundaryEnvelope::success(
        report.clone(),
        Vec::new(),
        WorthDecisionTrace {
            authority_anchor: commit_results.map(|commits| {
                WorthAuthorityTraceAnchor::from_commit_results(
                    read_basis.branch_id().clone(),
                    commits,
                )
            }),
            bridge_anchor: None,
            derived_anchor: Some(WorthDerivedTraceAnchor::from_read_basis(read_basis)),
            signal_anchor: None,
            authority: commit_results.map(|commits| {
                WorthAuthorityTraceEvidence::from_commit_results(
                    read_basis.branch_id().clone(),
                    commits,
                )
            }),
            bridge: None,
            derived: Some(milestone_two_derived_trace(&report)),
            signal: None,
        },
        certification_integrity_markers(read_basis, commit_results),
        milestone_two_performance_accounting(&report, query_evidence, replay_history_length),
    )
}

fn milestone_two_derived_trace(
    report: &WorthMilestoneTwoDerivedReadReport,
) -> WorthDerivedTraceEvidence {
    WorthDerivedTraceEvidence {
        availability: WorthTraceAvailability::Present,
        invalidation_target_count: report.derived_invalidation_report.triggered_target_count,
        fallback_classes: report
            .derived_fallback_report
            .materialization_fallback_class
            .map(|_| "WholeViewRebuild".to_string())
            .into_iter()
            .collect(),
        equivalence_digest: Some(
            report
                .derived_equivalence_contract_report
                .materialized_topology_digest
                .digest_hex
                .clone(),
        ),
    }
}

fn traced_milestone_two_failure(
    error: WorthMilestoneOneCertificationError,
    read_basis: &DerivedTopologyReadBasis,
    commit_results: Option<&[CommitResult]>,
    replay_history_length: usize,
) -> WorthBoundaryFailure<WorthMilestoneOneCertificationError> {
    WorthBoundaryFailure::failure(
        error,
        Vec::new(),
        WorthDecisionTrace {
            authority_anchor: commit_results.map(|commits| {
                WorthAuthorityTraceAnchor::from_commit_results(
                    read_basis.branch_id().clone(),
                    commits,
                )
            }),
            bridge_anchor: None,
            derived_anchor: Some(WorthDerivedTraceAnchor::from_read_basis(read_basis)),
            signal_anchor: None,
            authority: commit_results.map(|commits| {
                WorthAuthorityTraceEvidence::from_commit_results(
                    read_basis.branch_id().clone(),
                    commits,
                )
            }),
            bridge: None,
            derived: None,
            signal: None,
        },
        certification_integrity_markers(read_basis, commit_results),
        WorthPerformanceAccounting::new([WorthNamedCounter::new(
            "certification.replay_history_length",
            replay_history_length as u64,
        )]),
    )
}

fn milestone_two_performance_accounting(
    report: &WorthMilestoneTwoDerivedReadReport,
    query_evidence: WorthMilestoneTwoQueryEvidence,
    replay_history_length: usize,
) -> WorthPerformanceAccounting {
    WorthPerformanceAccounting::new([
        WorthNamedCounter::new(
            "certification.replay_history_length",
            replay_history_length as u64,
        ),
        WorthNamedCounter::new(
            "certification.derived_invalidation_target_count",
            report.derived_invalidation_report.triggered_target_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.affected_live_view_count",
            query_evidence.affected_live_view_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.affected_derived_view_count",
            query_evidence.affected_derived_view_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.considered_computed_view_count",
            query_evidence.considered_computed_view_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.validation_materialized_row_count",
            query_evidence.validation_materialized_row_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.equivalence_materialized_row_count",
            query_evidence.equivalence_materialized_row_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.validation_pending_refresh_fallback_count",
            query_evidence.validation_pending_refresh_fallback_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.equivalence_pending_refresh_fallback_count",
            query_evidence.equivalence_pending_refresh_fallback_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.declared_aspect_operation_count",
            query_evidence.declared_aspect_operation_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.mutation_metadata_key_count",
            query_evidence.mutation_metadata_key_count as u64,
        ),
    ])
}

pub fn certify_milestone_two_default_derived_corpus_impl<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneTwoDerivedCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let primitive_corpus =
        certify_milestone_one_default_primitive_corpus_impl(&mut runtime_factory, stem)?;
    let bridge_proof_report = certify_milestone_one_bridge_proof(&format!("{stem}.bridge"))?;

    Ok(WorthMilestoneTwoDerivedCorpusReport {
        materialized_topology_digest: aggregate_derived_digest(&primitive_corpus, |report| {
            report
                .derived_equivalence_contract_report
                .materialized_topology_digest
                .clone()
        }),
        interpreted_topology_digest: aggregate_derived_digest(&primitive_corpus, |report| {
            report
                .derived_equivalence_contract_report
                .interpreted_topology_digest
                .clone()
        }),
        derived_validation_digest: aggregate_derived_digest(&primitive_corpus, |report| {
            report
                .derived_equivalence_contract_report
                .derived_validation_digest
                .clone()
        }),
        derived_truth_basis_digest: aggregate_truth_basis_digest(&primitive_corpus),
        derived_family_coverage_matrix: build_derived_family_coverage_matrix(&primitive_corpus),
        derived_family_parity_matrix: build_derived_family_parity_matrix(
            &primitive_corpus.parity_report,
        ),
        derived_branch_local_parity_report: build_derived_branch_local_report(&primitive_corpus),
        derived_replay_parity_report: build_derived_replay_report(&primitive_corpus),
        derived_bridge_family_coverage_report: bridge_proof_report.family_coverage_report.clone(),
        bridge_routing_digest: bridge_proof_report.bridge_routing_digest.clone(),
        bridge_historical_evaluation_digest: bridge_proof_report
            .bridge_historical_evaluation_digest
            .clone(),
        milestone_2_counter_report: build_milestone_two_counter_report(&primitive_corpus),
        primitive_corpus,
        bridge_proof_report,
    })
}

pub fn certify_milestone_two_closeout_impl<F>(
    runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneTwoCloseoutReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let requirements = milestone_two_closeout_requirements();
    let derived_corpus = certify_milestone_two_default_derived_corpus_impl(runtime_factory, stem)?;
    let primitive_corpus = &derived_corpus.primitive_corpus;

    let closeout = WorthMilestoneTwoCloseoutReport {
        materialized_topology_digest: derived_corpus.materialized_topology_digest.clone(),
        interpreted_topology_digest: derived_corpus.interpreted_topology_digest.clone(),
        derived_validation_digest: derived_corpus.derived_validation_digest.clone(),
        derived_truth_basis_digest: derived_corpus.derived_truth_basis_digest.clone(),
        bridge_routing_digest: derived_corpus.bridge_routing_digest.clone(),
        bridge_historical_evaluation_digest: derived_corpus
            .bridge_historical_evaluation_digest
            .clone(),
        derived_family_coverage_matrix: derived_corpus.derived_family_coverage_matrix.clone(),
        derived_family_parity_matrix: derived_corpus.derived_family_parity_matrix.clone(),
        derived_validator_coverage_report: build_derived_validator_coverage_report(
            primitive_corpus,
        ),
        derived_invalidation_report: build_derived_invalidation_aggregate_report(primitive_corpus),
        derived_rebuild_report: build_derived_rebuild_aggregate_report(primitive_corpus),
        derived_equivalence_contract_report: build_derived_equivalence_aggregate_report(
            primitive_corpus,
        ),
        derived_fallback_report: build_derived_fallback_aggregate_report(primitive_corpus),
        derived_failure_locality_report: build_derived_failure_locality_report(primitive_corpus),
        derived_branch_local_parity_report: derived_corpus
            .derived_branch_local_parity_report
            .clone(),
        derived_replay_parity_report: derived_corpus.derived_replay_parity_report.clone(),
        derived_bridge_family_coverage_report: derived_corpus
            .derived_bridge_family_coverage_report
            .clone(),
        milestone_2_counter_report: derived_corpus.milestone_2_counter_report.clone(),
        derived_corpus,
    };

    ensure_milestone_two_family_coverage_closure(
        &closeout.derived_family_coverage_matrix,
        &requirements,
    )?;
    ensure_milestone_two_parity_closure(&closeout.derived_family_parity_matrix, &requirements)?;
    ensure_milestone_two_validator_closure(
        &closeout.derived_validator_coverage_report,
        &requirements,
    )?;
    ensure_milestone_two_bridge_closure(
        &closeout.derived_bridge_family_coverage_report,
        &requirements,
    )?;
    ensure_milestone_two_failure_locality_closure(
        &closeout.derived_failure_locality_report,
        &requirements,
    )?;
    ensure_milestone_two_required_output_closure(&closeout, &requirements)?;

    Ok(closeout)
}

fn aggregate_derived_digest(
    primitive_corpus: &WorthPrimitiveCorpusReport,
    select: impl Fn(&WorthMilestoneOneCertificationReport) -> WorthDeterministicDigest,
) -> WorthDeterministicDigest {
    digest_rows(primitive_corpus.cases.iter().map(|case| {
        let digest = select(&case.certification);
        format!(
            "{}:{}:{}:{}",
            case.stem, digest.algorithm, digest.digest_hex, digest.row_count
        )
    }))
}

fn aggregate_truth_basis_digest(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDeterministicDigest {
    digest_rows(primitive_corpus.cases.iter().map(|case| {
        let report = &case.certification.derived_equivalence_contract_report;
        format!(
            "{}:{}:{}:{}:{}:{}",
            case.stem,
            report.authority_snapshot_id,
            report.authority_branch_id,
            report.truth_basis_digest_hex,
            report.touched_aspect_count,
            report.triggered_invalidation_targets.len()
        )
    }))
}

fn build_derived_family_coverage_matrix(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedFamilyCoverageMatrix {
    WorthDerivedFamilyCoverageMatrix {
        rows: primitive_corpus
            .coverage_matrix
            .entries
            .iter()
            .map(|entry| WorthDerivedFamilyCoverageRow {
                family: entry.family.clone(),
                admitted_case_count: entry.admitted_smallest_count
                    + entry.admitted_generic_count
                    + entry.admitted_hostile_count,
                out_of_class_rejection_count: entry.rejected_out_of_class_count,
                coverage_complete: entry.role_closure_complete,
            })
            .collect(),
    }
}

fn build_derived_family_parity_matrix(
    parity_report: &WorthPrimitiveCorpusParityReport,
) -> WorthDerivedFamilyParityMatrix {
    WorthDerivedFamilyParityMatrix {
        rows: parity_report
            .entries
            .iter()
            .map(|entry| WorthDerivedFamilyParityRow {
                family: entry.family.clone(),
                mainline_case_count: entry.mainline_case_count,
                branch_local_case_count: entry.branch_local_case_count,
                replay_verified_case_count: entry.mainline_replay_verified_case_count,
                branch_local_replay_verified_case_count: entry
                    .branch_local_replay_verified_case_count,
                cross_branch_parity_case_count: entry.cross_branch_parity_case_count,
                parity_complete: entry.parity_closure_complete,
            })
            .collect(),
    }
}

fn build_derived_branch_local_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthMilestoneTwoBranchLocalParityReport {
    let branch_ids = primitive_corpus
        .parity_report
        .entries
        .iter()
        .flat_map(|entry| entry.branch_ids.clone())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mainline_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.mainline_case_count)
        .sum();
    let branch_local_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.branch_local_case_count)
        .sum();

    WorthMilestoneTwoBranchLocalParityReport {
        mainline_case_count,
        branch_local_case_count,
        branch_ids,
        branch_local_closure_complete: primitive_corpus
            .parity_report
            .entries
            .iter()
            .all(|entry| entry.parity_closure_complete),
    }
}

fn build_derived_replay_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthMilestoneTwoReplayParityReport {
    let replay_checked_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.mainline_replay_checked_case_count)
        .sum();
    let replay_verified_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.mainline_replay_verified_case_count)
        .sum();
    let branch_local_replay_checked_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.branch_local_replay_checked_case_count)
        .sum();
    let branch_local_replay_verified_case_count = primitive_corpus
        .parity_report
        .entries
        .iter()
        .map(|entry| entry.branch_local_replay_verified_case_count)
        .sum();
    let total_case_count = primitive_corpus.cases.len();

    WorthMilestoneTwoReplayParityReport {
        replay_checked_case_count,
        replay_verified_case_count,
        replay_mismatch_case_count: total_case_count.saturating_sub(replay_verified_case_count),
        branch_local_replay_checked_case_count,
        branch_local_replay_verified_case_count,
        replay_closure_complete: primitive_corpus
            .parity_report
            .entries
            .iter()
            .all(|entry| entry.parity_closure_complete),
    }
}

fn build_milestone_two_counter_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthMilestoneTwoCounters {
    let mut counters = WorthMilestoneTwoCounters {
        derived_read_count: 0,
        touched_aspect_count: 0,
        triggered_invalidation_target_count: 0,
        validation_row_count: 0,
        whole_view_rebuild_count: 0,
        explicit_fallback_count: 0,
        replay_checked_count: 0,
        branch_local_case_count: 0,
    };

    for case in &primitive_corpus.cases {
        counters.derived_read_count += 1;
        counters.touched_aspect_count += case
            .certification
            .derived_invalidation_report
            .touched_aspect_count;
        counters.triggered_invalidation_target_count += case
            .certification
            .derived_invalidation_report
            .triggered_target_count;
        counters.validation_row_count += case.certification.topology_validation_report.rows.len();
        counters.whole_view_rebuild_count +=
            usize::from(case.certification.derived_rebuild_report.whole_view_rebuild);
        counters.explicit_fallback_count += case
            .certification
            .derived_fallback_report
            .explicit_fallback_count;
        counters.replay_checked_count += usize::from(
            case.certification
                .milestone_1_replay_parity_report
                .relational_replay_checked,
        );
        counters.branch_local_case_count +=
            usize::from(case.certification.branch_local_topology_report.branch_local);
    }

    counters
}

fn build_derived_invalidation_aggregate_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedInvalidationAggregateReport {
    let mut rows =
        BTreeMap::<(String, String, String), WorthDerivedInvalidationAggregateRow>::new();
    let mut touched_aspect_count = 0usize;
    let mut triggered_target_count = 0usize;

    for case in &primitive_corpus.cases {
        let report = &case.certification.derived_invalidation_report;
        touched_aspect_count += report.touched_aspect_count;
        triggered_target_count += report.triggered_target_count;
        for row in &report.rows {
            let key = (
                case.family.clone(),
                format!("{:?}", row.target),
                row.bridge_scope.clone(),
            );
            let entry =
                rows.entry(key.clone())
                    .or_insert_with(|| WorthDerivedInvalidationAggregateRow {
                        family: key.0.clone(),
                        target: key.1.clone(),
                        bridge_scope: key.2.clone(),
                        source_count: 0,
                        triggered_case_count: 0,
                    });
            entry.source_count += 1;
            entry.triggered_case_count += usize::from(row.triggered);
        }
    }

    WorthDerivedInvalidationAggregateReport {
        touched_aspect_count,
        triggered_target_count,
        rows: rows.into_values().collect(),
    }
}

fn build_derived_validator_coverage_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedValidatorCoverageReport {
    let mut rows = BTreeMap::<(String, String, String), WorthDerivedValidatorCoverageRow>::new();
    for case in &primitive_corpus.cases {
        for validation_row in &case.certification.topology_validation_report.rows {
            if validation_row.phase == crate::validators::WorthTopologyValidationPhase::Truth {
                continue;
            }
            let phase = match validation_row.phase {
                crate::validators::WorthTopologyValidationPhase::DerivedMaterialization => {
                    "derived-materialization"
                }
                crate::validators::WorthTopologyValidationPhase::DerivedInterpretation => {
                    "derived-interpretation"
                }
                crate::validators::WorthTopologyValidationPhase::Truth => "truth",
            };
            let key = (
                case.family.clone(),
                validation_row.validator.clone(),
                phase.to_string(),
            );
            let entry =
                rows.entry(key.clone())
                    .or_insert_with(|| WorthDerivedValidatorCoverageRow {
                        family: key.0.clone(),
                        validator: key.1.clone(),
                        phase: key.2.clone(),
                        passed_count: 0,
                        source_count: 0,
                    });
            entry.passed_count += usize::from(validation_row.status == "passed");
            entry.source_count += 1;
        }
    }

    WorthDerivedValidatorCoverageReport {
        rows: rows.into_values().collect(),
    }
}

fn build_derived_rebuild_aggregate_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedRebuildAggregateReport {
    let mut rows = BTreeMap::<String, WorthDerivedRebuildAggregateRow>::new();
    for case in &primitive_corpus.cases {
        let report = &case.certification.derived_rebuild_report;
        let entry =
            rows.entry(case.family.clone())
                .or_insert_with(|| WorthDerivedRebuildAggregateRow {
                    family: case.family.clone(),
                    source_count: 0,
                    whole_view_rebuild_count: 0,
                    topology_entity_count: 0,
                    topology_relation_count: 0,
                    interpreted_wire_count: 0,
                    interpreted_shell_count: 0,
                    validation_row_count: 0,
                });
        entry.source_count += 1;
        entry.whole_view_rebuild_count += usize::from(report.whole_view_rebuild);
        entry.topology_entity_count += report.topology_entity_count;
        entry.topology_relation_count += report.topology_relation_count;
        entry.interpreted_wire_count += report.interpreted_wire_count;
        entry.interpreted_shell_count += report.interpreted_shell_count;
        entry.validation_row_count += report.validation_row_count;
    }
    WorthDerivedRebuildAggregateReport {
        rows: rows.into_values().collect(),
    }
}

fn build_derived_fallback_aggregate_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedFallbackAggregateReport {
    let mut rows = BTreeMap::<String, WorthDerivedFallbackAggregateRow>::new();
    for case in &primitive_corpus.cases {
        let report = &case.certification.derived_fallback_report;
        let entry =
            rows.entry(case.family.clone())
                .or_insert_with(|| WorthDerivedFallbackAggregateRow {
                    family: case.family.clone(),
                    source_count: 0,
                    whole_view_materialization_count: 0,
                    explicit_fallback_count: 0,
                    precision_fallback_count: 0,
                    precision_budget_fallback_count: 0,
                });
        entry.source_count += 1;
        entry.whole_view_materialization_count += usize::from(report.whole_view_materialization);
        entry.explicit_fallback_count += report.explicit_fallback_count;
        entry.precision_fallback_count += report.precision_fallback_count;
        entry.precision_budget_fallback_count += report.precision_budget_fallback_count;
    }
    WorthDerivedFallbackAggregateReport {
        rows: rows.into_values().collect(),
    }
}

fn build_derived_equivalence_aggregate_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthDerivedEquivalenceContractAggregateReport {
    WorthDerivedEquivalenceContractAggregateReport {
        rows: primitive_corpus
            .cases
            .iter()
            .map(|case| {
                let report = &case.certification.derived_equivalence_contract_report;
                WorthDerivedEquivalenceContractAggregateRow {
                    source: case.stem.clone(),
                    family: case.family.clone(),
                    truth_basis_digest_hex: report.truth_basis_digest_hex.clone(),
                    touched_aspect_count: report.touched_aspect_count,
                    triggered_invalidation_target_count: report
                        .triggered_invalidation_targets
                        .len(),
                    materialized_topology_digest: report.materialized_topology_digest.clone(),
                    interpreted_topology_digest: report.interpreted_topology_digest.clone(),
                    derived_validation_digest: report.derived_validation_digest.clone(),
                }
            })
            .collect(),
    }
}

fn build_derived_failure_locality_report(
    primitive_corpus: &WorthPrimitiveCorpusReport,
) -> WorthFailureLocalityReport {
    WorthFailureLocalityReport {
        rows: primitive_corpus
            .rejected_cases
            .iter()
            .map(
                |case| crate::certification::report::WorthFailureLocalityRow {
                    family: case.family.clone(),
                    role: format!("{:?}", case.role),
                    validator_family: case.rejection.validator_family.clone(),
                    rejection_class: case.rejection.rejection_class.clone(),
                    diagnostic_code: case.rejection.diagnostic_code.clone(),
                    localized_entity_count: case.rejection.localized_entity_count,
                    localized_relation_count: case.rejection.localized_relation_count,
                },
            )
            .collect(),
    }
}

fn ensure_milestone_two_family_coverage_closure(
    report: &WorthDerivedFamilyCoverageMatrix,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_family_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing derived family coverage row for family `{family}`"
            )));
        };
        if !row.coverage_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout derived family coverage is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_milestone_two_parity_closure(
    report: &WorthDerivedFamilyParityMatrix,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_parity_rows {
        let Some(row) = report.rows.iter().find(|row| row.family == *family) else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing derived parity row for family `{family}`"
            )));
        };
        if !row.parity_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout derived parity is incomplete for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_milestone_two_bridge_closure(
    report: &crate::certification::report::WorthBridgeFamilyCoverageReport,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for bridge_family in &requirements.required_bridge_rows {
        let Some(row) = report
            .rows
            .iter()
            .find(|row| row.family == bridge_family.family)
        else {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing bridge family row for family `{}`",
                bridge_family.family
            )));
        };
        if !row.proof_complete {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout bridge proof is incomplete for family `{}`",
                bridge_family.family
            )));
        }
    }
    Ok(())
}

fn ensure_milestone_two_validator_closure(
    report: &WorthDerivedValidatorCoverageReport,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for expectation in &requirements.validator_expectations {
        for validator in &expectation.validators {
            let Some(row) = report
                .rows
                .iter()
                .find(|row| row.family == expectation.family && row.validator == *validator)
            else {
                return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                    "milestone two closeout missing derived validator coverage for family `{}` validator `{validator}`",
                    expectation.family
                )));
            };
            if row.passed_count == 0 {
                return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                    "milestone two closeout derived validator coverage is incomplete for family `{}` validator `{validator}`",
                    expectation.family
                )));
            }
        }
    }
    Ok(())
}

fn ensure_milestone_two_failure_locality_closure(
    report: &WorthFailureLocalityReport,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for family in &requirements.required_rejection_rows {
        if !report.rows.iter().any(|row| row.family == *family) {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing failure locality for family `{family}`"
            )));
        }
    }
    Ok(())
}

fn ensure_milestone_two_required_output_closure(
    closeout: &WorthMilestoneTwoCloseoutReport,
    requirements: &crate::certification::core::WorthCertificationSuiteRequirements,
) -> Result<(), WorthMilestoneOneCertificationError> {
    for output in &requirements.required_outputs {
        let present = match output {
            crate::certification::core::WorthCertificationRequiredOutput::MaterializedTopologyDigest => {
                closeout.materialized_topology_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::InterpretedTopologyDigest => {
                closeout.interpreted_topology_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedValidationDigest => {
                closeout.derived_validation_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedTruthBasisDigest => {
                closeout.derived_truth_basis_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::BridgeRoutingDigest => {
                closeout.bridge_routing_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::BridgeHistoricalEvaluationDigest => {
                closeout.bridge_historical_evaluation_digest.row_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedFamilyCoverageMatrix => {
                !closeout.derived_family_coverage_matrix.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedFamilyParityMatrix => {
                !closeout.derived_family_parity_matrix.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedValidatorCoverageReport => {
                !closeout.derived_validator_coverage_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedInvalidationReport => {
                !closeout.derived_invalidation_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedRebuildReport => {
                !closeout.derived_rebuild_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedEquivalenceContractReport => {
                !closeout.derived_equivalence_contract_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedFallbackReport => {
                !closeout.derived_fallback_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedFailureLocalityReport => {
                !closeout.derived_failure_locality_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedBranchLocalParityReport => {
                !closeout.derived_branch_local_parity_report.branch_ids.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedReplayParityReport => {
                closeout.derived_replay_parity_report.replay_checked_case_count > 0
            }
            crate::certification::core::WorthCertificationRequiredOutput::DerivedBridgeFamilyCoverageReport => {
                !closeout.derived_bridge_family_coverage_report.rows.is_empty()
            }
            crate::certification::core::WorthCertificationRequiredOutput::MilestoneTwoCounterReport => {
                closeout.milestone_2_counter_report.derived_read_count > 0
            }
            _ => true,
        };
        if !present {
            return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                "milestone two closeout missing required output `{output:?}`"
            )));
        }
    }
    Ok(())
}
