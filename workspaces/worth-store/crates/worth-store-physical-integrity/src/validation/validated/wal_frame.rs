use worth_store_physical_format::wal_frame::BoundedWalFrameV1;
use worth_store_physical_format::WalSegmentIdentity;

use super::super::{
    PhysicalArtifactScope, PhysicalIntegrityValidationDigest, PhysicalIntegrityValidationMechanism,
    PhysicalIntegrityValidationRecord, UntrustedPhysicalArtifact,
};

#[derive(Debug)]
pub struct IntegrityValidatedWalFrame<'media> {
    scope: PhysicalArtifactScope,
    lsn_start: u64,
    lsn_end: u64,
    identity_digest: [u8; 32],
    payload_digest: [u8; 32],
    payload: &'media [u8],
    validation_record: PhysicalIntegrityValidationRecord,
    inspected: UntrustedPhysicalArtifact<'media>,
}

impl<'media> IntegrityValidatedWalFrame<'media> {
    pub(crate) fn new(
        scope: PhysicalArtifactScope,
        decoded: BoundedWalFrameV1<'media>,
        inspected: UntrustedPhysicalArtifact<'media>,
    ) -> Option<Self> {
        let header = decoded.header();
        if !scope.is_wal_frame()
            || scope.wal_segment_identity()? != header.identity()
            || inspected.byte_count() != scope.byte_range().length()
        {
            return None;
        }
        let validation_record = PhysicalIntegrityValidationRecord::from_validated_scope(
            scope,
            PhysicalIntegrityValidationDigest::sha256(scope.exact_wal_scope_digest()),
            PhysicalIntegrityValidationDigest::sha256(decoded.frame_digest()),
            PhysicalIntegrityValidationMechanism::Sha256V1,
        )?;
        Some(Self {
            scope,
            lsn_start: header.lsn_start(),
            lsn_end: header.lsn_end(),
            identity_digest: header.identity_digest(),
            payload_digest: header.payload_digest(),
            payload: decoded.payload(),
            validation_record,
            inspected,
        })
    }

    pub const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }

    pub const fn segment_identity(&self) -> WalSegmentIdentity {
        match self.scope.wal_segment_identity() {
            Some(identity) => identity,
            None => unreachable!(),
        }
    }

    pub const fn lsn_start(&self) -> u64 {
        self.lsn_start
    }

    pub const fn lsn_end(&self) -> u64 {
        self.lsn_end
    }

    pub const fn identity_digest(&self) -> [u8; 32] {
        self.identity_digest
    }

    pub const fn payload_digest(&self) -> [u8; 32] {
        self.payload_digest
    }

    pub const fn payload(&self) -> &'media [u8] {
        self.payload
    }

    pub const fn into_validation_record(self) -> PhysicalIntegrityValidationRecord {
        self.validation_record
    }

    pub fn matches_input(&self, input: UntrustedPhysicalArtifact<'media>) -> bool {
        self.inspected.same_incarnation(input)
    }
}
