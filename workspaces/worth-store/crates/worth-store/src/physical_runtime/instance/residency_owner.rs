use crate::physical_runtime::record_serving::RecordFramePorts;

/// Sole lifecycle owner for one Store instance's physical residency pool.
///
/// `RecordFramePorts` clones are capability facades for serving consumers.
/// They cannot replace this non-cloneable owner or suppress its terminal close.
pub(in crate::physical_runtime) struct PhysicalResidencyOwner {
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    admitted_policy: crate::physical_runtime::record_serving::AdmittedPhysicalRecordResidencyPolicy,
    ports: RecordFramePorts,
    closed: bool,
}

impl PhysicalResidencyOwner {
    pub(in crate::physical_runtime) fn admit(
        store: worth_store_physical_format::store_namespace::StableStoreIdentity,
        admitted_policy:
            crate::physical_runtime::record_serving::AdmittedPhysicalRecordResidencyPolicy,
    ) -> Result<Self, worth_store_buffer_pool::PhysicalResidencyDenial> {
        Ok(Self {
            store,
            admitted_policy,
            ports: RecordFramePorts::bounded(store, admitted_policy.limits())?,
            closed: false,
        })
    }

    pub(in crate::physical_runtime) const fn ports(&self) -> &RecordFramePorts {
        &self.ports
    }

    pub(in crate::physical_runtime) fn observation(
        &self,
        generation: crate::physical_runtime::LifecycleGeneration,
    ) -> crate::physical_runtime::record_serving::PhysicalResidencyObservation {
        crate::physical_runtime::record_serving::PhysicalResidencyObservation::new(
            self.store,
            generation,
            self.admitted_policy,
            self.ports.counters(),
            self.ports.allocation_events().snapshot(),
            self.ports.writeback_counters(),
        )
    }

    pub(in crate::physical_runtime) fn recovery_allocation_admission(
        &self,
    ) -> crate::physical_runtime::PhysicalRecoveryAllocationAdmission {
        crate::physical_runtime::PhysicalRecoveryAllocationAdmission::new(
            self.store,
            self.admitted_policy
                .scope_bytes(worth_store_buffer_pool::PhysicalOperationAllocationScope::Recovery),
        )
    }

    pub(in crate::physical_runtime) fn close(
        mut self,
    ) -> worth_store_buffer_pool::PhysicalResidencyShutdown {
        let shutdown = self.ports.close();
        self.closed = true;
        shutdown
    }
}

impl Drop for PhysicalResidencyOwner {
    fn drop(&mut self) {
        if !self.closed {
            let _shutdown = self.ports.close();
            self.closed = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::{NonZeroU32, NonZeroU64};

    use worth_store_buffer_pool::{
        PhysicalOperationAllocationScope, PhysicalResidencyDenial, PhysicalResidencyDimension,
    };
    use worth_store_physical_format::store_namespace::{
        ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
    };

    use super::PhysicalResidencyOwner;
    use crate::physical_runtime::record_serving::{
        AdmittedPhysicalRecordFormat, AdmittedPhysicalRecordResidencyPolicy,
        PhysicalRecordFormatDeclaration, PhysicalRecordResidencyPolicy,
        PhysicalSpeculativeWorkKind,
    };

    #[test]
    fn abandoned_owner_closes_surviving_facades_and_final_release_reconciles() {
        let owner = PhysicalResidencyOwner::admit(store(111), admitted_policy()).unwrap();
        let ports = owner.ports().clone();
        let observer = ports.allocation_events();
        drop(owner);

        assert_eq!(
            ports
                .begin_operation(
                    PhysicalOperationAllocationScope::ForegroundRead,
                    NonZeroU64::MIN,
                )
                .unwrap_err(),
            PhysicalResidencyDenial::PoolClosed,
        );
        drop(ports);
        for dimension in [
            PhysicalResidencyDimension::MetadataBytes,
            PhysicalResidencyDimension::TotalBytes,
        ] {
            assert_eq!(
                observer.snapshot().for_dimension(dimension).active_units(),
                0,
            );
        }
    }

    #[test]
    fn explicit_close_consumes_the_only_lifecycle_owner() {
        let owner = PhysicalResidencyOwner::admit(store(112), admitted_policy()).unwrap();
        let ports = owner.ports().clone();
        let shutdown = owner.close();
        assert!(!shutdown.requires_inspection());
        assert_eq!(
            ports
                .begin_operation(
                    PhysicalOperationAllocationScope::ForegroundRead,
                    NonZeroU64::MIN,
                )
                .unwrap_err(),
            PhysicalResidencyDenial::PoolClosed,
        );
    }

    #[test]
    fn explicit_close_classifies_live_facade_allocation_as_residue() {
        let owner = PhysicalResidencyOwner::admit(store(113), admitted_policy()).unwrap();
        let ports = owner.ports().clone();
        let allocation = ports
            .begin_operation(
                PhysicalOperationAllocationScope::ForegroundRead,
                NonZeroU64::MIN,
            )
            .unwrap();

        let shutdown = owner.close();
        assert!(shutdown.requires_inspection());
        assert!(shutdown.has_cancellable_work_residue());
        assert_eq!(shutdown.counters().active_operation_bytes(), 1);

        drop(allocation);
        assert_eq!(ports.counters().active_operation_bytes(), 0);
    }

    fn store(byte: u8) -> worth_store_physical_format::store_namespace::StableStoreIdentity {
        StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([byte; 16]).unwrap(),
        )
        .published_identity()
    }

    fn admitted_policy() -> AdmittedPhysicalRecordResidencyPolicy {
        use PhysicalOperationAllocationScope as Scope;
        use PhysicalSpeculativeWorkKind as Kind;

        let format = AdmittedPhysicalRecordFormat::admit(
            PhysicalRecordFormatDeclaration::builder().admit().unwrap(),
        );
        let page = u64::from(format.declaration().page_size().bytes());
        let mut builder = PhysicalRecordResidencyPolicy::builder()
            .total_bytes(nonzero(page * 8))
            .resident_bytes(nonzero(page * 4))
            .metadata_bytes(nonzero(page))
            .frame_entries(NonZeroU32::new(4).unwrap())
            .pinned_frames(NonZeroU32::new(4).unwrap())
            .pin_leases(NonZeroU32::new(4).unwrap())
            .dirty_frames(NonZeroU32::new(2).unwrap())
            .dirty_replacement_bytes(nonzero(page))
            .operation_bytes(nonzero(page));
        for scope in [
            Scope::ForegroundRead,
            Scope::ForegroundWrite,
            Scope::Recovery,
            Scope::Scrub,
            Scope::Maintenance,
            Scope::Verification,
            Scope::Blob,
        ] {
            builder = builder.scope_bytes(scope, nonzero(page));
        }
        for kind in [Kind::ReadAhead, Kind::Prefetch, Kind::WriteBehind] {
            builder = builder.speculative_frames(kind, NonZeroU32::new(2).unwrap());
        }
        builder.admit(format).into_result().unwrap()
    }

    fn nonzero(value: u64) -> NonZeroU64 {
        NonZeroU64::new(value).unwrap()
    }
}
