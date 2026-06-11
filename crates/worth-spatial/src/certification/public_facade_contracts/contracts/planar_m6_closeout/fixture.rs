use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::dirty_planar_clean_fail::DirtyPlanarCleanFailCase;
use worth_spatial::facade::planar_contract_bundle::{
    PlanarM7ReadinessBundle, PlanarM7ReadinessReceipt, PlanarM7ReadinessSupportPosture,
};
use worth_spatial::facade::planar_m6_closeout::{
    M6LegacyDeletionEvidenceRow, M6LegacyFixtureFence, M6LegacyFixtureFenceRow,
    M6PlanarCloseoutCertification, M6PlanarCloseoutContracts, M6PlanarCloseoutQueryDomain,
    M6PlanarCloseoutQueryWorld, M6PremetabossEvidenceRow, M6PremetabossPlatformTarget,
    M6QueryBoundaryEvidenceRow, M6ShortcutDeletionFamily,
};
use worth_spatial::facade::workload_inventory::SeedInventoryReport;

use crate::public_api_planar_contract_bundle::m7_readiness::fixture::{
    bundle_contracts, m7_readiness_parts,
};
use crate::public_api_planar_overlap::metaboss::boolean_readiness_workload::subject::certify_final_boss;
use crate::public_api_planar_overlap::metaboss::dirty_planar_clean_fail::subject::dirty_clean_fail_with_topology_seed;
use crate::public_api_planar_overlap::metaboss::high_valence_subject::certify_platform_high_valence_singularity;
use crate::public_api_planar_overlap::metaboss::open_planar_posture::subject::half_space_subject;
use crate::public_api_planar_overlap::metaboss::platform_storm_subject::certify_platform_storm;
use crate::public_api_planar_overlap::metaboss::projection_fact_parity::subject::certify_projection_fact_parity;
use crate::public_api_planar_overlap::metaboss::retained_cancellation_chain::subject::certify_retained_cancellation_chain;
use crate::public_api_planar_overlap::metaboss::thin_feature_scale_separation::subject::certify_platform_thin_feature_scale_separation;

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
        .with_legacy_fixture_fence(legacy_fixture_fence())
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
    run_with_real_closeout_target_stack(|| {
        vec![
            target_row(M6PremetabossPlatformTarget::from_coplanar_overlap_storm(
                &certify_platform_storm("m6-closeout-mb1-target").storm_receipt,
            )),
            target_row(M6PremetabossPlatformTarget::from_high_valence_singularity(
                &certify_platform_high_valence_singularity("m6-closeout-mb2-target").receipt,
            )),
            target_row(
                M6PremetabossPlatformTarget::from_thin_feature_scale_separation(
                    &certify_platform_thin_feature_scale_separation("m6-closeout-mb3-target")
                        .receipt,
                ),
            ),
            target_row(
                M6PremetabossPlatformTarget::from_retained_cancellation_chain(
                    &certify_retained_cancellation_chain("m6-closeout-mb4-target").receipt,
                ),
            ),
            target_row(M6PremetabossPlatformTarget::from_dirty_planar_clean_fail(
                &dirty_clean_fail_with_topology_seed(
                    "m6-closeout-mb5-target",
                    DirtyPlanarCleanFailCase::SelfIntersectingLoop,
                )
                .receipt,
            )),
            target_row(M6PremetabossPlatformTarget::from_open_planar_posture(
                &half_space_subject("m6-closeout-mb6-target").receipt,
            )),
            target_row(M6PremetabossPlatformTarget::from_projection_fact_parity(
                &certify_projection_fact_parity("m6-closeout-mb7-target").receipt,
            )),
            target_row(
                M6PremetabossPlatformTarget::from_boolean_readiness_final_boss(
                    &certify_final_boss("m6-closeout-mb8-target").receipt,
                ),
            ),
        ]
    })
}

fn target_row(target: M6PremetabossPlatformTarget) -> M6PremetabossEvidenceRow {
    M6PremetabossEvidenceRow::from_workload_platform_target(target)
}

fn run_with_real_closeout_target_stack<R>(build: impl FnOnce() -> R + Send + 'static) -> R
where
    R: Send + 'static,
{
    std::thread::Builder::new()
        .name("m6-closeout-real-platform-targets".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(build)
        .expect("M6 closeout target thread")
        .join()
        .expect("M6 closeout targets built")
}

pub(crate) fn all_legacy_deletion_rows() -> Vec<M6LegacyDeletionEvidenceRow> {
    M6ShortcutDeletionFamily::ALL
        .into_iter()
        .map(|family| {
            M6LegacyDeletionEvidenceRow::deleted(family, format!("deleted:{}", family.as_str()))
        })
        .collect()
}

pub(crate) fn legacy_fixture_fence() -> M6LegacyFixtureFence {
    let report = SeedInventoryReport::certify_existing_surfaces()
        .expect("existing seed inventory should certify");
    M6LegacyFixtureFence::from_rows(
        report
            .rows()
            .iter()
            .map(|row| M6LegacyFixtureFenceRow::classify(row.classification(), row.decision())),
    )
}
