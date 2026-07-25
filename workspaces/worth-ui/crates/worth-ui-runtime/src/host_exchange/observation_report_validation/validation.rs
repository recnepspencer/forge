use worth_ui_host_contract::UiHostObservationBatch;

use super::basis_admission::UiBasisAdmittedObservationBatch;
use super::sequence_coverage::UiSequenceCoveredObservationBatch;
use super::state::UiHostObservationBatchFingerprint;
use super::structural_admission::UiStructurallyAdmittedObservationBatch;
use super::{
    UiDuplicateHostObservationBatch, UiHostObservationReportDenial, UiHostObservationReportOutcome,
    UiHostObservationReportValidation, UiQuarantinedHostObservationBatch,
};

pub(crate) struct UiHostObservationValidationContext<'a> {
    pub(crate) host_session: u64,
    pub(crate) protocol: worth_ui_host_contract::UiHostProtocolAgreement,
    pub(crate) retention: &'a crate::mounting::UiMountedFrameRetentionCoordinator,
    pub(crate) presentation: &'a crate::mounting::UiMountedPresentationCoordinator,
}

impl UiHostObservationReportValidation {
    pub(crate) fn validate(
        &mut self,
        batch: UiHostObservationBatch,
        context: UiHostObservationValidationContext<'_>,
    ) -> UiHostObservationReportOutcome {
        match self.try_validate(batch, context) {
            Ok(outcome) => outcome,
            Err(denial) => UiHostObservationReportOutcome::Denied(denial),
        }
    }

    fn try_validate(
        &mut self,
        batch: UiHostObservationBatch,
        context: UiHostObservationValidationContext<'_>,
    ) -> Result<UiHostObservationReportOutcome, UiHostObservationReportDenial> {
        if self.shutdown {
            return Err(UiHostObservationReportDenial::Shutdown);
        }
        let admitted = UiStructurallyAdmittedObservationBatch::admit(batch, context.protocol)?;
        let core = admitted.core();
        let covered = UiSequenceCoveredObservationBatch::prove(admitted)?;
        let integrity = covered.integrity();
        if self.is_rejected(core.frame()) {
            return Err(UiHostObservationReportDenial::RejectedFrame);
        }
        if self.is_never_presented(core.frame()) {
            return Err(UiHostObservationReportDenial::NeverPresentedFrame);
        }
        if self.is_indeterminate(core.frame(), core.binding()) {
            return self.quarantine(core, integrity);
        }
        let basis = UiBasisAdmittedObservationBatch::admit(
            covered,
            context.retention,
            context.host_session,
        )?;
        if context
            .presentation
            .binding_requires_reconciliation(core.binding())
        {
            return self.quarantine(core, integrity);
        }
        self.retain_covered_batch(basis)
    }

    fn quarantine(
        &mut self,
        core: worth_ui_host_contract::UiHostObservationCanonicalCore,
        integrity: worth_ui_host_contract::UiHostObservationIntegrity,
    ) -> Result<UiHostObservationReportOutcome, UiHostObservationReportDenial> {
        if self.quarantine_fingerprints.iter().any(|candidate| {
            candidate.sequences == core.sequences() && candidate.integrity == integrity
        }) {
            return Ok(UiHostObservationReportOutcome::Duplicate(
                UiDuplicateHostObservationBatch::new(core.sequences(), integrity),
            ));
        }
        if self.quarantine.len() >= self.capacity.quarantined_batches() {
            return Err(UiHostObservationReportDenial::QuarantineCapacityExceeded);
        }
        let quarantined = UiQuarantinedHostObservationBatch::new(core);
        self.quarantine.push_back(quarantined);
        self.quarantine_fingerprints
            .push_back(UiHostObservationBatchFingerprint {
                sequences: core.sequences(),
                integrity,
            });
        Ok(UiHostObservationReportOutcome::Quarantined(quarantined))
    }
}
