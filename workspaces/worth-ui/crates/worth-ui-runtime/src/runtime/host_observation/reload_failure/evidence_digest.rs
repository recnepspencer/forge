use crate::runtime::{
    WorthUiActivationGateDenial, WorthUiActivationStagingDenial,
    WorthUiActivationStagingDenialReason, WorthUiDurableStateFamilyId,
    WorthUiDurableStateReconciliationDenial, WorthUiNodeLifecycleTransition,
    WorthUiPlanLoweringDenial, WorthUiQueryBindingDriftDenial, WorthUiQueryBindingDriftDenialKind,
    WorthUiQueryBindingUiRequirements, WorthUiQueryBindingUiRequirementsDriftFamily,
    WorthUiQueryLiveRebindPlanDenial, WorthUiReplacementCandidateDenial,
};

pub(crate) fn invalid_candidate_denial_digest(denial: WorthUiReplacementCandidateDenial) -> u64 {
    match denial {
        WorthUiReplacementCandidateDenial::MissingArtifactDigest => 0xC0_00_00_01,
        WorthUiReplacementCandidateDenial::MissingDependencyMetadata => 0xC0_00_00_02,
        WorthUiReplacementCandidateDenial::MissingLoweringBasis => 0xC0_00_00_03,
        WorthUiReplacementCandidateDenial::DependencyMetadataArtifactDigestMismatch => {
            0xC0_00_00_04
        }
        WorthUiReplacementCandidateDenial::StaleDependencyMetadata => 0xC0_00_00_05,
    }
}

pub(crate) fn reconciliation_denial_digest(
    denial: &WorthUiDurableStateReconciliationDenial,
) -> u64 {
    match denial {
        WorthUiDurableStateReconciliationDenial::AmbiguousNodeReplacementPlan { counters } => {
            0xD0_00_00_01 ^ counters.rejected_reconciliation_count() as u64
        }
        WorthUiDurableStateReconciliationDenial::InventoryDigestMismatch {
            plan_active_artifact_digest,
            inventory_active_artifact_digest,
            plan_candidate_artifact_digest,
            inventory_candidate_artifact_digest,
            counters,
        } => {
            0xD0_00_00_02
                ^ plan_active_artifact_digest
                ^ inventory_active_artifact_digest.rotate_left(11)
                ^ plan_candidate_artifact_digest.rotate_left(23)
                ^ inventory_candidate_artifact_digest.rotate_left(37)
                ^ (counters.rejected_reconciliation_count() as u64).rotate_left(47)
        }
        WorthUiDurableStateReconciliationDenial::MissingInventoryFamily {
            family_id,
            counters,
        } => {
            0xD0_00_00_03
                ^ durable_state_family_digest(family_id)
                ^ (counters.rejected_reconciliation_count() as u64).rotate_left(19)
        }
        WorthUiDurableStateReconciliationDenial::UnsupportedCustomTransition {
            identity_basis,
            family_id,
            transition,
            counters,
        } => {
            0xD0_00_00_04
                ^ stable_text_digest(identity_basis)
                ^ durable_state_family_digest(family_id).rotate_left(13)
                ^ node_transition_digest(*transition).rotate_left(29)
                ^ (counters.rejected_reconciliation_count() as u64).rotate_left(43)
        }
    }
}

pub(crate) fn activation_staging_denial_digest(denial: &WorthUiActivationStagingDenial) -> u64 {
    denial.active_artifact_digest()
        ^ denial.candidate_artifact_digest().rotate_left(7)
        ^ denial.frame_epoch().as_u64().rotate_left(19)
        ^ activation_staging_reason_digest(denial.reason()).rotate_left(31)
}

pub(crate) fn plan_lowering_denial_digest(denial: &WorthUiPlanLoweringDenial) -> u64 {
    denial.active_artifact_digest()
        ^ denial.candidate_artifact_digest().rotate_left(11)
        ^ denial.pending_frame_epoch().as_u64().rotate_left(23)
        ^ denial.active_frame_epoch().as_u64().rotate_left(37)
}

pub(crate) fn activation_gate_denial_digest(denial: &WorthUiActivationGateDenial) -> u64 {
    denial.active_artifact_digest()
        ^ denial.candidate_artifact_digest().rotate_left(13)
        ^ denial.ready_frame_epoch().as_u64().rotate_left(29)
        ^ denial.boundary_frame_epoch().as_u64().rotate_left(41)
}

pub(crate) fn query_live_rebind_denial_digest(denial: &WorthUiQueryLiveRebindPlanDenial) -> u64 {
    match denial {
        WorthUiQueryLiveRebindPlanDenial::AmbiguousNodeReplacementPlan => 0xE0_00_00_01,
        WorthUiQueryLiveRebindPlanDenial::ComparisonDigestMismatch {
            comparison_active_artifact_digest,
            plan_active_artifact_digest,
            comparison_candidate_artifact_digest,
            plan_candidate_artifact_digest,
        } => {
            0xE0_00_00_02
                ^ comparison_active_artifact_digest
                ^ plan_active_artifact_digest.rotate_left(11)
                ^ comparison_candidate_artifact_digest.rotate_left(23)
                ^ plan_candidate_artifact_digest.rotate_left(37)
        }
        WorthUiQueryLiveRebindPlanDenial::NarrowingDigestMismatch {
            comparison_active_artifact_digest,
            narrowing_active_artifact_digest,
            comparison_candidate_artifact_digest,
            narrowing_candidate_artifact_digest,
        } => {
            0xE0_00_00_03
                ^ comparison_active_artifact_digest
                ^ narrowing_active_artifact_digest.rotate_left(11)
                ^ comparison_candidate_artifact_digest.rotate_left(23)
                ^ narrowing_candidate_artifact_digest.rotate_left(37)
        }
        WorthUiQueryLiveRebindPlanDenial::AdmittedCandidateDigestMismatch {
            comparison_candidate_artifact_digest,
            admitted_candidate_artifact_digest,
        } => {
            0xE0_00_00_04
                ^ comparison_candidate_artifact_digest
                ^ admitted_candidate_artifact_digest.rotate_left(11)
        }
    }
}

pub(crate) fn query_binding_drift_denial_digest(denial: &WorthUiQueryBindingDriftDenial) -> u64 {
    0xE1_00_00_01
        ^ denial.identity().canonical_identity()
        ^ query_binding_ui_requirements_digest(denial.active_ui_requirements()).rotate_left(29)
        ^ query_binding_ui_requirements_digest(denial.candidate_ui_requirements()).rotate_left(37)
        ^ query_binding_drift_reason_digest(denial.reason()).rotate_left(43)
        ^ query_binding_drift_families_digest(denial.drift_families()).rotate_left(53)
}

fn query_binding_ui_requirements_digest(
    posture: Option<&WorthUiQueryBindingUiRequirements>,
) -> u64 {
    match posture {
        Some(posture) => posture.canonical_identity(),
        None => 0,
    }
}

fn query_binding_drift_reason_digest(reason: WorthUiQueryBindingDriftDenialKind) -> u64 {
    match reason {
        WorthUiQueryBindingDriftDenialKind::MissingCandidateUiRequirementsForRebind => {
            0xE2_00_00_01
        }
        WorthUiQueryBindingDriftDenialKind::MissingActiveUiRequirementsForRetirement => {
            0xE2_00_00_02
        }
    }
}

fn query_binding_drift_families_digest(
    families: &[WorthUiQueryBindingUiRequirementsDriftFamily],
) -> u64 {
    families.iter().fold(0xE3_00_00_01, |digest, family| {
        digest.rotate_left(5) ^ query_binding_drift_family_digest(*family)
    })
}

fn query_binding_drift_family_digest(family: WorthUiQueryBindingUiRequirementsDriftFamily) -> u64 {
    match family {
        WorthUiQueryBindingUiRequirementsDriftFamily::LifecycleDeclaration => 0xE4_00_00_01,
        WorthUiQueryBindingUiRequirementsDriftFamily::AsyncResultPresentation => 0xE4_00_00_02,
        WorthUiQueryBindingUiRequirementsDriftFamily::RecoveryPresentation => 0xE4_00_00_03,
        WorthUiQueryBindingUiRequirementsDriftFamily::InspectionRelevance => 0xE4_00_00_04,
        WorthUiQueryBindingUiRequirementsDriftFamily::ProjectionConsumption => 0xE4_00_00_05,
        WorthUiQueryBindingUiRequirementsDriftFamily::DenialPresentation => 0xE4_00_00_06,
    }
}

fn durable_state_family_digest(family_id: &WorthUiDurableStateFamilyId) -> u64 {
    match family_id {
        WorthUiDurableStateFamilyId::FocusChain => 0xF0_00_00_01,
        WorthUiDurableStateFamilyId::ScrollAnchor => 0xF0_00_00_02,
        WorthUiDurableStateFamilyId::SelectionRange => 0xF0_00_00_03,
        WorthUiDurableStateFamilyId::TextEditBuffer => 0xF0_00_00_04,
        WorthUiDurableStateFamilyId::SplitterPosition => 0xF0_00_00_05,
        WorthUiDurableStateFamilyId::TabState => 0xF0_00_00_06,
        WorthUiDurableStateFamilyId::PanelVisibility => 0xF0_00_00_07,
        WorthUiDurableStateFamilyId::Custom(id) => 0xF0_00_00_08 ^ stable_text_digest(id),
    }
}

fn node_transition_digest(transition: WorthUiNodeLifecycleTransition) -> u64 {
    match transition {
        WorthUiNodeLifecycleTransition::Preserve => 0xF1_00_00_01,
        WorthUiNodeLifecycleTransition::Replace => 0xF1_00_00_02,
        WorthUiNodeLifecycleTransition::Drop => 0xF1_00_00_03,
        WorthUiNodeLifecycleTransition::Create => 0xF1_00_00_04,
        WorthUiNodeLifecycleTransition::Move => 0xF1_00_00_05,
        WorthUiNodeLifecycleTransition::Rebind => 0xF1_00_00_06,
        WorthUiNodeLifecycleTransition::LaneChange => 0xF1_00_00_07,
    }
}

fn activation_staging_reason_digest(reason: WorthUiActivationStagingDenialReason) -> u64 {
    match reason {
        WorthUiActivationStagingDenialReason::CandidateApplicationAuthorityMismatch => {
            0xF2_00_00_09
        }
        WorthUiActivationStagingDenialReason::MissingDurableStateReconciliation => 0xF2_00_00_01,
        WorthUiActivationStagingDenialReason::MissingQueryLiveRebindPlan => 0xF2_00_00_02,
        WorthUiActivationStagingDenialReason::ActiveArtifactDigestMismatch => 0xF2_00_00_05,
        WorthUiActivationStagingDenialReason::CandidateArtifactDigestMismatch => 0xF2_00_00_06,
        WorthUiActivationStagingDenialReason::ActiveRuntimeMutatedDuringStaging => 0xF2_00_00_08,
    }
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
