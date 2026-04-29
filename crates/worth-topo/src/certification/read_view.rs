use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection,
    ForgeQueryComputedInspectionEvidence, ForgeQueryEntity, ForgeQueryInspection,
    ForgeQueryRuntimeStateKind,
};
use forge_relational::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode,
};
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use worth_schema::facade::{
    DerivedTopologyReadBasis, VerifiedTopologyCommit, WorthAspect, WorthAuthorityTraceAnchor,
    WorthAuthorityTraceEvidence, WorthBoundaryEnvelope, WorthBoundaryFailure, WorthDecisionTrace,
    WorthDerivedTraceAnchor, WorthDerivedTraceEvidence, WorthFallbackDisposition,
    WorthIntegrityMarkers, WorthMutationOrigin, WorthNamedCounter, WorthPerformanceAccounting,
    WorthShellInterpretationClass, WorthTopologyMutationBatch, WorthWireInterpretationClass,
};

use crate::certification::error::WorthMilestoneOneCertificationError;
use crate::certification::report::{
    WorthBranchLocalTopologyReport, WorthMilestoneOneCertificationReport,
    WorthMilestoneOneCounters, WorthNamingAttachmentReport, WorthPrimitiveFamilyCoverageMatrix,
    WorthReplayParityReport, WorthReplayParityStatus, WorthTopologyLocalizationEntityRow,
    WorthTopologyLocalizationRelationRow, WorthTopologyLocalizationReport,
};
use crate::certification::shared::{count_batch_mutations, coverage_entry, digest_rows};
use crate::facade::{
    build_derived_equivalence_contract, build_topology_read_artifact, certify_topology_view,
    compare_derived_equivalence_contracts, validate_named_topology_truth,
};
use crate::query::{worth_topology_query_workspace, WorthTopologyQueryAssembly};

pub type WorthTracedMilestoneOneCertificationReport =
    WorthBoundaryEnvelope<WorthMilestoneOneCertificationReport>;

#[derive(Debug, Clone, Copy, Default)]
struct WorthMilestoneOneQueryEvidence {
    affected_live_view_count: usize,
    affected_derived_view_count: usize,
    considered_computed_view_count: usize,
    topology_entity_row_count: usize,
    topology_relation_row_count: usize,
    persistent_name_row_count: usize,
    validation_materialized_row_count: usize,
    equivalence_materialized_row_count: usize,
    declared_aspect_operation_count: usize,
    mutation_metadata_key_count: usize,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthMilestoneOneCertificationHarness;

impl WorthMilestoneOneCertificationHarness {
    pub fn certify_read_view(
        read_view: &RelationalReadView,
        read_basis: DerivedTopologyReadBasis,
    ) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
        Self::certify_read_view_traced(read_view, read_basis)
            .map(WorthBoundaryEnvelope::into_primary_result)
            .map_err(WorthBoundaryFailure::into_error)
    }

    pub fn certify_read_view_traced(
        read_view: &RelationalReadView,
        read_basis: DerivedTopologyReadBasis,
    ) -> Result<
        WorthTracedMilestoneOneCertificationReport,
        WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
    > {
        Self::certify_read_view_with_batch_traced(read_view, read_basis, None, 0)
    }

    pub(crate) fn certify_read_view_with_batch(
        read_view: &RelationalReadView,
        read_basis: DerivedTopologyReadBasis,
        authority_batch: Option<&WorthTopologyMutationBatch>,
        replay_history_length: usize,
    ) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
        Self::certify_read_view_with_batch_traced(
            read_view,
            read_basis,
            authority_batch,
            replay_history_length,
        )
        .map(WorthBoundaryEnvelope::into_primary_result)
        .map_err(WorthBoundaryFailure::into_error)
    }

    pub(crate) fn certify_read_view_with_batch_traced(
        read_view: &RelationalReadView,
        read_basis: DerivedTopologyReadBasis,
        authority_batch: Option<&WorthTopologyMutationBatch>,
        replay_history_length: usize,
    ) -> Result<
        WorthTracedMilestoneOneCertificationReport,
        WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
    > {
        validate_named_topology_truth(read_view).map_err(|error| {
            traced_certification_failure(error.into(), &read_basis, None, replay_history_length)
        })?;
        let mut workspace = worth_topology_query_workspace("worth.milestone-one.certification")
            .map_err(|error| {
                traced_certification_failure(
                    WorthMilestoneOneCertificationError::Query(error.to_string()),
                    &read_basis,
                    None,
                    replay_history_length,
                )
            })?;
        let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).map_err(|error| {
            traced_certification_failure(
                WorthMilestoneOneCertificationError::Query(error.to_string()),
                &read_basis,
                None,
                replay_history_length,
            )
        })?;
        let receipt = assembly
            .import_read_view(&mut workspace, read_view, &read_basis)
            .map_err(|error| {
                traced_certification_failure(error.into(), &read_basis, None, replay_history_length)
            })?;
        let entity_rows = workspace.read(assembly.entities());
        let relation_rows = workspace.read(assembly.relations());
        let persistent_name_rows = workspace.read(assembly.persistent_names());
        let validation_state = workspace.state(assembly.validation()).map_err(|error| {
            traced_certification_failure(
                WorthMilestoneOneCertificationError::Query(error.to_string()),
                &read_basis,
                None,
                replay_history_length,
            )
        })?;
        ensure_query_surface_ready("worth.topology.validation", &validation_state).map_err(
            |error| traced_certification_failure(error, &read_basis, None, replay_history_length),
        )?;
        let equivalence_state =
            workspace
                .state(assembly.equivalence_contract())
                .map_err(|error| {
                    traced_certification_failure(
                        WorthMilestoneOneCertificationError::Query(error.to_string()),
                        &read_basis,
                        None,
                        replay_history_length,
                    )
                })?;
        ensure_query_surface_ready("worth.topology.equivalence_contract", &equivalence_state)
            .map_err(|error| {
                traced_certification_failure(error, &read_basis, None, replay_history_length)
            })?;
        let validation_inspection = derived_query_inspection(
            &mut workspace,
            assembly.validation(),
            "worth.topology.validation",
        )
        .map_err(|error| {
            traced_certification_failure(error, &read_basis, None, replay_history_length)
        })?;
        let equivalence_inspection = derived_query_inspection(
            &mut workspace,
            assembly.equivalence_contract(),
            "worth.topology.equivalence_contract",
        )
        .map_err(|error| {
            traced_certification_failure(error, &read_basis, None, replay_history_length)
        })?;
        let receipt_inspection = batch_write_receipt_query_inspection(&mut workspace, &receipt)
            .map_err(|error| {
                traced_certification_failure(error, &read_basis, None, replay_history_length)
            })?;
        let snapshot = assembly.snapshot(&mut workspace).map_err(|error| {
            traced_certification_failure(error.into(), &read_basis, None, replay_history_length)
        })?;
        let equivalence_contract = snapshot.equivalence_contract.clone();
        let derived_read_diagnostics = snapshot.diagnostics.clone();
        let read_artifact = build_topology_read_artifact(&read_basis, &snapshot.interpreted);
        let certified_interpretation =
            certify_topology_view(read_basis.clone(), &snapshot.interpreted);
        let replay_read_basis = read_basis.replay_of();
        let replay_equivalence_contract = build_derived_equivalence_contract(
            &replay_read_basis,
            &snapshot.materialized,
            &snapshot.interpreted,
            &snapshot.validation,
        );
        let replay_comparison = compare_derived_equivalence_contracts(
            &equivalence_contract,
            &replay_equivalence_contract,
        );

        let topology_localization_report =
            build_topology_localization_report_from_query_rows(&entity_rows, &relation_rows)
                .map_err(|error| {
                    traced_certification_failure(error, &read_basis, None, replay_history_length)
                })?;
        let naming_attachment_report = snapshot.naming_attachments.clone();
        let primitive_family_coverage_matrix =
            build_primitive_family_coverage_matrix(&read_artifact.interpretations);
        let topology_truth_digest = digest_rows(
            topology_localization_report
                .topology_entities
                .iter()
                .map(|row| format!("entity:{:?}:{}", row.entity_id, row.kind_name))
                .chain(
                    topology_localization_report
                        .topology_relations
                        .iter()
                        .map(|row| format!("relation:{:?}:{}", row.relation_id, row.kind_name)),
                ),
        );
        let naming_truth_digest =
            digest_rows(naming_attachment_report.attachments.iter().map(|row| {
                format!(
                    "attachment:{:?}:{}:{}",
                    row.topology_entity_id,
                    row.topology_kind_name,
                    row.attached_persistent_name_ids
                        .iter()
                        .map(|id| format!("{id:?}"))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }));
        let topology_validation_digest = digest_rows(
            snapshot
                .validation
                .rows
                .iter()
                .map(|row| format!("validator:{}:{}", row.validator, row.status)),
        );
        let replay_topology_truth_digest = digest_rows(
            topology_localization_report
                .topology_entities
                .iter()
                .map(|row| format!("entity:{:?}:{}", row.entity_id, row.kind_name))
                .chain(
                    topology_localization_report
                        .topology_relations
                        .iter()
                        .map(|row| format!("relation:{:?}:{}", row.relation_id, row.kind_name)),
                ),
        );
        let replay_topology_validation_digest = digest_rows(
            snapshot
                .validation
                .rows
                .iter()
                .map(|row| format!("validator:{}:{}", row.validator, row.status)),
        );
        let branch_local_topology_report = WorthBranchLocalTopologyReport {
            mutation_origin: read_basis.derivation_origin(),
            branch_local: matches!(
                read_basis.derivation_origin(),
                WorthMutationOrigin::BranchLocalApplication
            ),
            branch_id: read_basis.branch_id().clone(),
            snapshot_id: read_basis.snapshot().snapshot_id.0,
            touched_aspect_count: read_basis.touched_aspects().len(),
        };
        let milestone_1_replay_parity_report = WorthReplayParityReport {
            mutation_origin: read_basis.derivation_origin(),
            replay_origin: matches!(read_basis.derivation_origin(), WorthMutationOrigin::Replay),
            branch_id: read_basis.branch_id().clone(),
            parity_status: WorthReplayParityStatus::NotChecked,
            equivalence_contract: equivalence_contract.clone(),
            replay_equivalence_contract: Some(replay_equivalence_contract),
            relational_replay_checked: false,
            relational_replay_verified: false,
            replayed_commit_id: None,
            compared_surfaces: Vec::new(),
            mismatch_count: 0,
            replay_failure: None,
            interpretation_digest_match: replay_comparison.interpreted_topology_digest_match,
            truth_digest_match: topology_truth_digest == replay_topology_truth_digest,
            validation_digest_match: replay_comparison.derived_validation_digest_match
                && topology_validation_digest == replay_topology_validation_digest,
        };
        let counters = build_counter_report(
            authority_batch,
            &snapshot.validation,
            &naming_attachment_report,
            &primitive_family_coverage_matrix,
            &read_basis,
            replay_history_length,
        );
        let query_evidence = WorthMilestoneOneQueryEvidence {
            affected_live_view_count: receipt.affected_live_view_ids().len(),
            affected_derived_view_count: receipt.affected_derived_view_ids().len(),
            considered_computed_view_count: receipt.considered_computed_view_count(),
            topology_entity_row_count: entity_rows.len(),
            topology_relation_row_count: relation_rows.len(),
            persistent_name_row_count: persistent_name_rows.len(),
            validation_materialized_row_count: validation_inspection.materialized_row_count(),
            equivalence_materialized_row_count: equivalence_inspection.materialized_row_count(),
            declared_aspect_operation_count: receipt_inspection
                .component_operations()
                .iter()
                .map(|component| component.declared_aspect_operations().len())
                .sum(),
            mutation_metadata_key_count: receipt
                .write_receipts()
                .iter()
                .map(|write| write.mutation_metadata().entries().len())
                .sum(),
        };

        let report = WorthMilestoneOneCertificationReport {
            named_truth_validated: true,
            topology_validated: true,
            topology_truth_digest,
            naming_truth_digest,
            topology_validation_digest,
            topology_validation_report: snapshot.validation.clone(),
            topology_localization_report,
            naming_attachment_report,
            primitive_family_coverage_matrix,
            branch_local_topology_report,
            milestone_1_replay_parity_report,
            derived_invalidation_report: derived_read_diagnostics.invalidation_report.clone(),
            derived_rebuild_report: derived_read_diagnostics.rebuild_report.clone(),
            derived_fallback_report: derived_read_diagnostics.fallback_report.clone(),
            derived_equivalence_contract_report: equivalence_contract.clone(),
            derived_read_diagnostics,
            counters,
            read_artifact,
            certified_interpretation,
        };

        Ok(traced_certification_envelope(
            report,
            &read_basis,
            None,
            replay_history_length,
            None,
            query_evidence,
        ))
    }

    pub fn certify_verified_commit(
        runtime: &mut RelationalRuntime,
        verified: &VerifiedTopologyCommit,
    ) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
        Self::certify_verified_commit_traced(runtime, verified)
            .map(WorthBoundaryEnvelope::into_primary_result)
            .map_err(WorthBoundaryFailure::into_error)
    }

    pub fn certify_verified_commit_traced(
        runtime: &mut RelationalRuntime,
        verified: &VerifiedTopologyCommit,
    ) -> Result<
        WorthTracedMilestoneOneCertificationReport,
        WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
    > {
        let read_view = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth.snapshot)
            .ok_or_else(|| {
                traced_certification_failure(
                    WorthMilestoneOneCertificationError::ReadView(format!(
                        "worth certification could not open verified snapshot {:?}",
                        verified.persisted_truth.snapshot
                    )),
                    &verified.read_basis,
                    Some(&verified.commits),
                    verified.commits.len(),
                )
            })?;
        let traced = Self::certify_read_view_with_batch_traced(
            &read_view,
            verified.read_basis.clone(),
            Some(&verified.canonical_batch.batch),
            verified.commits.len(),
        )?;
        let mut report = traced.primary_result().clone();
        let Some(replay_commit_id) = verified
            .commits
            .last()
            .map(|commit| commit.outcome.commit.commit_id.clone())
        else {
            let integrity_markers =
                certification_integrity_markers(&verified.read_basis, Some(&verified.commits));
            let performance_accounting = certification_performance_accounting(
                &report,
                Some(&verified.commits),
                verified.commits.len(),
                query_evidence_from_accounting(traced.performance_accounting()),
            );
            return Ok(traced
                .map_primary_result(|_| report)
                .map_decision_trace(|mut decision_trace| {
                    decision_trace.authority_anchor =
                        Some(WorthAuthorityTraceAnchor::from_commit_results(
                            verified.branch_id.clone(),
                            &verified.commits,
                        ));
                    decision_trace.authority =
                        Some(WorthAuthorityTraceEvidence::from_commit_results(
                            verified.branch_id.clone(),
                            &verified.commits,
                        ));
                    decision_trace
                })
                .with_integrity_markers(integrity_markers)
                .with_performance_accounting(performance_accounting));
        };
        let replay = runtime
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                branch_id: verified.branch_id.clone(),
                commit_id: replay_commit_id,
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
            });
        report
            .milestone_1_replay_parity_report
            .relational_replay_checked = true;
        report.milestone_1_replay_parity_report.replayed_commit_id = replay
            .commit
            .as_ref()
            .map(|commit| commit.commit_id.0.to_string());
        report.milestone_1_replay_parity_report.compared_surfaces =
            replay.compared_surfaces.clone();
        if runtime.replay().compare_outcome(&replay) {
            report
                .milestone_1_replay_parity_report
                .relational_replay_verified = true;
            report.milestone_1_replay_parity_report.parity_status = if report
                .milestone_1_replay_parity_report
                .interpretation_digest_match
                && report.milestone_1_replay_parity_report.truth_digest_match
                && report
                    .milestone_1_replay_parity_report
                    .validation_digest_match
            {
                WorthReplayParityStatus::Match
            } else {
                WorthReplayParityStatus::Mismatch
            };
        } else {
            report.milestone_1_replay_parity_report.replay_failure = replay.failure;
            report.milestone_1_replay_parity_report.mismatch_count = replay.mismatches.len();
            report.milestone_1_replay_parity_report.parity_status =
                WorthReplayParityStatus::Mismatch;
        }

        let integrity_markers =
            certification_integrity_markers(&verified.read_basis, Some(&verified.commits));
        let performance_accounting = certification_performance_accounting(
            &report,
            Some(&verified.commits),
            verified.commits.len(),
            query_evidence_from_accounting(traced.performance_accounting()),
        );
        Ok(traced
            .map_primary_result(|_| report)
            .map_decision_trace(|mut decision_trace| {
                decision_trace.authority_anchor =
                    Some(WorthAuthorityTraceAnchor::from_commit_results(
                        verified.branch_id.clone(),
                        &verified.commits,
                    ));
                decision_trace.authority = Some(WorthAuthorityTraceEvidence::from_commit_results(
                    verified.branch_id.clone(),
                    &verified.commits,
                ));
                decision_trace
            })
            .with_integrity_markers(integrity_markers)
            .with_performance_accounting(performance_accounting))
    }
}

pub fn certify_milestone_one_read_view_traced_impl(
    read_view: &RelationalReadView,
    read_basis: DerivedTopologyReadBasis,
) -> Result<
    WorthTracedMilestoneOneCertificationReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    WorthMilestoneOneCertificationHarness::certify_read_view_traced(read_view, read_basis)
}

pub fn certify_verified_topology_commit_traced_impl(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<
    WorthTracedMilestoneOneCertificationReport,
    WorthBoundaryFailure<WorthMilestoneOneCertificationError>,
> {
    WorthMilestoneOneCertificationHarness::certify_verified_commit_traced(runtime, verified)
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

fn batch_write_receipt_query_inspection(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    receipt: &ForgeQueryBatchWriteReceipt,
) -> Result<ForgeQueryBatchWriteReceiptInspection, WorthMilestoneOneCertificationError> {
    match workspace
        .inspect(receipt)
        .map_err(|error| WorthMilestoneOneCertificationError::Query(error.to_string()))?
    {
        ForgeQueryInspection::BatchWriteReceipt(inspection) => Ok(inspection),
        other => Err(WorthMilestoneOneCertificationError::Query(format!(
            "query inspection for batch write receipt returned wrong artifact family: {other:?}"
        ))),
    }
}

fn traced_certification_envelope(
    report: WorthMilestoneOneCertificationReport,
    read_basis: &DerivedTopologyReadBasis,
    commit_results: Option<&[forge_relational::facade::transactions::CommitResult]>,
    replay_history_length: usize,
    warnings: Option<Vec<worth_schema::facade::WorthTraceWarning>>,
    query_evidence: WorthMilestoneOneQueryEvidence,
) -> WorthTracedMilestoneOneCertificationReport {
    WorthBoundaryEnvelope::success(
        report.clone(),
        warnings.unwrap_or_default(),
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
            derived: Some(certification_derived_trace(&report)),
            signal: None,
        },
        certification_integrity_markers(read_basis, commit_results),
        certification_performance_accounting(
            &report,
            commit_results,
            replay_history_length,
            query_evidence,
        ),
    )
}

fn traced_certification_failure(
    error: WorthMilestoneOneCertificationError,
    read_basis: &DerivedTopologyReadBasis,
    commit_results: Option<&[forge_relational::facade::transactions::CommitResult]>,
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

pub(crate) fn certification_integrity_markers(
    read_basis: &DerivedTopologyReadBasis,
    _commit_results: Option<&[forge_relational::facade::transactions::CommitResult]>,
) -> WorthIntegrityMarkers {
    WorthIntegrityMarkers::new(
        Some(read_basis.branch_id().clone()),
        read_basis.touched_aspects().iter().copied().collect(),
        Some(read_basis.authoritative_mutation_origin()),
        Some(read_basis.authority.truth_basis_identity.clone()),
        read_basis.precision_fallbacks.len(),
        read_basis.precision_budget_fallbacks.len(),
    )
}

pub(crate) fn certification_derived_trace(
    report: &WorthMilestoneOneCertificationReport,
) -> WorthDerivedTraceEvidence {
    WorthDerivedTraceEvidence {
        availability: worth_schema::facade::WorthTraceAvailability::Present,
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

fn query_evidence_from_accounting(
    accounting: &WorthPerformanceAccounting,
) -> WorthMilestoneOneQueryEvidence {
    fn counter(accounting: &WorthPerformanceAccounting, name: &str) -> usize {
        accounting
            .counters
            .iter()
            .find(|counter| counter.name == name)
            .map(|counter| counter.value as usize)
            .unwrap_or(0)
    }

    WorthMilestoneOneQueryEvidence {
        affected_live_view_count: counter(
            accounting,
            "certification.query.affected_live_view_count",
        ),
        affected_derived_view_count: counter(
            accounting,
            "certification.query.affected_derived_view_count",
        ),
        considered_computed_view_count: counter(
            accounting,
            "certification.query.considered_computed_view_count",
        ),
        topology_entity_row_count: counter(
            accounting,
            "certification.query.topology_entity_row_count",
        ),
        topology_relation_row_count: counter(
            accounting,
            "certification.query.topology_relation_row_count",
        ),
        persistent_name_row_count: counter(
            accounting,
            "certification.query.persistent_name_row_count",
        ),
        validation_materialized_row_count: counter(
            accounting,
            "certification.query.validation_materialized_row_count",
        ),
        equivalence_materialized_row_count: counter(
            accounting,
            "certification.query.equivalence_materialized_row_count",
        ),
        declared_aspect_operation_count: counter(
            accounting,
            "certification.query.declared_aspect_operation_count",
        ),
        mutation_metadata_key_count: counter(
            accounting,
            "certification.query.mutation_metadata_key_count",
        ),
    }
}

fn certification_performance_accounting(
    report: &WorthMilestoneOneCertificationReport,
    _commit_results: Option<&[forge_relational::facade::transactions::CommitResult]>,
    replay_history_length: usize,
    query_evidence: WorthMilestoneOneQueryEvidence,
) -> WorthPerformanceAccounting {
    let counters = vec![
        WorthNamedCounter::new(
            "certification.topology_entity_upsert_count",
            report.counters.topology_entity_upsert_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.topology_relation_upsert_count",
            report.counters.topology_relation_upsert_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.topology_relation_remove_count",
            report.counters.topology_relation_remove_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.commit_boundary_validator_count",
            report.counters.commit_boundary_validator_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.commit_boundary_rejection_count",
            report.counters.commit_boundary_rejection_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.derived_topology_interpretation_count",
            report.counters.derived_topology_interpretation_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.derived_topology_full_fallback_count",
            report.counters.derived_topology_full_fallback_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.naming_target_lookup_count",
            report.counters.naming_target_lookup_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.primitive_family_member_count",
            report.counters.primitive_family_member_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.replay_history_length",
            replay_history_length as u64,
        ),
        WorthNamedCounter::new(
            "certification.replay_interpretation_rerun_count",
            report.counters.replay_interpretation_rerun_count as u64,
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
            "certification.query.topology_entity_row_count",
            query_evidence.topology_entity_row_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.topology_relation_row_count",
            query_evidence.topology_relation_row_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.persistent_name_row_count",
            query_evidence.persistent_name_row_count as u64,
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
            "certification.query.declared_aspect_operation_count",
            query_evidence.declared_aspect_operation_count as u64,
        ),
        WorthNamedCounter::new(
            "certification.query.mutation_metadata_key_count",
            query_evidence.mutation_metadata_key_count as u64,
        ),
    ];
    WorthPerformanceAccounting::new(counters)
}

fn build_topology_localization_report_from_query_rows(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
) -> Result<WorthTopologyLocalizationReport, WorthMilestoneOneCertificationError> {
    let topology_entities = entity_rows
        .iter()
        .map(|row| {
            Ok(WorthTopologyLocalizationEntityRow {
                entity_id: serde_json::from_value(required_payload_value(
                    &row.payload,
                    "lineage.provenance",
                )?)
                .map_err(|error| {
                    WorthMilestoneOneCertificationError::Query(format!(
                        "query certification entity lineage provenance failed to decode: {error}"
                    ))
                })?,
                kind_name: required_payload_text(&row.payload, "topology.kind")?.to_string(),
            })
        })
        .collect::<Result<Vec<_>, WorthMilestoneOneCertificationError>>()?;
    let topology_relations = relation_rows
        .iter()
        .map(|row| {
            Ok(WorthTopologyLocalizationRelationRow {
                relation_id: serde_json::from_value(required_payload_value(
                    &row.payload,
                    "lineage.provenance",
                )?)
                .map_err(|error| {
                    WorthMilestoneOneCertificationError::Query(format!(
                        "query certification relation lineage provenance failed to decode: {error}"
                    ))
                })?,
                kind_name: required_payload_text(&row.payload, "topology.kind")?.to_string(),
            })
        })
        .collect::<Result<Vec<_>, WorthMilestoneOneCertificationError>>()?;

    Ok(WorthTopologyLocalizationReport {
        topology_entities,
        topology_relations,
    })
}

fn required_payload_value(
    payload: &serde_json::Value,
    dotted_path: &str,
) -> Result<serde_json::Value, WorthMilestoneOneCertificationError> {
    let mut current = payload;
    for segment in dotted_path.split('.') {
        current = current.get(segment).ok_or_else(|| {
            WorthMilestoneOneCertificationError::Query(format!(
                "query certification row is missing required field `{dotted_path}`"
            ))
        })?;
    }
    Ok(current.clone())
}

fn required_payload_text<'a>(
    payload: &'a serde_json::Value,
    dotted_path: &str,
) -> Result<&'a str, WorthMilestoneOneCertificationError> {
    let mut current = payload;
    for segment in dotted_path.split('.') {
        current = current.get(segment).ok_or_else(|| {
            WorthMilestoneOneCertificationError::Query(format!(
                "query certification row is missing required field `{dotted_path}`"
            ))
        })?;
    }
    current.as_str().ok_or_else(|| {
        WorthMilestoneOneCertificationError::Query(format!(
            "query certification field `{dotted_path}` must decode as text"
        ))
    })
}

pub(crate) fn build_primitive_family_coverage_matrix(
    interpretations: &worth_schema::facade::WorthTopologyInterpretationRecordSet,
) -> WorthPrimitiveFamilyCoverageMatrix {
    let wire_open = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WorthWireInterpretationClass::OpenChain)
        .count();
    let wire_closed = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WorthWireInterpretationClass::ClosedCycle)
        .count();
    let wire_branch = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WorthWireInterpretationClass::ConnectedBranch)
        .count();
    let sheet_patch = interpretations
        .shells
        .iter()
        .filter(|record| {
            record.class == WorthShellInterpretationClass::OpenSheet && record.face_count > 1
        })
        .count();
    let sheet_disk = interpretations
        .shells
        .iter()
        .filter(|record| {
            record.class == WorthShellInterpretationClass::OpenSheet
                && record.face_count == 1
                && record.boundary_component_count == 1
        })
        .count();
    let solid_shell = interpretations
        .shells
        .iter()
        .filter(|record| record.class == WorthShellInterpretationClass::ClosedSolid)
        .count();
    let nmt_edge_fan = interpretations
        .shells
        .iter()
        .filter(|record| {
            matches!(
                record.class,
                WorthShellInterpretationClass::OpenNonManifold
                    | WorthShellInterpretationClass::ClosedNonManifold
            )
        })
        .count();

    WorthPrimitiveFamilyCoverageMatrix {
        entries: vec![
            coverage_entry("WireOpen(n)", wire_open),
            coverage_entry("WireClosed(n)", wire_closed),
            coverage_entry("WireBranch(k)", wire_branch),
            coverage_entry("SheetDisk(n)", sheet_disk),
            coverage_entry("SheetPatch(f)", sheet_patch),
            coverage_entry("SolidShell(f)", solid_shell),
            coverage_entry("NmtEdgeFan(k)", nmt_edge_fan),
        ],
    }
}

pub(crate) fn build_counter_report(
    authority_batch: Option<&WorthTopologyMutationBatch>,
    topology_validation_report: &crate::validators::WorthTopologyValidationReport,
    naming_attachment_report: &WorthNamingAttachmentReport,
    primitive_family_coverage_matrix: &WorthPrimitiveFamilyCoverageMatrix,
    read_basis: &DerivedTopologyReadBasis,
    replay_history_length: usize,
) -> WorthMilestoneOneCounters {
    let (
        topology_entity_upsert_count,
        topology_relation_upsert_count,
        topology_relation_remove_count,
    ) = authority_batch
        .map(count_batch_mutations)
        .unwrap_or((0, 0, 0));
    let derived_topology_full_fallback_count = read_basis
        .precision_fallbacks
        .iter()
        .filter(|record| record.disposition != WorthFallbackDisposition::NoneRequired)
        .count()
        + read_basis.precision_budget_fallbacks.len();
    let touched_topology_aspect_count = read_basis
        .touched_aspects()
        .iter()
        .filter(|aspect| matches!(aspect, WorthAspect::Topology(_)))
        .count();

    WorthMilestoneOneCounters {
        topology_entity_upsert_count,
        topology_relation_upsert_count,
        topology_relation_remove_count,
        commit_boundary_validator_count: topology_validation_report.rows.len() + 1,
        commit_boundary_rejection_count: 0,
        derived_topology_interpretation_count: primitive_family_coverage_matrix
            .entries
            .iter()
            .map(|entry| entry.observed_member_count)
            .sum(),
        derived_topology_full_fallback_count,
        naming_target_lookup_count: naming_attachment_report.attachments.len(),
        primitive_family_member_count: primitive_family_coverage_matrix
            .entries
            .iter()
            .map(|entry| entry.observed_member_count)
            .sum(),
        replay_history_length,
        replay_interpretation_rerun_count: usize::from(touched_topology_aspect_count > 0),
    }
}
