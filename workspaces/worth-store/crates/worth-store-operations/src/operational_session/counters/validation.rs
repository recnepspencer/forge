use super::{OperationalCounterReceipt, OperationalCounterStructureDenial};
use crate::{OperationalSessionDisposition, OperationalSessionKind};

impl OperationalCounterReceipt {
    pub fn validate_structure(self) -> Result<(), OperationalCounterStructureDenial> {
        if self.work_units == 0 {
            return Err(OperationalCounterStructureDenial::EmptyWork);
        }
        self.require_streaming_breadth()?;
        self.require_authority_counts()?;
        self.require_owner_receipts()?;
        Ok(())
    }

    fn require_streaming_breadth(self) -> Result<(), OperationalCounterStructureDenial> {
        if self.disposition == OperationalSessionDisposition::Abandoned {
            return Ok(());
        }
        let reconstructive = matches!(
            self.kind,
            OperationalSessionKind::Backup
                | OperationalSessionKind::Restore
                | OperationalSessionKind::PointInTimeRecovery
                | OperationalSessionKind::Rollback
                | OperationalSessionKind::ReplicaBootstrap
                | OperationalSessionKind::ForensicAcquisition
        );
        let inspecting = self.kind == OperationalSessionKind::OfflineVerification;
        if (reconstructive || inspecting) && self.source_bytes_read == 0 {
            return Err(OperationalCounterStructureDenial::MissingStreamingBreadth);
        }
        if reconstructive && self.output_bytes_written == 0 {
            return Err(OperationalCounterStructureDenial::MissingStreamingBreadth);
        }
        if (reconstructive || inspecting) && self.maximum_resident_bytes == 0 {
            return Err(OperationalCounterStructureDenial::MissingResidentBound);
        }
        Ok(())
    }

    fn require_authority_counts(self) -> Result<(), OperationalCounterStructureDenial> {
        let expected_authorizations = u64::from(matches!(
            self.kind,
            OperationalSessionKind::Restore
                | OperationalSessionKind::PointInTimeRecovery
                | OperationalSessionKind::Rollback
                | OperationalSessionKind::Repair
                | OperationalSessionKind::ReplicaBootstrap
                | OperationalSessionKind::ReplicaPromotion
        ));
        if self.authorization_consumptions != expected_authorizations {
            return Err(OperationalCounterStructureDenial::InvalidAuthorizationCount);
        }
        let expected_fences = u64::from(self.kind == OperationalSessionKind::ReplicaPromotion);
        if self.external_fence_grants != expected_fences {
            return Err(OperationalCounterStructureDenial::InvalidFenceCount);
        }
        let expected_leases = u64::from(matches!(
            self.kind,
            OperationalSessionKind::Backup
                | OperationalSessionKind::PointInTimeRecovery
                | OperationalSessionKind::Rollback
                | OperationalSessionKind::ReplicaBootstrap
        ));
        if self.retained_source_leases != expected_leases {
            return Err(OperationalCounterStructureDenial::InvalidLeaseCount);
        }
        Ok(())
    }

    fn require_owner_receipts(self) -> Result<(), OperationalCounterStructureDenial> {
        let valid = match self.kind {
            OperationalSessionKind::OfflineVerification => self.owner_receipts == 0,
            OperationalSessionKind::Restore
            | OperationalSessionKind::PointInTimeRecovery
            | OperationalSessionKind::Rollback
            | OperationalSessionKind::ReplicaPromotion => self.owner_receipts == 2,
            OperationalSessionKind::Backup | OperationalSessionKind::ReplicaBootstrap => {
                self.owner_receipts == 1
            }
            OperationalSessionKind::Repair | OperationalSessionKind::ForensicAcquisition => {
                self.owner_receipts > 0
            }
        };
        if !valid {
            return Err(OperationalCounterStructureDenial::InvalidOwnerReceiptCount);
        }
        Ok(())
    }
}
