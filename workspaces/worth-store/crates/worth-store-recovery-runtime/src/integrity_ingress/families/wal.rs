use std::ffi::OsStr;

use worth_store_physical_format::{store_namespace::NamespaceEntryType, WalSegmentIdentity};
use worth_store_physical_integrity::IntegrityValidatedWalFrame;

use super::super::admission::require_observed_wal_source;
use super::super::{
    ObservedWalFrameSource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedWalFrame<'media> {
    source: ObservedWalFrameSource<'media>,
    validated: IntegrityValidatedWalFrame<'media>,
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
    payload: &'media [u8],
    digest: [u8; 32],
}

impl IntegrityAdmittedWalRedo<'_> {
    pub(crate) fn byte_count(&self) -> u64 {
        self.payload.len() as u64
    }

    pub(crate) const fn digest(&self) -> [u8; 32] {
        self.digest
    }
}

impl<'media> IntegrityAdmittedWalFrame<'media> {
    pub(in crate::integrity_ingress) fn bind(
        source: ObservedWalFrameSource<'media>,
        validated: IntegrityValidatedWalFrame<'media>,
    ) -> Result<Self, RecoveryIntegrityIngressRejection> {
        require_observed_wal_source(&source, validated.scope(), |input| {
            validated.matches_input(input)
        })?;
        Ok(Self { source, validated })
    }

    pub(crate) fn project<'view>(
        &'view self,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> WalFrameProjection<'view> {
        counters.record_owner_projection();
        WalFrameProjection {
            source_name: self.source.name(),
            source_entry_type: self.source.entry_type(),
            segment_identity: self.validated.segment_identity(),
            lsn_start: self.validated.lsn_start(),
            lsn_end: self.validated.lsn_end(),
            identity_digest: self.validated.identity_digest(),
            payload_digest: self.validated.payload_digest(),
            redo: IntegrityAdmittedWalRedo {
                payload: self.validated.payload(),
                digest: self.validated.payload_digest(),
            },
        }
    }

    pub(crate) const fn scope(&self) -> worth_store_physical_integrity::PhysicalArtifactScope {
        self.source.scope()
    }
}

#[cfg(test)]
pub(super) fn owner_valid_compile_contract() {
    fn bind<'media>(
        source: ObservedWalFrameSource<'media>,
        validated: IntegrityValidatedWalFrame<'media>,
    ) {
        let _ = IntegrityAdmittedWalFrame::bind(source, validated);
    }
    let _ = bind;
}
