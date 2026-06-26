use crate::runtime::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};
use crate::runtime::{
    WorthUiActivationGateDenial, WorthUiActivationStagingDenial,
    WorthUiDurableStateReconciliationDenial,
};

pub(crate) fn diagnostic_for_reconciliation(
    denial: &WorthUiDurableStateReconciliationDenial,
) -> WorthUiRuntimeDiagnostic {
    diagnostic(
        WorthUiRuntimeDiagnosticFamily::DurableStateReconciliation,
        WorthUiRuntimeDiagnosticCode::DurableStateReconciliationDenied,
        reconciliation_digest(denial),
    )
}

pub(crate) fn diagnostic_for_activation_staging(
    denial: &WorthUiActivationStagingDenial,
) -> WorthUiRuntimeDiagnostic {
    diagnostic(
        WorthUiRuntimeDiagnosticFamily::ActivationStaging,
        WorthUiRuntimeDiagnosticCode::ActivationStagingDenied,
        activation_staging_digest(denial),
    )
}

pub(crate) fn diagnostic_for_activation_gate(
    denial: &WorthUiActivationGateDenial,
) -> WorthUiRuntimeDiagnostic {
    diagnostic(
        WorthUiRuntimeDiagnosticFamily::ActivationGate,
        WorthUiRuntimeDiagnosticCode::ActivationGateDenied,
        activation_gate_digest(denial),
    )
}

fn diagnostic(
    family: WorthUiRuntimeDiagnosticFamily,
    code: WorthUiRuntimeDiagnosticCode,
    evidence_digest: u64,
) -> WorthUiRuntimeDiagnostic {
    WorthUiRuntimeDiagnostic::new(
        family,
        code,
        WorthUiDiagnosticSource::PhaseDenial { evidence_digest },
        Some(evidence_digest),
    )
}

fn reconciliation_digest(denial: &WorthUiDurableStateReconciliationDenial) -> u64 {
    match denial {
        WorthUiDurableStateReconciliationDenial::AmbiguousNodeReplacementPlan { counters } => fold(
            0xB4_00_00_01,
            counters.rejected_reconciliation_count() as u64,
        ),
        WorthUiDurableStateReconciliationDenial::InventoryDigestMismatch {
            plan_active_artifact_digest,
            inventory_active_artifact_digest,
            plan_candidate_artifact_digest,
            inventory_candidate_artifact_digest,
            counters,
        } => fold_all(
            0xB4_00_00_02,
            [
                *plan_active_artifact_digest,
                *inventory_active_artifact_digest,
                *plan_candidate_artifact_digest,
                *inventory_candidate_artifact_digest,
                counters.rejected_reconciliation_count() as u64,
            ],
        ),
        WorthUiDurableStateReconciliationDenial::MissingInventoryFamily {
            family_id,
            counters,
        } => fold_all(
            0xB4_00_00_03,
            [
                durable_state_family_digest(family_id),
                counters.rejected_reconciliation_count() as u64,
            ],
        ),
        WorthUiDurableStateReconciliationDenial::UnsupportedCustomTransition {
            identity_basis,
            family_id,
            transition,
            counters,
        } => fold_all(
            0xB4_00_00_04,
            [
                stable_text_digest(identity_basis),
                durable_state_family_digest(family_id),
                node_transition_digest(*transition),
                counters.rejected_reconciliation_count() as u64,
            ],
        ),
    }
}

fn activation_staging_digest(denial: &WorthUiActivationStagingDenial) -> u64 {
    fold_all(
        0xB5_00_00_01,
        [
            denial.active_artifact_digest(),
            denial.candidate_artifact_digest(),
            denial.frame_epoch().as_u64(),
            activation_staging_reason_digest(denial.reason()),
        ],
    )
}

fn activation_gate_digest(denial: &WorthUiActivationGateDenial) -> u64 {
    fold_all(
        0xB6_00_00_01,
        [
            denial.active_artifact_digest(),
            denial.candidate_artifact_digest(),
            denial.ready_frame_epoch().as_u64(),
            denial.boundary_frame_epoch().as_u64(),
            activation_gate_reason_digest(denial.reason()),
        ],
    )
}

fn durable_state_family_digest(family_id: &crate::runtime::WorthUiDurableStateFamilyId) -> u64 {
    match family_id {
        crate::runtime::WorthUiDurableStateFamilyId::FocusChain => 1,
        crate::runtime::WorthUiDurableStateFamilyId::ScrollAnchor => 2,
        crate::runtime::WorthUiDurableStateFamilyId::SelectionRange => 3,
        crate::runtime::WorthUiDurableStateFamilyId::TextEditBuffer => 4,
        crate::runtime::WorthUiDurableStateFamilyId::SplitterPosition => 5,
        crate::runtime::WorthUiDurableStateFamilyId::TabState => 6,
        crate::runtime::WorthUiDurableStateFamilyId::PanelVisibility => 7,
        crate::runtime::WorthUiDurableStateFamilyId::Custom(id) => fold(8, stable_text_digest(id)),
    }
}

fn node_transition_digest(transition: crate::runtime::WorthUiNodeLifecycleTransition) -> u64 {
    match transition {
        crate::runtime::WorthUiNodeLifecycleTransition::Preserve => 1,
        crate::runtime::WorthUiNodeLifecycleTransition::Replace => 2,
        crate::runtime::WorthUiNodeLifecycleTransition::Drop => 3,
        crate::runtime::WorthUiNodeLifecycleTransition::Create => 4,
        crate::runtime::WorthUiNodeLifecycleTransition::Move => 5,
        crate::runtime::WorthUiNodeLifecycleTransition::Rebind => 6,
        crate::runtime::WorthUiNodeLifecycleTransition::LaneChange => 7,
    }
}

fn activation_staging_reason_digest(
    reason: crate::runtime::WorthUiActivationStagingDenialReason,
) -> u64 {
    match reason {
        crate::runtime::WorthUiActivationStagingDenialReason::MissingDurableStateReconciliation => 1,
        crate::runtime::WorthUiActivationStagingDenialReason::MissingQueryLiveRebindPlan => 2,
        crate::runtime::WorthUiActivationStagingDenialReason::MissingExecutionPlanLoweringInput => 3,
        crate::runtime::WorthUiActivationStagingDenialReason::ExecutionPlanLoweringInputMismatch => 4,
        crate::runtime::WorthUiActivationStagingDenialReason::ActiveArtifactDigestMismatch => 5,
        crate::runtime::WorthUiActivationStagingDenialReason::CandidateArtifactDigestMismatch => 6,
        crate::runtime::WorthUiActivationStagingDenialReason::AdmittedQuerySupportReceiptChanged => 7,
        crate::runtime::WorthUiActivationStagingDenialReason::ActiveRuntimeMutatedDuringStaging => 8,
    }
}

fn activation_gate_reason_digest(reason: crate::runtime::WorthUiActivationGateDenialReason) -> u64 {
    match reason {
        crate::runtime::WorthUiActivationGateDenialReason::UnsafeFrameBoundary => 1,
        crate::runtime::WorthUiActivationGateDenialReason::BoundaryFrameEpochMismatch => 2,
        crate::runtime::WorthUiActivationGateDenialReason::StaleFrameEpoch => 3,
        crate::runtime::WorthUiActivationGateDenialReason::FutureFrameEpochMismatch => 4,
        crate::runtime::WorthUiActivationGateDenialReason::PendingActivationNotReady => 5,
        crate::runtime::WorthUiActivationGateDenialReason::PendingAndPlanInputMismatch => 6,
        crate::runtime::WorthUiActivationGateDenialReason::HandleAllocationReceiptMismatch => 7,
        crate::runtime::WorthUiActivationGateDenialReason::ExecutionPlanHandleReceiptMismatch => 8,
        crate::runtime::WorthUiActivationGateDenialReason::QueryRebindDenied => 9,
        crate::runtime::WorthUiActivationGateDenialReason::MissingLaneParityReport => 10,
        crate::runtime::WorthUiActivationGateDenialReason::LaneParityDoesNotCertifyActivation => 11,
        crate::runtime::WorthUiActivationGateDenialReason::LaneParityDigestMismatch => 12,
    }
}

fn fold_all<const N: usize>(seed: u64, values: [u64; N]) -> u64 {
    values.into_iter().fold(seed, fold)
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
