use super::*;

pub(super) fn traced_milestone_two_envelope(
    report: MilestoneTwoDerivedReadReport,
    query_evidence: MilestoneTwoQueryEvidence,
    read_basis: &DerivedTopologyReadBasis,
    commit_results: Option<&[CommitResult]>,
    replay_history_length: usize,
) -> TracedMilestoneTwoDerivedReadReport {
    BoundaryEnvelope::success(
        report.clone(),
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
            derived: Some(milestone_two_derived_trace(&report)),
            signal: None,
        },
        certification_integrity_markers(read_basis, commit_results),
        milestone_two_performance_accounting(&report, query_evidence, replay_history_length),
    )
}

fn milestone_two_derived_trace(report: &MilestoneTwoDerivedReadReport) -> DerivedTraceEvidence {
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

pub(super) fn traced_milestone_two_failure(
    error: MilestoneOneCertificationError,
    read_basis: &DerivedTopologyReadBasis,
    commit_results: Option<&[CommitResult]>,
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

fn milestone_two_performance_accounting(
    report: &MilestoneTwoDerivedReadReport,
    query_evidence: MilestoneTwoQueryEvidence,
    replay_history_length: usize,
) -> PerformanceAccounting {
    PerformanceAccounting::new([
        NamedCounter::new(
            "certification.replay_history_length",
            replay_history_length as u64,
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
            "certification.query.validation_materialized_row_count",
            query_evidence.validation_materialized_row_count as u64,
        ),
        NamedCounter::new(
            "certification.query.equivalence_materialized_row_count",
            query_evidence.equivalence_materialized_row_count as u64,
        ),
        NamedCounter::new(
            "certification.query.validation_pending_refresh_fallback_count",
            query_evidence.validation_pending_refresh_fallback_count as u64,
        ),
        NamedCounter::new(
            "certification.query.equivalence_pending_refresh_fallback_count",
            query_evidence.equivalence_pending_refresh_fallback_count as u64,
        ),
        NamedCounter::new(
            "certification.query.declared_aspect_operation_count",
            query_evidence.declared_aspect_operation_count as u64,
        ),
        NamedCounter::new(
            "certification.query.mutation_metadata_key_count",
            query_evidence.mutation_metadata_key_count as u64,
        ),
    ])
}
