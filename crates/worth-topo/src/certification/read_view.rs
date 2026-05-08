use forge_query::facade::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryEntity, ForgeQueryInspection,
    ForgeQueryRuntimeStateKind,
};
use forge_relational::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode,
};
use forge_relational::facade::runtime::RelationalRuntime;
use schema::facade::{
    Aspect, AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure,
    DecisionTrace, DerivedTopologyReadBasis, DerivedTraceAnchor, DerivedTraceEvidence,
    FallbackDisposition, IntegrityMarkers, MutationOrigin, NamedCounter, PerformanceAccounting,
    ShellInterpretationClass, TopologyMutationBatch, VerifiedTopologyCommit,
    WireInterpretationClass,
};

use crate::certification::error::MilestoneOneCertificationError;
use crate::certification::report::{
    BranchLocalTopologyReport, MilestoneOneCertificationReport, MilestoneOneCounters,
    NamingAttachmentReport, PrimitiveFamilyCoverageMatrix, ReplayParityReport, ReplayParityStatus,
    TopologyLocalizationEntityRow, TopologyLocalizationRelationRow, TopologyLocalizationReport,
};
use crate::certification::shared::{count_batch_mutations, coverage_entry, digest_rows};
use crate::facade::{
    build_derived_equivalence_contract, build_topology_read_artifact, certify_topology_view,
    compare_derived_equivalence_contracts, validate_named_topology_truth,
};
use crate::query::{topology_runtime, TopologyQueryAssembly, TopologyRuntimeAdapters};

pub type TracedMilestoneOneCertificationReport = BoundaryEnvelope<MilestoneOneCertificationReport>;

#[derive(Debug, Clone, Copy, Default)]
struct MilestoneOneQueryEvidence {
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
pub struct MilestoneOneCertificationHarness;

impl MilestoneOneCertificationHarness {
    pub(crate) fn certify_read_basis_with_runtime_traced(
        runtime: &mut RelationalRuntime,
        read_basis: DerivedTopologyReadBasis,
        authority_batch: Option<&TopologyMutationBatch>,
        replay_history_length: usize,
    ) -> Result<
        TracedMilestoneOneCertificationReport,
        BoundaryFailure<MilestoneOneCertificationError>,
    > {
        let read_view = runtime
            .read_truth()
            .read_snapshot(read_basis.snapshot())
            .ok_or_else(|| {
                traced_certification_failure(
                    MilestoneOneCertificationError::ReadView(format!(
                        " certification could not open snapshot {:?}",
                        read_basis.snapshot()
                    )),
                    &read_basis,
                    None,
                    replay_history_length,
                )
            })?;
        validate_named_topology_truth(&read_view).map_err(|error| {
            traced_certification_failure(error.into(), &read_basis, None, replay_history_length)
        })?;

        let adapters =
            TopologyRuntimeAdapters::snapshot_read_only(read_view, read_basis.snapshot().clone());
        let mut workspace =
            topology_runtime(adapters, ".milestone-one.certification").map_err(|error| {
                traced_certification_failure(
                    MilestoneOneCertificationError::Query(error.to_string()),
                    &read_basis,
                    None,
                    replay_history_length,
                )
            })?;
        let assembly = TopologyQueryAssembly::declare(&mut workspace).map_err(|error| {
            traced_certification_failure(
                MilestoneOneCertificationError::Query(error.to_string()),
                &read_basis,
                None,
                replay_history_length,
            )
        })?;
        let entity_rows = workspace.read(assembly.entities());
        let relation_rows = workspace.read(assembly.relations());
        let persistent_name_rows = workspace.read(assembly.persistent_names());
        let validation_state = workspace.state(assembly.validation()).map_err(|error| {
            traced_certification_failure(
                MilestoneOneCertificationError::Query(error.to_string()),
                &read_basis,
                None,
                replay_history_length,
            )
        })?;
        ensure_query_surface_ready(".topology.validation", &validation_state).map_err(|error| {
            traced_certification_failure(error, &read_basis, None, replay_history_length)
        })?;
        let equivalence_state =
            workspace
                .state(assembly.equivalence_contract())
                .map_err(|error| {
                    traced_certification_failure(
                        MilestoneOneCertificationError::Query(error.to_string()),
                        &read_basis,
                        None,
                        replay_history_length,
                    )
                })?;
        ensure_query_surface_ready(".topology.equivalence_contract", &equivalence_state).map_err(
            |error| traced_certification_failure(error, &read_basis, None, replay_history_length),
        )?;
        let validation_inspection = derived_query_inspection(
            &mut workspace,
            assembly.validation(),
            ".topology.validation",
        )
        .map_err(|error| {
            traced_certification_failure(error, &read_basis, None, replay_history_length)
        })?;
        let equivalence_inspection = derived_query_inspection(
            &mut workspace,
            assembly.equivalence_contract(),
            ".topology.equivalence_contract",
        )
        .map_err(|error| {
            traced_certification_failure(error, &read_basis, None, replay_history_length)
        })?;
        let snapshot = assembly
            .snapshot_for_read_basis(&mut workspace, &read_basis)
            .map_err(|error| {
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
        let branch_local_topology_report = BranchLocalTopologyReport {
            mutation_origin: read_basis.derivation_origin(),
            branch_local: matches!(
                read_basis.derivation_origin(),
                MutationOrigin::BranchLocalApplication
            ),
            branch_id: read_basis.branch_id().clone(),
            snapshot_id: read_basis.snapshot().snapshot_id.0,
            touched_aspect_count: read_basis.touched_aspects().len(),
        };
        let milestone_1_replay_parity_report = ReplayParityReport {
            mutation_origin: read_basis.derivation_origin(),
            replay_origin: matches!(read_basis.derivation_origin(), MutationOrigin::Replay),
            branch_id: read_basis.branch_id().clone(),
            parity_status: ReplayParityStatus::NotChecked,
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
        let query_evidence = MilestoneOneQueryEvidence {
            affected_live_view_count: 0,
            affected_derived_view_count: 0,
            considered_computed_view_count: 0,
            topology_entity_row_count: entity_rows.len(),
            topology_relation_row_count: relation_rows.len(),
            persistent_name_row_count: persistent_name_rows.len(),
            validation_materialized_row_count: validation_inspection.materialized_row_count(),
            equivalence_materialized_row_count: equivalence_inspection.materialized_row_count(),
            declared_aspect_operation_count: 0,
            mutation_metadata_key_count: 0,
        };
        let report = MilestoneOneCertificationReport {
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
    ) -> Result<MilestoneOneCertificationReport, MilestoneOneCertificationError> {
        Self::certify_verified_commit_traced(runtime, verified)
            .map(BoundaryEnvelope::into_primary_result)
            .map_err(BoundaryFailure::into_error)
    }

    pub fn certify_verified_commit_traced(
        runtime: &mut RelationalRuntime,
        verified: &VerifiedTopologyCommit,
    ) -> Result<
        TracedMilestoneOneCertificationReport,
        BoundaryFailure<MilestoneOneCertificationError>,
    > {
        let traced = Self::certify_read_basis_with_runtime_traced(
            runtime,
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
                        Some(AuthorityTraceAnchor::from_commit_results(
                            verified.branch_id.clone(),
                            &verified.commits,
                        ));
                    decision_trace.authority = Some(AuthorityTraceEvidence::from_commit_results(
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
                ReplayParityStatus::Match
            } else {
                ReplayParityStatus::Mismatch
            };
        } else {
            report.milestone_1_replay_parity_report.replay_failure = replay.failure;
            report.milestone_1_replay_parity_report.mismatch_count = replay.mismatches.len();
            report.milestone_1_replay_parity_report.parity_status = ReplayParityStatus::Mismatch;
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
                decision_trace.authority_anchor = Some(AuthorityTraceAnchor::from_commit_results(
                    verified.branch_id.clone(),
                    &verified.commits,
                ));
                decision_trace.authority = Some(AuthorityTraceEvidence::from_commit_results(
                    verified.branch_id.clone(),
                    &verified.commits,
                ));
                decision_trace
            })
            .with_integrity_markers(integrity_markers)
            .with_performance_accounting(performance_accounting))
    }
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

fn traced_certification_envelope(
    report: MilestoneOneCertificationReport,
    read_basis: &DerivedTopologyReadBasis,
    commit_results: Option<&[forge_relational::facade::transactions::CommitResult]>,
    replay_history_length: usize,
    warnings: Option<Vec<schema::facade::TraceWarning>>,
    query_evidence: MilestoneOneQueryEvidence,
) -> TracedMilestoneOneCertificationReport {
    BoundaryEnvelope::success(
        report.clone(),
        warnings.unwrap_or_default(),
        DecisionTrace {
            authority_anchor: commit_results.map(|commits| {
                AuthorityTraceAnchor::from_commit_results(read_basis.branch_id().clone(), commits)
            }),
            bridge_anchor: None,
            derived_anchor: Some(DerivedTraceAnchor::from_read_basis(read_basis)),
            signal_anchor: None,
            authority: commit_results.map(|commits| {
                AuthorityTraceEvidence::from_commit_results(read_basis.branch_id().clone(), commits)
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
    error: MilestoneOneCertificationError,
    read_basis: &DerivedTopologyReadBasis,
    commit_results: Option<&[forge_relational::facade::transactions::CommitResult]>,
    replay_history_length: usize,
) -> BoundaryFailure<MilestoneOneCertificationError> {
    BoundaryFailure::failure(
        error,
        Vec::new(),
        DecisionTrace {
            authority_anchor: commit_results.map(|commits| {
                AuthorityTraceAnchor::from_commit_results(read_basis.branch_id().clone(), commits)
            }),
            bridge_anchor: None,
            derived_anchor: Some(DerivedTraceAnchor::from_read_basis(read_basis)),
            signal_anchor: None,
            authority: commit_results.map(|commits| {
                AuthorityTraceEvidence::from_commit_results(read_basis.branch_id().clone(), commits)
            }),
            bridge: None,
            derived: None,
            signal: None,
        },
        certification_integrity_markers(read_basis, commit_results),
        PerformanceAccounting::new([NamedCounter::new(
            "certification.replay_history_length",
            replay_history_length as u64,
        )]),
    )
}

pub(crate) fn certification_integrity_markers(
    read_basis: &DerivedTopologyReadBasis,
    _commit_results: Option<&[forge_relational::facade::transactions::CommitResult]>,
) -> IntegrityMarkers {
    IntegrityMarkers::new(
        Some(read_basis.branch_id().clone()),
        read_basis.touched_aspects().iter().copied().collect(),
        Some(read_basis.authoritative_mutation_origin()),
        Some(read_basis.authority.truth_basis_identity.clone()),
        read_basis.precision_fallbacks.len(),
        read_basis.precision_budget_fallbacks.len(),
    )
}

pub(crate) fn certification_derived_trace(
    report: &MilestoneOneCertificationReport,
) -> DerivedTraceEvidence {
    DerivedTraceEvidence {
        availability: schema::facade::TraceAvailability::Present,
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

fn query_evidence_from_accounting(accounting: &PerformanceAccounting) -> MilestoneOneQueryEvidence {
    fn counter(accounting: &PerformanceAccounting, name: &str) -> usize {
        accounting
            .counters
            .iter()
            .find(|counter| counter.name == name)
            .map(|counter| counter.value as usize)
            .unwrap_or(0)
    }

    MilestoneOneQueryEvidence {
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
    report: &MilestoneOneCertificationReport,
    _commit_results: Option<&[forge_relational::facade::transactions::CommitResult]>,
    replay_history_length: usize,
    query_evidence: MilestoneOneQueryEvidence,
) -> PerformanceAccounting {
    let counters = vec![
        NamedCounter::new(
            "certification.topology_entity_upsert_count",
            report.counters.topology_entity_upsert_count as u64,
        ),
        NamedCounter::new(
            "certification.topology_relation_upsert_count",
            report.counters.topology_relation_upsert_count as u64,
        ),
        NamedCounter::new(
            "certification.topology_relation_remove_count",
            report.counters.topology_relation_remove_count as u64,
        ),
        NamedCounter::new(
            "certification.commit_boundary_validator_count",
            report.counters.commit_boundary_validator_count as u64,
        ),
        NamedCounter::new(
            "certification.commit_boundary_rejection_count",
            report.counters.commit_boundary_rejection_count as u64,
        ),
        NamedCounter::new(
            "certification.derived_topology_interpretation_count",
            report.counters.derived_topology_interpretation_count as u64,
        ),
        NamedCounter::new(
            "certification.derived_topology_full_fallback_count",
            report.counters.derived_topology_full_fallback_count as u64,
        ),
        NamedCounter::new(
            "certification.naming_target_lookup_count",
            report.counters.naming_target_lookup_count as u64,
        ),
        NamedCounter::new(
            "certification.primitive_family_member_count",
            report.counters.primitive_family_member_count as u64,
        ),
        NamedCounter::new(
            "certification.replay_history_length",
            replay_history_length as u64,
        ),
        NamedCounter::new(
            "certification.replay_interpretation_rerun_count",
            report.counters.replay_interpretation_rerun_count as u64,
        ),
        NamedCounter::new(
            "certification.derived_invalidation_target_count",
            report.derived_invalidation_report.triggered_target_count as u64,
        ),
        NamedCounter::new(
            "certification.query.affected_live_view_count",
            query_evidence.affected_live_view_count as u64,
        ),
        NamedCounter::new(
            "certification.query.affected_derived_view_count",
            query_evidence.affected_derived_view_count as u64,
        ),
        NamedCounter::new(
            "certification.query.considered_computed_view_count",
            query_evidence.considered_computed_view_count as u64,
        ),
        NamedCounter::new(
            "certification.query.topology_entity_row_count",
            query_evidence.topology_entity_row_count as u64,
        ),
        NamedCounter::new(
            "certification.query.topology_relation_row_count",
            query_evidence.topology_relation_row_count as u64,
        ),
        NamedCounter::new(
            "certification.query.persistent_name_row_count",
            query_evidence.persistent_name_row_count as u64,
        ),
        NamedCounter::new(
            "certification.query.validation_materialized_row_count",
            query_evidence.validation_materialized_row_count as u64,
        ),
        NamedCounter::new(
            "certification.query.equivalence_materialized_row_count",
            query_evidence.equivalence_materialized_row_count as u64,
        ),
        NamedCounter::new(
            "certification.query.declared_aspect_operation_count",
            query_evidence.declared_aspect_operation_count as u64,
        ),
        NamedCounter::new(
            "certification.query.mutation_metadata_key_count",
            query_evidence.mutation_metadata_key_count as u64,
        ),
    ];
    PerformanceAccounting::new(counters)
}

fn build_topology_localization_report_from_query_rows(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
) -> Result<TopologyLocalizationReport, MilestoneOneCertificationError> {
    let topology_entities = entity_rows
        .iter()
        .map(|row| {
            Ok(TopologyLocalizationEntityRow {
                entity_id: serde_json::from_value(required_payload_value(
                    &row.payload,
                    "lineage.provenance",
                )?)
                .map_err(|error| {
                    MilestoneOneCertificationError::Query(format!(
                        "query certification entity lineage provenance failed to decode: {error}"
                    ))
                })?,
                kind_name: required_payload_text(&row.payload, "topology.kind")?.to_string(),
            })
        })
        .collect::<Result<Vec<_>, MilestoneOneCertificationError>>()?;
    let topology_relations = relation_rows
        .iter()
        .map(|row| {
            Ok(TopologyLocalizationRelationRow {
                relation_id: serde_json::from_value(required_payload_value(
                    &row.payload,
                    "lineage.provenance",
                )?)
                .map_err(|error| {
                    MilestoneOneCertificationError::Query(format!(
                        "query certification relation lineage provenance failed to decode: {error}"
                    ))
                })?,
                kind_name: required_payload_text(&row.payload, "topology.kind")?.to_string(),
            })
        })
        .collect::<Result<Vec<_>, MilestoneOneCertificationError>>()?;

    Ok(TopologyLocalizationReport {
        topology_entities,
        topology_relations,
    })
}

fn required_payload_value(
    payload: &serde_json::Value,
    dotted_path: &str,
) -> Result<serde_json::Value, MilestoneOneCertificationError> {
    let mut current = payload;
    for segment in dotted_path.split('.') {
        current = current.get(segment).ok_or_else(|| {
            MilestoneOneCertificationError::Query(format!(
                "query certification row is missing required field `{dotted_path}`"
            ))
        })?;
    }
    Ok(current.clone())
}

fn required_payload_text<'a>(
    payload: &'a serde_json::Value,
    dotted_path: &str,
) -> Result<&'a str, MilestoneOneCertificationError> {
    let mut current = payload;
    for segment in dotted_path.split('.') {
        current = current.get(segment).ok_or_else(|| {
            MilestoneOneCertificationError::Query(format!(
                "query certification row is missing required field `{dotted_path}`"
            ))
        })?;
    }
    current.as_str().ok_or_else(|| {
        MilestoneOneCertificationError::Query(format!(
            "query certification field `{dotted_path}` must decode as text"
        ))
    })
}

pub(crate) fn build_primitive_family_coverage_matrix(
    interpretations: &schema::facade::TopologyInterpretationRecordSet,
) -> PrimitiveFamilyCoverageMatrix {
    let wire_open = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WireInterpretationClass::OpenChain)
        .count();
    let wire_closed = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WireInterpretationClass::ClosedCycle)
        .count();
    let wire_branch = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WireInterpretationClass::ConnectedBranch)
        .count();
    let sheet_patch = interpretations
        .shells
        .iter()
        .filter(|record| {
            record.class == ShellInterpretationClass::OpenSheet && record.face_count > 1
        })
        .count();
    let sheet_disk = interpretations
        .shells
        .iter()
        .filter(|record| {
            record.class == ShellInterpretationClass::OpenSheet
                && record.face_count == 1
                && record.boundary_component_count == 1
        })
        .count();
    let solid_shell = interpretations
        .shells
        .iter()
        .filter(|record| record.class == ShellInterpretationClass::ClosedSolid)
        .count();
    let nmt_edge_fan = interpretations
        .shells
        .iter()
        .filter(|record| {
            matches!(
                record.class,
                ShellInterpretationClass::OpenNonManifold
                    | ShellInterpretationClass::ClosedNonManifold
            )
        })
        .count();

    PrimitiveFamilyCoverageMatrix {
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
    authority_batch: Option<&TopologyMutationBatch>,
    topology_validation_report: &crate::validators::TopologyValidationReport,
    naming_attachment_report: &NamingAttachmentReport,
    primitive_family_coverage_matrix: &PrimitiveFamilyCoverageMatrix,
    read_basis: &DerivedTopologyReadBasis,
    replay_history_length: usize,
) -> MilestoneOneCounters {
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
        .filter(|record| record.disposition != FallbackDisposition::NoneRequired)
        .count()
        + read_basis.precision_budget_fallbacks.len();
    let touched_topology_aspect_count = read_basis
        .touched_aspects()
        .iter()
        .filter(|aspect| matches!(aspect, Aspect::Topology(_)))
        .count();

    MilestoneOneCounters {
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
