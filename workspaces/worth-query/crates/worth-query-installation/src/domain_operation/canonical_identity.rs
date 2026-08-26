mod conditional_nodes;
mod input_and_graph_contracts;
mod lifecycle_and_support_contracts;
mod workflow_contract;

use sha2::{Digest, Sha256};

use crate::canonical_hash_encoding::{
    hash_text_field, CanonicalHashSink, CanonicalHashWorkCounter,
};

use super::*;

pub(crate) trait WorthQueryDomainOperationCanonicalSemantics {
    fn parameters(&self) -> &WorthQueryOperationParameterContract;
    fn native_projection(&self) -> &WorthQueryOperationNativeProjectionContract;
    fn query_intent_digest(&self) -> &str;
    fn result_shape_digest(&self) -> &str;
    fn collection(&self) -> &WorthQueryOperationCollectionContract;
    fn required_capabilities(&self) -> &[WorthQueryOperationCapabilityRequirement];
    fn required_domains(&self) -> &[WorthQueryOperationRequiredDomainRole];
    fn conditional_nodes(&self) -> &[WorthQueryPortableConditionalNodeDeclaration];
    fn graph_reads(&self) -> &WorthQueryOperationGraphReadContract;
    fn touches(&self) -> &WorthQueryOperationTouchContract;
    fn effects(&self) -> &WorthQueryOperationEffectContract;
    fn invariants(&self) -> &WorthQueryOperationInvariantContract;
    fn invariant_execution(&self) -> &WorthQueryInvariantExecutionContract;
    fn decision_facts(&self) -> &WorthQueryOperationDecisionFactContract;
    fn evidence(&self) -> &WorthQueryDomainEvidenceContract;
    fn workflow(&self) -> &WorthQueryOperationWorkflowContract;
    fn resources(&self)
        -> &crate::domain_computation::WorthQueryOperationExecutionResourceContract;
    fn replay(&self) -> &WorthQueryOperationReplayContract;
    fn aftermath(
        &self,
    ) -> Option<&crate::application_aftermath::WorthQueryInstalledAftermathContract>;
    fn lineage(&self) -> WorthQueryOperationLineageContract;
    fn promotion(&self) -> WorthQueryOperationPromotionContract;
    fn publication(&self) -> &WorthQueryOperationPublicationContract;
    fn projection_consumption(&self) -> WorthQueryOperationProjectionConsumptionContract;
    fn terminal(&self) -> &WorthQueryOperationTerminalContract;
    fn cost(&self) -> WorthQueryOperationCostContract;
    fn support(&self) -> WorthQueryOperationSupportRequirements;
    fn lowering(&self) -> &WorthQueryOperationLoweringContract;
}

impl WorthQueryDomainOperationCanonicalSemantics for WorthQueryDomainOperationSemanticClosure {
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
        self.aftermath.as_ref()
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

pub(super) fn canonical_operation_identity(
    identity: &WorthQueryDomainOperationIdentity,
    semantics: &WorthQueryDomainOperationSemanticClosure,
) -> String {
    let mut hasher = Sha256::new();
    append_operation_identity(&mut hasher, identity, semantics);
    format!("{:x}", hasher.finalize())
}

pub(crate) fn canonical_operation_encoded_bytes(
    identity: &WorthQueryDomainOperationIdentity,
    semantics: &impl WorthQueryDomainOperationCanonicalSemantics,
) -> u64 {
    canonical_operation_reconstruction_work(identity, semantics).0
}

pub(crate) fn canonical_operation_reconstruction_work(
    identity: &WorthQueryDomainOperationIdentity,
    semantics: &impl WorthQueryDomainOperationCanonicalSemantics,
) -> (u64, u64) {
    let mut counter = CanonicalHashWorkCounter::default();
    append_operation_identity(&mut counter, identity, semantics);
    (counter.bytes(), counter.text_fields())
}

fn append_operation_identity(
    hasher: &mut impl CanonicalHashSink,
    identity: &WorthQueryDomainOperationIdentity,
    semantics: &impl WorthQueryDomainOperationCanonicalSemantics,
) {
    hash_text_field(hasher, "operation-name", identity.name());
    hash_text_field(hasher, "operation-version", &identity.version().to_string());
    input_and_graph_contracts::hash_input_and_graph_contracts(hasher, semantics);
    for family in semantics.decision_facts().required_families() {
        hash_text_field(hasher, "decision-fact-family", &family.canonical_token());
    }
    hash_text_field(
        hasher,
        "operation-evidence",
        &semantics.evidence().canonical_token(),
    );
    workflow_contract::hash_workflow_contract(hasher, semantics.workflow());
    hash_text_field(
        hasher,
        "operation-resources",
        &semantics.resources().canonical_token(),
    );
    lifecycle_and_support_contracts::hash_lifecycle_and_support_contracts(hasher, semantics);
}

pub(super) fn hash_sequence<'a>(
    hasher: &mut impl CanonicalHashSink,
    tag: &'static str,
    values: impl IntoIterator<Item = &'a str>,
) {
    for value in values {
        hash_text_field(hasher, tag, value);
    }
}

pub(super) fn bool_name(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}
