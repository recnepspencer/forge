use crate::domain_installation::operation_identity_basis::aftermath_material;
use crate::identity::hash_parts;

use super::{
    aftermath_authority_basis, WorthQueryAftermathAuthorityProof, WorthQueryAftermathCounters,
    WorthQueryAftermathKind,
};
use crate::domain_installation::WorthQueryAftermathPostcondition;

pub struct WorthQueryAftermathRelationReceipt {
    identity: String,
    original_trace_identity: String,
    aftermath_execution_identity: String,
    kind: WorthQueryAftermathKind,
    postcondition: WorthQueryAftermathPostcondition,
    counters: WorthQueryAftermathCounters,
    proof: WorthQueryAftermathAuthorityProof,
    foundational_attachment: super::WorthQueryFoundationalAftermathAttachment,
}

impl WorthQueryAftermathRelationReceipt {
    pub fn identity(&self) -> &str {
        &self.identity
    }
    pub fn original_trace_identity(&self) -> &str {
        &self.original_trace_identity
    }
    pub fn aftermath_execution_identity(&self) -> &str {
        &self.aftermath_execution_identity
    }
    pub fn kind(&self) -> WorthQueryAftermathKind {
        self.kind
    }
    pub fn postcondition(&self) -> &WorthQueryAftermathPostcondition {
        &self.postcondition
    }
    pub fn counters(&self) -> WorthQueryAftermathCounters {
        self.counters
    }
    pub fn authority_identity(&self) -> &str {
        self.proof.payload().identity()
    }
    pub fn runtime_authority(&self) -> u64 {
        aftermath_authority_basis(&self.proof).runtime_authority
    }
    pub fn installation_generation(&self) -> u64 {
        aftermath_authority_basis(&self.proof).installation_generation
    }
    pub fn basis_identity(&self) -> &str {
        &aftermath_authority_basis(&self.proof).basis_identity
    }
    pub fn original_operation_identity(&self) -> &str {
        &aftermath_authority_basis(&self.proof).original_operation_identity
    }
    pub fn aftermath_operation_identity(&self) -> &str {
        &aftermath_authority_basis(&self.proof).candidate_operation_identity
    }
    pub fn original_binding_identity(&self) -> &str {
        &aftermath_authority_basis(&self.proof).original_binding_identity
    }
    pub fn aftermath_binding_identity(&self) -> &str {
        &aftermath_authority_basis(&self.proof).candidate_binding_identity
    }
    pub fn original_capability_identity(&self) -> u64 {
        aftermath_authority_basis(&self.proof).original_capability_identity
    }
    pub fn aftermath_capability_identity(&self) -> u64 {
        aftermath_authority_basis(&self.proof).candidate_capability_identity
    }
    pub fn effect_receipt_identities(&self) -> &[String] {
        &aftermath_authority_basis(&self.proof).effect_receipt_identities
    }
    pub fn original_lineage_report_identity(&self) -> Option<&str> {
        aftermath_authority_basis(&self.proof)
            .original_lineage_report_identity
            .as_deref()
    }
    pub fn foundational_attachment(&self) -> &super::WorthQueryFoundationalAftermathAttachment {
        &self.foundational_attachment
    }
}

pub(crate) fn mint_aftermath_relation(
    proof: WorthQueryAftermathAuthorityProof,
    original_trace_identity: &str,
    kind: WorthQueryAftermathKind,
    postcondition: &WorthQueryAftermathPostcondition,
    aftermath_execution_identity: &str,
    mut counters: WorthQueryAftermathCounters,
) -> WorthQueryAftermathRelationReceipt {
    counters.execution_contacts += 1;
    let admission_identity = proof.payload().identity();
    let identity = hash_parts(&[
        "worth_query_aftermath_relation_v1".into(),
        format!("admission:{admission_identity}"),
        format!("original:{original_trace_identity}"),
        format!("aftermath:{aftermath_execution_identity}"),
        aftermath_material(kind, postcondition),
    ]);
    let foundational_attachment = super::materialize_aftermath_attachment(
        original_trace_identity,
        aftermath_execution_identity,
        kind,
    );
    WorthQueryAftermathRelationReceipt {
        identity,
        original_trace_identity: original_trace_identity.to_owned(),
        aftermath_execution_identity: aftermath_execution_identity.to_owned(),
        kind,
        postcondition: postcondition.clone(),
        counters,
        proof,
        foundational_attachment,
    }
}
