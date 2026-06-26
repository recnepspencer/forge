use super::capability_gap_cap::WorthGraphReadAdmissionGapCapReport;
use super::posture_record::WorthGraphReadAdmissionPostureRecord;
use super::query_admission_projection::WorthGraphReadAdmissionCapabilityGap;
use crate::graph_read_access_declarations::WorthGraphReadRequirementDerivationCapabilityGap;
use crate::graph_read_access_inventory::WorthGraphReadDeletionLedgerItem;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationPhaseSixSeed {
    posture_records: Vec<WorthGraphReadAdmissionPostureRecord>,
    admission_capability_gaps: Vec<WorthGraphReadAdmissionCapabilityGap>,
    carried_requirement_derivation_gaps: Vec<WorthGraphReadRequirementDerivationCapabilityGap>,
    deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
    gap_cap_report: WorthGraphReadAdmissionGapCapReport,
    admission_closeout_digest: String,
}

impl WorthGraphReadAccessDeclarationPhaseSixSeed {
    pub(crate) fn new(
        posture_records: Vec<WorthGraphReadAdmissionPostureRecord>,
        deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
        gap_cap_report: WorthGraphReadAdmissionGapCapReport,
        admission_closeout_digest: impl Into<String>,
    ) -> Self {
        let admission_capability_gaps = posture_records
            .iter()
            .filter_map(|record| record.posture_outcome().admission_gap().cloned())
            .collect();
        let carried_requirement_derivation_gaps = posture_records
            .iter()
            .filter_map(|record| {
                record
                    .posture_outcome()
                    .requirement_derivation_gap()
                    .cloned()
            })
            .collect();
        Self {
            posture_records,
            admission_capability_gaps,
            carried_requirement_derivation_gaps,
            deletion_items,
            gap_cap_report,
            admission_closeout_digest: admission_closeout_digest.into(),
        }
    }

    pub fn posture_records(&self) -> &[WorthGraphReadAdmissionPostureRecord] {
        &self.posture_records
    }

    pub fn admission_capability_gaps(&self) -> &[WorthGraphReadAdmissionCapabilityGap] {
        &self.admission_capability_gaps
    }

    pub fn carried_requirement_derivation_gaps(
        &self,
    ) -> &[WorthGraphReadRequirementDerivationCapabilityGap] {
        &self.carried_requirement_derivation_gaps
    }

    pub fn gap_cap_report(&self) -> &WorthGraphReadAdmissionGapCapReport {
        &self.gap_cap_report
    }

    pub fn deletion_items(&self) -> &[WorthGraphReadDeletionLedgerItem] {
        &self.deletion_items
    }

    pub fn admission_closeout_digest(&self) -> &str {
        &self.admission_closeout_digest
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }
}
