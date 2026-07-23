use std::sync::Arc;

use worth_store_physical_backend::MediaCounterSnapshot;
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::physical_runtime::{
    LifecycleGeneration, ObservationError, PhysicalMediaObserver, RecordServingObservationPhase,
    RuntimeIdentity,
};

use super::super::{
    lifecycle::record_lifecycle::{RecordServingCounterCells, RecordServingCounterSnapshot},
    AdmittedPhysicalRecordFormat, RecordPublicationResidueObservation,
};

pub struct PhysicalRecordObserver {
    media: PhysicalMediaObserver<RecordServingObservationPhase>,
    counters: Arc<RecordServingCounterCells>,
    runtime_identity: RuntimeIdentity,
    store_identity: StableStoreIdentity,
    format: AdmittedPhysicalRecordFormat,
    root_generation: u64,
    residue: RecordPublicationResidueObservation,
    acquisition_generation: LifecycleGeneration,
    acquisition_record_counters: RecordServingCounterSnapshot,
    acquisition_media_counters: MediaCounterSnapshot,
}

impl PhysicalRecordObserver {
    pub(in crate::physical_runtime::record_serving) fn new(
        media: PhysicalMediaObserver<RecordServingObservationPhase>,
        counters: Arc<RecordServingCounterCells>,
        format: AdmittedPhysicalRecordFormat,
        root_generation: u64,
        residue: RecordPublicationResidueObservation,
    ) -> Self {
        let acquisition_generation = media.observed_generation();
        let acquisition_record_counters = counters.snapshot();
        let acquisition_media_counters = media.media_counters();
        Self {
            runtime_identity: media.runtime_identity(),
            store_identity: media.store_identity(),
            media,
            counters,
            format,
            root_generation,
            residue,
            acquisition_generation,
            acquisition_record_counters,
            acquisition_media_counters,
        }
    }

    /// Returns the coherent record truth captured when this observer was acquired.
    ///
    /// Lifecycle validation is current, but record roots, residue, and counters all
    /// come from the same acquisition-time basis.
    pub fn acquisition_snapshot(&self) -> Result<PhysicalRecordObservation, ObservationError> {
        self.media.snapshot()?;
        Ok(PhysicalRecordObservation {
            runtime_identity: self.runtime_identity,
            store_identity: self.store_identity,
            format: self.format,
            root_generation: self.root_generation,
            residue: self.residue,
            lifecycle_generation: self.acquisition_generation,
            record_counters: self.acquisition_record_counters,
            media_counters: self.acquisition_media_counters,
        })
    }

    pub fn record_counters(&self) -> RecordServingCounterSnapshot {
        self.counters.snapshot()
    }

    pub fn media_counters(&self) -> MediaCounterSnapshot {
        self.media.media_counters()
    }

    pub fn media_snapshot(
        &self,
    ) -> Result<
        crate::physical_runtime::PhysicalMediaObservation<RecordServingObservationPhase>,
        ObservationError,
    > {
        self.media.snapshot()
    }
}

impl Clone for PhysicalRecordObserver {
    fn clone(&self) -> Self {
        Self {
            media: self.media.clone(),
            counters: Arc::clone(&self.counters),
            runtime_identity: self.runtime_identity,
            store_identity: self.store_identity,
            format: self.format,
            root_generation: self.root_generation,
            residue: self.residue,
            acquisition_generation: self.acquisition_generation,
            acquisition_record_counters: self.acquisition_record_counters,
            acquisition_media_counters: self.acquisition_media_counters,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordObservation {
    runtime_identity: RuntimeIdentity,
    store_identity: StableStoreIdentity,
    format: AdmittedPhysicalRecordFormat,
    root_generation: u64,
    residue: RecordPublicationResidueObservation,
    lifecycle_generation: LifecycleGeneration,
    record_counters: RecordServingCounterSnapshot,
    media_counters: MediaCounterSnapshot,
}

impl PhysicalRecordObservation {
    pub const fn runtime_identity(self) -> RuntimeIdentity {
        self.runtime_identity
    }
    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store_identity
    }
    pub const fn format(self) -> AdmittedPhysicalRecordFormat {
        self.format
    }
    pub const fn root_generation(self) -> u64 {
        self.root_generation
    }
    pub const fn residue(self) -> RecordPublicationResidueObservation {
        self.residue
    }
    pub const fn lifecycle_generation(self) -> LifecycleGeneration {
        self.lifecycle_generation
    }
    pub const fn record_counters(self) -> RecordServingCounterSnapshot {
        self.record_counters
    }
    pub const fn media_counters(self) -> MediaCounterSnapshot {
        self.media_counters
    }
}
