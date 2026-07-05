use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    CapabilityEvidenceClass, PhysicalBackendCapabilityAdmissionAuthority,
    PhysicalStoreDurabilityExecutor, StoreDurabilityAdmission, StoreDurabilityBoundaryReached,
    StoreDurabilityDenial, StoreDurabilityExecutionObservation, StoreDurabilityExecutionRequest,
    StoreDurabilityExecutionSession, StoreDurabilityFileSyncKind, StoreDurabilityPublicationKind,
    StoreDurabilityRequirement, StoreDurabilityWriteAccepted,
};

pub(super) fn admitted(requirement: StoreDurabilityRequirement) -> StoreDurabilityAdmission {
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap();
    StoreDurabilityAdmission::admit(requirement, &witness).unwrap()
}

pub(super) fn reach_boundary<S>(
    accepted: StoreDurabilityWriteAccepted<S>,
    sync: StoreDurabilityFileSyncKind,
    directory_sync_completed: bool,
    rename_completed: bool,
    ordering_barrier_completed: bool,
) -> Result<StoreDurabilityBoundaryReached<S>, StoreDurabilityDenial>
where
    S: Clone + Eq + core::fmt::Debug,
{
    let expected_scope = accepted.scope().clone();
    let expected_requirement = accepted.requirement();
    let expected_publication = expected_requirement.publication();
    let observation = execution_observation(
        accepted.requirement(),
        sync,
        directory_sync_completed,
        rename_completed,
        ordering_barrier_completed,
    );
    let mut backend = RequestAssertingDurabilityBackend {
        expected_scope,
        expected_requirement,
        expected_publication,
        observation,
    };
    let proof = StoreDurabilityExecutionSession::for_store_backend(
        &mut backend,
        forge_store_physical_backend::StoreOwnedDurabilityExecution::for_certification_test_authority(),
    )
    .execute(&accepted)
    .unwrap();
    accepted.reach_durability_boundary(proof)
}

fn execution_observation(
    requirement: StoreDurabilityRequirement,
    sync: StoreDurabilityFileSyncKind,
    directory_sync_completed: bool,
    rename_completed: bool,
    ordering_barrier_completed: bool,
) -> StoreDurabilityExecutionObservation {
    let mut observation =
        StoreDurabilityExecutionObservation::new(requirement.required_barriers(), sync);
    if directory_sync_completed {
        observation = observation.with_directory_sync_completed();
    }
    if rename_completed {
        observation = observation.with_rename_completed();
    }
    if ordering_barrier_completed {
        observation = observation.with_ordering_barrier_completed();
    }
    observation
}

struct RequestAssertingDurabilityBackend<S> {
    expected_scope: S,
    expected_requirement: StoreDurabilityRequirement,
    expected_publication: StoreDurabilityPublicationKind,
    observation: StoreDurabilityExecutionObservation,
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
        Ok(self.observation)
    }
}
