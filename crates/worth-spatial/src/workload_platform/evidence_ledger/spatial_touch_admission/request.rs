use super::authority::admit_spatial_geometry_evidence_touch_authority;
use super::counter_honesty::spatial_touch_counter_honesty;
use super::{
    SpatialGeometryEvidenceTouchAuthority, SpatialGeometryEvidenceTouchDenial,
    SpatialGeometryEvidenceTouchReceiptOnlyPreview,
};
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedgerError,
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceSupport,
};

pub struct SpatialGeometryEvidenceTouchRequest<'a, T: BooleanEvidenceReceipt + 'static> {
    receipt: &'a T,
}

pub(crate) struct SpatialGeometryEvidenceTouchRowRequest<'a> {
    row: &'a WorkloadEvidenceRow,
}

pub struct SpatialGeometryEvidenceTouchAdmissionInput<'a, T: BooleanEvidenceReceipt + 'static> {
    request: SpatialGeometryEvidenceTouchRequest<'a, T>,
    ledger: &'a CompleteWorkloadEvidenceLedger,
}

pub(crate) struct SpatialGeometryEvidenceTouchRowAdmissionInput<'a> {
    request: SpatialGeometryEvidenceTouchRowRequest<'a>,
    ledger: &'a CompleteWorkloadEvidenceLedger,
}

impl<'a, T: BooleanEvidenceReceipt + 'static> SpatialGeometryEvidenceTouchRequest<'a, T> {
    pub fn from_boolean_receipt(receipt: &'a T) -> Self {
        Self { receipt }
    }

    pub fn receipt_only_preview(&self) -> SpatialGeometryEvidenceTouchReceiptOnlyPreview {
        SpatialGeometryEvidenceTouchReceiptOnlyPreview::from_receipt(self.receipt)
    }

    pub fn with_complete_ledger(
        self,
        ledger: &'a CompleteWorkloadEvidenceLedger,
    ) -> SpatialGeometryEvidenceTouchAdmissionInput<'a, T> {
        SpatialGeometryEvidenceTouchAdmissionInput {
            request: self,
            ledger,
        }
    }
}

impl<'a> SpatialGeometryEvidenceTouchRowRequest<'a> {
    pub(crate) fn from_boolean_row(row: &'a WorkloadEvidenceRow) -> Self {
        Self { row }
    }

    pub(crate) fn with_complete_ledger(
        self,
        ledger: &'a CompleteWorkloadEvidenceLedger,
    ) -> SpatialGeometryEvidenceTouchRowAdmissionInput<'a> {
        SpatialGeometryEvidenceTouchRowAdmissionInput {
            request: self,
            ledger,
        }
    }
}

impl<'a, T: BooleanEvidenceReceipt + 'static> SpatialGeometryEvidenceTouchAdmissionInput<'a, T> {
    pub fn admit(
        self,
    ) -> Result<SpatialGeometryEvidenceTouchAuthority, SpatialGeometryEvidenceTouchDenial> {
        admit_lookup_product(
            self.ledger,
            self.ledger
                .require_boolean_receipt_lookup(self.request.receipt)
                .map_err(map_lookup_denial)?,
        )
    }
}

impl<'a> SpatialGeometryEvidenceTouchRowAdmissionInput<'a> {
    pub(crate) fn admit(
        self,
    ) -> Result<SpatialGeometryEvidenceTouchAuthority, SpatialGeometryEvidenceTouchDenial> {
        let row = self.request.row;
        admit_lookup_product(
            self.ledger,
            self.ledger
                .require_boolean_row_lookup(
                    row.stage(),
                    row.evidence_identity(),
                    row.support(),
                    row.counters(),
                )
                .map_err(map_lookup_denial)?,
        )
    }
}

fn authority_rows(ledger: &CompleteWorkloadEvidenceLedger) -> Vec<WorkloadEvidenceRow> {
    WorkloadEvidenceStage::AUTHORITY_STAGES
        .iter()
        .filter_map(|stage| ledger.row_for_stage(*stage).cloned())
        .collect()
}

fn require_spatial_touch_guard_contract(
    ledger: &CompleteWorkloadEvidenceLedger,
) -> Result<(), SpatialGeometryEvidenceTouchDenial> {
    ledger
        .guards()
        .assert_uses_real_topology()
        .and_then(|guard| guard.assert_binding_is_receipt_backed())
        .and_then(|guard| guard.assert_projection_is_receipt_backed())
        .and_then(|guard| guard.assert_transform_changed_geometry())
        .and_then(|guard| guard.assert_replay_consumed_retained_artifact())
        .and_then(|guard| guard.assert_counters_are_receipt_backed())
        .and_then(|guard| guard.assert_no_fixture_arithmetic_as_truth())
        .and_then(|guard| guard.assert_no_synthetic_end_to_end_claim())
        .map(|_| ())
        .map_err(WorkloadEvidenceLedgerError::from)
        .map_err(SpatialGeometryEvidenceTouchDenial::guard_failure)
}

fn admit_lookup_product(
    ledger: &CompleteWorkloadEvidenceLedger,
    lookup: crate::workload_platform::evidence_ledger::WorkloadEvidenceBooleanReceiptLookupProduct,
) -> Result<SpatialGeometryEvidenceTouchAuthority, SpatialGeometryEvidenceTouchDenial> {
    require_spatial_touch_guard_contract(ledger)?;
    if lookup.support() != WorkloadEvidenceSupport::Admitted {
        return Err(SpatialGeometryEvidenceTouchDenial::support_posture(
            lookup.evidence_stage(),
            lookup.support(),
        ));
    }
    if spatial_touch_counter_honesty(lookup.evidence_stage(), lookup.counters())
        .violation()
        .is_some()
    {
        return Err(SpatialGeometryEvidenceTouchDenial::counter_honesty(
            WorkloadEvidenceLedgerError::CounterlessBooleanStage(lookup.evidence_stage()),
        ));
    }
    let links = ledger
        .link_required_stages(&[lookup.evidence_stage()])
        .map_err(SpatialGeometryEvidenceTouchDenial::stage_link_failure)?;
    if !links.links_to_identity(lookup.evidence_stage(), lookup.evidence_identity()) {
        return Err(SpatialGeometryEvidenceTouchDenial::stage_link_failure(
            WorkloadEvidenceLedgerError::MismatchedBooleanStage(lookup.evidence_stage()),
        ));
    }
    Ok(admit_spatial_geometry_evidence_touch_authority(
        lookup.boolean_stage(),
        lookup.evidence_stage(),
        lookup.evidence_identity().to_string(),
        lookup.support(),
        lookup.counters(),
        lookup.lookup_counters(),
        lookup.stage_index_identity().to_string(),
        links.link_set_identity().to_string(),
        authority_rows(ledger),
    ))
}

fn map_lookup_denial(error: WorkloadEvidenceLedgerError) -> SpatialGeometryEvidenceTouchDenial {
    match error {
        WorkloadEvidenceLedgerError::ManualBooleanStage(_)
        | WorkloadEvidenceLedgerError::MismatchedBooleanStage(_) => {
            SpatialGeometryEvidenceTouchDenial::source_substitution(
                super::SpatialGeometryEvidenceTouchRejectedInputKind::WorkloadEvidenceRow,
                error.human_reason(),
            )
        }
        WorkloadEvidenceLedgerError::UnsupportedBooleanStage(stage) => {
            SpatialGeometryEvidenceTouchDenial::support_posture(
                stage,
                WorkloadEvidenceSupport::Unsupported,
            )
        }
        WorkloadEvidenceLedgerError::CounterlessBooleanStage(_) => {
            SpatialGeometryEvidenceTouchDenial::counter_honesty(error)
        }
        WorkloadEvidenceLedgerError::MismatchedBooleanStageCounters(_) => {
            SpatialGeometryEvidenceTouchDenial::counter_honesty(error)
        }
        WorkloadEvidenceLedgerError::MissingBooleanStage(_)
        | WorkloadEvidenceLedgerError::MissingAuthorityStage(_)
        | WorkloadEvidenceLedgerError::EmptyLedger
        | WorkloadEvidenceLedgerError::MissingEvidenceIdentity
        | WorkloadEvidenceLedgerError::DuplicateEvidenceStage(_)
        | WorkloadEvidenceLedgerError::SelectedLookupSliceExceedsScope(_)
        | WorkloadEvidenceLedgerError::ManualAuthorityStage(_)
        | WorkloadEvidenceLedgerError::UnadmittedAuthorityStage(_)
        | WorkloadEvidenceLedgerError::MismatchedAuthorityStageBinding(_, _) => {
            SpatialGeometryEvidenceTouchDenial::ledger_incompleteness(error)
        }
        WorkloadEvidenceLedgerError::GuardFailed(_) => {
            SpatialGeometryEvidenceTouchDenial::guard_failure(error)
        }
    }
}
