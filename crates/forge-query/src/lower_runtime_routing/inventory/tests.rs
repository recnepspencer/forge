use super::{
    forge_query_lower_runtime_closeout_registry, forge_query_lower_runtime_crossing_inventory,
    forge_query_lower_runtime_direct_import_audit, forge_query_lower_runtime_gap_registry,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeArtifactStrength, ForgeQueryLowerRuntimeAuthorityOwner,
    ForgeQueryLowerRuntimeCloseoutPosture, ForgeQueryLowerRuntimeCrossingClassification,
    ForgeQueryLowerRuntimeDirectImportPosture, ForgeQueryLowerRuntimeSeamKey,
};

#[test]
fn frontier_gap_is_closed_after_signal_receipt_contract_lands() {
    assert!(forge_query_lower_runtime_gap_registry()
        .rows()
        .iter()
        .all(|row| row.seam_key() != ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake));
}

#[test]
fn phase_two_audit_rows_no_longer_tolerate_transition_only_seams() {
    let rows = forge_query_lower_runtime_direct_import_audit();
    assert!(rows.rows().iter().any(|row| {
        row.seam_key() == ForgeQueryLowerRuntimeSeamKey::FrontierSignalAdapterModule
            && row.posture() == ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter
    }));
    assert!(rows.rows().iter().any(|row| {
        row.seam_key() == ForgeQueryLowerRuntimeSeamKey::EffectExecutionBridgeModule
            && row.posture() == ForgeQueryLowerRuntimeDirectImportPosture::AllowedAdapter
    }));
    assert!(rows.rows().iter().all(|row| {
        row.posture() != ForgeQueryLowerRuntimeDirectImportPosture::TransitionOnlyElimination
    }));
}

#[test]
fn downstream_runtime_boundary_subtree_is_classified() {
    assert!(forge_query_lower_runtime_direct_import_audit()
        .rows()
        .iter()
        .any(|row| {
            row.seam_key() == ForgeQueryLowerRuntimeSeamKey::DownstreamQueryRuntimeBoundarySubtree
                && row.posture()
                    == ForgeQueryLowerRuntimeDirectImportPosture::DownstreamRuntimeBoundarySubtree
        }));
}

#[test]
fn no_crossing_row_remains_uncategorized() {
    for row in forge_query_lower_runtime_crossing_inventory().rows() {
        assert!(!row.capability_label().is_empty());
        assert!(!row.concrete_seam().is_empty());
        assert!(!row.required_action().is_empty());
    }
}

#[test]
fn compatibility_debt_rows_match_locked_phase_one_seams() {
    let rows = forge_query_lower_runtime_crossing_inventory();
    assert!(rows.rows().iter().all(|row| {
        row.classification() != ForgeQueryLowerRuntimeCrossingClassification::CompatibilityDebtLane
    }));
}

#[test]
fn frontier_evidence_intake_is_now_an_allowed_adapter() {
    let rows = forge_query_lower_runtime_crossing_inventory();
    assert!(rows.rows().iter().any(|row| {
        row.seam_key() == ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake
            && row.classification()
                == ForgeQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter
            && row.lower_runtime_owner() == ForgeQueryLowerRuntimeAuthorityOwner::Signal
    }));
}

#[test]
fn writeback_execution_adapter_is_now_an_allowed_bridge_contract_adapter() {
    let rows = forge_query_lower_runtime_crossing_inventory();
    assert!(rows.rows().iter().any(|row| {
        row.seam_key() == ForgeQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback
            && row.classification()
                == ForgeQueryLowerRuntimeCrossingClassification::QueryBoundaryAdapter
            && row.lower_runtime_owner() == ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge
    }));
    assert!(forge_query_lower_runtime_gap_registry()
        .rows()
        .iter()
        .all(|row| row.seam_key() != ForgeQueryLowerRuntimeSeamKey::EffectBackedBridgeWriteback));
}

#[test]
fn receipt_upgrade_seams_no_longer_appear_in_gap_registry() {
    let gaps = forge_query_lower_runtime_gap_registry();
    for seam in [
        ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
    ] {
        assert!(gaps.rows().iter().all(|row| row.seam_key() != seam));
    }
}

#[test]
fn receipt_upgrade_seams_are_locked_to_typed_receipts() {
    let inventory = forge_query_lower_runtime_crossing_inventory();
    for seam in [
        ForgeQueryLowerRuntimeSeamKey::LiveViewSchemaAdmission,
        ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
    ] {
        assert!(inventory.rows().iter().any(|row| {
            row.seam_key() == seam
                && row.current_artifact_strength()
                    == ForgeQueryLowerRuntimeArtifactStrength::TypedReceipt
        }));
    }
}

#[test]
fn runtime_intent_module_no_longer_appears_in_gap_or_audit_registry() {
    assert!(forge_query_lower_runtime_gap_registry()
        .rows()
        .iter()
        .all(|row| row.seam_key() != ForgeQueryLowerRuntimeSeamKey::RuntimeIntentModule));
    assert!(forge_query_lower_runtime_direct_import_audit()
        .rows()
        .iter()
        .all(|row| row.seam_key() != ForgeQueryLowerRuntimeSeamKey::RuntimeIntentModule));
}

#[test]
fn closeout_registry_has_no_compatibility_debt_and_names_deferred_neighbors_explicitly() {
    let registry = forge_query_lower_runtime_closeout_registry();

    assert!(registry.rows().iter().any(|row| {
        row.seam_key() == ForgeQueryLowerRuntimeSeamKey::RuntimeIntentModule
            && row.posture() == ForgeQueryLowerRuntimeCloseoutPosture::SeamEliminated
    }));

    for seam in [
        ForgeQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor,
        ForgeQueryLowerRuntimeSeamKey::DurableRouteReplayNeighbor,
        ForgeQueryLowerRuntimeSeamKey::PersistedBoundaryExecutionReceiptNeighbor,
        ForgeQueryLowerRuntimeSeamKey::RestartStableBoundaryEnvelopeReloadNeighbor,
        ForgeQueryLowerRuntimeSeamKey::TemporalQueryBasisRoutingNeighbor,
        ForgeQueryLowerRuntimeSeamKey::AsyncResourceRoutingNeighbor,
        ForgeQueryLowerRuntimeSeamKey::MixedTruthTimeAsyncRoutingNeighbor,
        ForgeQueryLowerRuntimeSeamKey::FinalDeferredCertificationClosureNeighbor,
    ] {
        let row = registry
            .rows()
            .iter()
            .find(|row| row.seam_key() == seam)
            .expect("every deferred neighbor must be encoded in the closeout registry");
        assert_eq!(
            row.posture(),
            ForgeQueryLowerRuntimeCloseoutPosture::DeferredNeighbor
        );
        assert!(!row.closeout_target().is_empty());
        assert!(!row.required_closeout().is_empty());
        assert!(!row.certification_row().is_empty());
    }
}
