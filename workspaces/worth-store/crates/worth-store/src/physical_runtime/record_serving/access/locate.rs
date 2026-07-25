use std::ops::Range;

use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::{CurrentPhysicalRecordPlacement, DurablePhysicalRootManifest};

use super::super::{
    access::extent_read_session::ExtentReadState,
    residency::{
        frame_loading::{CanonicalFrameReadSource, LoadedPhysicalFrame},
        frame_ports::RecordFramePorts,
    },
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, ExternalPhysicalRecordLocator,
    PhysicalLocatorReadmissionOutcome, PhysicalRecordId, RecordReadDenial, RecordReadError,
    RecordReadLimits, RecordReadObservation,
};

mod cancellation;
mod extent;
#[path = "locate/failure_classification.rs"]
pub(super) mod failure_classification;
mod inline;
mod session;
pub use cancellation::RecordReadCancellation;
use failure_classification::manifest_failure;

enum ReadPlacement {
    Inline {
        frame: LoadedPhysicalFrame,
        payload: Range<usize>,
        offset: usize,
    },
    Extent(Box<ExtentReadState>),
}

pub struct RecordReadSession {
    placement: ReadPlacement,
    observation: RecordReadObservation,
    runtime: std::sync::Weak<crate::physical_runtime::instance::PhysicalStoreWorkRuntime>,
    health_permit: super::super::lifecycle::serving_health::ServingHealthPermit,
    _lifecycle: super::super::lifecycle::record_lifecycle::RecordReadSessionLease,
    _allocation: worth_store_buffer_pool::OperationAllocationGrant,
}

pub type OpenedPhysicalRecord = RecordReadSession;

pub struct PhysicalRecordReader {
    pub(in crate::physical_runtime::record_serving) store: StableStoreIdentity,
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) current_root: DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) runtime:
        std::sync::Weak<crate::physical_runtime::instance::PhysicalStoreWorkRuntime>,
    pub(in crate::physical_runtime::record_serving) lifecycle:
        super::super::lifecycle::record_lifecycle::RecordReaderLease,
    pub(in crate::physical_runtime::record_serving) frame_ports: RecordFramePorts,
    pub(in crate::physical_runtime::record_serving) source: CanonicalFrameReadSource,
}

impl PhysicalRecordReader {
    pub fn open(
        &self,
        record: PhysicalRecordId,
        limits: RecordReadLimits,
    ) -> Result<RecordReadSession, RecordReadError> {
        let mut observation = RecordReadObservation::default();
        let runtime = self.runtime.upgrade().ok_or_else(|| {
            RecordReadError::new(RecordReadDenial::ServingRequiresInspection, observation)
        })?;
        runtime.health.permit().map_err(|_| {
            RecordReadError::new(RecordReadDenial::ServingRequiresInspection, observation)
        })?;
        let allocation = self.begin_read_allocation(observation)?;
        let mut discovery =
            super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot::default();
        let placement = super::super::access::manifest_routing::ManifestReader::serving(
            self.frame_ports.clone(),
            self.source.clone(),
            self.format,
            self.access,
            self.current_root.clone(),
        )
        .locate(record.persisted(), &mut discovery);
        observation.observe_manifest(discovery);
        let placement =
            placement.map_err(|failure| self.read_error(manifest_failure(failure), observation))?;
        let placement = placement
            .ok_or_else(|| RecordReadError::new(RecordReadDenial::RecordNotFound, observation))?;
        self.require_caller_limit(placement.payload_bytes(), limits, &mut observation)?;
        self.open_known_placement_with_allocation(record, placement, observation, allocation)
    }

    pub(in crate::physical_runtime::record_serving) fn open_known_placement(
        &self,
        record: PhysicalRecordId,
        placement: CurrentPhysicalRecordPlacement,
        limits: RecordReadLimits,
        mut observation: RecordReadObservation,
    ) -> Result<RecordReadSession, RecordReadError> {
        self.require_caller_limit(placement.payload_bytes(), limits, &mut observation)?;
        let allocation = self.begin_read_allocation(observation)?;
        self.open_known_placement_with_allocation(record, placement, observation, allocation)
    }

    fn begin_read_allocation(
        &self,
        observation: RecordReadObservation,
    ) -> Result<worth_store_buffer_pool::OperationAllocationGrant, RecordReadError> {
        self.frame_ports
            .begin_operation(
                worth_store_buffer_pool::OperationAllocationScope::ForegroundRead,
                u64::from(self.format.declaration().page_size().bytes()),
            )
            .map_err(|reason| {
                RecordReadError::new(RecordReadDenial::ResidencyUnavailable(reason), observation)
            })
    }

    fn require_caller_limit(
        &self,
        payload_bytes: u64,
        limits: RecordReadLimits,
        observation: &mut RecordReadObservation,
    ) -> Result<(), RecordReadError> {
        observation.requested_bytes = payload_bytes;
        if payload_bytes > u64::from(limits.maximum_payload.get()) {
            return Err(RecordReadError::new(
                RecordReadDenial::CallerLimitExceeded,
                *observation,
            ));
        }
        Ok(())
    }

    fn open_known_placement_with_allocation(
        &self,
        record: PhysicalRecordId,
        placement: CurrentPhysicalRecordPlacement,
        mut observation: RecordReadObservation,
        allocation: worth_store_buffer_pool::OperationAllocationGrant,
    ) -> Result<RecordReadSession, RecordReadError> {
        let runtime = self.runtime.upgrade().ok_or_else(|| {
            RecordReadError::new(RecordReadDenial::ServingRequiresInspection, observation)
        })?;
        runtime.health.permit().map_err(|_| {
            RecordReadError::new(RecordReadDenial::ServingRequiresInspection, observation)
        })?;
        let result = match placement {
            CurrentPhysicalRecordPlacement::Inline(value) => {
                self.open_inline(record, value, &mut observation, allocation)
            }
            CurrentPhysicalRecordPlacement::Extent(value) => {
                self.open_extent(record, value, &mut observation, allocation)
            }
        };
        result.map_err(|denial| self.read_error(denial, observation))
    }

    pub fn readmit_locator(
        &self,
        locator: ExternalPhysicalRecordLocator,
    ) -> PhysicalLocatorReadmissionOutcome {
        super::super::access::readmission::readmit_locator(self, locator)
    }

    pub fn open_external(
        &self,
        locator: ExternalPhysicalRecordLocator,
        limits: RecordReadLimits,
    ) -> Result<RecordReadSession, RecordReadError> {
        let readmitted = super::super::access::readmission::readmit_locator_detailed(self, locator)
            .map_err(|failure| self.read_error(failure.read_denial(), failure.observation()))?;
        self.open_known_placement(
            readmitted.record(),
            readmitted.placement(),
            limits,
            readmitted.observation(),
        )
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    fn read_error(
        &self,
        denial: RecordReadDenial,
        observation: RecordReadObservation,
    ) -> RecordReadError {
        if let Some(runtime) = self.runtime.upgrade() {
            runtime.health.observe_read_denial(denial);
        }
        RecordReadError::new(denial, observation)
    }
}
