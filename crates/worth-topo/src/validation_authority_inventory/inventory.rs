use std::collections::BTreeSet;

use super::counters::WorthValidationAuthorityInventoryCounters;
use super::cut_line::WorthValidationAuthorityCutLine;
use super::discovery::WorthValidationAuthorityDiscoveredSource;
use super::disposition::WorthValidationAuthorityDisposition;
use super::error::WorthValidationAuthorityInventoryError;
use super::inventory_row::WorthValidationAuthorityInventoryRow;
use super::milestone_eight_seed_summary::WorthValidationAuthorityMilestoneEightSeedSummary;
use super::source_authority::{
    current_validation_authority_rows, required_validation_authority_sources,
    WorthValidationAuthoritySource,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthorityInventory {
    rows: Vec<WorthValidationAuthorityInventoryRow>,
    milestone_eight_seed_summary: Option<WorthValidationAuthorityMilestoneEightSeedSummary>,
    counters: WorthValidationAuthorityInventoryCounters,
    cut_line: WorthValidationAuthorityCutLine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthorityInventoryInput {
    milestone_eight_seed_summary: WorthValidationAuthorityMilestoneEightSeedSummary,
}

impl WorthValidationAuthorityInventoryInput {
    pub fn from_milestone_eight_seed_summary(
        milestone_eight_seed_summary: WorthValidationAuthorityMilestoneEightSeedSummary,
    ) -> Self {
        Self {
            milestone_eight_seed_summary,
        }
    }
}

impl WorthValidationAuthorityInventory {
    pub fn from_current_sources() -> Result<Self, WorthValidationAuthorityInventoryError> {
        Self::from_rows_and_seed_for_validation(current_validation_authority_rows(), None)
    }

    pub fn from_current_sources_with_input(
        input: WorthValidationAuthorityInventoryInput,
    ) -> Result<Self, WorthValidationAuthorityInventoryError> {
        if input
            .milestone_eight_seed_summary
            .claims_validator_selection()
        {
            return Err(
                WorthValidationAuthorityInventoryError::MilestoneEightSeedClaimsValidatorSelection(
                    input.milestone_eight_seed_summary.seed_digest().to_string(),
                ),
            );
        }
        Self::from_rows_and_seed_for_validation(
            current_validation_authority_rows(),
            Some(input.milestone_eight_seed_summary),
        )
    }

    pub fn rows(&self) -> &[WorthValidationAuthorityInventoryRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &WorthValidationAuthorityInventoryCounters {
        &self.counters
    }

    pub const fn milestone_eight_seed_summary(
        &self,
    ) -> Option<&WorthValidationAuthorityMilestoneEightSeedSummary> {
        self.milestone_eight_seed_summary.as_ref()
    }

    pub const fn cut_line(&self) -> &WorthValidationAuthorityCutLine {
        &self.cut_line
    }

    pub fn unclassified_count(&self) -> usize {
        0
    }

    pub fn keep_disposition_count(&self) -> usize {
        0
    }

    pub fn row_for_source(
        &self,
        source: WorthValidationAuthoritySource,
    ) -> Option<&WorthValidationAuthorityInventoryRow> {
        self.rows.iter().find(|row| row.source() == source)
    }

    #[cfg(test)]
    pub(crate) fn from_rows_for_validation(
        rows: Vec<WorthValidationAuthorityInventoryRow>,
    ) -> Result<Self, WorthValidationAuthorityInventoryError> {
        Self::from_rows_and_seed_for_validation(rows, None)
    }

    pub(crate) fn from_rows_and_seed_for_validation(
        rows: Vec<WorthValidationAuthorityInventoryRow>,
        milestone_eight_seed_summary: Option<WorthValidationAuthorityMilestoneEightSeedSummary>,
    ) -> Result<Self, WorthValidationAuthorityInventoryError> {
        validate_rows(&rows)?;
        let counters = WorthValidationAuthorityInventoryCounters::from_rows(&rows);
        let cut_line = WorthValidationAuthorityCutLine::from_counters(counters);
        Ok(Self {
            rows,
            milestone_eight_seed_summary,
            counters,
            cut_line,
        })
    }

    pub(crate) fn contains_discovered_source(
        &self,
        discovered: &WorthValidationAuthorityDiscoveredSource,
    ) -> bool {
        self.rows
            .iter()
            .any(|row| row.matches_discovered_source(discovered))
    }
}

fn validate_rows(
    rows: &[WorthValidationAuthorityInventoryRow],
) -> Result<(), WorthValidationAuthorityInventoryError> {
    let mut sources = BTreeSet::new();
    for row in rows {
        if !sources.insert(row.source()) {
            return Err(WorthValidationAuthorityInventoryError::duplicate_source(
                row.source(),
            ));
        }
        if row.owner().trim().is_empty() {
            return Err(WorthValidationAuthorityInventoryError::missing_owner(
                row.source(),
            ));
        }
        if row.removal_trigger().trim().is_empty() {
            return Err(
                WorthValidationAuthorityInventoryError::missing_removal_trigger(row.source()),
            );
        }
        if row.certification_only_comparison_allowed()
            && !matches!(
                row.disposition(),
                WorthValidationAuthorityDisposition::Cap
                    | WorthValidationAuthorityDisposition::Migrate
                    | WorthValidationAuthorityDisposition::QueryAccessGap
            )
        {
            return Err(
                WorthValidationAuthorityInventoryError::certification_only_without_cap(
                    row.source(),
                ),
            );
        }
    }

    for required in required_validation_authority_sources() {
        if !sources.contains(&required) {
            return Err(WorthValidationAuthorityInventoryError::missing_required_source(required));
        }
    }
    Ok(())
}
