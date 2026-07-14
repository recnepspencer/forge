use super::{
    worth_query_lower_runtime_closeout_registry, worth_query_lower_runtime_crossing_inventory,
    worth_query_lower_runtime_direct_import_audit, worth_query_lower_runtime_gap_registry,
};
use crate::lower_runtime_routing::{
    WorthQueryLowerRuntimeArtifactStrength, WorthQueryLowerRuntimeAuthorityOwner,
    WorthQueryLowerRuntimeCloseoutPosture, WorthQueryLowerRuntimeCrossingClassification,
    WorthQueryLowerRuntimeDirectImportPosture, WorthQueryLowerRuntimeSeamKey,
};

#[test]
fn frontier_gap_is_closed_after_signal_receipt_contract_lands() {
    assert!(worth_query_lower_runtime_gap_registry()
        .rows()
        .iter()
        .all(|row| row.seam_key() != WorthQueryLowerRuntimeSeamKey::FrontierEvidenceIntake));
}

#[test]
fn phase_two_audit_rows_no_longer_tolerate_transition_only_seams() {
    let rows = worth_query_lower_runtime_direct_import_audit();
    assert!(rows.rows().iter().any(|row| {
        row.seam_key() == WorthQueryLowerRuntimeSeamKey::FrontierSignalAdapterModule
            && row.posture() == WorthQueryLowerRuntimeDirectImportPosture::AllowedAdapter
    }));
    assert!(rows.rows().iter().any(|row| {
        row.seam_key() == WorthQueryLowerRuntimeSeamKey::EffectExecutionBridgeModule
            && row.posture() == WorthQueryLowerRuntimeDirectImportPosture::AllowedAdapter
    }));
    assert!(rows.rows().iter().all(|row| {
        row.posture() != WorthQueryLowerRuntimeDirectImportPosture::TransitionOnlyElimination
    }));
}

#[test]
fn no_crossing_row_remains_uncategorized() {
    for row in worth_query_lower_runtime_crossing_inventory().rows() {
        assert!(!row.capability_label().is_empty());
        assert!(!row.concrete_seam().is_empty());
        assert!(!row.required_action().is_empty());
    }
}

#[test]
fn compatibility_debt_rows_match_locked_phase_one_seams() {
    let rows = worth_query_lower_runtime_crossing_inventory();
    assert!(rows.rows().iter().all(|row| {
        row.classification() != WorthQueryLowerRuntimeCrossingClassification::CompatibilityDebtLane
    }));
}

#[test]
fn frontier_evidence_intake_is_now_an_allowed_adapter() {
    let rows = worth_query_lower_runtime_crossing_inventory();
    assert!(rows.rows().iter().any(|row| {
        row.seam_key() == WorthQueryLowerRuntimeSeamKey::FrontierEvidenceIntake
            && row.classification()
                == WorthQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter
            && row.lower_runtime_owner() == WorthQueryLowerRuntimeAuthorityOwner::Signal
    }));
}

#[test]
fn writeback_execution_adapter_is_now_an_allowed_bridge_contract_adapter() {
    let rows = worth_query_lower_runtime_crossing_inventory();
    assert!(rows.rows().iter().any(|row| {
        row.seam_key() == WorthQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback
            && row.classification()
                == WorthQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter
            && row.lower_runtime_owner() == WorthQueryLowerRuntimeAuthorityOwner::RuntimeBridge
    }));
    assert!(worth_query_lower_runtime_gap_registry()
        .rows()
        .iter()
        .all(|row| row.seam_key() != WorthQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback));
}

#[test]
fn receipt_upgrade_seams_no_longer_appear_in_gap_registry() {
    let gaps = worth_query_lower_runtime_gap_registry();
    for seam in [
        WorthQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        WorthQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        WorthQueryLowerRuntimeSeamKey::SubscriptionActivation,
    ] {
        assert!(gaps.rows().iter().all(|row| row.seam_key() != seam));
    }
}

#[test]
fn receipt_upgrade_seams_are_locked_to_typed_receipts() {
    let inventory = worth_query_lower_runtime_crossing_inventory();
    for seam in [
        WorthQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        WorthQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        WorthQueryLowerRuntimeSeamKey::SubscriptionActivation,
    ] {
        assert!(inventory.rows().iter().any(|row| {
            row.seam_key() == seam
                && row.current_artifact_strength()
                    == WorthQueryLowerRuntimeArtifactStrength::TypedReceipt
        }));
    }
}

#[test]
fn runtime_intent_module_no_longer_appears_in_gap_or_audit_registry() {
    assert!(worth_query_lower_runtime_gap_registry()
        .rows()
        .iter()
        .all(|row| row.seam_key() != WorthQueryLowerRuntimeSeamKey::RuntimeIntentModule));
    assert!(worth_query_lower_runtime_direct_import_audit()
        .rows()
        .iter()
        .all(|row| row.seam_key() != WorthQueryLowerRuntimeSeamKey::RuntimeIntentModule));
}

#[test]
fn closeout_registry_has_no_compatibility_debt_and_names_deferred_neighbors_explicitly() {
    let registry = worth_query_lower_runtime_closeout_registry();

    assert!(registry.rows().iter().any(|row| {
        row.seam_key() == WorthQueryLowerRuntimeSeamKey::RuntimeIntentModule
            && row.posture() == WorthQueryLowerRuntimeCloseoutPosture::SeamEliminated
    }));

    for seam in [
        WorthQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor,
        WorthQueryLowerRuntimeSeamKey::DurableRouteReplayNeighbor,
        WorthQueryLowerRuntimeSeamKey::PersistedBoundaryExecutionReceiptNeighbor,
        WorthQueryLowerRuntimeSeamKey::RestartStableBoundaryEnvelopeReloadNeighbor,
        WorthQueryLowerRuntimeSeamKey::TemporalQueryBasisRoutingNeighbor,
        WorthQueryLowerRuntimeSeamKey::AsyncResourceRoutingNeighbor,
        WorthQueryLowerRuntimeSeamKey::MixedTruthTimeAsyncRoutingNeighbor,
        WorthQueryLowerRuntimeSeamKey::FinalDeferredCertificationClosureNeighbor,
    ] {
        let row = registry
            .rows()
            .iter()
            .find(|row| row.seam_key() == seam)
            .expect("every deferred neighbor must be encoded in the closeout registry");
        assert_eq!(
            row.posture(),
            WorthQueryLowerRuntimeCloseoutPosture::DeferredNeighbor
        );
        assert!(!row.closeout_target().is_empty());
        assert!(!row.required_closeout().is_empty());
        assert!(!row.certification_row().is_empty());
    }
}
