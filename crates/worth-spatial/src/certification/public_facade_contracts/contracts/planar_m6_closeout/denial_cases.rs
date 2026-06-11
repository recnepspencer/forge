use worth_spatial::facade::planar_m6_closeout::{
    M6PlanarCloseoutCertification, M6PlanarCloseoutDenialKind, M6PlanarCloseoutQueryCertification,
    M6PremetabossFamily, M6QueryBoundaryEvidenceRow,
};

use super::fixture::{
    all_legacy_deletion_rows, all_premetaboss_rows, closeout_contracts, legacy_fixture_fence,
    readiness_receipt,
};

#[test]
fn m6_closeout_rejects_missing_or_duplicate_premetaboss_family() {
    let readiness = readiness_receipt("m6-closeout-missing-mb");
    let mut missing_rows = all_premetaboss_rows();
    missing_rows.retain(|row| row.family() != M6PremetabossFamily::BooleanReadinessFinalBoss);
    let missing = match M6PlanarCloseoutQueryCertification::from_certification(
        M6PlanarCloseoutCertification::from_m7_readiness(readiness.clone())
            .with_premetaboss_evidence(missing_rows)
            .with_legacy_deletion_evidence(all_legacy_deletion_rows())
            .with_legacy_fixture_fence(legacy_fixture_fence())
            .with_query_boundary_evidence(M6QueryBoundaryEvidenceRow::from_m7_readiness(
                &readiness,
            )),
    )
    .compile(&closeout_contracts("m6-closeout-missing-mb"))
    {
        Ok(_) => panic!("missing pre-MetaBoss row must deny"),
        Err(denial) => denial,
    };
    assert_eq!(
        missing.kind(),
        M6PlanarCloseoutDenialKind::MissingPremetabossFamily
    );

    let mut duplicate_rows = all_premetaboss_rows();
    duplicate_rows.push(duplicate_rows[0].clone());
    let duplicate = match M6PlanarCloseoutQueryCertification::from_certification(
        M6PlanarCloseoutCertification::from_m7_readiness(readiness.clone())
            .with_premetaboss_evidence(duplicate_rows)
            .with_legacy_deletion_evidence(all_legacy_deletion_rows())
            .with_legacy_fixture_fence(legacy_fixture_fence())
            .with_query_boundary_evidence(M6QueryBoundaryEvidenceRow::from_m7_readiness(
                &readiness,
            )),
    )
    .compile(&closeout_contracts("m6-closeout-duplicate-mb"))
    {
        Ok(_) => panic!("duplicate pre-MetaBoss row must deny"),
        Err(denial) => denial,
    };
    assert_eq!(
        duplicate.kind(),
        M6PlanarCloseoutDenialKind::DuplicatePremetabossFamily
    );
}

#[test]
fn m6_closeout_rejects_mismatched_query_boundary() {
    let readiness = readiness_receipt("m6-closeout-query-mismatch");
    let stale = readiness_receipt("m6-closeout-query-mismatch-stale");
    let denial = match M6PlanarCloseoutQueryCertification::from_certification(
        M6PlanarCloseoutCertification::from_m7_readiness(readiness)
            .with_premetaboss_evidence(all_premetaboss_rows())
            .with_legacy_deletion_evidence(all_legacy_deletion_rows())
            .with_legacy_fixture_fence(legacy_fixture_fence())
            .with_query_boundary_evidence(M6QueryBoundaryEvidenceRow::from_m7_readiness(&stale)),
    )
    .compile(&closeout_contracts("m6-closeout-query-mismatch"))
    {
        Ok(_) => panic!("mismatched Query boundary must deny"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        M6PlanarCloseoutDenialKind::QueryBoundaryMismatch
    );
}

#[test]
fn m6_closeout_requires_legacy_fixture_fence() {
    let readiness = readiness_receipt("m6-closeout-missing-legacy-fence");
    let denial = match M6PlanarCloseoutQueryCertification::from_certification(
        M6PlanarCloseoutCertification::from_m7_readiness(readiness.clone())
            .with_premetaboss_evidence(all_premetaboss_rows())
            .with_legacy_deletion_evidence(all_legacy_deletion_rows())
            .with_query_boundary_evidence(M6QueryBoundaryEvidenceRow::from_m7_readiness(
                &readiness,
            )),
    )
    .compile(&closeout_contracts("m6-closeout-missing-legacy-fence"))
    {
        Ok(_) => panic!("M6 closeout must require legacy fixture fence evidence"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        M6PlanarCloseoutDenialKind::MissingLegacyFixtureFence
    );
    assert!(denial.reason().contains("legacy fixture fence"));
}
