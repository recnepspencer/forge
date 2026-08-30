use std::ffi::OsStr;

use worth_store::physical_runtime::recovery_wal::{LogSequenceNumber, WalLsnRange};
use worth_store_physical_format::{store_namespace::NamespaceEntryType, WalSegmentIdentity};
use worth_store_physical_integrity::{IntegrityValidatedWalFrame, WalPayloadProjectionDenial};
use worth_store_recovery_physics::{
    decode_physical_redo_records, PhysicalRedoPlanningDenial, PhysicalRedoRecord,
};

use super::super::admission::require_observed_wal_source;
use super::super::{
    ObservedWalFrameSource, RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};

pub(crate) struct IntegrityAdmittedWalFrame<'media> {
    source: ObservedWalFrameSource<'media>,
    validated: IntegrityValidatedWalFrame<'media>,
    payload: &'media [u8],
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
    lsn_start: u64,
    lsn_end: u64,
}

impl IntegrityAdmittedWalRedo<'_> {
    pub(crate) fn byte_count(&self) -> u64 {
        self.payload.len() as u64
    }

    pub(crate) const fn digest(&self) -> [u8; 32] {
        self.digest
    }

    pub(crate) fn interpret(
        &self,
        maximum_targets: u64,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> Result<Box<[PhysicalRedoRecord]>, PhysicalRedoPlanningDenial> {
        counters.record_owner_decoder();
        let range = WalLsnRange::new(
            LogSequenceNumber::new(self.lsn_start),
            LogSequenceNumber::new(self.lsn_end),
        )
        .map_err(|_| PhysicalRedoPlanningDenial::LsnRangeMismatch)?;
        decode_physical_redo_records(self.payload, range, maximum_targets)
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
        let input = source.input()?;
        let projection = validated
            .project_payload(input, validated.segment_identity())
            .map_err(|denial| match denial {
                WalPayloadProjectionDenial::InputIncarnationMismatch => {
                    RecoveryIntegrityIngressRejection::SourceIncarnationMismatch
                }
                WalPayloadProjectionDenial::SegmentIdentityMismatch => {
                    RecoveryIntegrityIngressRejection::ScopeMismatch
                }
            })?;
        let payload = input
            .bytes()
            .get(projection.payload_range())
            .ok_or(RecoveryIntegrityIngressRejection::SourceRangeOutsideObservation)?;
        Ok(Self {
            source,
            validated,
            payload,
        })
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
                payload: self.payload,
                digest: self.validated.payload_digest(),
                lsn_start: self.validated.lsn_start(),
                lsn_end: self.validated.lsn_end(),
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
