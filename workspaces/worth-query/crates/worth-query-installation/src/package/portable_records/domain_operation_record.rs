//! Authority-free projection of one validated portable domain operation.

use crate::domain_operation::*;

/// Complete descriptive operation meaning without installation aftermath state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPortableDomainOperationSemanticRecord {
    parameters: WorthQueryOperationParameterContract,
    native_projection: WorthQueryOperationNativeProjectionContract,
    canonical_query: worth_query_declaration::facade::canonicalization::WorthQueryPortableCanonicalQueryBundleRecord,
    collection: WorthQueryOperationCollectionContract,
    required_capabilities: Vec<WorthQueryOperationCapabilityRequirement>,
    required_domains: Vec<WorthQueryOperationRequiredDomainRole>,
    workflow: WorthQueryOperationWorkflowContract,
    evidence: WorthQueryDomainEvidenceContract,
    conditional_nodes: Vec<WorthQueryPortableConditionalNodeDeclaration>,
    graph_reads: WorthQueryOperationGraphReadContract,
    decision_facts: WorthQueryOperationDecisionFactContract,
    touches: WorthQueryOperationTouchContract,
    effects: WorthQueryOperationEffectContract,
    invariants: WorthQueryOperationInvariantContract,
    invariant_execution: WorthQueryInvariantExecutionContract,
    replay: WorthQueryOperationReplayContract,
    lineage: WorthQueryOperationLineageContract,
    promotion: WorthQueryOperationPromotionContract,
    publication: WorthQueryOperationPublicationContract,
    projection_consumption: WorthQueryOperationProjectionConsumptionContract,
    terminal: WorthQueryOperationTerminalContract,
    cost: WorthQueryOperationCostContract,
    resources: crate::domain_computation::WorthQueryOperationExecutionResourceContract,
    support: WorthQueryOperationSupportRequirements,
    lowering: WorthQueryOperationLoweringContract,
}

impl WorthQueryPortableDomainOperationSemanticRecord {
    fn project(source: &WorthQueryDomainOperationSemanticClosure) -> Self {
        debug_assert!(source.aftermath.is_none());
        Self {
            parameters: source.parameters.clone(),
            native_projection: source.native_projection.clone(),
            canonical_query: worth_query_declaration::facade::canonicalization::WorthQueryPortableCanonicalQueryBundleRecord::project(&source.canonical_query),
            collection: source.collection.clone(),
            required_capabilities: source.required_capabilities.clone(),
            required_domains: source.required_domains.clone(),
            workflow: source.workflow.clone(),
            evidence: source.evidence.clone(),
            conditional_nodes: source.conditional_nodes.clone(),
            graph_reads: source.graph_reads.clone(),
            decision_facts: source.decision_facts.clone(),
            touches: source.touches.clone(),
            effects: source.effects.clone(),
            invariants: source.invariants.clone(),
            invariant_execution: source.invariant_execution.clone(),
            replay: source.replay,
            lineage: source.lineage,
            promotion: source.promotion,
            publication: source.publication.clone(),
            projection_consumption: source.projection_consumption,
            terminal: source.terminal.clone(),
            cost: source.cost,
            resources: source.resources.clone(),
            support: source.support,
            lowering: source.lowering.clone(),
        }
    }

    pub const fn parameters(&self) -> &WorthQueryOperationParameterContract {
        &self.parameters
    }
    pub const fn native_projection(&self) -> &WorthQueryOperationNativeProjectionContract {
        &self.native_projection
    }
    pub const fn canonical_query(
        &self,
    ) -> &worth_query_declaration::facade::canonicalization::WorthQueryPortableCanonicalQueryBundleRecord{
        &self.canonical_query
    }
    pub const fn collection(&self) -> &WorthQueryOperationCollectionContract {
        &self.collection
    }
    pub fn required_capabilities(&self) -> &[WorthQueryOperationCapabilityRequirement] {
        &self.required_capabilities
    }
    pub fn required_domains(&self) -> &[WorthQueryOperationRequiredDomainRole] {
        &self.required_domains
    }
    pub const fn workflow(&self) -> &WorthQueryOperationWorkflowContract {
        &self.workflow
    }
    pub const fn evidence(&self) -> &WorthQueryDomainEvidenceContract {
        &self.evidence
    }
    pub fn conditional_nodes(&self) -> &[WorthQueryPortableConditionalNodeDeclaration] {
        &self.conditional_nodes
    }
    pub const fn graph_reads(&self) -> &WorthQueryOperationGraphReadContract {
        &self.graph_reads
    }
    pub const fn decision_facts(&self) -> &WorthQueryOperationDecisionFactContract {
        &self.decision_facts
    }
    pub const fn touches(&self) -> &WorthQueryOperationTouchContract {
        &self.touches
    }
    pub const fn effects(&self) -> &WorthQueryOperationEffectContract {
        &self.effects
    }
    pub const fn invariants(&self) -> &WorthQueryOperationInvariantContract {
        &self.invariants
    }
    pub const fn invariant_execution(&self) -> &WorthQueryInvariantExecutionContract {
        &self.invariant_execution
    }
    pub const fn replay(&self) -> WorthQueryOperationReplayContract {
        self.replay
    }
    pub const fn lineage(&self) -> WorthQueryOperationLineageContract {
        self.lineage
    }
    pub const fn promotion(&self) -> WorthQueryOperationPromotionContract {
        self.promotion
    }
    pub const fn publication(&self) -> &WorthQueryOperationPublicationContract {
        &self.publication
    }
    pub const fn projection_consumption(&self) -> WorthQueryOperationProjectionConsumptionContract {
        self.projection_consumption
    }
    pub const fn terminal(&self) -> &WorthQueryOperationTerminalContract {
        &self.terminal
    }
    pub const fn cost(&self) -> WorthQueryOperationCostContract {
        self.cost
    }
    pub const fn resources(
        &self,
    ) -> &crate::domain_computation::WorthQueryOperationExecutionResourceContract {
        &self.resources
    }
    pub const fn support(&self) -> WorthQueryOperationSupportRequirements {
        self.support
    }
    pub const fn lowering(&self) -> &WorthQueryOperationLoweringContract {
        &self.lowering
    }
}

/// Stable portable record for one domain operation.
///
/// Canonicalization authority is intentionally unreachable through this
/// public export boundary:
///
/// ```compile_fail
/// fn cannot_recover_canonical_authority(
///     record: &worth_query_installation::facade::WorthQueryPortableDomainOperationRecord,
/// ) {
///     let _authority = record.semantics().canonical_query().query().authority();
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPortableDomainOperationRecord {
    identity: WorthQueryDomainOperationIdentity,
    semantics: WorthQueryPortableDomainOperationSemanticRecord,
    canonical_identity: String,
}

impl WorthQueryPortableDomainOperationRecord {
    pub(crate) fn project(source: &WorthQueryPortableDomainOperationDefinition) -> Self {
        Self {
            identity: source.identity().clone(),
            semantics: WorthQueryPortableDomainOperationSemanticRecord::project(source.semantics()),
            canonical_identity: source.canonical_identity().to_owned(),
        }
    }

    pub const fn identity(&self) -> &WorthQueryDomainOperationIdentity {
        &self.identity
    }

    pub const fn semantics(&self) -> &WorthQueryPortableDomainOperationSemanticRecord {
        &self.semantics
    }

    pub fn canonical_identity(&self) -> &str {
        &self.canonical_identity
    }
}
