fn production_receipt() -> super::UiAllocationReceipt {
    let (_, _, _, _, receipt, _, _, _) =
        crate::runtime::tests::production_catalog_activation_test_support::runtime_with_scroll_catalog();
    receipt
}

#[test]
fn local_allocation_inspection_cites_every_required_artifact() {
    use worth_ui_inspection::{
        UiAllocationInspectionEvidenceFamily as Family,
        UiAllocationInspectionKnowledge as Knowledge,
    };
    let inspection = production_receipt().inspection_receipt();
    let local = inspection.local_explanation();

    assert!(!local.stream_families().is_empty());
    assert!(!local.invalidation_families().is_empty());
    assert_eq!(
        local.invalidation_evidence_ref().family(),
        Family::InvalidationArtifact
    );
    assert_eq!(
        local.selection().evidence_ref().family(),
        Family::NeighborhoodSelectionArtifact
    );
    assert_eq!(
        local.reuse_evidence_ref().family(),
        Family::ReuseDecisionArtifact
    );
    assert_eq!(
        local.geometry().evidence_ref().family(),
        Family::GeometryArtifact
    );
    assert_eq!(
        local.selection().ordered_neighborhoods().first().copied(),
        Some(local.selection().primary_neighborhood())
    );
    assert!(matches!(
        local.geometry().parent_edges(),
        Knowledge::NotKnownAtAllocation
    ));
    assert!(matches!(
        local.geometry().sibling_edges(),
        Knowledge::NotKnownAtAllocation
    ));
    assert!(matches!(
        local.geometry().spacing_relationship_ids(),
        Knowledge::NotKnownAtAllocation
    ));
    assert!(matches!(
        local.geometry().baseline_relationship_ids(),
        Knowledge::NotKnownAtAllocation
    ));
}

#[test]
fn portal_inspection_preserves_exact_anchor_identity_and_coordinate_space() {
    use worth_ui_inspection::{
        UiAllocationInspectionAnchorPosture as Anchor,
        UiAllocationInspectionCoordinateSpace as Space,
    };
    let (_, _, receipt, _) =
        crate::runtime::tests::production_catalog_activation_test_support::runtime_with_portal_catalog();
    let inspection = receipt.inspection_receipt();

    assert_eq!(
        inspection.local_explanation().geometry().anchor(),
        Anchor::PortalAnchored {
            target:
                worth_ui_inspection::UiAllocationInspectionPortalAnchorTargetIdentity::diagnostic(
                    44
                ),
            coordinate_space: Space::PortalLayer,
        }
    );
}

#[test]
fn freshness_postures_have_distinct_consumer_legality() {
    use super::{
        UiAllocationFreshnessConsumptionDenial as Denial,
        UiAllocationFreshnessTransitionCause as Cause,
        UiAllocationReceiptFreshnessPosture as Posture,
    };
    let current = production_receipt().report().clone();
    let coalescing = current
        .transition(Posture::Coalescing, Cause::CoalescingWindowOpened, None)
        .expect("current may enter a coalescing window");
    assert_eq!(
        current.transition(
            Posture::StaleButBounded,
            Cause::PartialQuerySettlement,
            None
        ),
        Err(super::UiAllocationFreshnessTransitionDenial::StalePostureRequiresLagBound)
    );
    assert_eq!(
        current.transition(Posture::Coalescing, Cause::ReplacementRequired, None),
        Err(super::UiAllocationFreshnessTransitionDenial::InvalidSuccessor)
    );
    let stale = current.clone().apply_committed_transaction_freshness(
        &production_receipt()
            .transaction()
            .clone()
            .with_partial_query_policy_for_test(1),
    );
    assert_eq!(stale.freshness(), Posture::StaleButBounded);
    let recompute = current
        .transition(
            Posture::RecomputePending,
            Cause::LeafRemeasureRequired,
            None,
        )
        .expect("leaf remeasurement makes execution pending");

    assert_eq!(
        coalescing.transition(Posture::Current, Cause::ResolvedCommit, None),
        Err(super::UiAllocationFreshnessTransitionDenial::InvalidSuccessor)
    );
    assert_eq!(
        stale.transition(Posture::Current, Cause::ResolvedCommit, None),
        Err(super::UiAllocationFreshnessTransitionDenial::InvalidSuccessor)
    );

    assert!(super::admit_host_paint(&current).is_ok());
    assert!(super::admit_host_paint(&coalescing).is_ok());
    assert!(super::admit_host_paint(&stale).is_ok());
    assert_eq!(
        super::admit_host_paint(&recompute),
        Err(Denial::RecomputePending)
    );
    assert!(super::admit_execution_lowering(&current).is_ok());
    assert_eq!(
        super::admit_execution_lowering(&coalescing),
        Err(Denial::CoalescingCannotExecute)
    );
    assert!(super::admit_execution_lowering(&stale).is_ok());
    assert_eq!(
        super::admit_execution_lowering(&recompute),
        Err(Denial::RecomputePending)
    );
}

#[test]
fn denied_replan_inspection_preserves_local_causes_and_exact_refs() {
    use worth_ui_inspection::{
        UiAllocationInspectionAttemptResult as ResultPosture,
        UiAllocationInspectionDenialFamily as DenialFamily,
        UiAllocationInspectionEvidenceFamily as EvidenceFamily,
        UiAllocationInspectionReuseDenialPosture as ReuseDenial,
    };
    let (mut runtime, target, _, _) =
        crate::runtime::tests::production_catalog_activation_test_support::runtime_with_production_catalog_activation();
    let completion = runtime.execute_framework_turn(|turn| {
        turn.interaction(|source| {
            source
                .admit_and_submit(
                    target,
                    crate::runtime::WorthUiTransientInteractionState::TextInput,
                )
                .expect("typing source admits");
        });
    });
    let crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        plan,
        selection,
        ..
    } = completion
    else {
        panic!("typing reaches a local allocation transaction")
    };
    let denied = crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
        plan,
        selection,
        transaction: super::UiAllocationReplanTransactionOutcome::Denied(
            super::UiAllocationReplanTransactionCommitDenial::ReuseDenied {
                ordinal: 0,
                reason: super::UiAllocationReuseDenial::EquivalenceBasisMismatch,
            },
        ),
        planning_counters: Default::default(),
    };
    let inspection = denied
        .denied_replan_inspection()
        .expect("denied transaction carries a local explanation");
    assert!(!inspection.invalidation_families().is_empty());
    assert_eq!(inspection.denial_family(), DenialFamily::Reuse);
    assert_eq!(
        inspection.reuse_denial(),
        ReuseDenial::EquivalenceBasisMismatch
    );
    assert_eq!(
        inspection.result(),
        ResultPosture::PriorCommittedReceiptUnchanged
    );
    assert_eq!(
        inspection.invalidation_evidence_ref().family(),
        EvidenceFamily::InvalidationArtifact
    );
    assert_eq!(
        inspection.selection().evidence_ref().family(),
        EvidenceFamily::NeighborhoodSelectionArtifact
    );
    assert_eq!(
        inspection.denial_evidence_ref().family(),
        EvidenceFamily::DenialArtifact
    );
}

#[test]
fn committed_receipt_carries_every_mandatory_boundedness_counter() {
    use super::UiAllocationCounterName as Name;
    let receipt = production_receipt();
    let counters = receipt
        .report()
        .counters()
        .expect("commit attaches counter report");
    let names = [
        Name::InvalidationClassifications,
        Name::NeighborhoodSelections,
        Name::ReplannedNeighborhoods,
        Name::ReusedReceipts,
        Name::DeniedReuseAttempts,
        Name::ChurnBurstInputs,
        Name::CommittedReceipts,
        Name::RootWidenAttempts,
    ];
    assert_eq!(counters.values().len(), names.len());
    for name in names {
        assert!(
            counters.value(name).is_within_bound(),
            "{name:?} exceeded its policy bound"
        );
    }
    assert_eq!(
        counters.value(Name::InvalidationClassifications).observed(),
        1
    );
    assert_eq!(counters.value(Name::NeighborhoodSelections).observed(), 2);
    assert_eq!(counters.value(Name::ReplannedNeighborhoods).observed(), 2);
    assert_eq!(counters.value(Name::ReusedReceipts).observed(), 0);
    assert_eq!(counters.value(Name::DeniedReuseAttempts).observed(), 0);
    assert_eq!(counters.value(Name::ChurnBurstInputs).observed(), 0);
    assert_eq!(counters.value(Name::CommittedReceipts).observed(), 2);
    assert_eq!(counters.value(Name::RootWidenAttempts).observed(), 0);
}

#[test]
fn denial_taxonomy_never_uses_a_generic_fallback_family() {
    use super::{
        UiAllocationDenialFamily as Family, UiAllocationReplanTransactionCommitDenial as Denial,
    };
    let cases = [
        (Denial::StaleTransactionFrame, Family::GenerationMismatch),
        (
            Denial::ReuseDenied {
                ordinal: 2,
                reason: super::UiAllocationReuseDenial::GenerationMismatch,
            },
            Family::Reuse,
        ),
        (
            Denial::CommitBudgetExceeded {
                attempted: 5,
                maximum: 4,
            },
            Family::CommitBudget,
        ),
        (
            Denial::PortalPriorReceiptMismatch { ordinal: 1 },
            Family::PortalAnchor,
        ),
        (Denial::CatalogBindingMismatch, Family::CatalogBinding),
        (Denial::EvidenceCounterExhausted, Family::CounterExhaustion),
    ];
    for (denial, expected) in cases {
        let evidence = denial.evidence();
        assert_eq!(evidence.family(), expected);
        assert_ne!(evidence.identity().diagnostic_identity(), 0);
        assert_eq!(
            evidence.denied_reuse_attempts(),
            u16::from(expected == Family::Reuse)
        );
        assert!(
            evidence.denied_reuse_attempts() <= evidence.maximum_denied_reuse_attempts(),
            "denial evidence exceeded its one-artifact atomic transaction bound"
        );
    }
    assert_eq!(
        crate::runtime::UiAllocationInvalidationNarrowingDenial::HostEvidenceGenerationMismatch {
            ordinal: 3
        }
        .denial_evidence()
        .family(),
        Family::StaleHostEvidence,
    );
    assert_eq!(
        crate::runtime::UiAllocationInvalidationNarrowingDenial::ScrollOwnershipNotAdmitted {
            ordinal: 4
        }
        .denial_evidence()
        .family(),
        Family::UnsupportedScrollOwnership,
    );
    assert_eq!(
        crate::runtime::UiAllocationInvalidationNarrowingDenial::PortalAnchorObservationInvalid {
            ordinal: 5
        }
        .denial_evidence()
        .family(),
        Family::BrokenPortalAnchor,
    );

    let generation = Denial::ReuseDenied {
        ordinal: 2,
        reason: super::UiAllocationReuseDenial::GenerationMismatch,
    }
    .evidence();
    let basis = Denial::ReuseDenied {
        ordinal: 2,
        reason: super::UiAllocationReuseDenial::EquivalenceBasisMismatch,
    }
    .evidence();
    assert_eq!(
        generation.reuse_reason(),
        Some(super::UiAllocationReuseDenial::GenerationMismatch)
    );
    assert_eq!(
        basis.reuse_reason(),
        Some(super::UiAllocationReuseDenial::EquivalenceBasisMismatch)
    );
    assert_ne!(generation.identity(), basis.identity());
}
