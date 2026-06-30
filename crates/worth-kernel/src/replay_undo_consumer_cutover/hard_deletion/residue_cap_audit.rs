use crate::replay_undo_inventory::{
    ReplayUndoInventoryCategory, ReplayUndoInventoryDisposition, ReplayUndoInventoryOwner,
    ReplayUndoInventoryReport, ReplayUndoInventorySourceIdentity,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::error::{ReplayUndoHardDeletionError, ReplayUndoHardDeletionErrorKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoResidueBlocker {
    NonOrdinaryUndoAuthorityGap,
    QueryCapabilityGap,
    CertificationOnlyBoundary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoResidueCapAuditRow {
    source_identity: ReplayUndoInventorySourceIdentity,
    owner: ReplayUndoInventoryOwner,
    blocker: ReplayUndoResidueBlocker,
    cap: usize,
    observed_count: usize,
    removal_trigger: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoResidueCapAudit {
    rows: Vec<ReplayUndoResidueCapAuditRow>,
    audit_digest: String,
}

impl ReplayUndoResidueCapAudit {
    pub(crate) fn from_inventory(
        inventory: &ReplayUndoInventoryReport,
    ) -> Result<Self, ReplayUndoHardDeletionError> {
        let mut rows = Vec::new();
        for row in inventory.rows() {
            if !matches!(
                row.disposition(),
                ReplayUndoInventoryDisposition::Cap | ReplayUndoInventoryDisposition::QueryGap
            ) {
                continue;
            }
            let removal_trigger = row.removal_trigger().ok_or_else(|| {
                ReplayUndoHardDeletionError::new(
                    ReplayUndoHardDeletionErrorKind::MissingResidueRemovalTrigger,
                    format!(
                        "replay/undo residue `{}` is missing a removal trigger",
                        row.source_identity().as_str()
                    ),
                )
            })?;
            let cap = row.residue_cap().ok_or_else(|| {
                ReplayUndoHardDeletionError::new(
                    ReplayUndoHardDeletionErrorKind::MissingResidueCap,
                    format!(
                        "replay/undo residue `{}` is missing a declared cap",
                        row.source_identity().as_str()
                    ),
                )
            })?;
            rows.push(ReplayUndoResidueCapAuditRow {
                source_identity: row.source_identity(),
                owner: row.owner(),
                blocker: blocker_for(row.category(), row.disposition()),
                cap,
                observed_count: row.observed_residue_count(),
                removal_trigger: removal_trigger.to_string(),
            });
        }
        let audit_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &audit_digest_parts(&rows),
        );
        Ok(Self { rows, audit_digest })
    }

    pub(crate) fn require_capped(&self) -> Result<(), ReplayUndoHardDeletionError> {
        if self.uncapped_residue_count() == 0 {
            Ok(())
        } else {
            Err(ReplayUndoHardDeletionError::new(
                ReplayUndoHardDeletionErrorKind::UncappedResidue,
                "replay/undo hard deletion found residue above its declared cap",
            ))
        }
    }

    pub fn rows(&self) -> &[ReplayUndoResidueCapAuditRow] {
        &self.rows
    }

    pub fn audit_digest(&self) -> &str {
        &self.audit_digest
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn uncapped_residue_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.observed_count > row.cap)
            .count()
    }
}

impl ReplayUndoResidueCapAuditRow {
    pub const fn source_identity(&self) -> ReplayUndoInventorySourceIdentity {
        self.source_identity
    }

    pub const fn owner(&self) -> ReplayUndoInventoryOwner {
        self.owner
    }

    pub const fn blocker(&self) -> ReplayUndoResidueBlocker {
        self.blocker
    }

    pub const fn cap(&self) -> usize {
        self.cap
    }

    pub const fn observed_count(&self) -> usize {
        self.observed_count
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}

fn blocker_for(
    category: ReplayUndoInventoryCategory,
    disposition: ReplayUndoInventoryDisposition,
) -> ReplayUndoResidueBlocker {
    match disposition {
        ReplayUndoInventoryDisposition::QueryGap => ReplayUndoResidueBlocker::QueryCapabilityGap,
        ReplayUndoInventoryDisposition::Cap => match category {
            ReplayUndoInventoryCategory::UndoScope => {
                ReplayUndoResidueBlocker::NonOrdinaryUndoAuthorityGap
            }
            _ => ReplayUndoResidueBlocker::CertificationOnlyBoundary,
        },
        ReplayUndoInventoryDisposition::Migrate | ReplayUndoInventoryDisposition::Delete => {
            ReplayUndoResidueBlocker::CertificationOnlyBoundary
        }
    }
}

fn audit_digest_parts(rows: &[ReplayUndoResidueCapAuditRow]) -> Vec<String> {
    let mut parts = vec!["worth-kernel:replay-undo-residue-cap-audit:v1".to_string()];
    parts.extend(rows.iter().map(|row| {
        format!(
            "{}:{:?}:{:?}:cap:{}:observed:{}:{}",
            row.source_identity.as_str(),
            row.owner,
            row.blocker,
            row.cap,
            row.observed_count,
            row.removal_trigger
        )
    }));
    parts
}

#[cfg(test)]
mod tests {
    use crate::replay_undo_inventory::inventory_lane::{
        close_current_replay_undo_inventory, current_replay_undo_declared_source_catalog,
        lower_current_replay_undo_inventory, ReplayUndoInventoryReportRow,
    };

    use super::*;

    #[test]
    fn residue_audit_rejects_cap_row_without_cap() {
        let inventory = inventory_with_residue_row(
            ReplayUndoInventorySourceIdentity::KernelBooleanSplitReplayUndoBoundaryAdmission,
            ReplayUndoInventoryOwner::WorthKernel,
            ReplayUndoInventoryCategory::UndoScope,
            ReplayUndoInventoryDisposition::Cap,
            Some("remove when undo lane is ordinary"),
            None,
            0,
        );

        let error = ReplayUndoResidueCapAudit::from_inventory(&inventory)
            .expect_err("cap residue without cap must fail");

        assert_eq!(
            error.kind(),
            &ReplayUndoHardDeletionErrorKind::MissingResidueCap
        );
    }

    #[test]
    fn residue_audit_rejects_query_gap_without_cap() {
        let inventory = inventory_with_residue_row(
            ReplayUndoInventorySourceIdentity::KernelBooleanSplitReplayUndoBoundaryAdmission,
            ReplayUndoInventoryOwner::WorthKernel,
            ReplayUndoInventoryCategory::Residue,
            ReplayUndoInventoryDisposition::QueryGap,
            Some("remove when Query owns this proof"),
            None,
            0,
        );

        let error = ReplayUndoResidueCapAudit::from_inventory(&inventory)
            .expect_err("query-gap residue without cap must fail");

        assert_eq!(
            error.kind(),
            &ReplayUndoHardDeletionErrorKind::MissingResidueCap
        );
    }

    #[test]
    fn residue_audit_rejects_cap_row_without_removal_trigger() {
        let inventory = inventory_with_residue_row(
            ReplayUndoInventorySourceIdentity::KernelBooleanSplitReplayUndoBoundaryAdmission,
            ReplayUndoInventoryOwner::WorthKernel,
            ReplayUndoInventoryCategory::UndoScope,
            ReplayUndoInventoryDisposition::Cap,
            None,
            Some(1),
            0,
        );

        let error = ReplayUndoResidueCapAudit::from_inventory(&inventory)
            .expect_err("cap residue without removal trigger must fail");

        assert_eq!(
            error.kind(),
            &ReplayUndoHardDeletionErrorKind::MissingResidueRemovalTrigger
        );
    }

    #[test]
    fn residue_audit_rejects_observed_residue_above_cap() {
        let inventory = inventory_with_residue_row(
            ReplayUndoInventorySourceIdentity::KernelBooleanSplitReplayUndoBoundaryAdmission,
            ReplayUndoInventoryOwner::WorthKernel,
            ReplayUndoInventoryCategory::UndoScope,
            ReplayUndoInventoryDisposition::Cap,
            Some("remove when undo lane is ordinary"),
            Some(1),
            2,
        );
        let audit = ReplayUndoResidueCapAudit::from_inventory(&inventory)
            .expect("audit is structurally valid");

        let error = audit
            .require_capped()
            .expect_err("observed residue above cap must fail");

        assert_eq!(
            error.kind(),
            &ReplayUndoHardDeletionErrorKind::UncappedResidue
        );
    }

    fn inventory_with_residue_row(
        identity: ReplayUndoInventorySourceIdentity,
        owner: ReplayUndoInventoryOwner,
        category: ReplayUndoInventoryCategory,
        disposition: ReplayUndoInventoryDisposition,
        removal_trigger: Option<&str>,
        residue_cap: Option<usize>,
        observed_residue_count: usize,
    ) -> ReplayUndoInventoryReport {
        let declared = current_replay_undo_declared_source_catalog();
        let declared_source = declared
            .require_source(identity)
            .expect("declared inventory source");
        let mut lowered = lower_current_replay_undo_inventory(&declared);
        lowered.retain(|row| row.source_identity() != identity);
        lowered.push(ReplayUndoInventoryReportRow::new_with_residue_count(
            identity,
            declared_source.source_path(),
            declared_source.source_kind(),
            owner,
            category,
            disposition,
            declared_source.authority_roles().clone(),
            declared_source.observability_roles().clone(),
            removal_trigger,
            residue_cap,
            observed_residue_count,
        ));
        close_current_replay_undo_inventory(declared, lowered).expect("inventory closeout")
    }
}
