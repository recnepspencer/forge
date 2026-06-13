use crate::aspect_field_authoring::aspect_field_patch_from_external_json_values;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::workflow::{
    QueryWorkflowDeclaration, WorkflowBasisFamily, WorkflowFreshnessPolicy,
    WorkflowLoweringCounters,
};
use forge_relational::facade::commit_strategies::{
    IntentReconciliationInput, NativeStrategyCommitRequest, StrategyCallerProvenance,
    StrategyRequestOrigin,
};
use forge_relational::facade::history::BranchId;

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
    lowering_identity: ForgeQueryEvidenceIdentity,
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

    pub fn lowering_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.lowering_identity
    }

    pub fn counters(&self) -> &WorkflowLoweringCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MutationAuthorityBinding {
    binding_identity: ForgeQueryEvidenceIdentity,
    runtime_snapshot_identity: Option<ForgeQuerySnapshotIdentity>,
    runtime_target_branch: Option<BranchId>,
}

impl MutationAuthorityBinding {
    fn new(
        binding_identity: ForgeQueryEvidenceIdentity,
        runtime_snapshot_identity: Option<ForgeQuerySnapshotIdentity>,
        runtime_target_branch: Option<BranchId>,
    ) -> Self {
        Self {
            binding_identity,
            runtime_snapshot_identity,
            runtime_target_branch,
        }
    }

    pub fn binding_digest(&self) -> &str {
        self.binding_identity.as_str()
    }

    pub fn binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub fn runtime_snapshot_identity(&self) -> Option<&ForgeQuerySnapshotIdentity> {
        self.runtime_snapshot_identity.as_ref()
    }

    pub fn runtime_target_branch(&self) -> Option<&BranchId> {
        self.runtime_target_branch.as_ref()
    }
}

pub fn lower_mutation_intent_declaration(
    declaration: &QueryWorkflowDeclaration,
    authority_binding_identity: &ForgeQueryEvidenceIdentity,
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
    let lowering_identity = mutation_lowering_identity(
        declaration,
        &mutation_family,
        &strategy_target,
        authority_binding_identity,
        &freshness_binding,
        &request,
    );
    let lowering_digest = lowering_identity.as_str().to_string();

    Ok(LoweredMutationIntentDeclaration {
        declaration: declaration.clone(),
        mutation_family,
        strategy_target,
        authority_binding: MutationAuthorityBinding::new(
            authority_binding_identity.clone(),
            declaration.binding().runtime_snapshot_identity().cloned(),
            declaration.binding().runtime_target_branch().cloned(),
        ),
        strategy_request: request,
        freshness_binding,
        staleness_class: WorkflowStalenessClass::ExactBasisPreserved,
        lowering_identity,
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
        desired_aspect_fields_external_json,
    } = input;
    IntentReconciliationInput {
        entity_id,
        desired_aspect_fields: intent_reconciliation_field_patch(
            desired_aspect_fields_external_json,
        )?,
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
    desired_aspect_fields_external_json: serde_json::Value,
) -> Result<forge_relational::facade::transactions::AspectFieldPatch, WorkflowLoweringError> {
    let serde_json::Value::Object(fields) = desired_aspect_fields_external_json else {
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
    aspect_field_patch_from_external_json_values(
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

fn mutation_lowering_identity(
    declaration: &QueryWorkflowDeclaration,
    mutation_family: &MutationIntentFamily,
    strategy_target: &RelationalStrategyTarget,
    authority_binding_identity: &ForgeQueryEvidenceIdentity,
    freshness_binding: &WorkflowFreshnessBinding,
    request: &NativeStrategyCommitRequest,
) -> ForgeQueryEvidenceIdentity {
    let mut identity =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("declaration"),
                declaration.report().declaration_identity(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("mutation_family"),
                mutation_family.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("strategy_target"),
                strategy_target.as_str(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("authority_binding"),
                authority_binding_identity,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("freshness"),
                freshness_binding.as_str(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("request_origin"),
                request_origin_label(&request.caller_provenance().request_origin),
            );
    if let Some(runtime_snapshot_identity) = declaration.binding().runtime_snapshot_identity() {
        identity = identity.field_evidence_identity(
            ForgeQueryEvidenceTag::new("runtime_snapshot"),
            &runtime_snapshot_identity.evidence_identity(),
        );
    }
    identity.seal()
}

fn request_origin_label(origin: &StrategyRequestOrigin) -> &'static str {
    match origin {
        StrategyRequestOrigin::Api => "api",
        StrategyRequestOrigin::Harness => "harness",
        StrategyRequestOrigin::Replay => "replay",
        StrategyRequestOrigin::Test => "test",
    }
}
