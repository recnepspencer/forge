use worth_kernel::workload_composition::{
    BuiltWorkloadCatalogRecipe, GrazingBasketStackSpec, WorkloadCatalog,
};
use worth_spatial::facade::grazing_basket_stack::{
    BasketLayerIndex, GrazingBasketStackDenial, GrazingBasketStackOutcomeMatrix,
    GrazingBasketStackReceipt, GrazingBasketStackWorkload,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};

pub(crate) struct GrazingBasketStackSubject {
    pub catalog: BuiltWorkloadCatalogRecipe,
    pub receipt: GrazingBasketStackReceipt,
    pub outcome_matrix: GrazingBasketStackOutcomeMatrix,
    pub user_outcome: WorthUserOutcome,
}

pub(crate) fn grazing_basket_stack_subject(stem: &str) -> GrazingBasketStackSubject {
    let catalog = WorkloadCatalog::grazing_open_shell_basket_stack(
        GrazingBasketStackSpec::new().layers(6).strips_per_layer(12),
    )
    .declared(format!("{stem} grazing basket stack"))
    .build()
    .expect("grazing basket stack catalog must build");
    let receipt = GrazingBasketStackWorkload::from_platform_evidence(
        catalog
            .topology_construction()
            .expect("basket stack must expose topology construction"),
        catalog.workload().evidence_ledger(),
        catalog.projected_workload(),
        catalog.transform_receipts(),
        catalog
            .replay_receipts()
            .expect("basket stack must expose retained replay receipts"),
    )
    .certify()
    .expect("grazing basket stack must certify from real platform receipts");
    let outcome_matrix =
        GrazingBasketStackOutcomeMatrix::from_receipt(&receipt).expect("complete matrix");
    let user_outcome = respond(WorthUserResponseSource::from_grazing_basket_stack(&receipt));
    GrazingBasketStackSubject {
        catalog,
        receipt,
        outcome_matrix,
        user_outcome,
    }
}

pub(crate) fn cross_layer_retained_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_cross_layer_retained_replay(BasketLayerIndex::new(0), BasketLayerIndex::new(3))
        .expect_err("cross-layer retained replay must deny")
}

pub(crate) fn cross_layer_projection_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_cross_layer_projection_identity(BasketLayerIndex::new(1), BasketLayerIndex::new(4))
        .expect_err("cross-layer projection identity must deny")
}

pub(crate) fn surface_smuggling_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_surface_support_smuggling(BasketLayerIndex::new(2), "analytic non-plane")
        .expect_err("non-plane support must stay localized")
}

pub(crate) fn cross_layer_surface_smuggling_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_cross_layer_surface_support_smuggling(
            BasketLayerIndex::new(2),
            BasketLayerIndex::new(5),
        )
        .expect_err("cross-layer surface support must deny")
}

pub(crate) fn cross_layer_parity_lane_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_cross_layer_parity_lane_smuggling(
            BasketLayerIndex::new(1),
            BasketLayerIndex::new(4),
        )
        .expect_err("cross-layer parity lane must deny")
}

pub(crate) fn storm_smuggling_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    let storm = WorkloadCatalog::coplanar_overlap_storm()
        .declared("mb-m6-nmt-4 foreign storm")
        .build()
        .expect("storm catalog must build");
    let storm_digest = storm
        .projected_workload()
        .receipts()
        .stage_identity()
        .receipt_identity();
    subject
        .receipt
        .attempt_storm_extraction_smuggling(BasketLayerIndex::new(0), &storm_digest)
        .expect_err("storm extraction must not certify basket stack")
}

pub(crate) fn false_closure_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_false_closure(BasketLayerIndex::new(5))
        .expect_err("false closure must deny")
}

pub(crate) fn near_graze_uncertain_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    let layer = BasketLayerIndex::new(3);
    let boundary = subject
        .receipt
        .layer(layer)
        .expect("layer exists")
        .open_boundary()
        .clone();
    subject
        .receipt
        .attempt_near_graze_predicate_pressure(layer, boundary)
        .expect_err("near-graze pressure must localize as predicate uncertainty")
}

pub(crate) fn whole_stack_broadening_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_whole_stack_broadening(BasketLayerIndex::new(1))
        .expect_err("single hostile layer must not broaden to whole stack")
}

pub(crate) fn missing_boundary_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_missing_boundary_evidence(BasketLayerIndex::new(4))
        .expect_err("missing boundary evidence must produce no-options")
}

pub(crate) fn missing_projection_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_missing_projection_evidence(BasketLayerIndex::new(2))
        .expect_err("missing projection evidence must produce no-options")
}

pub(crate) fn missing_retained_checkpoint_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_missing_retained_checkpoint_evidence(BasketLayerIndex::new(3))
        .expect_err("missing retained checkpoint evidence must produce no-options")
}

pub(crate) fn respond(source: WorthUserResponseSource) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(source)
        .declared("mb-m6-nmt-4 grazing basket response")
        .respond()
        .expect("grazing basket response")
        .outcome()
        .clone()
}
