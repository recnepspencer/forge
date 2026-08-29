#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiServiceProposalStageIssuer {
    FamilyOwner {
        family: crate::capability::UiRuntimeServiceFamily,
        scope: super::super::UiServiceProposalOccupancyScopeIdentity,
    },
    ExistingPreparation,
    /// The Focus owner resolved against the assembled successor. When it emitted
    /// its one lawful reveal refinement, the witness names the exact Scroll owner
    /// scope that replanned for it; the compiler verifies that owner is a
    /// participating family at that exact scope before the batch may advance.
    FocusOwner {
        reveal_refinement: Option<super::super::UiServiceProposalOccupancyScopeIdentity>,
    },
    MotionOwner,
}

pub(in crate::runtime) struct UiServiceProposalStageReceipt {
    proposal: super::UiServiceProposalIdentity,
    completed: super::UiServiceProposalStage,
    issuer: UiServiceProposalStageIssuer,
    fact_references: Box<[super::UiServiceProducedFactReference]>,
    mounted_work_references: Box<[super::UiServiceMountedWorkReference]>,
}

#[must_use]
#[derive(Debug)]
pub(in crate::runtime) struct UiServiceProposalStaging {
    candidate: super::UiServiceProposalCandidate,
    leases: Box<[super::super::UiServiceProposalOccupancyLease]>,
    displacement: Option<super::super::UiServiceProposalDisplacement>,
    next_stage: usize,
    staged_families: super::super::UiServiceFamilyParticipation,
    fact_references: Vec<super::UiServiceProducedFactReference>,
    mounted_work_references: Vec<super::UiServiceMountedWorkReference>,
    retained_receipts: u16,
    reveal_refinement: Option<super::super::UiServiceProposalOccupancyScopeIdentity>,
}

#[must_use]
#[derive(Debug)]
pub(in crate::runtime) struct UiServiceProposalStagedBatch {
    candidate: super::UiServiceProposalCandidate,
    leases: Box<[super::super::UiServiceProposalOccupancyLease]>,
    displacement: Option<super::super::UiServiceProposalDisplacement>,
    fact_references: Box<[super::UiServiceProducedFactReference]>,
    mounted_work_references: Box<[super::UiServiceMountedWorkReference]>,
    digest: u64,
    retained_receipts: u16,
    reveal_refinement: Option<super::super::UiServiceProposalOccupancyScopeIdentity>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiServiceProposalStagingDenial {
    ForeignProposal,
    OutOfOrder {
        expected: super::UiServiceProposalStage,
        observed: super::UiServiceProposalStage,
    },
    WrongIssuer,
    NonParticipatingFamily,
    ScopeWidening,
    DuplicateFamilyWitness,
    /// The Focus owner claimed a reveal refinement without a participating Scroll
    /// owner at the exact staged scope, so no owner replanned for it.
    UnbackedRevealRefinement,
    ReferenceFamilyMismatch,
    ReferenceScopeMismatch,
    ReferenceBudgetMismatch,
    UnexpectedReferences,
    ReceiptCapacityExceeded,
    Incomplete {
        expected: super::UiServiceProposalStage,
    },
    AlreadyComplete,
    Census(super::super::UiServiceProposalCensusDenial),
    Occupancy(super::super::UiServiceProposalOccupancyDenial),
}

impl UiServiceProposalStaging {
    pub(super) fn new(reserved: super::UiReservedServiceProposal) -> Self {
        let (candidate, leases, displacement) = reserved.into_parts();
        Self {
            candidate,
            leases,
            displacement,
            next_stage: super::UiServiceProposalStage::FamilyOwnedStaging.ordinal(),
            staged_families: super::super::UiServiceFamilyParticipation::EMPTY,
            fact_references: Vec::new(),
            mounted_work_references: Vec::new(),
            retained_receipts: 0,
            reveal_refinement: None,
        }
    }

    pub(super) fn accept_stage_receipt(
        &mut self,
        receipt: UiServiceProposalStageReceipt,
    ) -> Result<(), UiServiceProposalStagingDenial> {
        if receipt.proposal != self.candidate.identity() {
            return Err(UiServiceProposalStagingDenial::ForeignProposal);
        }
        if self.next_stage >= super::UiServiceProposalStage::SubmitToExistingPublication.ordinal() {
            return Err(UiServiceProposalStagingDenial::AlreadyComplete);
        }
        let expected = super::UiServiceProposalStage::ORDER[self.next_stage];
        if receipt.completed != expected {
            return Err(UiServiceProposalStagingDenial::OutOfOrder {
                expected,
                observed: receipt.completed,
            });
        }
        let retained_receipts = self
            .retained_receipts
            .checked_add(1)
            .ok_or(UiServiceProposalStagingDenial::ReceiptCapacityExceeded)?;
        match expected {
            super::UiServiceProposalStage::FamilyOwnedStaging => {
                self.accept_family_witness(receipt)?;
                if self.staged_families.count()
                    == self.candidate.demand().participating_families().count()
                {
                    self.next_stage += 1;
                }
            }
            super::UiServiceProposalStage::AssembleSuccessor => {
                receipt::require_empty_references(&receipt)?;
                if receipt.issuer != UiServiceProposalStageIssuer::ExistingPreparation {
                    return Err(UiServiceProposalStagingDenial::WrongIssuer);
                }
                self.next_stage = self.next_owner_stage_after_assembly();
            }
            super::UiServiceProposalStage::ResolveFocusAndReveal => {
                receipt::require_empty_references(&receipt)?;
                let UiServiceProposalStageIssuer::FocusOwner { reveal_refinement } = receipt.issuer
                else {
                    return Err(UiServiceProposalStagingDenial::WrongIssuer);
                };
                self.accept_reveal_refinement(reveal_refinement)?;
                self.next_stage = if self
                    .candidate
                    .demand()
                    .participating_families()
                    .contains(crate::capability::UiRuntimeServiceFamily::Motion)
                {
                    super::UiServiceProposalStage::DeriveMotion.ordinal()
                } else {
                    super::UiServiceProposalStage::SubmitToExistingPublication.ordinal()
                };
            }
            super::UiServiceProposalStage::DeriveMotion => {
                receipt::require_empty_references(&receipt)?;
                if receipt.issuer != UiServiceProposalStageIssuer::MotionOwner {
                    return Err(UiServiceProposalStagingDenial::WrongIssuer);
                }
                self.next_stage += 1;
            }
            super::UiServiceProposalStage::ValidatePreState
            | super::UiServiceProposalStage::SubmitToExistingPublication
            | super::UiServiceProposalStage::SettleFamilyOwners => {
                return Err(UiServiceProposalStagingDenial::AlreadyComplete);
            }
        }
        self.retained_receipts = retained_receipts;
        Ok(())
    }

    fn accept_reveal_refinement(
        &mut self,
        reveal_refinement: Option<super::super::UiServiceProposalOccupancyScopeIdentity>,
    ) -> Result<(), UiServiceProposalStagingDenial> {
        self.reveal_refinement =
            reveal_refinement::admit(&self.candidate, self.staged_families, reveal_refinement)?;
        Ok(())
    }

    fn next_owner_stage_after_assembly(&self) -> usize {
        let families = self.candidate.demand().participating_families();
        if families.contains(crate::capability::UiRuntimeServiceFamily::Focus) {
            super::UiServiceProposalStage::ResolveFocusAndReveal.ordinal()
        } else if families.contains(crate::capability::UiRuntimeServiceFamily::Motion) {
            super::UiServiceProposalStage::DeriveMotion.ordinal()
        } else {
            super::UiServiceProposalStage::SubmitToExistingPublication.ordinal()
        }
    }

    pub(super) const fn is_before_first_effect(&self) -> bool {
        self.retained_receipts == 0
    }

    pub(in crate::runtime) const fn identity(&self) -> super::UiServiceProposalIdentity {
        self.candidate.identity()
    }

    pub(super) fn leases(&self) -> &[super::super::UiServiceProposalOccupancyLease] {
        &self.leases
    }

    fn accept_family_witness(
        &mut self,
        receipt: UiServiceProposalStageReceipt,
    ) -> Result<(), UiServiceProposalStagingDenial> {
        let UiServiceProposalStageIssuer::FamilyOwner { family, scope } = receipt.issuer else {
            return Err(UiServiceProposalStagingDenial::WrongIssuer);
        };
        if !self
            .candidate
            .demand()
            .participating_families()
            .contains(family)
        {
            return Err(UiServiceProposalStagingDenial::NonParticipatingFamily);
        }
        if self.staged_families.contains(family) {
            return Err(UiServiceProposalStagingDenial::DuplicateFamilyWitness);
        }
        let family_proposal = self
            .candidate
            .family_proposals()
            .iter()
            .find(|proposal| proposal.family() == family)
            .ok_or(UiServiceProposalStagingDenial::NonParticipatingFamily)?;
        if family_proposal.scope() != scope {
            return Err(UiServiceProposalStagingDenial::ScopeWidening);
        }
        receipt::validate_references(family, scope, &receipt)?;
        if receipt.fact_references.len() != usize::from(family_proposal.fact_references())
            || receipt.mounted_work_references.len()
                != usize::from(family_proposal.mounted_work_references())
        {
            return Err(UiServiceProposalStagingDenial::ReferenceBudgetMismatch);
        }
        self.staged_families = self
            .staged_families
            .with_family(family)
            .map_err(|_| UiServiceProposalStagingDenial::DuplicateFamilyWitness)?;
        self.fact_references.extend(receipt.fact_references);
        self.mounted_work_references
            .extend(receipt.mounted_work_references);
        Ok(())
    }

    pub(super) fn finish(
        mut self,
    ) -> Result<UiServiceProposalStagedBatch, (Self, UiServiceProposalStagingDenial)> {
        if self.next_stage != super::UiServiceProposalStage::SubmitToExistingPublication.ordinal() {
            let expected = super::UiServiceProposalStage::ORDER[self.next_stage];
            return Err((
                self,
                UiServiceProposalStagingDenial::Incomplete { expected },
            ));
        }
        self.fact_references
            .sort_by_key(|reference| (reference.family().index(), reference.diagnostic_value()));
        self.mounted_work_references
            .sort_by_key(|reference| (reference.family().index(), reference.diagnostic_value()));
        let digest = staged_batch_digest(
            self.candidate.identity(),
            &self.fact_references,
            &self.mounted_work_references,
        );
        Ok(UiServiceProposalStagedBatch {
            candidate: self.candidate,
            leases: self.leases,
            displacement: self.displacement,
            fact_references: self.fact_references.into_boxed_slice(),
            mounted_work_references: self.mounted_work_references.into_boxed_slice(),
            digest,
            retained_receipts: self.retained_receipts,
            reveal_refinement: self.reveal_refinement,
        })
    }

    pub(super) fn into_terminal_parts(self) -> UiServiceProposalTerminalParts {
        UiServiceProposalTerminalParts {
            candidate: self.candidate,
            leases: self.leases,
            retained_receipts: self.retained_receipts,
            owners_requiring_discard: self.staged_families,
        }
    }
}

impl UiServiceProposalStagedBatch {
    pub(in crate::runtime) const fn identity(&self) -> super::UiServiceProposalIdentity {
        self.candidate.identity()
    }

    pub(in crate::runtime) fn fact_references(&self) -> &[super::UiServiceProducedFactReference] {
        &self.fact_references
    }

    pub(in crate::runtime) fn mounted_work_references(
        &self,
    ) -> &[super::UiServiceMountedWorkReference] {
        &self.mounted_work_references
    }

    pub(in crate::runtime) const fn digest(&self) -> u64 {
        self.digest
    }

    /// The exact Scroll owner scope that replanned for the Focus owner's one
    /// lawful reveal refinement, when one was emitted.
    pub(in crate::runtime) const fn reveal_refinement(
        &self,
    ) -> Option<super::super::UiServiceProposalOccupancyScopeIdentity> {
        self.reveal_refinement
    }

    pub(super) const fn retained_receipts(&self) -> u16 {
        self.retained_receipts
    }

    pub(super) fn leases(&self) -> &[super::super::UiServiceProposalOccupancyLease] {
        &self.leases
    }

    pub(super) fn candidate(&self) -> &super::UiServiceProposalCandidate {
        &self.candidate
    }

    pub(super) const fn displacement(&self) -> Option<super::super::UiServiceProposalDisplacement> {
        self.displacement
    }

    pub(super) fn into_terminal_parts(self) -> UiServiceProposalTerminalParts {
        UiServiceProposalTerminalParts {
            owners_requiring_discard: self.candidate.demand().participating_families(),
            candidate: self.candidate,
            leases: self.leases,
            retained_receipts: self.retained_receipts,
        }
    }
}

#[derive(Debug)]
pub(super) struct UiServiceProposalTerminalParts {
    pub(super) candidate: super::UiServiceProposalCandidate,
    pub(super) leases: Box<[super::super::UiServiceProposalOccupancyLease]>,
    pub(super) retained_receipts: u16,
    pub(super) owners_requiring_discard: super::super::UiServiceFamilyParticipation,
}

#[path = "staging/digest.rs"]
mod digest;
use digest::staged_batch_digest;

#[path = "staging/receipt.rs"]
mod receipt;

#[path = "staging/reveal_refinement.rs"]
mod reveal_refinement;

#[cfg(test)]
#[path = "staging/reveal_refinement_tests.rs"]
mod reveal_refinement_tests;

#[cfg(test)]
#[path = "staging/tests.rs"]
mod tests;
