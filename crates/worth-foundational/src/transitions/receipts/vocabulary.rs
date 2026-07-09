use crate::boundary_artifacts::{
    claim_receipt_evidence_boundary_surface, claim_support_only_boundary_surface,
    FoundationalBoundaryCategoryConstructionDenial, FoundationalBoundaryReceiptSurface,
    FoundationalBoundarySummarySurface, FoundationalReceiptEvidenceBoundaryClaim,
    FoundationalSupportOnlyBoundaryClaim,
};
use crate::identities::BoundaryHandle;
use crate::transitions::{
    FoundationalAuthorityTransitionClass, FoundationalBranchComparisonBasis,
    FoundationalBranchForkBasis, FoundationalBranchId, FoundationalBranchObservationBasis,
    FoundationalCommitDeltaSummary, FoundationalCommitParentBasis, FoundationalMergeBasis,
    FoundationalNoOpCause, FoundationalTransitionCorrespondenceBasis,
    FoundationalTransitionRemapBasis, FoundationalTransitionStrategyDescriptorDigest,
    FoundationalTransitionStrategyIdentity, FoundationalTransitionStrategyOwnershipClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalCommitId(BoundaryHandle);

impl FoundationalCommitId {
    pub const fn new(handle: BoundaryHandle) -> Self {
        Self(handle)
    }

    pub const fn handle(&self) -> BoundaryHandle {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalCommitReceiptIdentity(BoundaryHandle);

impl FoundationalCommitReceiptIdentity {
    pub const fn new(handle: BoundaryHandle) -> Self {
        Self(handle)
    }

    pub const fn handle(&self) -> BoundaryHandle {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalTransitionIssuanceCause {
    CommitAttested,
    MetadataOnlyCommitAttested,
    PromotionCommitAttested,
    ReplayRevalidatedCommitAttested,
    NoOpAttested,
}

impl FoundationalTransitionIssuanceCause {
    pub const fn for_transition_class(class: FoundationalAuthorityTransitionClass) -> Self {
        match class {
            FoundationalAuthorityTransitionClass::Commit => Self::CommitAttested,
            FoundationalAuthorityTransitionClass::MetadataOnlyCommit => {
                Self::MetadataOnlyCommitAttested
            }
            FoundationalAuthorityTransitionClass::PromotionCommit => Self::PromotionCommitAttested,
            FoundationalAuthorityTransitionClass::ReplayRevalidatedCommit => {
                Self::ReplayRevalidatedCommitAttested
            }
            FoundationalAuthorityTransitionClass::NoOp => Self::NoOpAttested,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalTransitionProvenanceRow {
    source_branch: FoundationalBranchId,
    target_branch: FoundationalBranchId,
    parent_basis: FoundationalCommitParentBasis,
    merge_basis: FoundationalMergeBasis,
    transition_class: FoundationalAuthorityTransitionClass,
    no_op_cause: Option<FoundationalNoOpCause>,
    strategy_identity: FoundationalTransitionStrategyIdentity,
    strategy_descriptor_digest: FoundationalTransitionStrategyDescriptorDigest,
    observation_basis: FoundationalBranchObservationBasis,
    comparison_basis: Option<FoundationalBranchComparisonBasis>,
    correspondence_basis: Option<FoundationalTransitionCorrespondenceBasis>,
    remap_basis: Option<FoundationalTransitionRemapBasis>,
    issuance_cause: Option<FoundationalTransitionIssuanceCause>,
    commit_id: Option<FoundationalCommitId>,
    receipt_identity: Option<FoundationalCommitReceiptIdentity>,
}

impl FoundationalTransitionProvenanceRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        source_branch: FoundationalBranchId,
        target_branch: FoundationalBranchId,
        parent_basis: FoundationalCommitParentBasis,
        merge_basis: FoundationalMergeBasis,
        transition_class: FoundationalAuthorityTransitionClass,
        no_op_cause: Option<FoundationalNoOpCause>,
        strategy_identity: FoundationalTransitionStrategyIdentity,
        strategy_descriptor_digest: FoundationalTransitionStrategyDescriptorDigest,
        observation_basis: FoundationalBranchObservationBasis,
        comparison_basis: Option<FoundationalBranchComparisonBasis>,
        correspondence_basis: Option<FoundationalTransitionCorrespondenceBasis>,
        remap_basis: Option<FoundationalTransitionRemapBasis>,
        issuance_cause: Option<FoundationalTransitionIssuanceCause>,
        commit_id: Option<FoundationalCommitId>,
        receipt_identity: Option<FoundationalCommitReceiptIdentity>,
    ) -> Self {
        Self {
            source_branch,
            target_branch,
            parent_basis,
            merge_basis,
            transition_class,
            no_op_cause,
            strategy_identity,
            strategy_descriptor_digest,
            observation_basis,
            comparison_basis,
            correspondence_basis,
            remap_basis,
            issuance_cause,
            commit_id,
            receipt_identity,
        }
    }

    pub fn source_branch(&self) -> &FoundationalBranchId {
        &self.source_branch
    }

    pub fn target_branch(&self) -> &FoundationalBranchId {
        &self.target_branch
    }

    pub const fn parent_basis(&self) -> FoundationalCommitParentBasis {
        self.parent_basis
    }

    pub fn merge_basis(&self) -> &FoundationalMergeBasis {
        &self.merge_basis
    }

    pub const fn transition_class(&self) -> FoundationalAuthorityTransitionClass {
        self.transition_class
    }

    pub const fn no_op_cause(&self) -> Option<FoundationalNoOpCause> {
        self.no_op_cause
    }

    pub fn strategy_identity(&self) -> &FoundationalTransitionStrategyIdentity {
        &self.strategy_identity
    }

    pub const fn strategy_ownership(&self) -> FoundationalTransitionStrategyOwnershipClass {
        self.strategy_identity.ownership()
    }

    pub const fn strategy_descriptor_digest(
        &self,
    ) -> FoundationalTransitionStrategyDescriptorDigest {
        self.strategy_descriptor_digest
    }

    pub const fn observation_basis(&self) -> FoundationalBranchObservationBasis {
        self.observation_basis
    }

    pub fn comparison_basis(&self) -> Option<&FoundationalBranchComparisonBasis> {
        self.comparison_basis.as_ref()
    }

    pub const fn correspondence_basis(&self) -> Option<FoundationalTransitionCorrespondenceBasis> {
        self.correspondence_basis
    }

    pub const fn remap_basis(&self) -> Option<FoundationalTransitionRemapBasis> {
        self.remap_basis
    }

    pub const fn issuance_cause(&self) -> Option<FoundationalTransitionIssuanceCause> {
        self.issuance_cause
    }

    pub const fn commit_id(&self) -> Option<FoundationalCommitId> {
        self.commit_id
    }

    pub const fn receipt_identity(&self) -> Option<FoundationalCommitReceiptIdentity> {
        self.receipt_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalCommitReceiptIssuanceDenial {
    ReceiptSurface(FoundationalBoundaryCategoryConstructionDenial),
    CloseoutSurface(FoundationalBoundaryCategoryConstructionDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBranchCloseoutCause {
    ExplicitDiscard,
    ReplacedByRestaging,
    AbandonedAsInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalNonAuthoritativeResidueReport {
    retained_item_count: u32,
}

impl FoundationalNonAuthoritativeResidueReport {
    pub const fn zero() -> Self {
        Self {
            retained_item_count: 0,
        }
    }

    pub const fn retained_item_count(&self) -> u32 {
        self.retained_item_count
    }

    pub const fn is_zero_residue(&self) -> bool {
        self.retained_item_count == 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBranchDiscardReceipt {
    branch_id: FoundationalBranchId,
    fork_basis: FoundationalBranchForkBasis,
    closeout_cause: FoundationalBranchCloseoutCause,
    residue_report: FoundationalNonAuthoritativeResidueReport,
    summary: FoundationalSupportOnlyBoundaryClaim<FoundationalBoundarySummarySurface>,
}

impl FoundationalBranchDiscardReceipt {
    pub(crate) fn new(
        branch_id: FoundationalBranchId,
        fork_basis: FoundationalBranchForkBasis,
        closeout_cause: FoundationalBranchCloseoutCause,
    ) -> Result<Self, FoundationalCommitReceiptIssuanceDenial> {
        let summary = FoundationalBoundarySummarySurface::new(
            format!(
                "discarded non-authoritative branch-local work on {}",
                branch_id.as_str()
            ),
            3,
        )
        .map_err(FoundationalCommitReceiptIssuanceDenial::CloseoutSurface)?;

        Ok(Self {
            branch_id,
            fork_basis,
            closeout_cause,
            residue_report: FoundationalNonAuthoritativeResidueReport::zero(),
            summary: claim_support_only_boundary_surface(summary),
        })
    }

    pub fn branch_id(&self) -> &FoundationalBranchId {
        &self.branch_id
    }

    pub fn fork_basis(&self) -> &FoundationalBranchForkBasis {
        &self.fork_basis
    }

    pub const fn closeout_cause(&self) -> FoundationalBranchCloseoutCause {
        self.closeout_cause
    }

    pub const fn non_authoritative_residue_report(
        &self,
    ) -> FoundationalNonAuthoritativeResidueReport {
        self.residue_report
    }

    pub fn summary(
        &self,
    ) -> &FoundationalSupportOnlyBoundaryClaim<FoundationalBoundarySummarySurface> {
        &self.summary
    }
}

pub(crate) fn build_receipt_claim(
    branch_id: &FoundationalBranchId,
    commit_id: FoundationalCommitId,
    receipt_identity: FoundationalCommitReceiptIdentity,
    transition_class: FoundationalAuthorityTransitionClass,
    delta_summary: &FoundationalCommitDeltaSummary,
) -> Result<
    FoundationalReceiptEvidenceBoundaryClaim<FoundationalBoundaryReceiptSurface>,
    FoundationalCommitReceiptIssuanceDenial,
> {
    let surface = FoundationalBoundaryReceiptSurface::new(
        format!(
            "commit {:?} attests {:?} transition on {} with receipt {:?}",
            commit_id.handle(),
            transition_class,
            branch_id.as_str(),
            receipt_identity.handle(),
        ),
        delta_summary.delta_count() as usize,
    )
    .map_err(FoundationalCommitReceiptIssuanceDenial::ReceiptSurface)?;
    Ok(claim_receipt_evidence_boundary_surface(surface))
}

pub(crate) fn build_summary_claim(
    branch_id: &FoundationalBranchId,
    transition_class: FoundationalAuthorityTransitionClass,
    delta_summary: &FoundationalCommitDeltaSummary,
) -> Result<
    FoundationalSupportOnlyBoundaryClaim<FoundationalBoundarySummarySurface>,
    FoundationalCommitReceiptIssuanceDenial,
> {
    let surface = FoundationalBoundarySummarySurface::new(
        format!(
            "{:?} transition on {} touched {} committed loci",
            transition_class,
            branch_id.as_str(),
            delta_summary.delta_count(),
        ),
        4,
    )
    .map_err(FoundationalCommitReceiptIssuanceDenial::ReceiptSurface)?;
    Ok(claim_support_only_boundary_surface(surface))
}
