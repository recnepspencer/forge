use super::execution::{
    PhysicalStoreDurabilityExecutor, StoreDurabilityExecutionObservation,
    StoreDurabilityExecutionRequest, StoreDurabilityExecutionSession,
};
use crate::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    CapabilityEvidenceClass, PhysicalBackendCapabilityAdmissionAuthority,
    StoreDurabilityExecutionProof, StoreDurabilityFileSyncKind, StoreDurabilityPublicationKind,
    StoreDurabilityRequirement, StoreDurabilityWriteAccepted, WalDurabilityBarrierSet,
};

pub(super) fn execution_proof(
    accepted: &StoreDurabilityWriteAccepted<&'static str>,
    sync: StoreDurabilityFileSyncKind,
    directory_sync_completed: bool,
    rename_completed: bool,
    ordering_barrier_completed: bool,
) -> StoreDurabilityExecutionProof<&'static str> {
    let mut observation =
        StoreDurabilityExecutionObservation::new(accepted.requirement().required_barriers(), sync);
    if directory_sync_completed {
        observation = observation.with_directory_sync_completed();
    }
    if rename_completed {
        observation = observation.with_rename_completed();
    }
    if ordering_barrier_completed {
        observation = observation.with_ordering_barrier_completed();
    }
    scripted_execution_proof(accepted, observation)
}

pub(super) fn scripted_execution_proof<S>(
    accepted: &StoreDurabilityWriteAccepted<S>,
    observation: StoreDurabilityExecutionObservation,
) -> StoreDurabilityExecutionProof<S>
where
    S: Clone,
    ScriptedDurabilityBackend: PhysicalStoreDurabilityExecutor<S, Error = ()>,
{
    let mut backend = ScriptedDurabilityBackend {
        observation: observation.with_persisted_artifact(
            std::path::PathBuf::from("scripted-durability-artifact"),
            0,
            1,
        ),
    };
    match StoreDurabilityExecutionSession::for_owned_backend(&mut backend).execute(accepted) {
        Ok(proof) => proof,
        Err(()) => panic!("scripted durability backend should not fail"),
    }
}

pub(super) struct ScriptedDurabilityBackend {
    observation: StoreDurabilityExecutionObservation,
}

impl<S> PhysicalStoreDurabilityExecutor<S> for ScriptedDurabilityBackend {
    type Error = ();

    fn execute_durability(
        &mut self,
        request: StoreDurabilityExecutionRequest<S>,
    ) -> Result<StoreDurabilityExecutionObservation, Self::Error> {
        assert_ne!(
            request.requirement().required_barriers(),
            WalDurabilityBarrierSet::EMPTY
        );
        Ok(self.observation.clone())
    }
}

pub(super) struct RequestAssertingDurabilityBackend<S> {
    pub(super) expected_scope: S,
    pub(super) expected_requirement: StoreDurabilityRequirement,
    pub(super) expected_publication: StoreDurabilityPublicationKind,
    pub(super) observation: StoreDurabilityExecutionObservation,
}

impl<S> PhysicalStoreDurabilityExecutor<S> for RequestAssertingDurabilityBackend<S>
where
    S: Eq + core::fmt::Debug,
{
    type Error = ();

    fn execute_durability(
        &mut self,
        request: StoreDurabilityExecutionRequest<S>,
    ) -> Result<StoreDurabilityExecutionObservation, Self::Error> {
        assert_eq!(request.scope(), &self.expected_scope);
        assert_eq!(
            request.profile(),
            BackendTargetProfile::PosixFileFsyncDirSync
        );
        assert_eq!(
            request.evidence_class(),
            CapabilityEvidenceClass::CertifiedBackendProfile
        );
        assert_eq!(request.requirement(), self.expected_requirement);
        assert_eq!(
            request.requirement().publication(),
            self.expected_publication
        );
        Ok(self.observation.clone())
    }
}

pub(super) fn witness(
    basis: BackendCapabilityEvidenceBasis,
    support: BackendCapabilitySupportSet,
    media: BackendMediaAssumptionSet,
) -> crate::AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            basis,
            support,
            media,
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap()
}
