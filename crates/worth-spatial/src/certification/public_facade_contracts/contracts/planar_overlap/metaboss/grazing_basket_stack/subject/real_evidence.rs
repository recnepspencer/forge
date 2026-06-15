use forge_query::facade::ForgeQueryApplicationFacade;
use worth_kernel::workload_composition::{BuiltWorkloadCatalogRecipe, WorkloadCatalog};
use worth_spatial::facade::grazing_basket_stack::{
    BasketLayerIndex, GrazingBasketDeniedMotionEvidence, GrazingBasketLayerAuthorityEvidence,
    GrazingBasketPredicateUncertaintyEvidence, GrazingBasketStackDenial,
    GrazingBasketStackOutcomeMatrix, GrazingBasketStackOutcomeRow, GrazingBasketStackReceipt,
    GrazingBasketStormExtractionEvidence, GrazingBasketUnsupportedSurfaceEvidence,
};
use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld, PlanarPredicateCoincidencePolicy,
    PlanarPredicateInputBasis,
};
use worth_spatial::facade::surface_support::{SurfaceFamily, SurfaceSupportWorkload};
use worth_spatial::facade::transform_workload::{
    TransformSequence, TransformWorkload, UnsupportedTransformReasonCode,
};

use super::GrazingBasketStackSubject;

pub(crate) fn real_evidence_outcome_matrix(
    catalog: &BuiltWorkloadCatalogRecipe,
    receipt: &GrazingBasketStackReceipt,
) -> GrazingBasketStackOutcomeMatrix {
    let first = BasketLayerIndex::new(0);
    let second = BasketLayerIndex::new(1);
    let third = BasketLayerIndex::new(2);
    let fourth = BasketLayerIndex::new(3);
    let rows = vec![
        GrazingBasketStackOutcomeRow::admitted(receipt),
        GrazingBasketStackOutcomeRow::equivalent_transform_admitted(receipt, second)
            .expect("equivalent transform row"),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &label_only_motion_denial(catalog, receipt, first),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_open_boundary_perturbation(first)
                .expect_err("open-boundary perturbation denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_cross_layer_retained_replay_evidence(
                    &layer_retained_evidence(receipt, first),
                    second,
                )
                .expect_err("cross-layer retained denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_cross_layer_projection_identity_evidence(
                    &layer_projection_evidence(receipt, second),
                    fourth,
                )
                .expect_err("cross-layer projection denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_cross_layer_surface_support_smuggling_evidence(
                    &layer_surface_evidence(receipt, third),
                    fourth,
                )
                .expect_err("cross-layer surface support denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_cross_layer_parity_lane_smuggling_evidence(
                    &layer_parity_evidence(receipt, first),
                    third,
                )
                .expect_err("cross-layer parity lane denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_unsupported_surface_support(
                    first,
                    &unsupported_surface_evidence_from_catalog(catalog),
                )
                .expect_err("surface support denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_storm_extraction_smuggling_evidence(first, &storm_extraction_evidence())
                .expect_err("storm smuggling denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_false_closure_evidence(&layer_boundary_evidence(receipt, fourth), fourth)
                .expect_err("false closure denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_whole_stack_broadening_evidence(
                    &layer_projection_evidence(receipt, second),
                    second,
                )
                .expect_err("whole-stack broadening denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_missing_boundary_evidence(first)
                .expect_err("missing boundary denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_missing_projection_evidence(second)
                .expect_err("missing projection denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &receipt
                .attempt_missing_retained_checkpoint_evidence(third)
                .expect_err("missing retained checkpoint denial"),
        ),
        GrazingBasketStackOutcomeRow::from_denial(
            receipt,
            &near_graze_uncertain_denial_from_receipt(receipt),
        ),
    ];
    GrazingBasketStackOutcomeMatrix::from_rows(rows).expect("complete matrix")
}

pub(crate) fn layer_retained_evidence(
    receipt: &GrazingBasketStackReceipt,
    layer: BasketLayerIndex,
) -> GrazingBasketLayerAuthorityEvidence {
    GrazingBasketLayerAuthorityEvidence::retained_replay_from_layer(
        receipt.layer(layer).expect("layer exists"),
    )
}

pub(crate) fn layer_projection_evidence(
    receipt: &GrazingBasketStackReceipt,
    layer: BasketLayerIndex,
) -> GrazingBasketLayerAuthorityEvidence {
    GrazingBasketLayerAuthorityEvidence::projection_from_layer(
        receipt.layer(layer).expect("layer exists"),
    )
}

pub(crate) fn layer_surface_evidence(
    receipt: &GrazingBasketStackReceipt,
    layer: BasketLayerIndex,
) -> GrazingBasketLayerAuthorityEvidence {
    GrazingBasketLayerAuthorityEvidence::surface_support_from_layer(
        receipt.layer(layer).expect("layer exists"),
    )
}

pub(crate) fn layer_parity_evidence(
    receipt: &GrazingBasketStackReceipt,
    layer: BasketLayerIndex,
) -> GrazingBasketLayerAuthorityEvidence {
    GrazingBasketLayerAuthorityEvidence::parity_lane_from_layer(
        receipt.layer(layer).expect("layer exists"),
    )
}

pub(crate) fn layer_boundary_evidence(
    receipt: &GrazingBasketStackReceipt,
    layer: BasketLayerIndex,
) -> GrazingBasketLayerAuthorityEvidence {
    GrazingBasketLayerAuthorityEvidence::open_boundary_from_layer(
        receipt.layer(layer).expect("layer exists"),
    )
}

pub(crate) fn unsupported_surface_evidence(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketUnsupportedSurfaceEvidence {
    unsupported_surface_evidence_from_catalog(&subject.catalog)
}

pub(crate) fn near_graze_predicate_evidence(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketPredicateUncertaintyEvidence {
    near_graze_predicate_evidence_from_receipt(&subject.receipt)
}

fn label_only_motion_denial(
    catalog: &BuiltWorkloadCatalogRecipe,
    receipt: &GrazingBasketStackReceipt,
    layer: BasketLayerIndex,
) -> GrazingBasketStackDenial {
    let denied = TransformWorkload::for_projected_workload(catalog.projected_workload().clone())
        .declared("mb-m6-nmt-4 label-only motion adversary")
        .with_transform_sequence(TransformSequence::identity_label_only(
            "basket stack label-only motion adversary",
        ))
        .transform()
        .expect_err("label-only transform must deny");
    assert_eq!(
        denied.reason_code(),
        UnsupportedTransformReasonCode::LabelOnlyMotionEvidence
    );
    receipt
        .attempt_label_only_motion_evidence(
            layer,
            &GrazingBasketDeniedMotionEvidence::from_unsupported_transform(&denied)
                .expect("typed label-only evidence"),
        )
        .expect_err("label-only basket motion must deny")
}

fn unsupported_surface_evidence_from_catalog(
    catalog: &BuiltWorkloadCatalogRecipe,
) -> GrazingBasketUnsupportedSurfaceEvidence {
    let unsupported = SurfaceSupportWorkload::for_bound_geometry(catalog.bound_geometry().clone())
        .declared("mb-m6-nmt-4 unsupported layer surface adversary")
        .with_surface_family(SurfaceFamily::AnalyticNonPlanar)
        .certify()
        .expect_err("non-planar surface support must deny");
    GrazingBasketUnsupportedSurfaceEvidence::from_unsupported_surface_support(&unsupported)
        .expect("typed unsupported surface evidence")
}

fn storm_extraction_evidence() -> GrazingBasketStormExtractionEvidence {
    let storm = WorkloadCatalog::coplanar_overlap_storm()
        .declared("mb-m6-nmt-4 foreign storm matrix")
        .build()
        .expect("storm catalog must build");
    GrazingBasketStormExtractionEvidence::from_projected_workload(storm.projected_workload())
}

fn near_graze_predicate_evidence_from_receipt(
    receipt: &GrazingBasketStackReceipt,
) -> GrazingBasketPredicateUncertaintyEvidence {
    let layer = receipt
        .layer(BasketLayerIndex::new(3))
        .expect("layer exists");
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        layer.local_frame_identity(),
        layer.open_boundary().boundary_identity(),
        layer.transform_posture_identity(),
        "basket-stack-local-feature-scale",
        [[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]],
    )
    .with_coincidence_policy(PlanarPredicateCoincidencePolicy::DenyCertifiedZeroBeforeRepair);
    let entry = planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis));
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarPredicateAuthorityQueryDomain)
        .with_operating_context(PlanarPredicateAuthorityQueryWorld::new(
            "mb-m6-nmt-4-near-graze",
        ))
        .validate()
        .expect("validated predicate handle")
        .admit()
        .expect("admitted predicate handle");
    let error = planar_predicate_authority_facts(&entry, &handle)
        .expect_err("near-graze certified zero must be policy gated");
    GrazingBasketPredicateUncertaintyEvidence::from_predicate_error(&error)
        .expect("typed predicate uncertainty evidence")
}

fn near_graze_uncertain_denial_from_receipt(
    receipt: &GrazingBasketStackReceipt,
) -> GrazingBasketStackDenial {
    let layer = BasketLayerIndex::new(3);
    let boundary = receipt
        .layer(layer)
        .expect("layer exists")
        .open_boundary()
        .clone();
    receipt
        .attempt_near_graze_predicate_pressure_evidence(
            layer,
            boundary,
            &near_graze_predicate_evidence_from_receipt(receipt),
        )
        .expect_err("near-graze pressure must deny")
}
