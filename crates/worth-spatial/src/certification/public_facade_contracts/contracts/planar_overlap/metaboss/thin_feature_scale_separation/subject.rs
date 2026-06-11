use forge_query::facade::ForgeQueryApplicationFacade;
use worth_kernel::workload_composition::WorkloadCatalog;
use worth_spatial::facade::planar_precision::{
    planar_precision_certification_entry, planar_precision_certification_facts,
    PlanarPrecisionBasis, PlanarPrecisionCertificateReceipt, PlanarPrecisionCertificationCase,
    PlanarPrecisionCertificationQueryDomain, PlanarPrecisionCertificationQueryWorld,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
};
use worth_spatial::facade::thin_feature_scale_separation::{
    ThinFeaturePredicateCertification, ThinFeatureScalePolicy, ThinFeatureScaleSeparationReceipt,
    ThinFeatureScaleSeparationWorkload, ThinFeatureScaleSeparationWorkloadError,
    ThinFeatureTinyRotationPressure,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};

use crate::public_api_planar_projection_consumption::contract_subject::{
    projection_consumed_planar_parts, ProjectionConsumedPlanarParts,
};
use crate::public_api_planar_projection_consumption::runtime_handles::projection_consumption_handle;

pub(crate) struct ThinFeaturePlatformSubject {
    pub(crate) receipt: ThinFeatureScaleSeparationReceipt,
    pub(crate) user_outcome: WorthUserOutcome,
    pub(crate) precision_scale_orders: Vec<i32>,
}

pub(crate) fn certify_platform_thin_feature_scale_separation(
    world: &'static str,
) -> ThinFeaturePlatformSubject {
    let catalog = WorkloadCatalog::thin_feature_wall()
        .declared(format!(
            "MB-M6-3 platform thin-feature scale separation {world}"
        ))
        .build()
        .expect("thin-feature workload catalog should build");
    let parts = projection_consumed_planar_parts(world);
    let projection_consumption = projection_consumption_receipt(world, &parts);
    let extra_precision = precision_scale_witnesses(world, &parts.bundle_parts.precision);
    let receipt = thin_feature_workload(world, &catalog, &parts, &projection_consumption)
        .with_precision_scale_witness(&extra_precision[0])
        .with_precision_scale_witness(&extra_precision[1])
        .certify()
        .expect("platform thin-feature scale separation should certify");
    let user_outcome = user_outcome_from_thin_feature_receipt(&receipt);
    let mut precision_scale_orders = vec![parts
        .bundle_parts
        .precision
        .basis()
        .local_feature_scale_order()];
    precision_scale_orders.extend(
        extra_precision
            .iter()
            .map(|receipt| receipt.basis().local_feature_scale_order()),
    );

    ThinFeaturePlatformSubject {
        receipt,
        user_outcome,
        precision_scale_orders,
    }
}

pub(crate) fn thin_feature_policy_required_outcome(world: &'static str) -> WorthUserOutcome {
    thin_feature_error_outcome(world, |workload| {
        workload.requiring_scale_policy(ThinFeatureScalePolicy::RequiresUserDecision)
    })
}

pub(crate) fn thin_feature_predicate_uncertain_outcome(world: &'static str) -> WorthUserOutcome {
    thin_feature_error_outcome(world, |workload| {
        workload.requiring_predicate_certification(ThinFeaturePredicateCertification::Uncertain)
    })
}

pub(crate) fn thin_feature_precision_basis_failure_outcome(
    world: &'static str,
) -> WorthUserOutcome {
    thin_feature_error_outcome(world, |workload| {
        workload.with_required_local_scale_orders([-15, -9, -6])
    })
}

pub(crate) fn thin_feature_world_magnitude_floor_outcome(world: &'static str) -> WorthUserOutcome {
    thin_feature_error_outcome(world, |workload| {
        workload.with_required_world_magnitude_order(0)
    })
}

pub(crate) fn thin_feature_foreign_precision_witness_outcome(
    world: &'static str,
) -> WorthUserOutcome {
    let catalog = WorkloadCatalog::thin_feature_wall()
        .declared(format!("MB-M6-3 foreign precision witness {world}"))
        .build()
        .expect("thin-feature workload catalog should build");
    let parts = projection_consumed_planar_parts(world);
    let projection_consumption = projection_consumption_receipt(world, &parts);
    let foreign_witnesses = [
        precision_scale_witness(
            world,
            "frame:foreign-thin-feature",
            "topology:foreign-thin-feature",
            "movement:foreign-thin-feature",
            "tolerance:foreign-thin-feature",
            -12,
        ),
        precision_scale_witness(
            world,
            "frame:foreign-thin-feature",
            "topology:foreign-thin-feature",
            "movement:foreign-thin-feature",
            "tolerance:foreign-thin-feature",
            -6,
        ),
    ];
    let error = thin_feature_workload(world, &catalog, &parts, &projection_consumption)
        .with_precision_scale_witness(&foreign_witnesses[0])
        .with_precision_scale_witness(&foreign_witnesses[1])
        .certify()
        .expect_err("foreign precision witnesses must deny");
    user_outcome_from_thin_feature_error(error)
}

pub(crate) fn thin_feature_unsupported_tiny_rotation_outcome(
    world: &'static str,
) -> WorthUserOutcome {
    thin_feature_error_outcome(world, |workload| {
        workload.requiring_tiny_rotation_pressure(ThinFeatureTinyRotationPressure::Unsupported)
    })
}

pub(crate) fn thin_feature_integrity_mismatch_outcome(world: &'static str) -> WorthUserOutcome {
    let catalog = WorkloadCatalog::thin_feature_wall()
        .declared(format!("MB-M6-3 mismatched projection basis {world}"))
        .build()
        .expect("thin-feature workload catalog should build");
    let parts = projection_consumed_planar_parts(world);
    let drifted_parts = projection_consumed_planar_parts("matrix-integrity-drifted-basis");
    let drifted_projection_consumption = projection_consumption_receipt(world, &drifted_parts);
    let extra_precision = precision_scale_witnesses(world, &parts.bundle_parts.precision);
    let error = thin_feature_workload(world, &catalog, &parts, &drifted_projection_consumption)
        .with_precision_scale_witness(&extra_precision[0])
        .with_precision_scale_witness(&extra_precision[1])
        .certify()
        .expect_err("mismatched projection-consumed basis must deny");
    user_outcome_from_thin_feature_error(error)
}

pub(crate) fn thin_feature_missing_local_frame_outcome(world: &'static str) -> WorthUserOutcome {
    let catalog = WorkloadCatalog::thin_feature_wall()
        .declared(format!("MB-M6-3 missing local frame {world}"))
        .build()
        .expect("thin-feature workload catalog should build");
    let parts = projection_consumed_planar_parts(world);
    let projection_consumption = projection_consumption_receipt(world, &parts);
    let error = ThinFeatureScaleSeparationWorkload::from_platform_evidence(
        catalog.workload().evidence_ledger(),
    )
    .with_precision_receipt(&parts.bundle_parts.precision)
    .with_projection_consumption_receipt(&projection_consumption)
    .certify()
    .expect_err("missing local frame must deny");
    user_outcome_from_thin_feature_error(error)
}

fn thin_feature_error_outcome<F>(world: &'static str, configure: F) -> WorthUserOutcome
where
    F: for<'a> FnOnce(
        ThinFeatureScaleSeparationWorkload<'a>,
    ) -> ThinFeatureScaleSeparationWorkload<'a>,
{
    let catalog = WorkloadCatalog::thin_feature_wall()
        .declared(format!("MB-M6-3 platform thin-feature branch {world}"))
        .build()
        .expect("thin-feature workload catalog should build");
    let parts = projection_consumed_planar_parts(world);
    let projection_consumption = projection_consumption_receipt(world, &parts);
    let extra_precision = precision_scale_witnesses(world, &parts.bundle_parts.precision);
    let error = configure(thin_feature_workload(
        world,
        &catalog,
        &parts,
        &projection_consumption,
    ))
    .with_precision_scale_witness(&extra_precision[0])
    .with_precision_scale_witness(&extra_precision[1])
    .certify()
    .expect_err("configured thin-feature branch must deny");
    user_outcome_from_thin_feature_error(error)
}

fn thin_feature_workload<'a>(
    _world: &'static str,
    catalog: &'a worth_kernel::workload_composition::BuiltWorkloadCatalogRecipe,
    parts: &'a ProjectionConsumedPlanarParts,
    projection_consumption: &'a worth_spatial::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt,
) -> ThinFeatureScaleSeparationWorkload<'a> {
    ThinFeatureScaleSeparationWorkload::from_platform_evidence(catalog.workload().evidence_ledger())
        .with_precision_receipt(&parts.bundle_parts.precision)
        .with_local_frame_receipt(&parts.bundle_parts.frame)
        .with_projection_consumption_receipt(projection_consumption)
}

fn projection_consumption_receipt(
    world: &'static str,
    parts: &ProjectionConsumedPlanarParts,
) -> worth_spatial::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt {
    ProjectionConsumedPlanarFacts::from_retained_planar_facts(parts.retained.clone())
        .consume_bundle_projection_receipts(parts.projections.clone())
        .materialize_as(parts.bundle_parts.frame.fact_digest())
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(world),
        ))
        .expect("thin-feature projection-consumption plan")
        .consume()
        .expect("thin-feature projection-consumed receipt")
}

fn precision_scale_witnesses(
    world: &'static str,
    primary_precision: &PlanarPrecisionCertificateReceipt,
) -> [PlanarPrecisionCertificateReceipt; 2] {
    let primary_basis = primary_precision.basis();
    [
        precision_scale_witness(
            world,
            primary_basis.local_frame_identity(),
            primary_basis.topology_basis_identity(),
            primary_basis.movement_rotation_posture_identity(),
            primary_basis.tolerance_policy_identity(),
            -12,
        ),
        precision_scale_witness(
            world,
            primary_basis.local_frame_identity(),
            primary_basis.topology_basis_identity(),
            primary_basis.movement_rotation_posture_identity(),
            primary_basis.tolerance_policy_identity(),
            -6,
        ),
    ]
}

fn precision_scale_witness(
    world: &'static str,
    local_frame_identity: &str,
    topology_basis_identity: &str,
    movement_rotation_posture_identity: &str,
    tolerance_policy_identity: &str,
    local_scale_order: i32,
) -> PlanarPrecisionCertificateReceipt {
    let predicate = predicate_receipt(
        world,
        local_frame_identity,
        topology_basis_identity,
        movement_rotation_posture_identity,
        tolerance_policy_identity,
        local_scale_order,
    );
    let basis = PlanarPrecisionBasis::builder()
        .local_frame_identity(local_frame_identity)
        .topology_basis_identity(topology_basis_identity)
        .movement_rotation_posture_identity(movement_rotation_posture_identity)
        .tolerance_policy_identity(tolerance_policy_identity)
        .local_feature_scale_order(local_scale_order)
        .world_magnitude_order(12)
        .normalization_scale(10.0_f64.powi(local_scale_order))
        .predicate_receipt(&predicate)
        .build()
        .expect("thin-feature precision scale witness basis");
    planar_precision_certification_facts(
        &planar_precision_certification_entry(
            PlanarPrecisionCertificationCase::from_predicate_receipt(predicate, basis),
        ),
        &precision_handle(world),
    )
    .expect("thin-feature precision scale witness receipt")
}

fn predicate_receipt(
    world: &'static str,
    local_frame_identity: &str,
    topology_basis_identity: &str,
    movement_rotation_posture_identity: &str,
    tolerance_policy_identity: &str,
    local_scale_order: i32,
) -> PlanarPredicateFactReceipt {
    let scale = 10.0_f64.powi(local_scale_order);
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        local_frame_identity,
        topology_basis_identity,
        movement_rotation_posture_identity,
        tolerance_policy_identity,
        [[0.0, 0.0], [scale, 0.0], [0.0, scale]],
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        &predicate_handle(world),
    )
    .expect("thin-feature precision witness predicate receipt")
}

fn precision_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPrecisionCertificationQueryDomain,
    PlanarPrecisionCertificationQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPrecisionCertificationQueryDomain)
        .with_operating_context(PlanarPrecisionCertificationQueryWorld::new(world))
        .validate()
        .expect("validated thin-feature precision handle")
        .admit()
        .expect("admitted thin-feature precision handle")
}

fn predicate_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(world))
        .validate()
        .expect("validated thin-feature predicate handle")
        .admit()
        .expect("admitted thin-feature predicate handle")
}

fn user_outcome_from_thin_feature_receipt(
    receipt: &ThinFeatureScaleSeparationReceipt,
) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_thin_feature_scale_separation(receipt),
    )
    .declared("explain thin-feature scale separation outcome")
    .respond()
    .expect("thin-feature user response should certify")
    .outcome()
    .clone()
}

fn user_outcome_from_thin_feature_error(
    error: ThinFeatureScaleSeparationWorkloadError,
) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_thin_feature_scale_separation_error(error),
    )
    .declared("explain thin-feature scale separation denial")
    .respond()
    .expect("thin-feature denial response should certify")
    .outcome()
    .clone()
}
