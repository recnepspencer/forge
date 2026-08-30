use worth_store::physical_runtime::ObservedWalArtifact;
use worth_store_physical_integrity::{IntegrityValidatedWalFrame, PhysicalByteRange};

use super::super::admitted_artifact::IntegrityAdmittedRecoveryArtifact;
use super::super::families::wal::IntegrityAdmittedWalFrame;
use super::super::{ObservedWalFrameSource, RecoveryIntegrityIngressCounters};
use super::{recorded, RecoveryIntegrityIngressAttempt};

impl<'media> IntegrityAdmittedRecoveryArtifact<'media> {
    pub(crate) fn bind_wal_frame(
        observed: &'media ObservedWalArtifact,
        relative_range: PhysicalByteRange,
        validated: IntegrityValidatedWalFrame<'media>,
        counters: &mut RecoveryIntegrityIngressCounters,
    ) -> RecoveryIntegrityIngressAttempt<'media> {
        let scope = validated.scope();
        recorded(
            scope,
            IntegrityAdmittedWalFrame::bind(
                ObservedWalFrameSource::new(observed, scope, relative_range),
                validated,
            )
            .map(Self::WalFrame),
            counters,
        )
    }
}
