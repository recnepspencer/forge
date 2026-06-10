use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::planar_contract_bundle::{
    PlanarM7ReadinessBundle, PlanarM7ReadinessReceipt, PlanarM7ReadinessSupportPosture,
};
use worth_spatial::facade::planar_m6_closeout::{
    M6LegacyDeletionEvidenceRow, M6PlanarCloseoutCertification, M6PlanarCloseoutContracts,
    M6PlanarCloseoutQueryDomain, M6PlanarCloseoutQueryWorld, M6PremetabossEvidenceRow,
    M6PremetabossFamily, M6QueryBoundaryEvidenceRow, M6ShortcutDeletionFamily,
};

use crate::public_api_planar_contract_bundle::m7_readiness::fixture::{
    bundle_contracts, m7_readiness_parts,
};

pub(crate) fn readiness_receipt(world: &'static str) -> PlanarM7ReadinessReceipt {
    let parts = m7_readiness_parts(world);
    PlanarM7ReadinessBundle::from_certified_planar_bundle(parts.readiness)
        .with_structural_identity(parts.structural)
        .with_motion_posture(parts.motion)
        .with_retained_planar_facts(parts.retained)
        .with_projection_consumed_facts(parts.projected)
        .with_recovery_posture(parts.recovery)
        .with_diagnostics(parts.diagnostics)
        .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
            "M7 boolean split/classify/assemble is support-gated until Milestone 7",
        ))
        .compile(&bundle_contracts(world))
        .expect("M6 closeout M7 readiness plan")
        .certify()
        .expect("M6 closeout M7 readiness receipt")
}

pub(crate) fn complete_certification(world: &'static str) -> M6PlanarCloseoutCertification {
    let readiness = readiness_receipt(world);
    M6PlanarCloseoutCertification::from_m7_readiness(readiness.clone())
        .with_premetaboss_evidence(all_premetaboss_rows())
        .with_legacy_deletion_evidence(all_legacy_deletion_rows())
        .with_query_boundary_evidence(M6QueryBoundaryEvidenceRow::from_m7_readiness(&readiness))
}

pub(crate) fn closeout_contracts(
    world: &'static str,
) -> M6PlanarCloseoutContracts<M6PlanarCloseoutQueryWorld> {
    M6PlanarCloseoutContracts::new(
        ForgeQueryApplicationFacade::runtime_backed_default()
            .domain(M6PlanarCloseoutQueryDomain)
            .with_operating_context(M6PlanarCloseoutQueryWorld::new(world))
            .validate()
            .expect("validated M6 closeout domain")
            .admit()
            .expect("admitted M6 closeout domain"),
    )
}

pub(crate) fn all_premetaboss_rows() -> Vec<M6PremetabossEvidenceRow> {
    M6PremetabossFamily::ALL
        .into_iter()
        .map(|family| {
            M6PremetabossEvidenceRow::passed(family, format!("evidence:{}", family.as_str()))
        })
        .collect()
}

pub(crate) fn all_legacy_deletion_rows() -> Vec<M6LegacyDeletionEvidenceRow> {
    M6ShortcutDeletionFamily::ALL
        .into_iter()
        .map(|family| {
            M6LegacyDeletionEvidenceRow::deleted(family, format!("deleted:{}", family.as_str()))
        })
        .collect()
}
