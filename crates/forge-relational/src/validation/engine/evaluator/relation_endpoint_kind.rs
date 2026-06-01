use crate::config::data::CrossContextPolicy;
use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::LoweredEndpointKindContract;
use crate::validation::data::{
    InvariantClass, InvariantViolation, InvariantViolationFields, RelationEndpointBoundary,
};

use super::super::context::InvariantExecutionContext;
use super::common::{canonicalize_violations, entity_reference_kind, relation_violation};

pub(super) fn evaluate_endpoint_kind_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredEndpointKindContract,
) -> Vec<InvariantViolation> {
    let Some(scope) = context.relation_integrity_scope(contract.relation_kind_id) else {
        return Vec::new();
    };
    if scope.planned_edges.is_empty() {
        return Vec::new();
    }

    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    for edge in &scope.planned_edges {
        context.metrics().count_relation_endpoint_kind_checks(1);
        let source_kind = match entity_reference_kind(context, class, &edge.source) {
            Ok(Some(kind_id)) => kind_id,
            Ok(None) => continue,
            Err(violation) => {
                violations.push(violation);
                continue;
            }
        };
        let target_kind = match entity_reference_kind(context, class, &edge.target) {
            Ok(Some(kind_id)) => kind_id,
            Ok(None) => continue,
            Err(violation) => {
                violations.push(violation);
                continue;
            }
        };
        if !contract.allows_source_kind(source_kind) {
            violations.push(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' rejected source kind {:?} for relation kind {:?}",
                    contract.contract_id, source_kind, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindMismatch {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source: edge.source.clone(),
                    target: edge.target.clone(),
                    source_kind_id: source_kind,
                    target_kind_id: target_kind,
                    boundary: RelationEndpointBoundary::Source,
                },
            ));
        }
        if !contract.allows_target_kind(target_kind) {
            violations.push(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' rejected target kind {:?} for relation kind {:?}",
                    contract.contract_id, target_kind, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindMismatch {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source: edge.source.clone(),
                    target: edge.target.clone(),
                    source_kind_id: source_kind,
                    target_kind_id: target_kind,
                    boundary: RelationEndpointBoundary::Target,
                },
            ));
        }
        if !contract.self_edges_allowed && edge.source == edge.target {
            violations.push(relation_violation(
                class,
                DiagnosticCode::RelationEndpointKindViolation,
                format!(
                    "relation contract '{}' forbids self edges for relation kind {:?}",
                    contract.contract_id, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindSelfEdge {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source: edge.source.clone(),
                    target: edge.target.clone(),
                    self_edge: true,
                },
            ));
        }
        if edge.source.partition_id() != edge.target.partition_id()
            && contract.cross_context_policy != CrossContextPolicy::AllowExplicit
        {
            violations.push(relation_violation(
                class,
                DiagnosticCode::InvalidRelationEndpoint,
                format!(
                    "relation contract '{}' forbids cross-context endpoints for relation kind {:?}",
                    contract.contract_id, contract.relation_kind_id
                ),
                InvariantViolationFields::RelationEndpointKindCrossContext {
                    contract_id: contract.contract_id.clone(),
                    relation_kind_id: contract.relation_kind_id,
                    source_partition_id: edge.source.partition_id(),
                    target_partition_id: edge.target.partition_id(),
                },
            ));
        }
    }
    canonicalize_violations(violations)
}
