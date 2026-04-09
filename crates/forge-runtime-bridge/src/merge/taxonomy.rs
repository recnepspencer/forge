#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalRelationalMergeClass {
    AspectReconciliation,
    Deletion,
    TopologyRewire,
    PolicyResolvedConflict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeConsumptionClass {
    AspectReconciliationMerge,
    DeletionMerge,
    TopologyRewireMerge,
    PolicyResolvedConflictMerge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeOntologyLoweringKind {
    DirectWrapper,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeAuthorityBasisKind {
    OrderedMergeCommit,
    HistoricalMergeEnvelope,
    BranchHeadMergeArtifact,
    ReplayMergeRecord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeAuthoritativeLineageDisposition {
    CanonicalSuccessor,
    NoAuthoritativeSuccessor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeCausalFrontierDisposition {
    Admitted,
    Truncated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeSchemaPolicyDisposition {
    Admitted,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeStructuralAdvisoryDisposition {
    NotConsulted,
    AdvisoryConsistent,
    AdvisoryContradiction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergePrecedenceStage {
    MergeClassAdmission,
    AuthoritativeLineage,
    DeletionTopologyGate,
    CausalFrontierAdmissibility,
    SchemaPolicyOutcomeAdmissibility,
    StructuralAdvisoryRefinement,
    ContinuityOrRemapPublication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeStageDecisionClass {
    Admitted,
    Denied,
    Refined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeDenialClass {
    UnsupportedMergeClass,
    NoAuthoritativeSuccessor,
    DeletionGate,
    TopologyRewireGate,
    CausalFrontierTruncated,
    SchemaPolicyRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMergeRoutingOutcomeClass {
    ContinuityCandidate,
    StructuralContradiction,
    Denied,
}

#[cfg(test)]
mod tests {
    use super::{
        BridgeMergeAuthoritativeLineageDisposition, BridgeMergeAuthorityBasisKind,
        BridgeMergeCausalFrontierDisposition, BridgeMergeConsumptionClass, BridgeMergeDenialClass,
        BridgeMergeOntologyLoweringKind, BridgeMergePrecedenceStage,
        BridgeMergeRoutingOutcomeClass, BridgeMergeSchemaPolicyDisposition,
        BridgeMergeStageDecisionClass, BridgeMergeStructuralAdvisoryDisposition,
        CanonicalRelationalMergeClass,
    };

    #[test]
    fn merge_taxonomy_remains_closed_world_for_phase_m9_0() {
        let canonical = [
            CanonicalRelationalMergeClass::AspectReconciliation,
            CanonicalRelationalMergeClass::Deletion,
            CanonicalRelationalMergeClass::TopologyRewire,
            CanonicalRelationalMergeClass::PolicyResolvedConflict,
        ];
        let bridge = [
            BridgeMergeConsumptionClass::AspectReconciliationMerge,
            BridgeMergeConsumptionClass::DeletionMerge,
            BridgeMergeConsumptionClass::TopologyRewireMerge,
            BridgeMergeConsumptionClass::PolicyResolvedConflictMerge,
        ];
        let lowering = BridgeMergeOntologyLoweringKind::DirectWrapper;
        let authority_basis = [
            BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            BridgeMergeAuthorityBasisKind::HistoricalMergeEnvelope,
            BridgeMergeAuthorityBasisKind::BranchHeadMergeArtifact,
            BridgeMergeAuthorityBasisKind::ReplayMergeRecord,
        ];
        let precedence = [
            BridgeMergePrecedenceStage::MergeClassAdmission,
            BridgeMergePrecedenceStage::AuthoritativeLineage,
            BridgeMergePrecedenceStage::DeletionTopologyGate,
            BridgeMergePrecedenceStage::CausalFrontierAdmissibility,
            BridgeMergePrecedenceStage::SchemaPolicyOutcomeAdmissibility,
            BridgeMergePrecedenceStage::StructuralAdvisoryRefinement,
            BridgeMergePrecedenceStage::ContinuityOrRemapPublication,
        ];
        let stage_decisions = [
            BridgeMergeStageDecisionClass::Admitted,
            BridgeMergeStageDecisionClass::Denied,
            BridgeMergeStageDecisionClass::Refined,
        ];
        let denial_classes = [
            BridgeMergeDenialClass::UnsupportedMergeClass,
            BridgeMergeDenialClass::NoAuthoritativeSuccessor,
            BridgeMergeDenialClass::DeletionGate,
            BridgeMergeDenialClass::TopologyRewireGate,
            BridgeMergeDenialClass::CausalFrontierTruncated,
            BridgeMergeDenialClass::SchemaPolicyRejected,
        ];
        let lineage_dispositions = [
            BridgeMergeAuthoritativeLineageDisposition::CanonicalSuccessor,
            BridgeMergeAuthoritativeLineageDisposition::NoAuthoritativeSuccessor,
        ];
        let causal_dispositions = [
            BridgeMergeCausalFrontierDisposition::Admitted,
            BridgeMergeCausalFrontierDisposition::Truncated,
        ];
        let policy_dispositions = [
            BridgeMergeSchemaPolicyDisposition::Admitted,
            BridgeMergeSchemaPolicyDisposition::Rejected,
        ];
        let structural_dispositions = [
            BridgeMergeStructuralAdvisoryDisposition::NotConsulted,
            BridgeMergeStructuralAdvisoryDisposition::AdvisoryConsistent,
            BridgeMergeStructuralAdvisoryDisposition::AdvisoryContradiction,
        ];
        let routing_outcomes = [
            BridgeMergeRoutingOutcomeClass::ContinuityCandidate,
            BridgeMergeRoutingOutcomeClass::StructuralContradiction,
            BridgeMergeRoutingOutcomeClass::Denied,
        ];

        assert_eq!(canonical.len(), bridge.len());
        assert_eq!(lowering, BridgeMergeOntologyLoweringKind::DirectWrapper);
        assert_eq!(authority_basis.len(), 4);
        assert_eq!(precedence.len(), 7);
        assert_eq!(stage_decisions.len(), 3);
        assert_eq!(denial_classes.len(), 6);
        assert_eq!(lineage_dispositions.len(), 2);
        assert_eq!(causal_dispositions.len(), 2);
        assert_eq!(policy_dispositions.len(), 2);
        assert_eq!(structural_dispositions.len(), 3);
        assert_eq!(routing_outcomes.len(), 3);
    }
}
