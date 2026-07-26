use worth_store_physical_format::CurrentPhysicalRecordPlacement;

use super::super::{
    ExternalPhysicalRecordLocator, PhysicalRecordId, PhysicalRecordReader, RecordReadDenial,
    RecordReadObservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalLocatorReadmissionDenial {
    ServingRequiresInspection,
    StoreIdentityMismatch,
    RecordNotFound,
    CurrentRootUnavailable,
}

pub type PhysicalLocatorReadmissionOutcome =
    worth_proof::DenialTransitionOutcome<PhysicalRecordId, PhysicalLocatorReadmissionDenial>;

pub(in crate::physical_runtime::record_serving) struct DetailedLocatorReadmission {
    record: PhysicalRecordId,
    placement: CurrentPhysicalRecordPlacement,
    observation: RecordReadObservation,
}

pub(in crate::physical_runtime::record_serving) struct DetailedLocatorReadmissionFailure {
    denial: PhysicalLocatorReadmissionDenial,
    read_denial: RecordReadDenial,
    observation: RecordReadObservation,
}

impl DetailedLocatorReadmission {
    pub(super) const fn record(&self) -> PhysicalRecordId {
        self.record
    }

    pub(super) const fn placement(&self) -> CurrentPhysicalRecordPlacement {
        self.placement
    }

    pub(super) const fn observation(&self) -> RecordReadObservation {
        self.observation
    }
}

impl DetailedLocatorReadmissionFailure {
    pub(super) const fn read_denial(&self) -> RecordReadDenial {
        self.read_denial
    }

    pub(super) const fn observation(&self) -> RecordReadObservation {
        self.observation
    }
}

pub(in crate::physical_runtime::record_serving) fn readmit_locator(
    reader: &PhysicalRecordReader,
    locator: ExternalPhysicalRecordLocator,
) -> PhysicalLocatorReadmissionOutcome {
    let allocation = match reader.begin_read_allocation(RecordReadObservation::default()) {
        Ok(allocation) => allocation,
        Err(_) => {
            return worth_proof::TransitionOutcome::denied(
                PhysicalLocatorReadmissionDenial::CurrentRootUnavailable,
            )
        }
    };
    match readmit_locator_detailed(reader, &allocation, locator) {
        Ok(readmitted) => worth_proof::TransitionOutcome::success(readmitted.record),
        Err(failure) => worth_proof::TransitionOutcome::denied(failure.denial),
    }
}

pub(in crate::physical_runtime::record_serving) fn readmit_locator_detailed(
    reader: &PhysicalRecordReader,
    allocation: &worth_store_buffer_pool::OperationAllocationGrant,
    locator: ExternalPhysicalRecordLocator,
) -> Result<DetailedLocatorReadmission, DetailedLocatorReadmissionFailure> {
    let mut observation = RecordReadObservation::default();
    let Some(runtime) = reader.runtime.upgrade() else {
        return Err(failure(
            PhysicalLocatorReadmissionDenial::ServingRequiresInspection,
            RecordReadDenial::ServingRequiresInspection,
            observation,
        ));
    };
    if runtime.health.permit().is_err() {
        return Err(failure(
            PhysicalLocatorReadmissionDenial::ServingRequiresInspection,
            RecordReadDenial::ServingRequiresInspection,
            observation,
        ));
    }
    if locator.store_identity_bytes() != reader.store.bytes() {
        return Err(failure(
            PhysicalLocatorReadmissionDenial::StoreIdentityMismatch,
            RecordReadDenial::StoreIdentityMismatch,
            observation,
        ));
    }
    let record = locator.readmitted_record_id();
    let mut counters =
        super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot::default();
    let found = super::super::access::manifest_routing::ManifestReader::serving(
        reader.residency.clone(),
        reader.format,
        reader.access,
        reader.current_root.clone(),
    )
    .locate(allocation, record.persisted(), &mut counters);
    observation.observe_manifest(counters);
    let found = found.map_err(|reason| {
        let read_denial = super::locate::failure_classification::manifest_failure(reason);
        runtime.health.observe_read_denial(read_denial);
        failure(
            PhysicalLocatorReadmissionDenial::CurrentRootUnavailable,
            read_denial,
            observation,
        )
    })?;
    let Some(placement) = found else {
        return Err(failure(
            PhysicalLocatorReadmissionDenial::RecordNotFound,
            RecordReadDenial::RecordNotFound,
            observation,
        ));
    };
    Ok(DetailedLocatorReadmission {
        record,
        placement,
        observation,
    })
}

const fn failure(
    denial: PhysicalLocatorReadmissionDenial,
    read_denial: RecordReadDenial,
    observation: RecordReadObservation,
) -> DetailedLocatorReadmissionFailure {
    DetailedLocatorReadmissionFailure {
        denial,
        read_denial,
        observation,
    }
}
