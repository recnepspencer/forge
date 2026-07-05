use super::*;
use crate::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    PhysicalBackendCapabilityAdmissionAuthority,
};
use forge_store_aspect_native::StorePhysicalBoundaryWitness;
use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalReferenceAuthority, PhysicalSegmentId,
};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreKeyVersionPosture,
    StoreSecurityScopeIdentity, StoreTenantScope,
};
use std::cell::Cell;

struct RecordingBackend {
    appended: Vec<u8>,
    stored: Vec<u8>,
    appends: u32,
    reads: Cell<u32>,
    reference: PhysicalReference,
}

impl RecordingBackend {
    fn new(reference: PhysicalReference, stored: &[u8]) -> Self {
        Self {
            appended: Vec::new(),
            stored: stored.to_vec(),
            appends: 0,
            reads: Cell::new(0),
            reference,
        }
    }
}

impl PhysicalStoreBackend for RecordingBackend {
    type Error = ();

    fn append_framed_record(&mut self, bytes: &[u8]) -> Result<PhysicalReference, Self::Error> {
        self.appended = bytes.to_vec();
        self.appends += 1;
        Ok(self.reference)
    }

    fn read_framed_record(&self, reference: PhysicalReference) -> Result<Vec<u8>, Self::Error> {
        assert_eq!(reference, self.reference);
        self.reads.set(self.reads.get() + 1);
        Ok(self.stored.clone())
    }
}

#[test]
fn owned_backend_session_completes_after_real_append_with_observed_counters() {
    let reference = physical_reference(1);
    let witness = backend_witness(BackendTargetProfile::PosixFileFsyncDirSync);
    let binding = binding_for(&witness);
    let scope = speculative_scope();
    let observations = BackendQueueExecutionObservedCounters::new()
        .observe_queue_depth(3)
        .observe_read_ahead(1, scope)
        .observe_write_back(1, scope)
        .observe_mechanical_adaptation(2, 1, 1)
        .observe_backpressure(BackendQueueExecutionBackpressure::BackendTemporarilySaturated)
        .observe_foreground_wait_events(4);
    let mut backend = RecordingBackend::new(reference, b"readback");

    let (written_reference, completion) =
        BackendQueueExecutionSession::for_owned_backend(&mut backend)
            .complete_after_append(
                binding,
                &witness,
                BackendQueueExecutionAdaptation::RetryShortWrite,
                b"payload",
                observations,
            )
            .expect("owned backend session should complete append execution");

    assert_eq!(backend.appends, 1);
    assert_eq!(backend.appended, b"payload");
    assert_eq!(written_reference, reference);
    assert_eq!(completion.binding(), binding);
    assert_eq!(completion.posture().profile(), witness.profile());
    assert_eq!(
        completion.posture().evidence_class(),
        witness.evidence_class()
    );
    assert_eq!(
        completion.posture().adaptation(),
        BackendQueueExecutionAdaptation::RetryShortWrite
    );
    assert_eq!(completion.queue_depth_sample(), 3);
    assert_eq!(completion.read_ahead_units(), 1);
    assert_eq!(completion.read_ahead_scope(), Some(scope));
    assert_eq!(completion.write_back_units(), 1);
    assert_eq!(completion.write_back_scope(), Some(scope));
    assert_eq!(completion.mechanical_retries(), 2);
    assert_eq!(completion.partial_read_events(), 1);
    assert_eq!(completion.short_write_events(), 1);
    assert_eq!(
        completion.backpressure(),
        Some(BackendQueueExecutionBackpressure::BackendTemporarilySaturated)
    );
    assert_eq!(completion.foreground_wait_events(), 4);
}

#[test]
fn owned_backend_session_completes_after_real_read() {
    let reference = physical_reference(2);
    let witness = backend_witness(BackendTargetProfile::PosixFileFsyncDirSync);
    let binding = binding_for(&witness);
    let mut backend = RecordingBackend::new(reference, b"stored-bytes");

    let (bytes, completion) = BackendQueueExecutionSession::for_owned_backend(&mut backend)
        .complete_after_read(
            binding,
            &witness,
            BackendQueueExecutionAdaptation::RetryPartialRead,
            reference,
            BackendQueueExecutionObservedCounters::new().observe_queue_depth(1),
        )
        .expect("owned backend session should complete read execution");

    assert_eq!(bytes, b"stored-bytes");
    assert_eq!(backend.reads.get(), 1);
    assert_eq!(completion.binding(), binding);
    assert_eq!(
        completion.posture().adaptation(),
        BackendQueueExecutionAdaptation::RetryPartialRead
    );
    assert_eq!(completion.queue_depth_sample(), 1);
}

#[test]
fn owned_backend_session_reports_typed_ticket_denial() {
    let reference = physical_reference(3);
    let witness = backend_witness(BackendTargetProfile::PosixFileFsyncDirSync);
    let mismatched = backend_witness(BackendTargetProfile::SimulatedStrictDurable);
    let binding = binding_for(&mismatched);
    let mut backend = RecordingBackend::new(reference, b"unused");

    let error = BackendQueueExecutionSession::for_owned_backend(&mut backend)
        .complete_after_append(
            binding,
            &witness,
            BackendQueueExecutionAdaptation::None,
            b"payload",
            BackendQueueExecutionObservedCounters::new(),
        )
        .expect_err("profile mismatch must deny backend ticket authority");

    assert_eq!(
        error,
        BackendQueueExecutionRunError::TicketDenied(
            BackendQueueExecutionTicketDenial::BackendProfileMismatch
        )
    );
}

fn binding_for(witness: &AdmittedBackendCapabilityWitness) -> BackendQueueExecutionPlanBinding {
    BackendQueueExecutionPlanBinding::from_store_replay_binding(
        replay_binding(),
        None,
        witness.profile(),
        witness.evidence_class(),
        0,
    )
}

fn replay_binding() -> crate::BackendQueueExecutionReplayBinding {
    let scope = security_scope();
    crate::BackendQueueExecutionReplayBinding::from_store_queue_replay(
        1,
        1,
        1,
        scope,
        scope.tenant_scope(),
        scope.key_scope(),
        scope.authenticity_requirement(),
        7,
        0,
        0,
        crate::BackendQueueExecutionBudgetBinding::new(1, 4096, 0, 0, 1, 1, 0, 1, 1, 0),
    )
}

fn speculative_scope() -> BackendQueueSpeculativeScope {
    let scope = security_scope();
    BackendQueueSpeculativeScope::admitted(scope, scope.tenant_scope(), scope.key_scope())
}

fn security_scope() -> StoreSecurityScopeIdentity {
    let authority = StorePhysicalAuthorityWitness::for_aspect_native_boundary(
        ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
    )
    .expect("physical authority should admit");
    StoreSecurityScopeIdentity::from_physical_security_scope(
        StorePhysicalBoundaryWitness::from_physical_authority(authority)
            .expect("boundary witness should admit"),
        StoreKeyScope::PageEnvelope,
        StoreKeyVersionPosture::Current,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn backend_witness(profile: BackendTargetProfile) -> AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            profile,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_direct_io_alignment()
                .with_sector_atomicity()
                .with_page_cache_policy()
                .with_mmap_coherence()
                .with_async_ordering()
                .with_secure_frame_io()
                .with_flush_ordering()
                .with_fdatasync_durability(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("backend witness should admit")
}

fn physical_reference(slot: u16) -> PhysicalReference {
    let cell = PhysicalGenerationAuthority::s1()
        .slot_cell(
            PhysicalSegmentId::from_raw(1).unwrap(),
            PhysicalPageId::from_raw(1).unwrap(),
            PhysicalRecordSlot::from_raw(slot).unwrap(),
        )
        .with_slot_generation(PhysicalGeneration::from_raw(1).unwrap());
    PhysicalReferenceAuthority::s1()
        .admit_page_slot(cell)
        .reference()
}
