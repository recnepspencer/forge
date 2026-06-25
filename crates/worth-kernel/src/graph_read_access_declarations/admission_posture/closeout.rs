use super::capability_gap_cap::{
    cap_report_from_posture_records, WorthGraphReadAdmissionGapCapReport,
};
use super::errors::{
    WorthGraphReadAccessAdmissionPostureError, WorthGraphReadAccessAdmissionPostureErrorKind,
};
use super::phase_six_seed::WorthGraphReadAccessDeclarationPhaseSixSeed;
use super::posture_record::WorthGraphReadAdmissionPostureRecord;
use super::stable_identity_digest::stable_digest;
use crate::graph_read_access_declarations::WorthGraphReadAccessDeclarationPhaseFiveSeed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessAdmissionPostureCloseout {
    posture_records: Vec<WorthGraphReadAdmissionPostureRecord>,
    gap_cap_report: WorthGraphReadAdmissionGapCapReport,
    phase_six_seed: WorthGraphReadAccessDeclarationPhaseSixSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_access_admission_posture_closeout(
    seed: &WorthGraphReadAccessDeclarationPhaseFiveSeed,
) -> Result<WorthGraphReadAccessAdmissionPostureCloseout, WorthGraphReadAccessAdmissionPostureError>
{
    if seed.requirement_records().is_empty() {
        return Err(WorthGraphReadAccessAdmissionPostureError::new(
            WorthGraphReadAccessAdmissionPostureErrorKind::MissingRequirementRecord,
        ));
    }
    let posture_records = seed
        .requirement_records()
        .iter()
        .map(WorthGraphReadAdmissionPostureRecord::from_requirement_record)
        .collect::<Vec<_>>();
    let gap_cap_report = cap_report_from_posture_records(&posture_records)?;
    let closeout_digest = closeout_digest(seed, &posture_records, &gap_cap_report);
    let phase_six_seed = WorthGraphReadAccessDeclarationPhaseSixSeed::new(
        posture_records.clone(),
        seed.deletion_items().to_vec(),
        gap_cap_report.clone(),
        closeout_digest.clone(),
    );
    Ok(WorthGraphReadAccessAdmissionPostureCloseout {
        posture_records,
        gap_cap_report,
        phase_six_seed,
        closeout_digest,
    })
}

impl WorthGraphReadAccessAdmissionPostureCloseout {
    pub fn posture_records(&self) -> &[WorthGraphReadAdmissionPostureRecord] {
        &self.posture_records
    }

    pub fn gap_cap_report(&self) -> &WorthGraphReadAdmissionGapCapReport {
        &self.gap_cap_report
    }

    pub fn admission_capability_gaps(
        &self,
    ) -> &[super::query_admission_projection::WorthGraphReadAdmissionCapabilityGap] {
        self.phase_six_seed.admission_capability_gaps()
    }

    pub fn carried_requirement_derivation_gaps(
        &self,
    ) -> &[crate::graph_read_access_declarations::WorthGraphReadRequirementDerivationCapabilityGap]
    {
        self.phase_six_seed.carried_requirement_derivation_gaps()
    }

    pub fn phase_six_seed(&self) -> &WorthGraphReadAccessDeclarationPhaseSixSeed {
        &self.phase_six_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_graph_read_execution(&self) -> bool {
        false
    }

    pub const fn claims_access_plan_consumption(&self) -> bool {
        false
    }

    pub const fn claims_graph_read_receipts_complete(&self) -> bool {
        false
    }
}

fn closeout_digest(
    seed: &WorthGraphReadAccessDeclarationPhaseFiveSeed,
    records: &[WorthGraphReadAdmissionPostureRecord],
    cap_report: &WorthGraphReadAdmissionGapCapReport,
) -> String {
    let mut parts = vec![
        "worth_graph_read_access_admission_posture_closeout_v1".to_string(),
        format!(
            "requirement_derivation:{}",
            seed.requirement_derivation_digest()
        ),
        format!("gap_cap_report:{}", cap_report.report_digest()),
    ];
    parts.extend(
        records
            .iter()
            .map(|record| format!("record:{}", record.record_digest())),
    );
    stable_digest(&parts)
}
