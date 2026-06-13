pub(crate) mod subject;

use std::collections::BTreeSet;

use subject::{
    cross_layer_parity_lane_denial, cross_layer_projection_denial, cross_layer_retained_denial,
    cross_layer_surface_smuggling_denial, false_closure_denial,
    grazing_basket_stack_closeout_evidence, grazing_basket_stack_subject, missing_boundary_denial,
    missing_projection_denial, missing_retained_checkpoint_denial, near_graze_uncertain_denial,
    respond, storm_smuggling_denial, surface_smuggling_denial, whole_stack_broadening_denial,
};
use worth_spatial::facade::grazing_basket_stack::{
    BasketLayerIndex, GrazingBasketStackDenialKind, GrazingBasketStackOutcomeKind,
    LayerTransformPressure,
};
use worth_spatial::facade::nmt_certification_context::{NmtBossCloseoutReceipt, NmtBossId};
use worth_spatial::facade::user_response::{
    WorthUserOutcomeCauseKind, WorthUserOutcomeKind, WorthUserResponseSource,
};

#[test]
fn mb_m6_nmt_4_grazing_basket_stack_preserves_open_layer_identity() {
    run_with_real_workload_stack(|| {
        let subject = grazing_basket_stack_subject("mb-m6-nmt-4-admitted");
        let counters = subject.receipt.counters();

        assert_eq!(
            subject.catalog.recipe().human_name(),
            "grazing open shell basket stack workload recipe"
        );
        assert_eq!(counters.total_layers(), 6);
        assert_eq!(counters.strips_per_layer(), 12);
        assert_eq!(counters.touched_layers(), 6);
        assert_eq!(counters.projection_breadth(), 6);
        assert_eq!(counters.projection_consumption_breadth(), 6);
        assert_eq!(counters.retained_checkpoint_breadth(), 6);
        assert_eq!(counters.local_frame_breadth(), 6);
        assert_eq!(counters.radial_adjacency_breadth(), 6);
        assert!(counters.open_boundary_breadth() > 0);
        assert_eq!(subject.receipt.layers().len(), 6);
        assert_eq!(subject.user_outcome.kind(), WorthUserOutcomeKind::Admitted);

        let layer_identities = subject
            .receipt
            .layers()
            .iter()
            .map(|layer| layer.layer_identity())
            .collect::<BTreeSet<_>>();
        let projection_identities = subject
            .receipt
            .layers()
            .iter()
            .map(|layer| layer.projection_identity())
            .collect::<BTreeSet<_>>();
        let replay_identities = subject
            .receipt
            .layers()
            .iter()
            .map(|layer| layer.retained_replay_identity())
            .collect::<BTreeSet<_>>();
        let motion_identities = subject
            .receipt
            .layers()
            .iter()
            .map(|layer| layer.transform_posture_identity())
            .collect::<BTreeSet<_>>();
        let local_frame_identities = subject
            .receipt
            .layers()
            .iter()
            .map(|layer| layer.local_frame_identity())
            .collect::<BTreeSet<_>>();
        let radial_adjacency_identities = subject
            .receipt
            .layers()
            .iter()
            .map(|layer| layer.radial_adjacency_identity())
            .collect::<BTreeSet<_>>();
        assert_eq!(layer_identities.len(), 6);
        assert_eq!(projection_identities.len(), 6);
        assert_eq!(replay_identities.len(), 6);
        assert_eq!(motion_identities.len(), 6);
        assert_eq!(local_frame_identities.len(), 6);
        assert_eq!(radial_adjacency_identities.len(), 6);
        for layer in subject.receipt.layers() {
            assert!(!layer.topology_posture_identity().is_empty());
            assert!(!layer.projection_identity().is_empty());
            assert!(!layer.retained_replay_identity().is_empty());
            assert!(!layer.transform_posture_identity().is_empty());
            assert!(!layer.local_frame_identity().is_empty());
            assert!(!layer.radial_adjacency_identity().is_empty());
            assert!(!layer.open_boundary().boundary_identity().is_empty());
        }
        assert!(subject.receipt.layers().iter().any(|layer| matches!(
            layer.transform_pressure(),
            Some(LayerTransformPressure::MovementRotationStack { .. })
        )));
        assert!(subject.receipt.layers().iter().any(|layer| matches!(
            layer.transform_pressure(),
            Some(LayerTransformPressure::HostileCancellation { .. })
        )));
        let closeout = grazing_basket_stack_closeout_evidence("mb-m6-nmt-4-closeout");
        let closeout_receipt = NmtBossCloseoutReceipt::from_certified_scope_set(
            NmtBossId::GrazingBasketStack,
            &closeout.certified_scopes,
            &closeout.matrix,
        )
        .expect("grazing basket stack must close out from certified scope evidence");
        assert_eq!(closeout_receipt.boss(), NmtBossId::GrazingBasketStack);
        assert_eq!(closeout_receipt.outcome_count(), 5);

        let moved_layer = BasketLayerIndex::new(1);
        let moved_receipt = subject.receipt.layer(moved_layer).expect("moved layer");
        let equivalent_variant = subject
            .receipt
            .admit_equivalent_transform_variant(
                moved_layer,
                moved_receipt
                    .transform_pressure()
                    .expect("moved layer carries transform pressure"),
            )
            .expect("equivalent transform pressure admits");
        assert_eq!(
            equivalent_variant.layer_identity(),
            moved_receipt.layer_identity()
        );
        assert_eq!(
            equivalent_variant.transform_posture_identity(),
            moved_receipt.transform_posture_identity()
        );

        for required in GrazingBasketStackOutcomeKind::REQUIRED {
            assert!(
                subject
                    .outcome_matrix
                    .rows()
                    .iter()
                    .any(|row| row.kind() == required),
                "basket stack matrix must branch {required:?}"
            );
        }
        for required in [
            GrazingBasketStackDenialKind::LabelOnlyMotion,
            GrazingBasketStackDenialKind::OpenBoundaryPerturbation,
            GrazingBasketStackDenialKind::CrossLayerRetainedReplay,
            GrazingBasketStackDenialKind::CrossLayerProjectionIdentity,
            GrazingBasketStackDenialKind::SurfaceSupportSmuggling,
            GrazingBasketStackDenialKind::CrossLayerParityLane,
            GrazingBasketStackDenialKind::UnsupportedSurfaceFamily,
            GrazingBasketStackDenialKind::StormExtractionSmuggling,
            GrazingBasketStackDenialKind::FalseClosure,
            GrazingBasketStackDenialKind::WholeStackBroadening,
            GrazingBasketStackDenialKind::MissingLayerEvidence,
            GrazingBasketStackDenialKind::MissingProjectionEvidence,
            GrazingBasketStackDenialKind::MissingRetainedCheckpointEvidence,
            GrazingBasketStackDenialKind::PredicateUncertain,
        ] {
            assert!(
                subject
                    .outcome_matrix
                    .rows()
                    .iter()
                    .any(|row| row.denial_kind() == Some(required)),
                "basket stack matrix must branch denial stop {required:?}"
            );
        }
    });
}

#[test]
fn mb_m6_nmt_4_rejects_cross_layer_checkpoint_projection_and_surface_smuggling() {
    run_with_real_workload_stack(|| {
        let subject = grazing_basket_stack_subject("mb-m6-nmt-4-smuggling");
        let retained = cross_layer_retained_denial(&subject);
        assert_eq!(
            retained.kind(),
            GrazingBasketStackDenialKind::CrossLayerRetainedReplay
        );
        assert_eq!(retained.source_layer(), Some(BasketLayerIndex::new(0)));
        assert_eq!(retained.target_layer(), Some(BasketLayerIndex::new(3)));
        assert_eq!(retained.touched_layers(), 2);
        assert_human_readable(retained.human_reason());

        let projection = cross_layer_projection_denial(&subject);
        assert_eq!(
            projection.kind(),
            GrazingBasketStackDenialKind::CrossLayerProjectionIdentity
        );
        assert_eq!(projection.touched_layers(), 2);
        assert!(projection.human_reason().contains("Projection identity"));

        let parity = cross_layer_parity_lane_denial(&subject);
        assert_eq!(
            parity.kind(),
            GrazingBasketStackDenialKind::CrossLayerParityLane
        );
        assert_eq!(parity.touched_layers(), 2);
        assert!(parity.human_reason().contains("Parity lane"));

        let cross_surface = cross_layer_surface_smuggling_denial(&subject);
        assert_eq!(
            cross_surface.kind(),
            GrazingBasketStackDenialKind::SurfaceSupportSmuggling
        );
        assert_eq!(cross_surface.touched_layers(), 2);
        assert!(cross_surface
            .human_reason()
            .contains("Surface support receipt"));

        let surface = surface_smuggling_denial(&subject);
        assert_eq!(
            surface.kind(),
            GrazingBasketStackDenialKind::UnsupportedSurfaceFamily
        );
        assert_eq!(surface.touched_layers(), 1);
        assert!(surface.human_reason().contains("non-planar"));

        let storm = storm_smuggling_denial(&subject);
        assert_eq!(
            storm.kind(),
            GrazingBasketStackDenialKind::StormExtractionSmuggling
        );
        assert_eq!(storm.touched_layers(), 1);
        assert!(storm
            .human_reason()
            .contains("Closed storm extraction bundle"));
    });
}

#[test]
fn mb_m6_nmt_4_denies_false_closure_and_localizes_near_graze_pressure() {
    run_with_real_workload_stack(|| {
        let subject = grazing_basket_stack_subject("mb-m6-nmt-4-false-closure");
        let false_closure = false_closure_denial(&subject);
        assert_eq!(
            false_closure.kind(),
            GrazingBasketStackDenialKind::FalseClosure
        );
        assert_eq!(false_closure.touched_layers(), 1);
        assert!(false_closure.human_reason().contains("closed-shell"));
        assert!(false_closure.human_reason().contains("boundary"));

        let near_graze = near_graze_uncertain_denial(&subject);
        assert_eq!(
            near_graze.kind(),
            GrazingBasketStackDenialKind::PredicateUncertain
        );
        assert_eq!(near_graze.touched_layers(), 1);
        assert!(near_graze.boundary().is_some());
        assert!(near_graze.human_reason().contains("local frame"));
        assert!(near_graze.human_reason().contains("precision tier"));

        let response = respond(WorthUserResponseSource::from_grazing_basket_stack_denial(
            &near_graze,
        ));
        assert_eq!(response.kind(), WorthUserOutcomeKind::PredicateUncertain);
        assert_eq!(
            response.cause().map(|cause| cause.kind()),
            Some(WorthUserOutcomeCauseKind::PredicateUncertain)
        );

        let boundary_perturbation = subject
            .receipt
            .attempt_open_boundary_perturbation(BasketLayerIndex::new(2))
            .expect_err("open-boundary perturbation must deny before projection success");
        assert_eq!(
            boundary_perturbation.kind(),
            GrazingBasketStackDenialKind::OpenBoundaryPerturbation
        );
        assert_eq!(boundary_perturbation.touched_layers(), 1);
        assert!(boundary_perturbation.boundary().is_some());
        assert!(boundary_perturbation
            .human_reason()
            .contains("open-boundary perturbation"));
    });
}

#[test]
fn mb_m6_nmt_4_touched_layer_counters_block_whole_stack_laundering() {
    run_with_real_workload_stack(|| {
        let subject = grazing_basket_stack_subject("mb-m6-nmt-4-counters");
        let broadening = whole_stack_broadening_denial(&subject);
        assert_eq!(
            broadening.kind(),
            GrazingBasketStackDenialKind::WholeStackBroadening
        );
        assert_eq!(broadening.touched_layers(), 1);
        assert!(broadening.human_reason().contains("touched one layer"));

        let missing = missing_boundary_denial(&subject);
        assert_eq!(
            missing.kind(),
            GrazingBasketStackDenialKind::MissingLayerEvidence
        );
        assert_eq!(missing.touched_layers(), 1);
        let response = respond(WorthUserResponseSource::from_grazing_basket_stack_denial(
            &missing,
        ));
        assert_eq!(response.kind(), WorthUserOutcomeKind::NoOptions);
        assert_eq!(
            response.cause().map(|cause| cause.kind()),
            Some(WorthUserOutcomeCauseKind::MissingEvidence)
        );

        let missing_projection = missing_projection_denial(&subject);
        assert_eq!(
            missing_projection.kind(),
            GrazingBasketStackDenialKind::MissingProjectionEvidence
        );
        assert_eq!(missing_projection.touched_layers(), 1);
        assert!(missing_projection.human_reason().contains("projection"));
        let projection_response = respond(
            WorthUserResponseSource::from_grazing_basket_stack_denial(&missing_projection),
        );
        assert_eq!(projection_response.kind(), WorthUserOutcomeKind::NoOptions);
        assert_eq!(
            projection_response.cause().map(|cause| cause.kind()),
            Some(WorthUserOutcomeCauseKind::MissingEvidence)
        );

        let missing_retained = missing_retained_checkpoint_denial(&subject);
        assert_eq!(
            missing_retained.kind(),
            GrazingBasketStackDenialKind::MissingRetainedCheckpointEvidence
        );
        assert_eq!(missing_retained.touched_layers(), 1);
        assert!(missing_retained
            .human_reason()
            .contains("retained checkpoint"));
        let retained_response = respond(WorthUserResponseSource::from_grazing_basket_stack_denial(
            &missing_retained,
        ));
        assert_eq!(retained_response.kind(), WorthUserOutcomeKind::NoOptions);
        assert_eq!(
            retained_response.cause().map(|cause| cause.kind()),
            Some(WorthUserOutcomeCauseKind::MissingEvidence)
        );

        for row in subject.outcome_matrix.rows() {
            assert!(!row.evidence_digest().is_empty());
            assert_human_readable(row.human_reason());
            assert_eq!(row.counters().total_layers(), 6);
            assert_eq!(row.counters().strips_per_layer(), 12);
            assert!(row.counters().open_boundary_breadth() > 0);
            if row.kind() != GrazingBasketStackOutcomeKind::Admitted {
                assert!(
                    row.counters().touched_layers() <= 2,
                    "attack rows must expose local breadth rather than whole-stack laundering"
                );
            }
        }
    });
}

fn assert_human_readable(message: &str) {
    assert!(!message.trim().is_empty());
    assert!(
        !message.contains('_'),
        "basket stack response must not leak machine tokens: {message}"
    );
    assert!(
        !message
            .split_whitespace()
            .any(|word| word.matches('-').count() >= 3),
        "basket stack response must explain causes in prose: {message}"
    );
}

fn run_with_real_workload_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("mb-m6-nmt-4-real-workload".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(test)
        .expect("spawn real workload stack")
        .join()
        .expect("real workload stack test");
}
