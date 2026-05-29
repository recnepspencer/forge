use crate::identity::hash_parts;
use crate::relational_aspect_write::field_patch_from_values;
use crate::workflow::{
    QueryWorkflowDeclaration, WorkflowBasisFamily, WorkflowFreshnessPolicy,
    WorkflowLoweringCounters,
};
use forge_relational::facade::commit_strategies::{
    IntentReconciliationInput, NativeStrategyCommitRequest, StrategyCallerProvenance,
    StrategyRequestOrigin,
};

use super::counters::{
    lowering_denial_counters, mutation_lowering_success_counters, LoweringDenialClass,
};
use super::errors::{
    ensure_mutation_workflow_family, WorkflowLoweringError, WorkflowLoweringFailureClass,
};
use super::terms::{
    MutationIntentFamily, MutationLoweringInput, RelationalStrategyTarget,
    WorkflowFreshnessBinding, WorkflowStalenessClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LoweredMutationIntentDeclaration {
    declaration: QueryWorkflowDeclaration,
    mutation_family: MutationIntentFamily,
    strategy_target: RelationalStrategyTarget,
    authority_binding: MutationAuthorityBinding,
    strategy_request: NativeStrategyCommitRequest,
    freshness_binding: WorkflowFreshnessBinding,
    staleness_class: WorkflowStalenessClass,
    lowering_digest: String,
    counters: WorkflowLoweringCounters,
}

impl LoweredMutationIntentDeclaration {
    pub fn declaration(&self) -> &QueryWorkflowDeclaration {
        &self.declaration
    }

    pub fn mutation_family(&self) -> &MutationIntentFamily {
        &self.mutation_family
    }

    pub fn strategy_target(&self) -> &RelationalStrategyTarget {
        &self.strategy_target
    }

    pub fn authority_binding(&self) -> &MutationAuthorityBinding {
        &self.authority_binding
    }

    pub fn strategy_request(&self) -> &NativeStrategyCommitRequest {
        &self.strategy_request
    }

    pub fn freshness_binding(&self) -> &WorkflowFreshnessBinding {
        &self.freshness_binding
    }

    pub fn staleness_class(&self) -> &WorkflowStalenessClass {
        &self.staleness_class
    }

    pub fn lowering_digest(&self) -> &str {
        &self.lowering_digest
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MutationAuthorityBinding {
    binding_digest: String,
    runtime_snapshot_token: Option<String>,
}

impl MutationAuthorityBinding {
    fn new(binding_digest: impl Into<String>, runtime_snapshot_token: Option<String>) -> Self {
        Self {
            binding_digest: binding_digest.into(),
            runtime_snapshot_token,
        }
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn runtime_snapshot_token(&self) -> Option<&str> {
        self.runtime_snapshot_token.as_deref()
    }
}

pub fn lower_mutation_intent_declaration(
    declaration: &QueryWorkflowDeclaration,
    authority_binding_digest: &str,
    input: MutationLoweringInput,
) -> Result<LoweredMutationIntentDeclaration, WorkflowLoweringError> {
    ensure_mutation_workflow_family(declaration)?;
    let freshness_binding = mutation_freshness_binding(
        declaration.binding(),
        declaration.request().freshness_policy(),
    )?;
    let mutation_family = input.family();
    let strategy_target = match mutation_family {
        MutationIntentFamily::IntentReconciliation => {
            RelationalStrategyTarget::IntentReconciliation
        }
    };
    let request = intent_reconciliation_strategy_request(declaration, input)?;
    let lowering_digest = mutation_lowering_digest(
        declaration,
        &mutation_family,
        &strategy_target,
        authority_binding_digest,
        &freshness_binding,
        &request,
    );

    Ok(LoweredMutationIntentDeclaration {
        declaration: declaration.clone(),
        mutation_family,
        strategy_target,
        authority_binding: MutationAuthorityBinding::new(
            authority_binding_digest,
            declaration
                .binding()
                .runtime_snapshot_token()
                .map(str::to_string),
        ),
        strategy_request: request,
        freshness_binding,
        staleness_class: WorkflowStalenessClass::ExactBasisPreserved,
        lowering_digest,
        counters: mutation_lowering_success_counters(1),
    })
}

fn intent_reconciliation_strategy_request(
    declaration: &QueryWorkflowDeclaration,
    input: MutationLoweringInput,
) -> Result<NativeStrategyCommitRequest, WorkflowLoweringError> {
    let MutationLoweringInput::IntentReconciliation {
        entity_id,
        desired_aspect_fields_json,
    } = input;
    IntentReconciliationInput {
        entity_id,
        desired_fields: intent_reconciliation_field_patch(desired_aspect_fields_json)?,
    }
    .into_native_canonical_request(StrategyCallerProvenance {
        request_origin: StrategyRequestOrigin::Api,
        actor_identity: Some("forge-query".to_string()),
        correlation_id: Some(declaration.report().declaration_digest().to_string()),
    })
    .map_err(|_| {
        WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::LoweringSerializationFailed,
            "mutation lowering could not encode native intent reconciliation input",
            WorkflowStalenessClass::ExactBasisPreserved,
            lowering_denial_counters(1, LoweringDenialClass::General),
        )
    })
}

fn intent_reconciliation_field_patch(
    desired_aspect_fields_json: serde_json::Value,
) -> Result<forge_relational::facade::transactions::AspectFieldPatch, WorkflowLoweringError> {
    let serde_json::Value::Object(fields) = desired_aspect_fields_json else {
        return Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::LoweringSerializationFailed,
            "intent reconciliation desired aspect fields must be a flat object of aspect fields",
            WorkflowStalenessClass::ExactBasisPreserved,
            lowering_denial_counters(1, LoweringDenialClass::General),
        ));
    };
    let flattened_fields = fields
        .into_iter()
        .map(|(field, value)| (field.clone(), field, value))
        .collect::<Vec<_>>();
    field_patch_from_values(
        flattened_fields
            .iter()
            .map(|(aspect, field, value)| (aspect.as_str(), field.as_str(), value.clone())),
    )
    .map_err(|_| {
        WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::LoweringSerializationFailed,
            "intent reconciliation desired aspect fields could not lower into aspect field patches",
            WorkflowStalenessClass::ExactBasisPreserved,
            lowering_denial_counters(1, LoweringDenialClass::General),
        )
    })
}

fn mutation_freshness_binding(
    binding: &crate::workflow::WorkflowContextBinding,
    freshness_policy: &WorkflowFreshnessPolicy,
) -> Result<WorkflowFreshnessBinding, WorkflowLoweringError> {
    match binding.basis_family() {
        WorkflowBasisFamily::RuntimePreflight => Ok(WorkflowFreshnessBinding::RuntimeBasisExact),
        WorkflowBasisFamily::PreviewFoundation
        | WorkflowBasisFamily::PreviewPromotionComparison => {
            let (failure_class, message, staleness_class, denial_class) =
                mutation_preview_freshness_denial(freshness_policy);
            Err(WorkflowLoweringError::new(
                failure_class,
                message,
                staleness_class,
                lowering_denial_counters(1, denial_class),
            ))
        }
        WorkflowBasisFamily::CorrespondenceHistorical => Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::UnsupportedRelationalStrategyTarget,
            "correspondence/historical contexts cannot lower query-owned mutation intent",
            WorkflowStalenessClass::ExplicitRebindRequired,
            lowering_denial_counters(1, LoweringDenialClass::AmbientBasisFallback),
        )),
    }
}

fn mutation_preview_freshness_denial(
    freshness_policy: &WorkflowFreshnessPolicy,
) -> (
    WorkflowLoweringFailureClass,
    &'static str,
    WorkflowStalenessClass,
    LoweringDenialClass,
) {
    if freshness_policy == &WorkflowFreshnessPolicy::ExactBasis {
        (
            WorkflowLoweringFailureClass::StaleWorkflowDenied,
            "mutation lowering cannot preserve exact basis from preview workflow contexts without authoritative revalidation",
            WorkflowStalenessClass::StaleDenied,
            LoweringDenialClass::StaleDenied,
        )
    } else {
        (
            WorkflowLoweringFailureClass::ExplicitRebindRequired,
            "mutation lowering remains runtime-basis only until relational authority rebind is explicit",
            WorkflowStalenessClass::ExplicitRebindRequired,
            LoweringDenialClass::ExplicitRebind,
        )
    }
}

fn mutation_lowering_digest(
    declaration: &QueryWorkflowDeclaration,
    mutation_family: &MutationIntentFamily,
    strategy_target: &RelationalStrategyTarget,
    authority_binding_digest: &str,
    freshness_binding: &WorkflowFreshnessBinding,
    request: &NativeStrategyCommitRequest,
) -> String {
    hash_parts(&[
        format!("declaration:{}", declaration.report().declaration_digest()),
        format!("mutation_family:{}", mutation_family.as_str()),
        format!("strategy_target:{}", strategy_target.as_str()),
        format!("authority_binding:{authority_binding_digest}"),
        format!(
            "runtime_snapshot:{}",
            declaration
                .binding()
                .runtime_snapshot_token()
                .unwrap_or("none")
        ),
        format!("freshness:{}", freshness_binding.as_str()),
        format!(
            "request_origin:{:?}",
            request.caller_provenance().request_origin
        ),
    ])
}
