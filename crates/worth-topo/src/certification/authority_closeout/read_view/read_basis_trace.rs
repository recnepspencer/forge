use super::localization_report::{
    build_counter_report, build_primitive_family_coverage_matrix,
    build_topology_localization_report_from_query_rows,
};
use super::query_evidence::{
    certification_integrity_markers, certification_performance_accounting,
    derived_query_inspection, ensure_query_surface_ready, query_evidence_from_accounting,
    traced_certification_envelope, traced_certification_failure,
};
use super::*;
use crate::committed_artifact::TopologyCommittedArtifact;

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
        let surfaces =
            crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
                &mut workspace,
            )
            .map_err(|error| {
                traced_certification_failure(
                    MilestoneOneCertificationError::Query(error.to_string()),
                    &read_basis,
                    None,
                    replay_history_length,
                )
            })?;
        let entity_rows = workspace.read(surfaces.entities());
        let relation_rows = workspace.read(surfaces.relations());
        let persistent_name_rows = workspace.read(surfaces.persistent_names());
        let validation_state = workspace.state(surfaces.validation()).map_err(|error| {
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
                .state(surfaces.equivalence_contract())
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
            surfaces.validation(),
            ".topology.validation",
        )
        .map_err(|error| {
            traced_certification_failure(error, &read_basis, None, replay_history_length)
        })?;
        let equivalence_inspection = derived_query_inspection(
            &mut workspace,
            surfaces.equivalence_contract(),
            ".topology.equivalence_contract",
        )
        .map_err(|error| {
            traced_certification_failure(error, &read_basis, None, replay_history_length)
        })?;
        let snapshot = surfaces
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
        verified: &TopologyCommittedArtifact,
    ) -> Result<MilestoneOneCertificationReport, MilestoneOneCertificationError> {
        Self::certify_verified_commit_traced(runtime, verified)
            .map(BoundaryEnvelope::into_primary_result)
            .map_err(BoundaryFailure::into_error)
    }

    pub fn certify_verified_commit_traced(
        runtime: &mut RelationalRuntime,
        verified: &TopologyCommittedArtifact,
    ) -> Result<
        TracedMilestoneOneCertificationReport,
        BoundaryFailure<MilestoneOneCertificationError>,
    > {
        let traced = Self::certify_read_basis_with_runtime_traced(
            runtime,
            verified.read_basis().clone(),
            Some(&verified.canonical_batch().batch),
            verified.commits().len(),
        )?;
        let mut report = traced.primary_result().clone();
        let Some(replay_commit_id) = verified
            .commits()
            .last()
            .map(|commit| commit.outcome.commit.commit_id.clone())
        else {
            let integrity_markers =
                certification_integrity_markers(&verified.read_basis(), Some(verified.commits()));
            let performance_accounting = certification_performance_accounting(
                &report,
                Some(verified.commits()),
                verified.commits().len(),
                query_evidence_from_accounting(traced.performance_accounting()),
            );
            return Ok(traced
                .map_primary_result(|_| report)
                .map_decision_trace(|mut decision_trace| {
                    decision_trace.authority_anchor =
                        Some(AuthorityTraceAnchor::from_commit_results(
                            verified.branch_id().clone(),
                            verified.commits(),
                        ));
                    decision_trace.authority = Some(AuthorityTraceEvidence::from_commit_results(
                        verified.branch_id().clone(),
                        verified.commits(),
                    ));
                    decision_trace
                })
                .with_integrity_markers(integrity_markers)
                .with_performance_accounting(performance_accounting));
        };
        let replay = runtime
            .replay_authority()
            .replay_commit(RelationalReplayRequest {
                branch_id: verified.branch_id().clone(),
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
            certification_integrity_markers(&verified.read_basis(), Some(verified.commits()));
        let performance_accounting = certification_performance_accounting(
            &report,
            Some(verified.commits()),
            verified.commits().len(),
            query_evidence_from_accounting(traced.performance_accounting()),
        );
        Ok(traced
            .map_primary_result(|_| report)
            .map_decision_trace(|mut decision_trace| {
                decision_trace.authority_anchor = Some(AuthorityTraceAnchor::from_commit_results(
                    verified.branch_id().clone(),
                    verified.commits(),
                ));
                decision_trace.authority = Some(AuthorityTraceEvidence::from_commit_results(
                    verified.branch_id().clone(),
                    verified.commits(),
                ));
                decision_trace
            })
            .with_integrity_markers(integrity_markers)
            .with_performance_accounting(performance_accounting))
    }
}
