use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::LoweredPartitionIsolationContract;
use crate::validation::data::{InvariantClass, InvariantViolation, InvariantViolationFields};

use super::super::context::InvariantExecutionContext;
use super::common::canonicalize_violations;

pub(super) fn evaluate_partition_isolation_contract(
    context: &InvariantExecutionContext<'_>,
    class: InvariantClass,
    contract: &LoweredPartitionIsolationContract,
) -> Vec<InvariantViolation> {
    context.metrics().count_relation_contracts_evaluated(1);
    let mut violations = Vec::new();
    if let Some(plan) = context.merged_plan() {
        for intent in &plan.merged_intents {
            match intent {
                crate::transactions::data::MutationIntent::Create(
                    crate::transactions::data::CreateIntent::Relation(spec),
                ) if spec.kind_id == contract.relation_kind_id => {
                    context.metrics().count_relation_slot_scans(1);
                    if spec.source.partition_id() != spec.target.partition_id() {
                        violations.push(InvariantViolation {
                            class,
                            code: DiagnosticCode::InvariantViolation,
                            detail: format!(
                                "relation contract '{}' requires same-partition endpoints for relation kind {:?}",
                                contract.contract_id, contract.relation_kind_id
                            ),
                            fields: InvariantViolationFields::PartitionIsolation {
                                contract_id: contract.contract_id.clone(),
                                relation_kind_id: contract.relation_kind_id,
                                relation_id: None,
                                source_partition_id: spec.source.partition_id(),
                                target_partition_id: spec.target.partition_id(),
                            },
                        });
                    }
                }
                crate::transactions::data::MutationIntent::Create(
                    crate::transactions::data::CreateIntent::BulkRelations(spec),
                ) if spec.kind_id == contract.relation_kind_id => {
                    for (source, target) in &spec.endpoints {
                        context.metrics().count_relation_slot_scans(1);
                        if source.partition_id() != target.partition_id() {
                            violations.push(InvariantViolation {
                                class,
                                code: DiagnosticCode::InvariantViolation,
                                detail: format!(
                                    "relation contract '{}' requires same-partition endpoints for relation kind {:?}",
                                    contract.contract_id, contract.relation_kind_id
                                ),
                                fields: InvariantViolationFields::PartitionIsolation {
                                    contract_id: contract.contract_id.clone(),
                                    relation_kind_id: contract.relation_kind_id,
                                    relation_id: None,
                                    source_partition_id: source.partition_id(),
                                    target_partition_id: target.partition_id(),
                                },
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }

    if let Some(relation_ids) = context.state_view().touched_visible_relation_ids() {
        for relation_id in relation_ids {
            context.metrics().count_relation_slot_scans(1);
            let Some(metadata) = context.state_view().relation_metadata(relation_id) else {
                continue;
            };
            if metadata.kind_id != contract.relation_kind_id {
                continue;
            }
            if metadata.source.partition_id != metadata.target.partition_id {
                violations.push(InvariantViolation {
                    class,
                    code: DiagnosticCode::InvariantViolation,
                    detail: format!(
                        "relation contract '{}' requires same-partition endpoints for relation kind {:?}",
                        contract.contract_id, contract.relation_kind_id
                    ),
                    fields: InvariantViolationFields::PartitionIsolation {
                        contract_id: contract.contract_id.clone(),
                        relation_kind_id: contract.relation_kind_id,
                        relation_id: Some(metadata.relation_id),
                        source_partition_id: metadata.source.partition_id,
                        target_partition_id: metadata.target.partition_id,
                    },
                });
            }
        }
    }
    canonicalize_violations(violations)
}
