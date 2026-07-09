use std::sync::Arc;

use crate::merge::{
    BridgeMergeAuthoritativeLineageDisposition, BridgeMergeCausalFrontierDisposition,
    BridgeMergeConsumptionClass, BridgeMergeDenialClass, BridgeMergePrecedenceStage,
    BridgeMergeSchemaPolicyDisposition, BridgeMergeStageDecisionClass,
    BridgeMergeStructuralAdvisoryDisposition,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeClassAdmissionStage {
    bridge_class: BridgeMergeConsumptionClass,
}

impl MergeClassAdmissionStage {
    pub(crate) fn new(bridge_class: BridgeMergeConsumptionClass) -> Self {
        Self { bridge_class }
    }

    pub fn bridge_class(&self) -> BridgeMergeConsumptionClass {
        self.bridge_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeAuthoritativeLineageStage {
    disposition: BridgeMergeAuthoritativeLineageDisposition,
}

impl MergeAuthoritativeLineageStage {
    pub(crate) fn new(disposition: BridgeMergeAuthoritativeLineageDisposition) -> Self {
        Self { disposition }
    }

    pub fn disposition(&self) -> BridgeMergeAuthoritativeLineageDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeDeletionTopologyGateStage {
    bridge_class: BridgeMergeConsumptionClass,
    denial_class: Option<BridgeMergeDenialClass>,
}

impl MergeDeletionTopologyGateStage {
    pub(crate) fn new(
        bridge_class: BridgeMergeConsumptionClass,
        denial_class: Option<BridgeMergeDenialClass>,
    ) -> Self {
        Self {
            bridge_class,
            denial_class,
        }
    }

    pub fn bridge_class(&self) -> BridgeMergeConsumptionClass {
        self.bridge_class
    }

    pub fn denial_class(&self) -> Option<BridgeMergeDenialClass> {
        self.denial_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeCausalFrontierStage {
    disposition: BridgeMergeCausalFrontierDisposition,
}

impl MergeCausalFrontierStage {
    pub(crate) fn new(disposition: BridgeMergeCausalFrontierDisposition) -> Self {
        Self { disposition }
    }

    pub fn disposition(&self) -> BridgeMergeCausalFrontierDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeSchemaPolicyStage {
    disposition: BridgeMergeSchemaPolicyDisposition,
}

impl MergeSchemaPolicyStage {
    pub(crate) fn new(disposition: BridgeMergeSchemaPolicyDisposition) -> Self {
        Self { disposition }
    }

    pub fn disposition(&self) -> BridgeMergeSchemaPolicyDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeStructuralAdvisoryStage {
    disposition: BridgeMergeStructuralAdvisoryDisposition,
}

impl MergeStructuralAdvisoryStage {
    pub(crate) fn new(disposition: BridgeMergeStructuralAdvisoryDisposition) -> Self {
        Self { disposition }
    }

    pub fn disposition(&self) -> BridgeMergeStructuralAdvisoryDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergePublicationStage;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergePrecedenceStageOutput {
    MergeClassAdmission(MergeClassAdmissionStage),
    AuthoritativeLineage(MergeAuthoritativeLineageStage),
    DeletionTopologyGate(MergeDeletionTopologyGateStage),
    CausalFrontierAdmissibility(MergeCausalFrontierStage),
    SchemaPolicyOutcomeAdmissibility(MergeSchemaPolicyStage),
    StructuralAdvisoryRefinement(MergeStructuralAdvisoryStage),
    ContinuityOrRemapPublication(MergePublicationStage),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeDecisionLogEntry {
    stage: BridgeMergePrecedenceStage,
    decision_class: BridgeMergeStageDecisionClass,
    detail: Arc<str>,
}

impl MergeDecisionLogEntry {
    pub(crate) fn new(
        stage: BridgeMergePrecedenceStage,
        decision_class: BridgeMergeStageDecisionClass,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            stage,
            decision_class,
            detail: detail.into(),
        }
    }

    pub fn stage(&self) -> BridgeMergePrecedenceStage {
        self.stage
    }

    pub fn decision_class(&self) -> BridgeMergeStageDecisionClass {
        self.decision_class
    }

    pub fn detail(&self) -> &str {
        self.detail.as_ref()
    }
}
