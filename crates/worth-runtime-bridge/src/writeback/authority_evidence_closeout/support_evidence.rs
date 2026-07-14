#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeMutationEvidenceCarryForwardSection {
    DeclaredResolvedTargetEvidence,
    BatchSessionCausalityProvenance,
    ExistingTruthBinding,
    SameBatchSymbolicTargetReference,
    NamingMutationEvidence,
    ContinuityMutationEvidence,
    ReplaySafeRequestReceiptDigests,
}

impl BridgeMutationEvidenceCarryForwardSection {
    pub(super) const fn digest_entry(self) -> &'static str {
        match self {
            Self::DeclaredResolvedTargetEvidence => "declared-resolved-target-evidence",
            Self::BatchSessionCausalityProvenance => "batch-session-causality-provenance",
            Self::ExistingTruthBinding => "existing-truth-binding",
            Self::SameBatchSymbolicTargetReference => "same-batch-symbolic-target-reference",
            Self::NamingMutationEvidence => "naming-mutation-evidence",
            Self::ContinuityMutationEvidence => "continuity-mutation-evidence",
            Self::ReplaySafeRequestReceiptDigests => "replay-safe-request-receipt-digests",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeMutationEvidenceExistingTruthBindingFamily {
    DirectEntityIdentity,
    DirectRelationIdentity,
}

impl BridgeMutationEvidenceExistingTruthBindingFamily {
    pub(super) const fn digest_entry(self) -> &'static str {
        match self {
            Self::DirectEntityIdentity => "direct-entity-identity",
            Self::DirectRelationIdentity => "direct-relation-identity",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeMutationEvidenceSymbolicTargetReferenceFamily {
    SameBatchDeclaredTarget,
}

impl BridgeMutationEvidenceSymbolicTargetReferenceFamily {
    pub(super) const fn digest_entry(self) -> &'static str {
        match self {
            Self::SameBatchDeclaredTarget => "same-batch-declared-target",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeMutationEvidenceNamingFamily {
    AttachNewTarget,
    AttachExistingTarget,
    RebindTarget,
    Remove,
}

impl BridgeMutationEvidenceNamingFamily {
    pub(super) const fn digest_entry(self) -> &'static str {
        match self {
            Self::AttachNewTarget => "attach-new-target",
            Self::AttachExistingTarget => "attach-existing-target",
            Self::RebindTarget => "rebind-target",
            Self::Remove => "remove",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeMutationEvidenceContinuityFamily {
    RebindExistingTarget,
    SplitExistingTarget,
}

impl BridgeMutationEvidenceContinuityFamily {
    pub(super) const fn digest_entry(self) -> &'static str {
        match self {
            Self::RebindExistingTarget => "rebind-existing-target",
            Self::SplitExistingTarget => "split-existing-target",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeAggregateMutationEvidenceDigest {
    ExistingTruthBinding,
    SymbolicTargetReference,
    NamingMutation,
    ContinuityMutation,
    Causality,
    Provenance,
}

impl BridgeAggregateMutationEvidenceDigest {
    pub(super) const fn digest_entry(self) -> &'static str {
        match self {
            Self::ExistingTruthBinding => "aggregate-existing-truth-binding",
            Self::SymbolicTargetReference => "aggregate-symbolic-target-reference",
            Self::NamingMutation => "aggregate-naming-mutation",
            Self::ContinuityMutation => "aggregate-continuity-mutation",
            Self::Causality => "aggregate-causality",
            Self::Provenance => "aggregate-provenance",
        }
    }
}

pub(super) fn standard_carry_forward_sections() -> Vec<BridgeMutationEvidenceCarryForwardSection> {
    vec![
        BridgeMutationEvidenceCarryForwardSection::DeclaredResolvedTargetEvidence,
        BridgeMutationEvidenceCarryForwardSection::BatchSessionCausalityProvenance,
        BridgeMutationEvidenceCarryForwardSection::ExistingTruthBinding,
        BridgeMutationEvidenceCarryForwardSection::SameBatchSymbolicTargetReference,
        BridgeMutationEvidenceCarryForwardSection::NamingMutationEvidence,
        BridgeMutationEvidenceCarryForwardSection::ContinuityMutationEvidence,
        BridgeMutationEvidenceCarryForwardSection::ReplaySafeRequestReceiptDigests,
    ]
}

pub(super) fn standard_existing_truth_binding_families(
) -> Vec<BridgeMutationEvidenceExistingTruthBindingFamily> {
    vec![
        BridgeMutationEvidenceExistingTruthBindingFamily::DirectEntityIdentity,
        BridgeMutationEvidenceExistingTruthBindingFamily::DirectRelationIdentity,
    ]
}

pub(super) fn standard_symbolic_target_reference_families(
) -> Vec<BridgeMutationEvidenceSymbolicTargetReferenceFamily> {
    vec![BridgeMutationEvidenceSymbolicTargetReferenceFamily::SameBatchDeclaredTarget]
}

pub(super) fn standard_naming_mutation_families() -> Vec<BridgeMutationEvidenceNamingFamily> {
    vec![
        BridgeMutationEvidenceNamingFamily::AttachNewTarget,
        BridgeMutationEvidenceNamingFamily::AttachExistingTarget,
        BridgeMutationEvidenceNamingFamily::RebindTarget,
        BridgeMutationEvidenceNamingFamily::Remove,
    ]
}

pub(super) fn standard_continuity_mutation_families() -> Vec<BridgeMutationEvidenceContinuityFamily>
{
    vec![
        BridgeMutationEvidenceContinuityFamily::RebindExistingTarget,
        BridgeMutationEvidenceContinuityFamily::SplitExistingTarget,
    ]
}

pub(super) fn standard_aggregate_evidence_digests() -> Vec<BridgeAggregateMutationEvidenceDigest> {
    vec![
        BridgeAggregateMutationEvidenceDigest::ExistingTruthBinding,
        BridgeAggregateMutationEvidenceDigest::SymbolicTargetReference,
        BridgeAggregateMutationEvidenceDigest::NamingMutation,
        BridgeAggregateMutationEvidenceDigest::ContinuityMutation,
        BridgeAggregateMutationEvidenceDigest::Causality,
        BridgeAggregateMutationEvidenceDigest::Provenance,
    ]
}
