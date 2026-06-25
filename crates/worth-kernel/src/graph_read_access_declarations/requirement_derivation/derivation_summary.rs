use std::collections::BTreeSet;

use super::derivation_outcome::WorthGraphReadRequirementDerivationOutcome;
use super::derivation_record::WorthGraphReadRequirementDerivationRecord;
use super::stable_identity_digest::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementDerivationSummary {
    catalog_record_count: usize,
    query_derived_record_count: usize,
    derivation_gap_count: usize,
    derived_row_count: usize,
    distinct_requirement_kind_count: usize,
    vocabulary_mismatch_count: usize,
    execution_claim_count: usize,
    receipt_claim_count: usize,
    summary_digest: String,
}

impl WorthGraphReadRequirementDerivationSummary {
    pub(crate) fn from_records(records: &[WorthGraphReadRequirementDerivationRecord]) -> Self {
        let mut derived_row_count = 0;
        let mut distinct_requirement_kinds = BTreeSet::new();
        let mut query_derived_record_count = 0;
        let mut derivation_gap_count = 0;

        for record in records {
            match record.derivation_outcome() {
                WorthGraphReadRequirementDerivationOutcome::QueryDerived(evidence) => {
                    query_derived_record_count += 1;
                    derived_row_count += evidence.query_requirement_rows().len();
                    distinct_requirement_kinds.extend(
                        evidence
                            .query_requirement_rows()
                            .iter()
                            .map(|row| row.kind().as_str().to_string()),
                    );
                }
                WorthGraphReadRequirementDerivationOutcome::QueryCapabilityGap(_) => {
                    derivation_gap_count += 1;
                }
            }
        }

        let summary_digest = stable_digest(&[
            "worth_graph_read_requirement_derivation_summary_v1".to_string(),
            format!("catalog_records:{}", records.len()),
            format!("query_derived:{query_derived_record_count}"),
            format!("gaps:{derivation_gap_count}"),
            format!("derived_rows:{derived_row_count}"),
            format!("distinct_kinds:{}", distinct_requirement_kinds.len()),
        ]);
        Self {
            catalog_record_count: records.len(),
            query_derived_record_count,
            derivation_gap_count,
            derived_row_count,
            distinct_requirement_kind_count: distinct_requirement_kinds.len(),
            vocabulary_mismatch_count: 0,
            execution_claim_count: 0,
            receipt_claim_count: 0,
            summary_digest,
        }
    }

    pub const fn catalog_record_count(&self) -> usize {
        self.catalog_record_count
    }

    pub const fn query_derived_record_count(&self) -> usize {
        self.query_derived_record_count
    }

    pub const fn derivation_gap_count(&self) -> usize {
        self.derivation_gap_count
    }

    pub const fn derived_row_count(&self) -> usize {
        self.derived_row_count
    }

    pub const fn distinct_requirement_kind_count(&self) -> usize {
        self.distinct_requirement_kind_count
    }

    pub const fn vocabulary_mismatch_count(&self) -> usize {
        self.vocabulary_mismatch_count
    }

    pub const fn execution_claim_count(&self) -> usize {
        self.execution_claim_count
    }

    pub const fn receipt_claim_count(&self) -> usize {
        self.receipt_claim_count
    }

    pub fn summary_digest(&self) -> &str {
        &self.summary_digest
    }
}
