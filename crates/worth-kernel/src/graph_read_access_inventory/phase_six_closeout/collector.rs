use std::collections::BTreeMap;

use super::super::candidates::WorthGraphReadDeclarationCandidate;
use super::super::capability_gaps::WorthGraphReadQueryAccessCapabilityGap;
use super::super::deletion_ledger::WorthGraphReadDeletionLedgerItem;
use super::super::inventory_lane::{
    WorthGraphReadAccessClassification, WorthGraphReadAccessInventoryCloseout,
    WorthGraphReadAccessMilestoneSevenDisposition,
};
use super::closeout::WorthGraphReadAccessPhaseSixCloseout;
use super::counters::WorthGraphReadAccessPhaseSixCounters;
use super::errors::{WorthGraphReadAccessPhaseSixError, WorthGraphReadAccessPhaseSixErrorKind};
use super::row_identity::WorthGraphReadAccessInventoryRowIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthGraphReadAccessPhaseSixDispositionKind {
    DeclarationCandidate,
    CapabilityGap,
    DeletionItem,
}

pub struct WorthGraphReadAccessPhaseSixCollector<'a> {
    inventory: &'a WorthGraphReadAccessInventoryCloseout,
    inventory_row_requirements: BTreeMap<
        WorthGraphReadAccessInventoryRowIdentity,
        WorthGraphReadAccessPhaseSixRowRequirement,
    >,
    admitted_dispositions: BTreeMap<
        WorthGraphReadAccessInventoryRowIdentity,
        WorthGraphReadAccessPhaseSixDispositionKind,
    >,
    declaration_candidates: Vec<WorthGraphReadDeclarationCandidate>,
    capability_gaps: Vec<WorthGraphReadQueryAccessCapabilityGap>,
    deletion_items: Vec<WorthGraphReadDeletionLedgerItem>,
}

impl<'a> WorthGraphReadAccessPhaseSixCollector<'a> {
    pub fn from_inventory(inventory: &'a WorthGraphReadAccessInventoryCloseout) -> Self {
        let inventory_row_requirements = inventory
            .rows()
            .iter()
            .map(|row| {
                (
                    WorthGraphReadAccessInventoryRowIdentity::from_row(row),
                    WorthGraphReadAccessPhaseSixRowRequirement {
                        classification: row.classification(),
                        milestone_seven_disposition: row.milestone_seven_disposition(),
                    },
                )
            })
            .collect();
        Self {
            inventory,
            inventory_row_requirements,
            admitted_dispositions: BTreeMap::new(),
            declaration_candidates: Vec::new(),
            capability_gaps: Vec::new(),
            deletion_items: Vec::new(),
        }
    }

    pub fn admit_declaration_candidate(
        mut self,
        candidate: WorthGraphReadDeclarationCandidate,
    ) -> Result<Self, WorthGraphReadAccessPhaseSixError> {
        self.admit_disposition(
            candidate.inventory_row_identity(),
            WorthGraphReadAccessPhaseSixDispositionKind::DeclarationCandidate,
        )?;
        self.declaration_candidates.push(candidate);
        Ok(self)
    }

    pub fn admit_capability_gap(
        mut self,
        gap: WorthGraphReadQueryAccessCapabilityGap,
    ) -> Result<Self, WorthGraphReadAccessPhaseSixError> {
        self.admit_disposition(
            gap.inventory_row_identity(),
            WorthGraphReadAccessPhaseSixDispositionKind::CapabilityGap,
        )?;
        self.capability_gaps.push(gap);
        Ok(self)
    }

    pub fn admit_deletion_item(
        mut self,
        item: WorthGraphReadDeletionLedgerItem,
    ) -> Result<Self, WorthGraphReadAccessPhaseSixError> {
        self.admit_disposition(
            item.inventory_row_identity(),
            WorthGraphReadAccessPhaseSixDispositionKind::DeletionItem,
        )?;
        self.deletion_items.push(item);
        Ok(self)
    }

    pub fn closeout(
        self,
    ) -> Result<WorthGraphReadAccessPhaseSixCloseout, WorthGraphReadAccessPhaseSixError> {
        if self.inventory.rows().is_empty() {
            return Err(error(
                WorthGraphReadAccessPhaseSixErrorKind::EmptyPhaseSixCloseout,
            ));
        }
        self.require_every_required_inventory_row_has_disposition()?;
        let counters = WorthGraphReadAccessPhaseSixCounters::new(
            self.declaration_candidates.len(),
            self.capability_gaps.len(),
            self.deletion_items.len(),
            self.count_inventory_classification(
                WorthGraphReadAccessClassification::CertificationOnlySupport,
            ),
            self.count_inventory_classification(
                WorthGraphReadAccessClassification::OutOfScopeNonGraphRead,
            ),
        );
        Ok(WorthGraphReadAccessPhaseSixCloseout::new(
            self.declaration_candidates,
            self.capability_gaps,
            self.deletion_items,
            counters,
        ))
    }

    fn admit_disposition(
        &mut self,
        identity: &WorthGraphReadAccessInventoryRowIdentity,
        disposition: WorthGraphReadAccessPhaseSixDispositionKind,
    ) -> Result<(), WorthGraphReadAccessPhaseSixError> {
        let Some(requirement) = self.inventory_row_requirements.get(identity) else {
            return Err(error(
                WorthGraphReadAccessPhaseSixErrorKind::UnknownInventoryRow,
            ));
        };
        if !requirement.accepts_disposition(disposition) {
            return Err(error(
                WorthGraphReadAccessPhaseSixErrorKind::InventoryRowDispositionMismatch,
            ));
        }
        if self
            .admitted_dispositions
            .insert(identity.clone(), disposition)
            .is_some()
        {
            return Err(error(
                WorthGraphReadAccessPhaseSixErrorKind::DuplicateInventoryRowDisposition,
            ));
        }
        Ok(())
    }

    fn require_every_required_inventory_row_has_disposition(
        &self,
    ) -> Result<(), WorthGraphReadAccessPhaseSixError> {
        for row in self.inventory.rows() {
            let classification = row.classification();
            if !classification_requires_phase_six_disposition(classification) {
                continue;
            }
            let identity = WorthGraphReadAccessInventoryRowIdentity::from_row(row);
            if !self.admitted_dispositions.contains_key(&identity) {
                return Err(error(
                    WorthGraphReadAccessPhaseSixErrorKind::MissingInventoryRowDisposition,
                ));
            }
        }
        Ok(())
    }

    fn count_inventory_classification(
        &self,
        classification: WorthGraphReadAccessClassification,
    ) -> usize {
        self.inventory
            .rows()
            .iter()
            .filter(|row| row.classification() == classification)
            .count()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorthGraphReadAccessPhaseSixRowRequirement {
    classification: WorthGraphReadAccessClassification,
    milestone_seven_disposition: WorthGraphReadAccessMilestoneSevenDisposition,
}

impl WorthGraphReadAccessPhaseSixRowRequirement {
    const fn accepts_disposition(
        self,
        disposition: WorthGraphReadAccessPhaseSixDispositionKind,
    ) -> bool {
        classification_accepts_disposition(self.classification, disposition)
            && milestone_seven_disposition_accepts_phase_six_disposition(
                self.milestone_seven_disposition,
                disposition,
            )
    }
}

pub fn reject_keep_local_graph_read_disposition(
    _reason: &str,
) -> Result<(), WorthGraphReadAccessPhaseSixError> {
    Err(error(
        WorthGraphReadAccessPhaseSixErrorKind::KeepLocalGraphReadDispositionDenied,
    ))
}

fn classification_accepts_disposition(
    classification: WorthGraphReadAccessClassification,
    disposition: WorthGraphReadAccessPhaseSixDispositionKind,
) -> bool {
    matches!(
        (classification, disposition),
        (
            WorthGraphReadAccessClassification::QueryDeclarationCandidate,
            WorthGraphReadAccessPhaseSixDispositionKind::DeclarationCandidate
        ) | (
            WorthGraphReadAccessClassification::QueryAccessCapabilityGap,
            WorthGraphReadAccessPhaseSixDispositionKind::CapabilityGap
        ) | (
            WorthGraphReadAccessClassification::CappedResidue,
            WorthGraphReadAccessPhaseSixDispositionKind::CapabilityGap
                | WorthGraphReadAccessPhaseSixDispositionKind::DeletionItem
        ) | (
            WorthGraphReadAccessClassification::DeletionTarget,
            WorthGraphReadAccessPhaseSixDispositionKind::DeletionItem
        )
    )
}

fn classification_requires_phase_six_disposition(
    classification: WorthGraphReadAccessClassification,
) -> bool {
    matches!(
        classification,
        WorthGraphReadAccessClassification::QueryDeclarationCandidate
            | WorthGraphReadAccessClassification::QueryAccessCapabilityGap
            | WorthGraphReadAccessClassification::CappedResidue
            | WorthGraphReadAccessClassification::DeletionTarget
    )
}

const fn milestone_seven_disposition_accepts_phase_six_disposition(
    milestone_seven_disposition: WorthGraphReadAccessMilestoneSevenDisposition,
    disposition: WorthGraphReadAccessPhaseSixDispositionKind,
) -> bool {
    matches!(
        (milestone_seven_disposition, disposition),
        (
            WorthGraphReadAccessMilestoneSevenDisposition::DeclarationCandidate,
            WorthGraphReadAccessPhaseSixDispositionKind::DeclarationCandidate
        ) | (
            WorthGraphReadAccessMilestoneSevenDisposition::CapabilityGap,
            WorthGraphReadAccessPhaseSixDispositionKind::CapabilityGap
        ) | (
            WorthGraphReadAccessMilestoneSevenDisposition::DeletionOnly,
            WorthGraphReadAccessPhaseSixDispositionKind::DeletionItem
        )
    )
}

const fn error(kind: WorthGraphReadAccessPhaseSixErrorKind) -> WorthGraphReadAccessPhaseSixError {
    WorthGraphReadAccessPhaseSixError::new(kind)
}
