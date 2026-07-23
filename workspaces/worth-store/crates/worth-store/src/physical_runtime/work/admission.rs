use crate::physical_runtime::{lifecycle::ObservedLifecyclePhase, record_serving::ServingHealth};

use super::{
    submission::PhysicalWorkSubmissionOwner, AdmittedPhysicalWork,
    AdmittedPhysicalWorkAuthority, PhysicalWorkAdmissionAuthority, PhysicalWorkIntent,
    PhysicalWorkSubmissionReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkPreEffectDenial {
    ForeignStore,
    ForeignRuntime,
    StaleGeneration,
    SignalProfileMismatch,
    UnhealthyServing,
    CapabilityAbsent,
    DependencyBlocked,
    CommandAbsent,
    SignalOwnerUnavailable,
    PhysicalAuthorityMismatch,
}

pub struct PhysicalWorkAdmission;

impl PhysicalWorkAdmission {
    pub(in crate::physical_runtime) fn admit(
        owner: &PhysicalWorkSubmissionOwner,
        receipt: PhysicalWorkSubmissionReceipt,
        physical: &PhysicalWorkAdmissionAuthority,
        health: &ServingHealth,
    ) -> Result<AdmittedPhysicalWork, PhysicalWorkPreEffectDenial> {
        let state = owner.state();
        let identity = receipt.identity();
        require_physical_authority(state, physical)?;
        require_current_identity(state, identity, receipt.signal_profile(), health)?;
        let (intent, capacity) = state
            .admit_declared(identity)
            .ok_or(PhysicalWorkPreEffectDenial::CommandAbsent)?;
        let binding = state
            .bindings()
            .binding_for_identity(intent.semantic_basis().aspect_identity())
            .filter(|binding| {
                binding.contract().binding_stamp() == intent.semantic_basis().binding_stamp()
                    && binding.serves_family(signal_family(intent.operation()))
            })
            .ok_or(PhysicalWorkPreEffectDenial::SignalProfileMismatch)?;
        let physical_authority =
            AdmittedPhysicalWorkAuthority::seal(&intent, binding.digest(), physical);
        Ok(AdmittedPhysicalWork::new(
            intent,
            physical_authority,
            capacity,
        ))
    }

    pub(in crate::physical_runtime) fn require_current(
        owner: &PhysicalWorkSubmissionOwner,
        intent: &PhysicalWorkIntent,
        health: &ServingHealth,
    ) -> Result<(), PhysicalWorkPreEffectDenial> {
        require_current_identity(
            owner.state(),
            intent.identity(),
            intent.signal_profile(),
            health,
        )
    }
}

fn require_physical_authority(
    state: &super::submission::PhysicalSubmissionState,
    physical: &PhysicalWorkAdmissionAuthority,
) -> Result<(), PhysicalWorkPreEffectDenial> {
    (physical.store() == state.store()
        && physical.runtime() == state.runtime()
        && physical.generation() == state.generation())
    .then_some(())
    .ok_or(PhysicalWorkPreEffectDenial::PhysicalAuthorityMismatch)
}

const fn signal_family(
    operation: super::PhysicalWorkOperationFamily,
) -> super::PhysicalWorkSignalFamily {
    match operation {
        super::PhysicalWorkOperationFamily::ArtifactRangeRead => {
            super::PhysicalWorkSignalFamily::ReadFault
        }
        super::PhysicalWorkOperationFamily::ArtifactRangeWrite => {
            super::PhysicalWorkSignalFamily::ExactWriteback
        }
        super::PhysicalWorkOperationFamily::ArtifactPublication => {
            super::PhysicalWorkSignalFamily::Publication
        }
    }
}

fn require_current_identity(
    state: &super::submission::PhysicalSubmissionState,
    identity: super::PhysicalWorkIdentity,
    signal_profile: super::PhysicalSignalProfileIdentity,
    health: &ServingHealth,
) -> Result<(), PhysicalWorkPreEffectDenial> {
    if identity.store() != state.store() {
        return Err(PhysicalWorkPreEffectDenial::ForeignStore);
    }
    if identity.runtime() != state.runtime() {
        return Err(PhysicalWorkPreEffectDenial::ForeignRuntime);
    }
    let lifecycle = state.lifecycle_snapshot();
    if identity.generation().lifecycle() != state.generation()
        || lifecycle.generation != state.generation()
        || lifecycle.phase != ObservedLifecyclePhase::RecordServing
    {
        return Err(PhysicalWorkPreEffectDenial::StaleGeneration);
    }
    if signal_profile != state.signal_profile() {
        return Err(PhysicalWorkPreEffectDenial::SignalProfileMismatch);
    }
    if health.requires_inspection() {
        return Err(PhysicalWorkPreEffectDenial::UnhealthyServing);
    }
    if !state.signal_available() {
        return Err(PhysicalWorkPreEffectDenial::SignalOwnerUnavailable);
    }
    Ok(())
}
