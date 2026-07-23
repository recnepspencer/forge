use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::DurablePhysicalRootManifest;

use super::super::{ExternalPhysicalRecordLocator, PhysicalRecordId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalLocatorReadmissionDenial {
    StoreIdentityMismatch,
    RecordNotFound,
    CurrentRootUnavailable,
}

pub type PhysicalLocatorReadmissionOutcome =
    worth_proof::DenialTransitionOutcome<PhysicalRecordId, PhysicalLocatorReadmissionDenial>;

pub(in crate::physical_runtime::record_serving) fn readmit_locator(
    media: &QualifiedFilesystemMedia,
    frame_load: &(dyn super::super::residency::frame_ports::FrameLoadPort + Send + Sync),
    format: super::super::AdmittedPhysicalRecordFormat,
    access: super::super::AdmittedRecordAccessPolicy,
    current_root: &DurablePhysicalRootManifest,
    health: &super::super::lifecycle::serving_health::ServingHealth,
    locator: ExternalPhysicalRecordLocator,
) -> PhysicalLocatorReadmissionOutcome {
    if locator.store_identity_bytes() != media.store_identity().bytes() {
        return worth_proof::TransitionOutcome::denied(
            PhysicalLocatorReadmissionDenial::StoreIdentityMismatch,
        );
    }
    let record = locator.readmitted_record_id();
    let mut counters =
        super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot::default();
    let found = super::super::access::manifest_routing::ManifestReader::with_loader(
        media,
        frame_load,
        format,
        access,
        current_root,
    )
    .locate(record.persisted(), &mut counters);
    let Ok(found) = found else {
        health.observe_read_denial(super::super::RecordReadDenial::ArtifactUnavailable);
        return worth_proof::TransitionOutcome::denied(
            PhysicalLocatorReadmissionDenial::CurrentRootUnavailable,
        );
    };
    if found.is_none() {
        return worth_proof::TransitionOutcome::denied(
            PhysicalLocatorReadmissionDenial::RecordNotFound,
        );
    }
    worth_proof::TransitionOutcome::success(record)
}
