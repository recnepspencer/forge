mod real_evidence;

use real_evidence::{
    layer_boundary_evidence, layer_parity_evidence, layer_projection_evidence,
    layer_retained_evidence, layer_surface_evidence, near_graze_predicate_evidence,
    real_evidence_outcome_matrix, unsupported_surface_evidence,
};
use topology::facade::NmtTopologyScopeSet;
use worth_kernel::workload_composition::{
    BuiltWorkloadCatalogRecipe, GrazingBasketStackSpec, WorkloadCatalog,
};
use worth_spatial::facade::grazing_basket_stack::{
    BasketLayerIndex, GrazingBasketStackDenial, GrazingBasketStackOutcomeMatrix,
    GrazingBasketStackReceipt, GrazingBasketStackWorkload, GrazingBasketStormExtractionEvidence,
};
use worth_spatial::facade::nmt_certification_context::{
    NmtBossOutcomeMatrixEvidence, NmtCertifiedScopeSet,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};

pub(crate) struct GrazingBasketStackSubject {
    pub catalog: BuiltWorkloadCatalogRecipe,
    pub certified_scopes: NmtCertifiedScopeSet,
    pub receipt: GrazingBasketStackReceipt,
    pub outcome_matrix: GrazingBasketStackOutcomeMatrix,
    pub user_outcome: WorthUserOutcome,
}

pub(crate) struct GrazingBasketStackCloseoutEvidence {
    pub certified_scopes: NmtCertifiedScopeSet,
    pub matrix: NmtBossOutcomeMatrixEvidence,
}

pub(crate) fn grazing_basket_stack_subject(stem: &str) -> GrazingBasketStackSubject {
    let catalog = WorkloadCatalog::grazing_open_shell_basket_stack(
        GrazingBasketStackSpec::new().layers(6).strips_per_layer(12),
    )
    .declared(format!("{stem} grazing basket stack"))
    .build()
    .expect("grazing basket stack catalog must build");
    let topology = catalog
        .topology_construction()
        .expect("basket stack must expose topology construction");
    let scopes =
        NmtTopologyScopeSet::from_construction(topology).expect("basket stack scopes must compile");
    let certified_scopes = NmtCertifiedScopeSet::from_platform_evidence(
        topology,
        catalog.workload().evidence_ledger(),
        catalog.bound_geometry(),
        catalog.projected_workload(),
        catalog.transform_receipts(),
        catalog
            .replay_receipts()
            .expect("basket stack must expose retained replay receipts"),
        scopes,
    )
    .compile()
    .expect("basket stack must compile certified NMT scopes");
    let receipt = GrazingBasketStackWorkload::from_certified_scopes(&certified_scopes)
        .certify()
        .expect("grazing basket stack must certify from certified NMT scopes");
    let outcome_matrix = real_evidence_outcome_matrix(&catalog, &receipt);
    let user_outcome = respond(WorthUserResponseSource::from_grazing_basket_stack(&receipt));
    GrazingBasketStackSubject {
        catalog,
        certified_scopes,
        receipt,
        outcome_matrix,
        user_outcome,
    }
}

pub(crate) fn grazing_basket_stack_closeout_evidence(
    stem: &str,
) -> GrazingBasketStackCloseoutEvidence {
    let subject = grazing_basket_stack_subject(stem);
    let label_only = subject
        .receipt
        .attempt_label_only_motion(BasketLayerIndex::new(0))
        .expect_err("label-only motion denial");
    let outcomes = vec![
        subject.user_outcome.clone(),
        respond(WorthUserResponseSource::from_grazing_basket_stack_denial(
            &label_only,
        )),
        respond(WorthUserResponseSource::from_grazing_basket_stack_denial(
            &cross_layer_retained_denial(&subject),
        )),
        respond(WorthUserResponseSource::from_grazing_basket_stack_denial(
            &surface_smuggling_denial(&subject),
        )),
        respond(WorthUserResponseSource::from_grazing_basket_stack_denial(
            &missing_boundary_denial(&subject),
        )),
    ];
    GrazingBasketStackCloseoutEvidence {
        certified_scopes: subject.certified_scopes,
        matrix: NmtBossOutcomeMatrixEvidence::from_outcomes(outcomes),
    }
}

pub(crate) fn cross_layer_retained_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_cross_layer_retained_replay_evidence(
            &layer_retained_evidence(&subject.receipt, BasketLayerIndex::new(0)),
            BasketLayerIndex::new(3),
        )
        .expect_err("cross-layer retained replay must deny")
}

pub(crate) fn cross_layer_projection_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_cross_layer_projection_identity_evidence(
            &layer_projection_evidence(&subject.receipt, BasketLayerIndex::new(1)),
            BasketLayerIndex::new(4),
        )
        .expect_err("cross-layer projection identity must deny")
}

pub(crate) fn surface_smuggling_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_unsupported_surface_support(
            BasketLayerIndex::new(2),
            &unsupported_surface_evidence(subject),
        )
        .expect_err("non-plane support must stay localized")
}

pub(crate) fn cross_layer_surface_smuggling_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_cross_layer_surface_support_smuggling_evidence(
            &layer_surface_evidence(&subject.receipt, BasketLayerIndex::new(2)),
            BasketLayerIndex::new(5),
        )
        .expect_err("cross-layer surface support must deny")
}

pub(crate) fn cross_layer_parity_lane_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    subject
        .receipt
        .attempt_cross_layer_parity_lane_smuggling_evidence(
            &layer_parity_evidence(&subject.receipt, BasketLayerIndex::new(1)),
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
    subject
        .receipt
        .attempt_storm_extraction_smuggling_evidence(
            BasketLayerIndex::new(0),
            &GrazingBasketStormExtractionEvidence::from_projected_workload(
                storm.projected_workload(),
            ),
        )
        .expect_err("storm extraction must not certify basket stack")
}

pub(crate) fn false_closure_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    let layer = BasketLayerIndex::new(5);
    subject
        .receipt
        .attempt_false_closure_evidence(&layer_boundary_evidence(&subject.receipt, layer), layer)
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
        .attempt_near_graze_predicate_pressure_evidence(
            layer,
            boundary,
            &near_graze_predicate_evidence(subject),
        )
        .expect_err("near-graze pressure must localize as predicate uncertainty")
}

pub(crate) fn whole_stack_broadening_denial(
    subject: &GrazingBasketStackSubject,
) -> GrazingBasketStackDenial {
    let layer = BasketLayerIndex::new(1);
    subject
        .receipt
        .attempt_whole_stack_broadening_evidence(
            &layer_projection_evidence(&subject.receipt, layer),
            layer,
        )
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
