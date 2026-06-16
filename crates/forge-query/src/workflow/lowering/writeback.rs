use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
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
    binding_identity: ForgeQueryEvidenceIdentity,
    basis_family: WorkflowBasisFamily,
    request_kind: BridgeRequestKind,
    causality_identity: ForgeQueryEvidenceIdentity,
    basis_identity: ForgeQueryEvidenceIdentity,
}

impl WritebackCausalityBinding {
    pub fn binding_for_reporting(&self) -> &str {
        self.binding_identity.as_str()
    }

    pub fn basis_family(&self) -> &WorkflowBasisFamily {
        &self.basis_family
    }

    pub fn basis_for_reporting(&self) -> &str {
        self.basis_identity.as_str()
    }

    pub fn request_kind(&self) -> BridgeRequestKind {
        self.request_kind
    }

    pub fn causality_for_reporting(&self) -> &str {
        self.causality_identity.as_str()
    }

    pub fn causality_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.causality_identity
    }

    pub fn binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub fn basis_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.basis_identity
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
    lowering_identity: ForgeQueryEvidenceIdentity,
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

    pub fn lowering_for_reporting(&self) -> &str {
        self.lowering_identity.as_str()
    }

    pub fn lowering_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.lowering_identity
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
    let binding_identity = declaration.binding().binding_identity().clone();
    let causality_identity = writeback_causality_identity(declaration, request_kind);
    let basis_identity = writeback_basis_identity(declaration);
    let bridge_declaration =
        writeback_bridge_declaration(declaration, input.family(), request_kind);
    let lowering_identity = writeback_lowering_identity(
        declaration,
        input.family(),
        &bridge_declaration,
        &causality_identity,
    );

    Ok(QueryWritebackDeclaration {
        declaration: declaration.clone(),
        family: input.family().clone(),
        causality_binding: WritebackCausalityBinding {
            binding_identity,
            basis_family: declaration.binding().basis_family().clone(),
            request_kind,
            causality_identity,
            basis_identity,
        },
        bridge_declaration,
        freshness_binding: WorkflowFreshnessBinding::RuntimeBasisExact,
        staleness_class: WorkflowStalenessClass::AuthorityValidationRequired,
        lowering_identity,
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
    request_kind: BridgeRequestKind,
) -> BridgeWritebackDeclaration {
    let (family_kind, effect_class, strategy_class, idempotence_class) =
        writeback_bridge_family_terms(family);
    let bridge_declaration_identity =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WorkflowMutationLowering)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "workflow_writeback_bridge_declaration_v1",
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("declaration"),
                declaration.report().declaration_identity(),
            )
            .seal();
    BridgeWritebackDeclaration::writeback_capable(
        BridgeWritebackDeclarationIdentity::from_bridge_evidence(
            &bridge_declaration_identity.bridge_external_identity_evidence(),
        ),
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

fn writeback_causality_identity(
    declaration: &QueryWorkflowDeclaration,
    request_kind: BridgeRequestKind,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeBridgeWritebackAuthority)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "writeback_causality")
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("binding"),
            declaration.binding().binding_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("basis_family"),
            declaration.binding().basis_family().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            declaration.binding().basis_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("request_kind"),
            format!("{request_kind:?}"),
        )
        .seal()
}

fn writeback_basis_identity(declaration: &QueryWorkflowDeclaration) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeBridgeWritebackAuthority)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "writeback_basis")
        .field_shape(
            ForgeQueryEvidenceTag::new("basis_family"),
            declaration.binding().basis_family().as_str(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("basis"),
            declaration.binding().basis_identity(),
        )
        .seal()
}

fn writeback_lowering_identity(
    declaration: &QueryWorkflowDeclaration,
    family: &WritebackDeclarationFamily,
    bridge_declaration: &BridgeWritebackDeclaration,
    causality_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeBridgeWritebackAuthority)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "writeback_lowering")
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("declaration"),
            declaration.report().declaration_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("writeback_family"),
            family.as_str(),
        )
        .field_bridge_authority_identity(
            ForgeQueryEvidenceTag::new("bridge_declaration"),
            &bridge_declaration
                .declaration_identity()
                .bridge_trust_boundary(),
        )
        .field_evidence_identity(ForgeQueryEvidenceTag::new("causality"), causality_identity)
        .seal()
}
