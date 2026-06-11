use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_m6_closeout::{
    M6LegacyDeletionEvidenceRow, M6PlanarCloseoutCertification, M6PlanarCloseoutContracts,
    M6PlanarCloseoutDenialKind, M6PlanarCloseoutQueryCertification, M6PlanarCloseoutQueryDomain,
    M6PlanarCloseoutQueryWorld, M6PremetabossEvidenceRow, M6PremetabossFamily,
    M6QueryBoundaryEvidenceRow, M6ShortcutDeletionFamily,
};
use worth_spatial::facade::planar_m6_closeout::{M6LegacyFixtureFence, M6LegacyFixtureFenceRow};
use worth_spatial::facade::workload_inventory::SeedInventoryReport;

use super::super::bundle_closeout::boolean_readiness::m7_readiness_receipt;

#[test]
fn kernel_cannot_synthesize_m6_closeout_from_readiness_summary() {
    let readiness = m7_readiness_receipt();
    let contracts = closeout_contracts();
    let denial = match M6PlanarCloseoutQueryCertification::from_certification(
        M6PlanarCloseoutCertification::from_m7_readiness(readiness.clone())
            .with_premetaboss_evidence(synthetic_premetaboss_rows())
            .with_legacy_deletion_evidence(all_legacy_deletion_rows())
            .with_legacy_fixture_fence(legacy_fixture_fence())
            .with_query_boundary_evidence(M6QueryBoundaryEvidenceRow::from_m7_readiness(
                &readiness,
            )),
    )
    .compile(&contracts)
    {
        Ok(_) => panic!("kernel summary rows must not synthesize M6 closeout authority"),
        Err(denial) => denial,
    };

    assert_eq!(
        denial.kind(),
        M6PlanarCloseoutDenialKind::SyntheticEndToEndBlocked
    );
    assert!(
        denial
            .reason()
            .contains("cannot register synthetic MB closeout evidence"),
        "{}",
        denial.reason()
    );
}

fn synthetic_premetaboss_rows() -> Vec<M6PremetabossEvidenceRow> {
    M6PremetabossFamily::ALL
        .into_iter()
        .map(|family| {
            M6PremetabossEvidenceRow::synthetic_end_to_end_claim(
                family,
                format!("kernel-readiness-summary:{}", family.as_str()),
            )
        })
        .collect()
}

fn all_legacy_deletion_rows() -> Vec<M6LegacyDeletionEvidenceRow> {
    M6ShortcutDeletionFamily::ALL
        .into_iter()
        .map(|family| {
            M6LegacyDeletionEvidenceRow::deleted(
                family,
                format!("kernel-deleted:{}", family.as_str()),
            )
        })
        .collect()
}

fn legacy_fixture_fence() -> M6LegacyFixtureFence {
    let report = SeedInventoryReport::certify_existing_surfaces()
        .expect("existing seed inventory should certify");
    M6LegacyFixtureFence::from_rows(
        report
            .rows()
            .iter()
            .map(|row| M6LegacyFixtureFenceRow::classify(row.classification(), row.decision())),
    )
}

fn closeout_contracts() -> M6PlanarCloseoutContracts<M6PlanarCloseoutQueryWorld> {
    M6PlanarCloseoutContracts::new(
        ForgeQueryApplicationFacade::runtime_backed_default()
            .domain(M6PlanarCloseoutQueryDomain)
            .with_operating_context(M6PlanarCloseoutQueryWorld::new("kernel-m6-closeout"))
            .validate()
            .expect("validated kernel M6 closeout domain")
            .admit()
            .expect("admitted kernel M6 closeout domain"),
    )
}
