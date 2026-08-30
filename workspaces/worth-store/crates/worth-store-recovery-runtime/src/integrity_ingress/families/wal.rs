use std::ffi::OsStr;

use worth_store::physical_runtime::{
    IntegrityAdmittedRecoveryWalFrame as StoreAdmittedWalFrame, RecoveryWalIntegrityAdmissionDenial,
};
use worth_store_physical_format::{store_namespace::NamespaceEntryType, WalSegmentIdentity};
use worth_store_physical_integrity::IntegrityValidatedWalFrame;

use super::super::{
    ObservedWalFrameSource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedWalFrame<'media> {
    admitted: StoreAdmittedWalFrame,
    _source_lifetime: std::marker::PhantomData<&'media ()>,
}

pub(crate) struct WalFrameProjection<'view> {
    pub source_name: &'view OsStr,
    pub source_entry_type: NamespaceEntryType,
    pub segment_identity: WalSegmentIdentity,
    pub lsn_start: u64,
    pub lsn_end: u64,
    pub identity_digest: [u8; 32],
    pub payload_digest: [u8; 32],
    pub redo: IntegrityAdmittedWalRedo<'view>,
}

/// Opaque, source-borrowed redo content. It deliberately exposes no byte slice.
pub(crate) struct IntegrityAdmittedWalRedo<'media> {
    admitted: &'media StoreAdmittedWalFrame,
    digest: [u8; 32],
}

impl IntegrityAdmittedWalRedo<'_> {
    pub(crate) fn byte_count(&self) -> u64 {
        self.admitted.payload_byte_count()
    }

    pub(crate) const fn digest(&self) -> [u8; 32] {
        self.digest
    }

    pub(crate) const fn admitted_frame(&self) -> &StoreAdmittedWalFrame {
        self.admitted
    }
}

impl<'media> IntegrityAdmittedWalFrame<'media> {
    pub(in crate::integrity_ingress) fn bind(
        owner: &worth_store::physical_runtime::PhysicalRecoveryCoordination,
        source: ObservedWalFrameSource<'media>,
        validated: IntegrityValidatedWalFrame<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        let scope = source.scope();
        let admitted = owner
            .admit_recovery_wal_frame(source.observed(), scope, source.relative_range(), validated)
            .map_err(map_store_denial)?;
        Ok(Self {
            admitted,
            _source_lifetime: std::marker::PhantomData,
        })
    }

    pub(crate) fn project<'view>(
        &'view self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> WalFrameProjection<'view> {
        counters.record_owner_projection();
        WalFrameProjection {
            source_name: self.admitted.source_name(),
            source_entry_type: self.admitted.source_entry_type(),
            segment_identity: self.admitted.segment_identity(),
            lsn_start: self.admitted.lsn_start(),
            lsn_end: self.admitted.lsn_end(),
            identity_digest: self.admitted.identity_digest(),
            payload_digest: self.admitted.payload_digest(),
            redo: IntegrityAdmittedWalRedo {
                admitted: &self.admitted,
                digest: self.admitted.payload_digest(),
            },
        }
    }

    pub(crate) const fn scope(&self) -> worth_store_physical_integrity::PhysicalArtifactScope {
        self.admitted.scope()
    }

    pub(crate) fn into_store_admission(self) -> StoreAdmittedWalFrame {
        self.admitted
    }
}

fn map_store_denial(
    denial: RecoveryWalIntegrityAdmissionDenial,
) -> RecoveryIntegrityIngressRejection {
    match denial {
        RecoveryWalIntegrityAdmissionDenial::MissingBoundedArtifact => {
            RecoveryIntegrityIngressRejection::MissingBoundedArtifact
        }
        RecoveryWalIntegrityAdmissionDenial::ScopeMismatch => {
            RecoveryIntegrityIngressRejection::ScopeMismatch
        }
        RecoveryWalIntegrityAdmissionDenial::SourceRangeOutsideObservation => {
            RecoveryIntegrityIngressRejection::SourceRangeOutsideObservation
        }
        RecoveryWalIntegrityAdmissionDenial::SourceIncarnationMismatch => {
            RecoveryIntegrityIngressRejection::SourceIncarnationMismatch
        }
    }
}

#[cfg(test)]
pub(super) fn owner_valid_compile_contract() {
    fn bind<'media>(
        source: ObservedWalFrameSource<'media>,
        validated: IntegrityValidatedWalFrame<'media>,
    ) {
        let _ = (source, validated);
    }
    let _ = bind;
}
