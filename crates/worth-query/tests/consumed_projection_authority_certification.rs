use worth_query::facade::certification::{
    certify_consumed_projection_authority, certify_projection_consumption_closeout_core,
    consumed_projection_authority_support_matrix, ConsumedProjectionAuthorityCertificationLane,
    ProjectionConsumptionCertificationLane, ProjectionConsumptionCertifiedSourceSurface,
};
use worth_query::facade::foundation::{
    ConsumedProjectionAuthorityComplexityAxis, ConsumedProjectionAuthorityDenialKind,
    ConsumedProjectionAuthoritySupportStatus, ProjectionAuthorityRequirement,
};

#[test]
fn projection_consumption_closeout_binds_the_authority_product_bundle() {
    let closeout = certify_projection_consumption_closeout_core();
    let authority = closeout.consumed_authority_certification();

    assert!(authority.satisfied());
    assert_eq!(
        closeout.output_digest("consumed_projection_authority_certification_bundle"),
        Some(authority.bundle_digest())
    );
    assert!(closeout.rows().iter().any(|row| {
        row.lane() == ProjectionConsumptionCertificationLane::DownstreamAuthoritySurface
    }));
}

#[test]
fn certification_bundle_closes_every_named_authority_lane() {
    let bundle = certify_consumed_projection_authority();
    let lanes = bundle
        .rows()
        .iter()
        .map(|row| row.lane())
        .collect::<std::collections::BTreeSet<_>>();

    assert!(bundle.satisfied());
    assert_eq!(bundle.rows().len(), 5);
    assert_eq!(lanes.len(), 5);
    assert!(bundle.rows().iter().all(|row| {
        row.satisfied() && !row.evidence_detail().is_empty() && !row.row_digest().is_empty()
    }));
    assert!(!bundle.bundle_digest().is_empty());
    assert_eq!(bundle.admitted_counters().authority_constructions(), 1);
    assert_eq!(bundle.denial().counters().authority_constructions(), 0);
    assert_eq!(
        bundle.denial().kind(),
        ConsumedProjectionAuthorityDenialKind::MissingRequirement(
            ProjectionAuthorityRequirement::TargetIdentity
        )
    );
    for lane in [
        ConsumedProjectionAuthorityCertificationLane::CanonicalAdmission,
        ConsumedProjectionAuthorityCertificationLane::DeterministicReplay,
        ConsumedProjectionAuthorityCertificationLane::TypedDenial,
        ConsumedProjectionAuthorityCertificationLane::AuthorityProductSupport,
        ConsumedProjectionAuthorityCertificationLane::ExactComplexity,
    ] {
        assert!(lanes.contains(&lane), "missing certification lane {lane:?}");
    }
}

#[test]
fn authority_complexity_is_constant_except_for_consumed_fact_width() {
    let bundle = certify_consumed_projection_authority();
    let evidence = bundle.complexity();

    assert_eq!(evidence.relationship_checks(), [10, 10, 10]);
    assert_eq!(evidence.requirement_checks(), [2, 2, 2]);
    assert_eq!(evidence.consumed_fact_visits(), [2, 4, 6]);
    assert_eq!(evidence.authority_constructions(), [1, 1, 1]);
    assert!(!evidence.evidence_digest().is_empty());
    assert!(evidence.satisfied());
    assert_eq!(evidence.rows().len(), 5);

    let requirements = evidence.row(ConsumedProjectionAuthorityComplexityAxis::RequirementWidth);
    assert_eq!(requirements.scale(), [1, 2, 3]);
    assert_eq!(
        requirements
            .counters()
            .iter()
            .map(|counter| counter.requirement_checks())
            .collect::<Vec<_>>(),
        [1, 2, 3]
    );
    for axis in [
        ConsumedProjectionAuthorityComplexityAxis::UnrelatedWorkspaceGrowth,
        ConsumedProjectionAuthorityComplexityAxis::HistoricalBasisGrowth,
        ConsumedProjectionAuthorityComplexityAxis::ConsumerGraphGrowth,
    ] {
        let row = evidence.row(axis);
        assert!(row.satisfied(), "growth axis must be independent: {axis:?}");
        assert!(row.counters().windows(2).all(|pair| pair[0] == pair[1]));
        assert!(!row.row_digest().is_empty());
    }
}

#[test]
fn authority_product_support_is_distinct_and_complete_for_every_source_surface() {
    let matrix = consumed_projection_authority_support_matrix();

    assert_eq!(
        matrix.rows().len(),
        ProjectionConsumptionCertifiedSourceSurface::all().len()
    );
    assert!(matrix.rows().iter().all(|row| {
        !row.row_digest().is_empty()
            && (!row.admitted_fact_kinds().is_empty()
                || matches!(
                    row.status(),
                    ConsumedProjectionAuthoritySupportStatus::Deferred
                        | ConsumedProjectionAuthoritySupportStatus::SourceMismatch
                ))
    }));
    assert!(matrix
        .rows()
        .iter()
        .any(|row| row.status() == ConsumedProjectionAuthoritySupportStatus::Admitted));
    assert!(matrix
        .row(ProjectionConsumptionCertifiedSourceSurface::QueryReadCurrent)
        .is_some());
    assert!(!matrix.matrix_digest().is_empty());
}
