use super::derivation_record::WorthGraphReadRequirementDerivationRecord;
use super::derivation_summary::WorthGraphReadRequirementDerivationSummary;
use super::errors::{
    WorthGraphReadRequirementDerivationError, WorthGraphReadRequirementDerivationErrorKind,
};
use super::phase_five_seed::WorthGraphReadAccessDeclarationPhaseFiveSeed;
use super::stable_identity_digest::stable_digest;
use crate::graph_read_access_declarations::WorthGraphReadAccessDeclarationPhaseTwoCloseout;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementDerivationCloseout {
    requirement_records: Vec<WorthGraphReadRequirementDerivationRecord>,
    derivation_summary: WorthGraphReadRequirementDerivationSummary,
    phase_five_seed: WorthGraphReadAccessDeclarationPhaseFiveSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_requirement_derivation_closeout(
    phase_two: &WorthGraphReadAccessDeclarationPhaseTwoCloseout,
) -> Result<WorthGraphReadRequirementDerivationCloseout, WorthGraphReadRequirementDerivationError> {
    let records = requirement_records_from_phase_two_closeout(phase_two)?;
    let derivation_summary = WorthGraphReadRequirementDerivationSummary::from_records(&records);
    let mut closeout_digest_parts = vec![
        "worth_graph_read_requirement_derivation_closeout_v1".to_string(),
        format!(
            "catalog:{}",
            phase_two.declaration_catalog().catalog_digest()
        ),
        format!("summary:{}", derivation_summary.summary_digest()),
    ];
    closeout_digest_parts.extend(
        records
            .iter()
            .map(|record| format!("record:{}", record.record_digest())),
    );
    let closeout_digest = stable_digest(&closeout_digest_parts);
    let phase_five_seed = WorthGraphReadAccessDeclarationPhaseFiveSeed::new(
        records.clone(),
        phase_two.deletion_items().to_vec(),
        closeout_digest.clone(),
    );
    Ok(WorthGraphReadRequirementDerivationCloseout {
        requirement_records: records,
        derivation_summary,
        phase_five_seed,
        closeout_digest,
    })
}

impl WorthGraphReadRequirementDerivationCloseout {
    pub fn requirement_records(&self) -> &[WorthGraphReadRequirementDerivationRecord] {
        &self.requirement_records
    }

    pub fn derivation_summary(&self) -> &WorthGraphReadRequirementDerivationSummary {
        &self.derivation_summary
    }

    pub fn phase_five_seed(&self) -> &WorthGraphReadAccessDeclarationPhaseFiveSeed {
        &self.phase_five_seed
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

fn requirement_records_from_phase_two_closeout(
    phase_two: &WorthGraphReadAccessDeclarationPhaseTwoCloseout,
) -> Result<Vec<WorthGraphReadRequirementDerivationRecord>, WorthGraphReadRequirementDerivationError>
{
    let catalog_records = phase_two.declaration_catalog().records();
    if catalog_records.is_empty() {
        return Err(WorthGraphReadRequirementDerivationError::new(
            WorthGraphReadRequirementDerivationErrorKind::MissingCatalogRecord,
        ));
    }
    catalog_records
        .iter()
        .map(WorthGraphReadRequirementDerivationRecord::from_catalog_record)
        .collect()
}
