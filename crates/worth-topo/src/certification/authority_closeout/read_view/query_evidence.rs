use super::*;
use crate::certification::{TraceAvailability, TraceWarning};

pub(super) fn traced_certification_envelope(
    report: MilestoneOneCertificationReport,
    read_basis: &DerivedTopologyReadBasis,
    commit_results: Option<&[forge_relational::facade::transactions::CommitResult]>,
    replay_history_length: usize,
    warnings: Option<Vec<TraceWarning>>,
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

pub(super) fn traced_certification_failure(
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
        availability: TraceAvailability::Present,
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

pub(super) fn query_evidence_from_accounting(
    accounting: &PerformanceAccounting,
) -> MilestoneOneQueryEvidence {
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

pub(super) fn certification_performance_accounting(
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
