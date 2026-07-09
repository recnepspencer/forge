use serde::Serialize;

use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};

use super::ResourceSupersessionPolicyDeclaration;

const SUPERSESSION_NEW_GENERATION_SUPERSEDES_PRIOR_NAME: &str =
    "signal.resource.supersession.new-generation-supersedes-prior";
const SUPERSESSION_OVERLAPPING_GENERATION_RETAINS_OLD_HOST_WORK_NAME: &str =
    "signal.resource.supersession.overlapping-generation-retains-old-host-work";
const SUPERSESSION_OVERLAPPING_GENERATION_CANCELS_OLD_HOST_WORK_NAME: &str =
    "signal.resource.supersession.overlapping-generation-cancels-old-host-work";
const SUPERSESSION_INTENT_EQUIVALENT_COALESCES_TO_ACTIVE_NAME: &str =
    "signal.resource.supersession.intent-equivalent-coalesces-to-active";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceSupersessionDecisionClass {
    NewGenerationSupersedesPrior,
    OverlappingGenerationRetainsOldHostWork,
    OverlappingGenerationCancelsOldHostWork,
    IntentEquivalentCoalescesToActive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceSupersessionOverlapDisposition {
    NoOverlapAdmission,
    ExplicitOverlapAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceSupersessionOldHostWorkPosture {
    LeaveRunning,
    AdvisoryCancelRequested,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceSupersessionDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceSupersessionDecisionClass,
    overlap_disposition: ResourceSupersessionOverlapDisposition,
    old_host_work_posture: ResourceSupersessionOldHostWorkPosture,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceSupersessionDecisionPlan {
    pub(crate) fn lower(
        declaration: &ResourceSupersessionPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        match declaration {
            ResourceSupersessionPolicyDeclaration::NewGenerationSupersedesPrior => {
                ensure_descriptor_name(
                    frozen,
                    SUPERSESSION_NEW_GENERATION_SUPERSEDES_PRIOR_NAME,
                    "new generation supersedes prior",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceSupersessionDecisionClass::NewGenerationSupersedesPrior,
                    ResourceSupersessionOverlapDisposition::NoOverlapAdmission,
                    ResourceSupersessionOldHostWorkPosture::LeaveRunning,
                ))
            }
            ResourceSupersessionPolicyDeclaration::OverlappingGenerationRetainsOldHostWork => {
                ensure_descriptor_name(
                    frozen,
                    SUPERSESSION_OVERLAPPING_GENERATION_RETAINS_OLD_HOST_WORK_NAME,
                    "overlapping generation retains old host work",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceSupersessionDecisionClass::OverlappingGenerationRetainsOldHostWork,
                    ResourceSupersessionOverlapDisposition::ExplicitOverlapAdmission,
                    ResourceSupersessionOldHostWorkPosture::LeaveRunning,
                ))
            }
            ResourceSupersessionPolicyDeclaration::OverlappingGenerationCancelsOldHostWork => {
                ensure_descriptor_name(
                    frozen,
                    SUPERSESSION_OVERLAPPING_GENERATION_CANCELS_OLD_HOST_WORK_NAME,
                    "overlapping generation cancels old host work",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceSupersessionDecisionClass::OverlappingGenerationCancelsOldHostWork,
                    ResourceSupersessionOverlapDisposition::ExplicitOverlapAdmission,
                    ResourceSupersessionOldHostWorkPosture::AdvisoryCancelRequested,
                ))
            }
            ResourceSupersessionPolicyDeclaration::IntentEquivalentCoalescesToActive => {
                ensure_descriptor_name(
                    frozen,
                    SUPERSESSION_INTENT_EQUIVALENT_COALESCES_TO_ACTIVE_NAME,
                    "intent-equivalent coalesces to active",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceSupersessionDecisionClass::IntentEquivalentCoalescesToActive,
                    ResourceSupersessionOverlapDisposition::NoOverlapAdmission,
                    ResourceSupersessionOldHostWorkPosture::LeaveRunning,
                ))
            }
            ResourceSupersessionPolicyDeclaration::Named { name } => {
                Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::Supersession,
                    name: name.clone(),
                    reason:
                        "named supersession policies are descriptor-only in the first ship runtime",
                })
            }
        }
    }

    fn new(
        frozen: &FrozenResourcePolicyDescriptor,
        class: ResourceSupersessionDecisionClass,
        overlap_disposition: ResourceSupersessionOverlapDisposition,
        old_host_work_posture: ResourceSupersessionOldHostWorkPosture,
    ) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-supersession-plan:{}:{}:{}:{}",
            frozen.frozen_digest().as_str(),
            class.as_str(),
            overlap_disposition.as_str(),
            old_host_work_posture.as_str(),
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class,
            overlap_disposition,
            old_host_work_posture,
            decision_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn semantic_name(&self) -> &str {
        &self.semantic_name
    }

    pub fn class(&self) -> ResourceSupersessionDecisionClass {
        self.class
    }

    pub fn overlap_disposition(&self) -> ResourceSupersessionOverlapDisposition {
        self.overlap_disposition
    }

    pub fn old_host_work_posture(&self) -> ResourceSupersessionOldHostWorkPosture {
        self.old_host_work_posture
    }

    pub fn permits_overlapping_generation_admission(&self) -> bool {
        matches!(
            self.overlap_disposition,
            ResourceSupersessionOverlapDisposition::ExplicitOverlapAdmission
        )
    }

    pub fn requests_old_host_work_advisory_cancel(&self) -> bool {
        matches!(
            self.old_host_work_posture,
            ResourceSupersessionOldHostWorkPosture::AdvisoryCancelRequested
        )
    }

    pub fn permits_intent_equivalence_coalescing(&self) -> bool {
        matches!(
            self.class,
            ResourceSupersessionDecisionClass::IntentEquivalentCoalescesToActive
        )
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }
}

impl ResourceSupersessionDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::NewGenerationSupersedesPrior => "new-generation-supersedes-prior",
            Self::OverlappingGenerationRetainsOldHostWork => {
                "overlapping-generation-retains-old-host-work"
            }
            Self::OverlappingGenerationCancelsOldHostWork => {
                "overlapping-generation-cancels-old-host-work"
            }
            Self::IntentEquivalentCoalescesToActive => "intent-equivalent-coalesces-to-active",
        }
    }
}

impl ResourceSupersessionOverlapDisposition {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::NoOverlapAdmission => "no-overlap-admission",
            Self::ExplicitOverlapAdmission => "explicit-overlap-admission",
        }
    }
}

impl ResourceSupersessionOldHostWorkPosture {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::LeaveRunning => "leave-running",
            Self::AdvisoryCancelRequested => "advisory-cancel-requested",
        }
    }
}

fn ensure_descriptor_name(
    frozen: &FrozenResourcePolicyDescriptor,
    expected: &str,
    reason: &'static str,
) -> Result<(), ResourcePolicyResolutionError> {
    if frozen.descriptor().semantic_name().as_str() == expected {
        return Ok(());
    }
    Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
        kind: ResourcePolicyKind::Supersession,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
