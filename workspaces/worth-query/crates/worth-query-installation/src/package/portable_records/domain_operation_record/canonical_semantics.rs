//! Canonical-operation view of authority-free portable semantics.

use super::*;

impl crate::domain_operation::WorthQueryDomainOperationCanonicalSemantics
    for WorthQueryPortableDomainOperationSemanticRecord
{
    fn parameters(&self) -> &WorthQueryOperationParameterContract {
        &self.parameters
    }
    fn native_projection(&self) -> &WorthQueryOperationNativeProjectionContract {
        &self.native_projection
    }
    fn query_intent_digest(&self) -> &str {
        self.canonical_query.query().digest().as_str()
    }
    fn result_shape_digest(&self) -> &str {
        self.canonical_query.result_shape().digest().as_str()
    }
    fn collection(&self) -> &WorthQueryOperationCollectionContract {
        &self.collection
    }
    fn required_capabilities(&self) -> &[WorthQueryOperationCapabilityRequirement] {
        &self.required_capabilities
    }
    fn required_domains(&self) -> &[WorthQueryOperationRequiredDomainRole] {
        &self.required_domains
    }
    fn conditional_nodes(&self) -> &[WorthQueryPortableConditionalNodeDeclaration] {
        &self.conditional_nodes
    }
    fn graph_reads(&self) -> &WorthQueryOperationGraphReadContract {
        &self.graph_reads
    }
    fn touches(&self) -> &WorthQueryOperationTouchContract {
        &self.touches
    }
    fn effects(&self) -> &WorthQueryOperationEffectContract {
        &self.effects
    }
    fn invariants(&self) -> &WorthQueryOperationInvariantContract {
        &self.invariants
    }
    fn invariant_execution(&self) -> &WorthQueryInvariantExecutionContract {
        &self.invariant_execution
    }
    fn decision_facts(&self) -> &WorthQueryOperationDecisionFactContract {
        &self.decision_facts
    }
    fn evidence(&self) -> &WorthQueryDomainEvidenceContract {
        &self.evidence
    }
    fn workflow(&self) -> &WorthQueryOperationWorkflowContract {
        &self.workflow
    }
    fn resources(
        &self,
    ) -> &crate::domain_computation::WorthQueryOperationExecutionResourceContract {
        &self.resources
    }
    fn replay(&self) -> &WorthQueryOperationReplayContract {
        &self.replay
    }
    fn aftermath(
        &self,
    ) -> Option<&crate::application_aftermath::WorthQueryInstalledAftermathContract> {
        None
    }
    fn lineage(&self) -> WorthQueryOperationLineageContract {
        self.lineage
    }
    fn promotion(&self) -> WorthQueryOperationPromotionContract {
        self.promotion
    }
    fn publication(&self) -> &WorthQueryOperationPublicationContract {
        &self.publication
    }
    fn projection_consumption(&self) -> WorthQueryOperationProjectionConsumptionContract {
        self.projection_consumption
    }
    fn terminal(&self) -> &WorthQueryOperationTerminalContract {
        &self.terminal
    }
    fn cost(&self) -> WorthQueryOperationCostContract {
        self.cost
    }
    fn support(&self) -> WorthQueryOperationSupportRequirements {
        self.support
    }
    fn lowering(&self) -> &WorthQueryOperationLoweringContract {
        &self.lowering
    }
}
