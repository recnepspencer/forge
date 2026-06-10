use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_m6_closeout::{
    M6LegacyDeletionEvidenceRow, M6PlanarCloseoutCertification, M6PlanarCloseoutContracts,
    M6PlanarCloseoutQueryCertification, M6PlanarCloseoutQueryDomain, M6PlanarCloseoutQueryWorld,
    M6PremetabossEvidenceRow, M6PremetabossFamily, M6QueryBoundaryEvidenceRow,
    M6ShortcutDeletionFamily,
};

use super::super::bundle_closeout::boolean_readiness::m7_readiness_receipt;

#[test]
fn kernel_consumes_m6_closeout_without_planar_truth_synthesis() {
    let readiness = m7_readiness_receipt();
    let contracts = closeout_contracts();
    let receipt = M6PlanarCloseoutQueryCertification::from_certification(
        M6PlanarCloseoutCertification::from_m7_readiness(readiness.clone())
            .with_premetaboss_evidence(all_premetaboss_rows())
            .with_legacy_deletion_evidence(all_legacy_deletion_rows())
            .with_query_boundary_evidence(M6QueryBoundaryEvidenceRow::from_m7_readiness(
                &readiness,
            )),
    )
    .compile(&contracts)
    .expect("kernel M6 closeout plan")
    .certify()
    .expect("kernel M6 closeout receipt");

    assert!(receipt.proves_all_premetaboss_families());
    assert!(receipt.proves_no_kernel_local_planar_shortcuts());
    assert!(receipt.proves_query_owned_runtime_lanes());
    assert_eq!(receipt.boolean_result(), None);
    assert_eq!(receipt.imprint_action(), None);
    assert_eq!(
        receipt.counters().premetaboss_rows(),
        M6PremetabossFamily::ALL.len()
    );
    assert_eq!(
        receipt.counters().rejected_shortcut_rows(),
        M6ShortcutDeletionFamily::ALL.len()
    );
}

fn all_premetaboss_rows() -> Vec<M6PremetabossEvidenceRow> {
    M6PremetabossFamily::ALL
        .into_iter()
        .map(|family| {
            M6PremetabossEvidenceRow::passed(family, format!("kernel-evidence:{}", family.as_str()))
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
