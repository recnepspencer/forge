use crate::identity::hash_parts;
use crate::workflow::{
    QueryWorkflowDeclaration, WorkflowBasisFamily, WorkflowContextBinding, WorkflowFreshnessPolicy,
    WorkflowLoweringCounters,
};
use forge_runtime_bridge::facade::{
    BridgeRequestKind, BridgeWritebackDeclaration, BridgeWritebackDeclarationIdentity,
    BridgeWritebackEffectClass, BridgeWritebackFamilyKind, BridgeWritebackIdempotenceClass,
    BridgeWritebackStrategyClass,
};

use super::counters::{
    lowering_denial_counters, writeback_lowering_success_counters, LoweringDenialClass,
};
use super::errors::{
    ensure_writeback_workflow_family, WorkflowLoweringError, WorkflowLoweringFailureClass,
};
use super::terms::{
    WorkflowFreshnessBinding, WorkflowStalenessClass, WritebackDeclarationFamily,
    WritebackLoweringInput,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WritebackCausalityBinding {
    binding_digest: String,
    basis_family: WorkflowBasisFamily,
    basis_digest: String,
    request_kind: BridgeRequestKind,
    causality_digest: String,
}

impl WritebackCausalityBinding {
    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn request_kind(&self) -> BridgeRequestKind {
        self.request_kind
    }

    pub fn causality_digest(&self) -> &str {
        &self.causality_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryWritebackDeclaration {
    declaration: QueryWorkflowDeclaration,
    family: WritebackDeclarationFamily,
    causality_binding: WritebackCausalityBinding,
    bridge_declaration: BridgeWritebackDeclaration,
    freshness_binding: WorkflowFreshnessBinding,
    staleness_class: WorkflowStalenessClass,
    lowering_digest: String,
    counters: WorkflowLoweringCounters,
}

impl QueryWritebackDeclaration {
    pub fn declaration(&self) -> &QueryWorkflowDeclaration {
        &self.declaration
    }

    pub fn family(&self) -> &WritebackDeclarationFamily {
        &self.family
    }

    pub fn causality_binding(&self) -> &WritebackCausalityBinding {
        &self.causality_binding
    }

    pub fn bridge_declaration(&self) -> &BridgeWritebackDeclaration {
        &self.bridge_declaration
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

pub fn lower_query_writeback_declaration(
    declaration: &QueryWorkflowDeclaration,
    input: WritebackLoweringInput,
) -> Result<QueryWritebackDeclaration, WorkflowLoweringError> {
    ensure_writeback_workflow_family(declaration)?;
    let request_kind = writeback_request_kind(
        declaration.binding(),
        declaration.request().freshness_policy(),
    )?;
    let causality_digest = writeback_causality_digest(declaration, request_kind);
    let bridge_declaration =
        writeback_bridge_declaration(declaration, input.family(), &causality_digest, request_kind);
    let lowering_digest = writeback_lowering_digest(
        declaration,
        input.family(),
        &bridge_declaration,
        &causality_digest,
    );

    Ok(QueryWritebackDeclaration {
        declaration: declaration.clone(),
        family: input.family().clone(),
        causality_binding: WritebackCausalityBinding {
            binding_digest: declaration.binding().digest().to_string(),
            basis_family: declaration.binding().basis_family().clone(),
            basis_digest: declaration.binding().basis_digest().to_string(),
            request_kind,
            causality_digest,
        },
        bridge_declaration,
        freshness_binding: WorkflowFreshnessBinding::RuntimeBasisExact,
        staleness_class: WorkflowStalenessClass::AuthorityValidationRequired,
        lowering_digest,
        counters: writeback_lowering_success_counters(1),
    })
}

fn writeback_request_kind(
    binding: &WorkflowContextBinding,
    freshness_policy: &WorkflowFreshnessPolicy,
) -> Result<BridgeRequestKind, WorkflowLoweringError> {
    match binding.basis_family() {
        WorkflowBasisFamily::RuntimePreflight => Ok(BridgeRequestKind::Authoritative),
        WorkflowBasisFamily::PreviewFoundation
        | WorkflowBasisFamily::PreviewPromotionComparison => {
            let (failure_class, message, staleness_class, denial_class) =
                writeback_preview_freshness_denial(freshness_policy);
            Err(WorkflowLoweringError::new(
                failure_class,
                message,
                staleness_class,
                lowering_denial_counters(1, denial_class),
            ))
        }
        WorkflowBasisFamily::CorrespondenceHistorical => Err(WorkflowLoweringError::new(
            WorkflowLoweringFailureClass::UnsupportedWritebackCausality,
            "correspondence/historical workflow contexts cannot mint bridge writeback causality",
            WorkflowStalenessClass::ExplicitRebindRequired,
            lowering_denial_counters(1, LoweringDenialClass::AmbientBasisFallback),
        )),
    }
}

fn writeback_preview_freshness_denial(
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
            "query-triggered writeback cannot preserve exact basis from preview workflow contexts without authoritative revalidation",
            WorkflowStalenessClass::StaleDenied,
            LoweringDenialClass::StaleDenied,
        )
    } else {
        (
            WorkflowLoweringFailureClass::ExplicitRebindRequired,
            "query-triggered writeback requires authoritative rebind before bridge lowering",
            WorkflowStalenessClass::ExplicitRebindRequired,
            LoweringDenialClass::WritebackExplicitRebind,
        )
    }
}

fn writeback_bridge_declaration(
    declaration: &QueryWorkflowDeclaration,
    family: &WritebackDeclarationFamily,
    _causality_digest: &str,
    request_kind: BridgeRequestKind,
) -> BridgeWritebackDeclaration {
    let (family_kind, effect_class, strategy_class, idempotence_class) =
        writeback_bridge_family_terms(family);
    BridgeWritebackDeclaration::writeback_capable(
        BridgeWritebackDeclarationIdentity::new(format!(
            "forge-query:{}",
            declaration.report().declaration_digest()
        )),
        request_kind,
        family_kind,
        effect_class,
        strategy_class,
        idempotence_class,
    )
}

fn writeback_bridge_family_terms(
    family: &WritebackDeclarationFamily,
) -> (
    BridgeWritebackFamilyKind,
    BridgeWritebackEffectClass,
    BridgeWritebackStrategyClass,
    BridgeWritebackIdempotenceClass,
) {
    match family {
        WritebackDeclarationFamily::ProjectedStateDiff => (
            BridgeWritebackFamilyKind::ProjectedStateDiff,
            BridgeWritebackEffectClass::ProjectedStateDiff,
            BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ),
    }
}

fn writeback_causality_digest(
    declaration: &QueryWorkflowDeclaration,
    request_kind: BridgeRequestKind,
) -> String {
    hash_parts(&[
        format!("binding:{}", declaration.binding().digest()),
        format!(
            "basis_family:{}",
            declaration.binding().basis_family().as_str()
        ),
        format!("basis_digest:{}", declaration.binding().basis_digest()),
        format!("request_kind:{request_kind:?}"),
    ])
}

fn writeback_lowering_digest(
    declaration: &QueryWorkflowDeclaration,
    family: &WritebackDeclarationFamily,
    bridge_declaration: &BridgeWritebackDeclaration,
    causality_digest: &str,
) -> String {
    hash_parts(&[
        format!("declaration:{}", declaration.report().declaration_digest()),
        format!("writeback_family:{}", family.as_str()),
        format!("bridge_declaration:{}", bridge_declaration.digest()),
        format!("causality:{causality_digest}"),
    ])
}
