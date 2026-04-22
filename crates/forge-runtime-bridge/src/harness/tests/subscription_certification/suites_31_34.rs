use std::sync::Arc;

use super::support::*;
use crate::facade::{
    BridgePreviewSessionIdentity, BridgeRuntimePolicy,
    BridgeSubscriptionConsumerBackpressurePosture, BridgeSubscriptionConsumerContractFamily,
    BridgeSubscriptionConsumerDiagnosticsRetention, BridgeSubscriptionConsumerPacingCapability,
    BridgeSubscriptionContinuationCandidateInput, BridgeSubscriptionContinuationKind,
    BridgeSubscriptionContinuationRejectionKind, BridgeSubscriptionDeliveryDensityPosture,
    BridgeSubscriptionDeliveryFamilyKind, BridgeSubscriptionDuplicateReplayPolicyKind,
    BridgeSubscriptionFanoutPlanRejectionKind, BridgeSubscriptionPreviewPromotionOutcomeClass,
    BridgeSubscriptionPreviewWorkInput, BridgeSubscriptionPreviewWorkKind,
    BridgeSubscriptionPreviewWorkTraceRejectionKind,
    BridgeSubscriptionResumeAdmissionRejectionKind, RuntimeBridge,
};

#[test]
fn bridge_harness_subscription_suite_31_shared_fanout_parity_is_canonical() {
    let bridge = runtime(BridgeRuntimePolicy::development());
    let declaration = detail_subscription(&bridge);
    let shared_active = active_subscription_for(
        &bridge,
        &declaration,
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let plan = bridge
        .plan_shared_subscription_fanout(&shared_active, vec![canonical_consumer(&bridge)])
        .expect("equivalent consumers should share one active subscription");
    let layout = bridge.build_subscription_fanout_layout(
        plan,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let shared_window = sealed_window_with_members(
        &bridge,
        &shared_active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        0,
        fixture_members(2),
    );
    let projections = bridge
        .project_subscription_delivery_to_fanout(&layout, &shared_window)
        .expect("shared fanout projection should match layout");

    let separate_runtime = runtime(BridgeRuntimePolicy::development());
    let separate_declaration = detail_subscription(&separate_runtime);
    let separate_active = active_subscription_for(
        &separate_runtime,
        &separate_declaration,
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let separate_window = sealed_window_with_members(
        &separate_runtime,
        &separate_active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        0,
        fixture_members(2),
    );

    assert_eq!(shared_active.digest(), separate_active.digest());
    assert_eq!(layout.consumer_bindings().len(), 2);
    assert_eq!(
        shared_window.members()[0].digest(),
        separate_window.members()[0].digest()
    );
    assert_eq!(
        shared_window.members()[1].digest(),
        separate_window.members()[1].digest()
    );
    assert!(projections
        .canonical_member_digest_basis()
        .contains(shared_window.members()[0].digest()));
    assert!(projections
        .canonical_member_digest_basis()
        .contains(shared_window.members()[1].digest()));
    assert_eq!(projections.len(), 2);
    assert_eq!(
        layout
            .counters()
            .subscription_fanout_per_member_consumer_scan_count(),
        0
    );
    assert_eq!(
        layout
            .counters()
            .subscription_callback_identity_scan_count(),
        0
    );
    assert_eq!(
        layout.counters().subscription_active_registry_scan_count(),
        0
    );
    let shared_checkpoint = checkpoint_from_sealed(
        &bridge,
        &shared_active,
        &shared_window,
        1,
        BridgeSubscriptionDuplicateReplayPolicyKind::SuppressAcknowledgedMembers,
    );
    let separate_checkpoint = checkpoint_from_sealed(
        &separate_runtime,
        &separate_active,
        &separate_window,
        1,
        BridgeSubscriptionDuplicateReplayPolicyKind::SuppressAcknowledgedMembers,
    );
    assert_eq!(
        shared_checkpoint.checkpoint_identity(),
        separate_checkpoint.checkpoint_identity()
    );
    assert_eq!(shared_checkpoint.digest(), separate_checkpoint.digest());

    let collection_declaration = collection_subscription(&bridge);
    let coalesced_active = active_subscription_for(
        &bridge,
        &collection_declaration,
        BridgeSubscriptionDeliveryDensityPosture::BoundedCoalescedWindow,
        2,
    );
    let coalesced_plan = bridge
        .plan_shared_subscription_fanout(&coalesced_active, vec![canonical_consumer(&bridge)])
        .expect("equivalent coalescing-admitted consumers should share");
    let coalesced_layout = bridge.build_subscription_fanout_layout(
        coalesced_plan,
        BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced,
    );
    let coalesced_window = sealed_window_with_members(
        &bridge,
        &coalesced_active,
        BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced,
        0,
        fixture_members(2),
    );
    let coalesced_projection = bridge
        .project_subscription_delivery_to_fanout(&coalesced_layout, &coalesced_window)
        .expect("admitted coalesced fanout projection should preserve member truth");
    assert!(coalesced_projection
        .canonical_member_digest_basis()
        .contains(coalesced_window.members()[0].digest()));
    assert!(coalesced_projection
        .canonical_member_digest_basis()
        .contains(coalesced_window.members()[1].digest()));
    assert_eq!(
        coalesced_layout
            .counters()
            .subscription_fanout_per_member_consumer_scan_count(),
        0
    );

    let lag_runtime = runtime(BridgeRuntimePolicy::development());
    let lag_declaration = detail_subscription(&lag_runtime);
    let lag_primary = lag_runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::LagBounded,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("lag-bounded primary consumer should admit");
    let lag_ready = activation_ready_for(&lag_runtime, &lag_declaration);
    let lag_cost = lag_runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            4,
            4,
            2,
        )
        .expect("lag fanout cost profile should admit");
    let lag_active = lag_runtime.activate_subscription_delivery(lag_ready, lag_cost, lag_primary);
    let lag_additional = lag_runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::LagBounded,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("lag-bounded additional consumer should admit");
    let lag_plan = lag_runtime
        .plan_shared_subscription_fanout(&lag_active, vec![lag_additional])
        .expect("equivalent lag-bounded consumers should share");
    assert_eq!(lag_plan.consumer_contract_identity_count(), 2);

    let diagnostics_rich = bridge
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::RetainedDetail,
        )
        .expect("diagnostics-rich canonical consumer should admit");
    let diagnostics_rejection = bridge
        .plan_shared_subscription_fanout(&shared_active, vec![diagnostics_rich])
        .expect_err("diagnostics-rich consumer must not share minimal fanout silently");
    assert_eq!(
        diagnostics_rejection.rejection_kind(),
        BridgeSubscriptionFanoutPlanRejectionKind::DiagnosticsRetentionMismatch
    );

    let replay_audit = bridge
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::ReplayAudit,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::RetainedDetail,
        )
        .expect("replay-audit consumer should admit");
    let rejection = bridge
        .plan_shared_subscription_fanout(&shared_active, vec![replay_audit])
        .expect_err("incompatible consumers must reject before delivery");
    assert_eq!(
        rejection.rejection_kind(),
        BridgeSubscriptionFanoutPlanRejectionKind::ContractFamilyMismatch
    );
}

#[test]
fn bridge_harness_subscription_suite_32_continuation_identity_evolution_is_typed() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let detail = detail_subscription(&runtime);
    let active = active_subscription_for(
        &runtime,
        &detail,
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        1,
    );
    let candidates = vec![
        BridgeSubscriptionContinuationCandidateInput::one_to_one_replace(
            "lineage:suite-32:replace",
            "subscription-locality:entity-1/profile/name",
            "basis:suite-32:replace",
        ),
        BridgeSubscriptionContinuationCandidateInput::one_to_many_split(
            "lineage:suite-32:split",
            "subscription-locality:entity-1/profile/name",
            vec![
                Arc::from("basis:suite-32:child-a"),
                Arc::from("basis:suite-32:child-b"),
            ],
        ),
        BridgeSubscriptionContinuationCandidateInput::merge_like_continue(
            "lineage:suite-32:merge-like",
            "subscription-locality:entity-1/profile/name",
            "basis:suite-32:merge",
        ),
        BridgeSubscriptionContinuationCandidateInput::branch_local_continue(
            "lineage:suite-32:branch-local",
            "subscription-locality:entity-1/profile/name",
            "basis:suite-32:branch-local",
        ),
        BridgeSubscriptionContinuationCandidateInput::rejected_ambiguous(
            "lineage:suite-32:ambiguous",
            "subscription-locality:entity-1/profile/name",
        ),
    ];
    let index = runtime
        .build_subscription_continuation_index(&active, candidates.clone())
        .expect("continuation index should build from locality candidates");

    let replace = runtime
        .plan_subscription_continuation(&active, &index, 0)
        .expect("replace continuation should plan");
    let split = runtime
        .plan_subscription_continuation(&active, &index, 1)
        .expect("split continuation should plan");
    let merge_like = runtime
        .plan_subscription_continuation(&active, &index, 2)
        .expect("merge-like continuation should plan");
    let branch_local = runtime
        .plan_subscription_continuation(&active, &index, 3)
        .expect("branch-local continuation should plan");
    let ambiguous = runtime
        .plan_subscription_continuation(&active, &index, 4)
        .expect_err("ambiguous continuation must reject typed");

    assert_eq!(
        replace.continuation_kind(),
        BridgeSubscriptionContinuationKind::OneToOneReplace
    );
    assert_eq!(replace.children().len(), 1);
    assert_eq!(
        split.continuation_kind(),
        BridgeSubscriptionContinuationKind::OneToManySplit
    );
    assert_eq!(split.children().len(), 2);
    assert_ne!(split.children()[0].digest(), split.children()[1].digest());
    assert_eq!(
        merge_like.continuation_kind(),
        BridgeSubscriptionContinuationKind::MergeLikeContinue
    );
    assert_eq!(
        branch_local.continuation_kind(),
        BridgeSubscriptionContinuationKind::BranchLocalContinue
    );
    let unrelated_collection = collection_subscription(&runtime);
    let unrelated_active = active_subscription_for(
        &runtime,
        &unrelated_collection,
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        1,
    );
    let unrelated_rejection = runtime
        .plan_subscription_continuation(&unrelated_active, &index, 3)
        .expect_err("branch-local continuation index must not leak to unrelated active");
    assert_eq!(
        unrelated_rejection.rejection_kind(),
        BridgeSubscriptionContinuationRejectionKind::ActiveSubscriptionMismatch
    );
    assert_eq!(
        ambiguous.rejection_kind(),
        BridgeSubscriptionContinuationRejectionKind::Ambiguous
    );
    assert_eq!(
        index
            .counters()
            .subscription_continuation_full_registry_scan_count(),
        0
    );
    assert_eq!(
        ambiguous
            .counters()
            .subscription_continuation_full_registry_scan_count(),
        0
    );

    let restart_runtime = super::support::runtime(BridgeRuntimePolicy::development());
    let restart_detail = detail_subscription(&restart_runtime);
    let restart_active = active_subscription_for(
        &restart_runtime,
        &restart_detail,
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        1,
    );
    let restart_index = restart_runtime
        .build_subscription_continuation_index(&restart_active, candidates)
        .expect("restart continuation index should rebuild from retained candidate basis");
    let restart_replace = restart_runtime
        .plan_subscription_continuation(&restart_active, &restart_index, 0)
        .expect("restart replace continuation should plan");
    let restart_split = restart_runtime
        .plan_subscription_continuation(&restart_active, &restart_index, 1)
        .expect("restart split continuation should plan");
    let restart_ambiguous = restart_runtime
        .plan_subscription_continuation(&restart_active, &restart_index, 4)
        .expect_err("restart ambiguous continuation must reject typed");

    assert_eq!(active.digest(), restart_active.digest());
    assert_eq!(index.digest(), restart_index.digest());
    assert_eq!(replace.digest(), restart_replace.digest());
    assert_eq!(split.digest(), restart_split.digest());
    assert_eq!(ambiguous.digest(), restart_ambiguous.digest());
}

#[test]
fn bridge_harness_subscription_suite_33_checkpoint_resume_and_replay_are_exact() {
    for build_declaration in [
        detail_subscription as fn(&RuntimeBridge) -> crate::facade::BridgeSubscriptionDeclaration,
        collection_subscription,
    ] {
        let bridge = runtime(BridgeRuntimePolicy::development());
        let declaration = build_declaration(&bridge);
        let active = active_subscription_for(
            &bridge,
            &declaration,
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            1,
        );
        let first_window = sealed_window_with_members(
            &bridge,
            &active,
            BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
            0,
            fixture_members(2),
        );
        let second_window = sealed_window_with_members(
            &bridge,
            &active,
            BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
            1,
            fixture_members(2),
        );
        let checkpoint_window = sealed_window_with_members(
            &bridge,
            &active,
            BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
            2,
            fixture_members(2),
        );
        assert_ne!(first_window.digest(), second_window.digest());
        assert_ne!(second_window.digest(), checkpoint_window.digest());
        let checkpoint = checkpoint_from_sealed(
            &bridge,
            &active,
            &checkpoint_window,
            1,
            BridgeSubscriptionDuplicateReplayPolicyKind::SuppressAcknowledgedMembers,
        );
        let admission = bridge
            .admit_subscription_resume(&active, &checkpoint)
            .expect("resume admission should accept matching subscription checkpoint");
        let resume_plan = bridge.plan_subscription_resume(admission.clone());
        let retained =
            bridge.retain_subscription_delivery_window_seed(&sealed_window_with_members(
                &bridge,
                &active,
                BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
                3,
                fixture_members(1),
            ));
        let replay_plan = bridge
            .plan_subscription_delivery_replay(&active, admission, vec![retained])
            .expect("retained delivery replay should plan from subscription checkpoint");

        assert_eq!(
            resume_plan.active_subscription_identity(),
            active.active_subscription_identity()
        );
        assert_eq!(
            resume_plan.checkpoint_identity(),
            checkpoint.checkpoint_identity()
        );
        assert_eq!(resume_plan.expected_next_canonical_sequence(), 2);
        assert_eq!(
            replay_plan.active_subscription_identity(),
            active.active_subscription_identity()
        );
        assert_eq!(
            replay_plan.delivery_family_identity(),
            checkpoint.delivery_family_identity()
        );
        assert_eq!(replay_plan.retained_window_count(), 1);
        assert_eq!(replay_plan.retained_member_count(), 1);
        assert_eq!(replay_plan.counters().replay_mismatch_count(), 0);

        let restart_bridge = runtime(BridgeRuntimePolicy::development());
        let restart_declaration = build_declaration(&restart_bridge);
        let restart_active = active_subscription_for(
            &restart_bridge,
            &restart_declaration,
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            1,
        );
        let restart_checkpoint_window = sealed_window_with_members(
            &restart_bridge,
            &restart_active,
            BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
            2,
            fixture_members(2),
        );
        let restart_checkpoint = checkpoint_from_sealed(
            &restart_bridge,
            &restart_active,
            &restart_checkpoint_window,
            1,
            BridgeSubscriptionDuplicateReplayPolicyKind::SuppressAcknowledgedMembers,
        );
        let restart_admission = restart_bridge
            .admit_subscription_resume(&restart_active, &restart_checkpoint)
            .expect("restart resume admission should accept matching checkpoint");
        let restart_retained =
            restart_bridge.retain_subscription_delivery_window_seed(&sealed_window_with_members(
                &restart_bridge,
                &restart_active,
                BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
                3,
                fixture_members(1),
            ));
        let restart_replay_plan = restart_bridge
            .plan_subscription_delivery_replay(
                &restart_active,
                restart_admission,
                vec![restart_retained],
            )
            .expect("restart retained replay should plan from canonical artifacts");
        assert_eq!(checkpoint.digest(), restart_checkpoint.digest());
        assert_eq!(replay_plan.digest(), restart_replay_plan.digest());

        let stale_admission = bridge
            .admit_subscription_resume(&active, &checkpoint)
            .expect("resume admission should accept matching checkpoint for stale test");
        let stale_seed = bridge.retain_subscription_delivery_window_seed(&checkpoint_window);
        let stale_rejection = bridge
            .plan_subscription_delivery_replay(&active, stale_admission, vec![stale_seed])
            .expect_err("checkpoint window cannot be replayed as future retained work");
        assert_eq!(
            stale_rejection.rejection_kind(),
            crate::facade::BridgeSubscriptionDeliveryReplayPlanRejectionKind::RetainedWindowNotAfterCheckpoint
        );

        let (_other_runtime, other_active) = {
            let other_runtime = runtime(BridgeRuntimePolicy::development());
            let other_declaration = detail_subscription(&other_runtime);
            let other_active = active_subscription_for(
                &other_runtime,
                &other_declaration,
                BridgeSubscriptionDeliveryDensityPosture::BoundedCoalescedWindow,
                1,
            );
            (other_runtime, other_active)
        };
        let resume_rejection = bridge
            .admit_subscription_resume(&other_active, &checkpoint)
            .expect_err("checkpoint from another active subscription must reject");
        assert_eq!(
            resume_rejection.rejection_kind(),
            BridgeSubscriptionResumeAdmissionRejectionKind::ActiveSubscriptionMismatch
        );
    }
}

#[test]
fn bridge_harness_subscription_suite_34_preview_zero_residue_and_promotion_are_explicit() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let detail = detail_subscription(&runtime);
    let collection = collection_subscription(&runtime);

    for (suffix, declaration) in [
        ("detail-discard", &detail),
        ("collection-discard", &collection),
    ] {
        let preview_active = preview_active_subscription_for(&runtime, suffix, declaration);
        let preview_identity = preview_active
            .preview_active_subscription_identity()
            .clone();
        let preview_scope_identity = preview_active.preview_scope_identity().clone();
        let work_trace = runtime
            .record_preview_subscription_work(
                &preview_active,
                vec![
                    BridgeSubscriptionPreviewWorkInput::routing(format!(
                        "preview-routing:{suffix}"
                    )),
                    BridgeSubscriptionPreviewWorkInput::delivery(format!(
                        "preview-delivery:{suffix}"
                    )),
                    BridgeSubscriptionPreviewWorkInput::diagnostics(format!(
                        "preview-diagnostics:{suffix}"
                    )),
                    BridgeSubscriptionPreviewWorkInput::continuation(format!(
                        "preview-continuation:{suffix}"
                    )),
                ],
            )
            .expect("preview residue must be backed by all preview work descriptors");
        assert_eq!(work_trace.records().len(), 4);
        assert_eq!(
            work_trace.preview_active_subscription_identity(),
            &preview_identity
        );
        assert_eq!(work_trace.preview_scope_identity(), &preview_scope_identity);
        assert_eq!(
            work_trace.records()[0].kind(),
            BridgeSubscriptionPreviewWorkKind::Routing
        );
        assert_eq!(
            work_trace.records()[1].kind(),
            BridgeSubscriptionPreviewWorkKind::Delivery
        );
        assert_eq!(
            work_trace.records()[2].kind(),
            BridgeSubscriptionPreviewWorkKind::Diagnostics
        );
        assert_eq!(
            work_trace.records()[3].kind(),
            BridgeSubscriptionPreviewWorkKind::Continuation
        );
        assert!(work_trace
            .canonical_basis()
            .contains(work_trace.records()[0].digest()));
        let residue_index = runtime.build_subscription_preview_residue_scope_index(
            &preview_active,
            work_trace.zero_residue_inputs(),
        );
        let discard = runtime
            .discard_preview_subscription(preview_active, residue_index)
            .expect("zero preview residue should discard");

        assert_eq!(discard.total_residue_count(), 0);
        assert_eq!(discard.artifact_records().len(), 7);
        assert_eq!(discard.counters().subscription_preview_discard_count(), 1);
        assert_eq!(
            discard
                .counters()
                .subscription_preview_residue_nonzero_count(),
            0
        );
        assert_eq!(
            discard
                .counters()
                .subscription_preview_non_scope_registry_scan_count(),
            0
        );
        assert!(discard
            .artifact_records()
            .iter()
            .all(|artifact| artifact.evidence_digest().contains(work_trace.digest())));
        assert!(discard.artifact_records().iter().any(|artifact| {
            artifact
                .evidence_digest()
                .contains(work_trace.record_digest_for(BridgeSubscriptionPreviewWorkKind::Delivery))
        }));
    }

    let malformed_preview = preview_active_subscription_for(&runtime, "malformed-work", &detail);
    let duplicate_rejection = runtime
        .record_preview_subscription_work(
            &malformed_preview,
            vec![
                BridgeSubscriptionPreviewWorkInput::routing("preview-routing:duplicate-a"),
                BridgeSubscriptionPreviewWorkInput::routing("preview-routing:duplicate-b"),
                BridgeSubscriptionPreviewWorkInput::delivery("preview-delivery:duplicate"),
                BridgeSubscriptionPreviewWorkInput::diagnostics("preview-diagnostics:duplicate"),
                BridgeSubscriptionPreviewWorkInput::continuation("preview-continuation:duplicate"),
            ],
        )
        .expect_err("duplicate preview work kind must reject before residue indexing");
    assert_eq!(
        duplicate_rejection.rejection_kind(),
        BridgeSubscriptionPreviewWorkTraceRejectionKind::DuplicateWorkKind
    );
    let missing_rejection = runtime
        .record_preview_subscription_work(
            &malformed_preview,
            vec![
                BridgeSubscriptionPreviewWorkInput::routing("preview-routing:missing"),
                BridgeSubscriptionPreviewWorkInput::delivery("preview-delivery:missing"),
                BridgeSubscriptionPreviewWorkInput::diagnostics("preview-diagnostics:missing"),
            ],
        )
        .expect_err("missing continuation preview work must reject before residue indexing");
    assert_eq!(
        missing_rejection.rejection_kind(),
        BridgeSubscriptionPreviewWorkTraceRejectionKind::MissingWorkKind
    );

    let promotion_ready = activation_ready_for(&runtime, &detail);
    let admitted_preview = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:subscription:promotion"),
            preview_declaration("promotion"),
        )
        .expect("preview session should admit for promotion");
    let (active_preview_session, execution_record) =
        runtime.activate_preview_session(admitted_preview, 3, 1, 2);
    let preview_basis = runtime
        .admit_subscription_preview_basis(&active_preview_session, &execution_record)
        .expect("preview basis should admit");
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            4,
            4,
            1,
        )
        .expect("cost profile should admit");
    let preview_active = runtime.activate_preview_subscription_delivery(
        promotion_ready,
        preview_basis,
        cost_profile,
        canonical_consumer(&runtime),
    );
    let proof = active_preview_session.promotion_admissibility_proof();
    let (_promoted_session, speculation_promotion) = runtime
        .promote_preview_session(
            active_preview_session,
            &execution_record,
            &proof,
            "commit-boundary:subscription-promotion",
            "authoritative-artifact:subscription-promotion",
        )
        .expect("speculation promotion should succeed");
    let promoted_ready = activation_ready_for(&runtime, &detail);
    let preview_identity = preview_active
        .preview_active_subscription_identity()
        .clone();
    let promotion_work_trace = runtime
        .record_preview_subscription_work(
            &preview_active,
            vec![
                BridgeSubscriptionPreviewWorkInput::routing(
                    "preview-routing:subscription-promotion",
                ),
                BridgeSubscriptionPreviewWorkInput::delivery(
                    "preview-delivery:subscription-promotion",
                ),
                BridgeSubscriptionPreviewWorkInput::diagnostics(
                    "preview-diagnostics:subscription-promotion",
                ),
                BridgeSubscriptionPreviewWorkInput::continuation(
                    "preview-continuation:subscription-promotion",
                ),
            ],
        )
        .expect("promotion must be backed by scope-local preview work");
    let promoted_identity = promoted_ready
        .admitted()
        .admitted_subscription_identity()
        .clone();
    let promotion = runtime
        .promote_preview_subscription(
            preview_active,
            &promotion_work_trace,
            &speculation_promotion,
            &promoted_ready,
        )
        .expect("subscription promotion boundary should admit");
    let explanation = runtime.inspect_subscription_preview_promotion_record(&promotion);

    assert_eq!(
        promotion.outcome_class(),
        BridgeSubscriptionPreviewPromotionOutcomeClass::PromotedAuthoritativeBoundary
    );
    assert_eq!(
        promotion.preview_active_subscription_identity(),
        &preview_identity
    );
    assert_eq!(
        promotion.promoted_admitted_subscription_identity(),
        &promoted_identity
    );
    assert_eq!(
        promotion.preview_work_trace_identity(),
        promotion_work_trace.preview_work_trace_identity()
    );
    assert_eq!(
        explanation.preview_work_trace_digest(),
        promotion_work_trace.digest()
    );
    assert_ne!(
        promotion.preview_active_subscription_identity().as_str(),
        promotion.promoted_admitted_subscription_identity().as_str()
    );
    assert_eq!(
        explanation.promotion_record_identity(),
        promotion.promotion_record_identity()
    );
    assert_eq!(
        explanation.speculation_promotion_record_digest(),
        speculation_promotion.digest()
    );
    assert_eq!(explanation.counters().diagnostics_bundle_count(), 1);
}
