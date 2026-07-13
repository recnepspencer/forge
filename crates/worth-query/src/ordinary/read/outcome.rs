use crate::runtime::{
    WorthQueryReadDenialKind, WorthQueryReadResult, WorthQueryRuntimeError,
    WorthQueryRuntimeMissingComponent, WorthQueryStopClass,
};

use super::{WorthQueryReadContextDenial, WorthQueryReadContextReceipt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryReadNextAction {
    ReviseDeclaration,
    SupplyFreshBasis,
    SupplyPolicyAuthority,
    SupplyPolicyNarrowingContext,
    SupplyTenantAuthority,
    SupplyBranchAuthority,
    SupplyRelationshipProofAuthority,
    ConfigureRuntime(WorthQueryRuntimeMissingComponent),
    SelectSupportedCapability,
    ResolveDomainInvariant,
    InspectOperationalFailure,
}

#[derive(Debug)]
pub enum WorthQueryReadStopSource {
    Context(WorthQueryReadContextDenial),
    Runtime(WorthQueryRuntimeError),
}

#[derive(Debug)]
pub struct WorthQueryReadStop {
    next_action: WorthQueryReadNextAction,
    source: WorthQueryReadStopSource,
    context_receipt: Option<WorthQueryReadContextReceipt>,
}

impl WorthQueryReadStop {
    pub fn next_action(&self) -> WorthQueryReadNextAction {
        self.next_action
    }

    pub fn source(&self) -> &WorthQueryReadStopSource {
        &self.source
    }

    pub fn context_denial(&self) -> Option<&WorthQueryReadContextDenial> {
        match &self.source {
            WorthQueryReadStopSource::Context(denial) => Some(denial),
            WorthQueryReadStopSource::Runtime(_) => None,
        }
    }

    pub fn runtime_error(&self) -> Option<&WorthQueryRuntimeError> {
        match &self.source {
            WorthQueryReadStopSource::Runtime(error) => Some(error),
            WorthQueryReadStopSource::Context(_) => None,
        }
    }

    pub fn context_receipt(&self) -> Option<&WorthQueryReadContextReceipt> {
        self.context_receipt.as_ref()
    }

    pub(crate) fn context(source: WorthQueryReadContextDenial) -> Self {
        let next_action = classify_context_next_action(&source);
        Self {
            next_action,
            source: WorthQueryReadStopSource::Context(source),
            context_receipt: None,
        }
    }

    pub(crate) fn runtime(
        source: WorthQueryRuntimeError,
        context_receipt: WorthQueryReadContextReceipt,
    ) -> Self {
        let next_action = classify_runtime_next_action(&source);
        Self {
            next_action,
            source: WorthQueryReadStopSource::Runtime(source),
            context_receipt: Some(context_receipt),
        }
    }
}

#[derive(Debug)]
pub struct WorthQueryReadCompletion {
    result: WorthQueryReadResult,
    context_receipt: WorthQueryReadContextReceipt,
}

impl WorthQueryReadCompletion {
    pub fn result(&self) -> &WorthQueryReadResult {
        &self.result
    }

    pub fn context_receipt(&self) -> &WorthQueryReadContextReceipt {
        &self.context_receipt
    }

    pub fn into_result(self) -> WorthQueryReadResult {
        self.result
    }

    pub(crate) fn new(
        result: WorthQueryReadResult,
        context_receipt: WorthQueryReadContextReceipt,
    ) -> Self {
        Self {
            result,
            context_receipt,
        }
    }
}

#[derive(Debug)]
pub enum WorthQueryReadOutcome {
    Completed(WorthQueryReadCompletion),
    Stopped(WorthQueryReadStop),
}

impl WorthQueryReadOutcome {
    pub fn completed(&self) -> Option<&WorthQueryReadCompletion> {
        match self {
            Self::Completed(result) => Some(result),
            Self::Stopped(_) => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryReadStop> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(stop) => Some(stop),
        }
    }

    pub fn into_result(self) -> Result<WorthQueryReadCompletion, WorthQueryReadStop> {
        match self {
            Self::Completed(result) => Ok(result),
            Self::Stopped(stop) => Err(stop),
        }
    }
}

fn classify_runtime_next_action(error: &WorthQueryRuntimeError) -> WorthQueryReadNextAction {
    match error.stop_class() {
        WorthQueryStopClass::ReadCompositionDenied { denial } => {
            next_action_for_read_denial(denial.kind())
        }
        WorthQueryStopClass::ReadCompositionDomainInvariantDenied { .. } => {
            WorthQueryReadNextAction::ResolveDomainInvariant
        }
        WorthQueryStopClass::MissingRuntimeComponent { component } => {
            WorthQueryReadNextAction::ConfigureRuntime(component)
        }
        WorthQueryStopClass::SharedReadStaleBasis { .. } => {
            WorthQueryReadNextAction::SupplyFreshBasis
        }
        WorthQueryStopClass::FamilyAdmissionDenied { .. } => {
            WorthQueryReadNextAction::SelectSupportedCapability
        }
        _ => WorthQueryReadNextAction::InspectOperationalFailure,
    }
}

fn classify_context_next_action(denial: &WorthQueryReadContextDenial) -> WorthQueryReadNextAction {
    use super::WorthQueryReadContextDenialSource;
    use crate::policy_basis::PolicyTenantAdmissionFailureClass as PolicyFailure;
    use crate::relationship_proof::RelationshipProofFailureClass as RelationshipFailure;
    use crate::runtime::WorthQueryGraphReadAccessAuthorityDenialKind as AuthorityFailure;

    match denial.source() {
        WorthQueryReadContextDenialSource::MissingRelationshipProof => {
            WorthQueryReadNextAction::SupplyRelationshipProofAuthority
        }
        WorthQueryReadContextDenialSource::PolicyNarrowingContextRequired(_) => {
            WorthQueryReadNextAction::SupplyPolicyNarrowingContext
        }
        WorthQueryReadContextDenialSource::PolicyTenant(error) => match error.failure_class() {
            PolicyFailure::BranchAccessDenied => WorthQueryReadNextAction::SupplyBranchAuthority,
            PolicyFailure::TenantAdmissionDenied => WorthQueryReadNextAction::SupplyTenantAuthority,
            PolicyFailure::UnsupportedExecutionMode => {
                WorthQueryReadNextAction::SelectSupportedCapability
            }
            PolicyFailure::PolicyQueryFamilyDenied
            | PolicyFailure::RawMiddlewarePolicySourceForbidden
            | PolicyFailure::PolicyWorkBudgetDenied
            | PolicyFailure::SavedQueryPolicyTenantBypassForbidden => {
                WorthQueryReadNextAction::SupplyPolicyAuthority
            }
        },
        WorthQueryReadContextDenialSource::RelationshipProof(error) => {
            match error.failure_class() {
                RelationshipFailure::QueryShapeMismatch
                | RelationshipFailure::PolicyMismatch
                | RelationshipFailure::TenantSchemaMismatch
                | RelationshipFailure::MissingProofBasis
                | RelationshipFailure::HostCallbackForbidden
                | RelationshipFailure::UnboundedRecursiveWalk
                | RelationshipFailure::RelationshipProofBudgetExceeded
                | RelationshipFailure::UnboundedProofTopology => {
                    WorthQueryReadNextAction::SupplyRelationshipProofAuthority
                }
            }
        }
        WorthQueryReadContextDenialSource::GraphAuthority(error) => match error.kind() {
            AuthorityFailure::PolicyTenantDenied => WorthQueryReadNextAction::SupplyPolicyAuthority,
            AuthorityFailure::PolicyTenantBasisScopeMismatch => {
                WorthQueryReadNextAction::SupplyFreshBasis
            }
            AuthorityFailure::RelationshipProofRequiresPolicyTenantContext
            | AuthorityFailure::RelationshipProofPolicyTenantMismatch => {
                WorthQueryReadNextAction::SupplyRelationshipProofAuthority
            }
        },
    }
}

fn next_action_for_read_denial(kind: &WorthQueryReadDenialKind) -> WorthQueryReadNextAction {
    match kind {
        WorthQueryReadDenialKind::BasisResolutionDenied
        | WorthQueryReadDenialKind::BasisPreflightDenied => {
            WorthQueryReadNextAction::SupplyFreshBasis
        }
        WorthQueryReadDenialKind::InvalidRoot
        | WorthQueryReadDenialKind::BuiltInOperatorDenied
        | WorthQueryReadDenialKind::RelationshipProofAdmissionDenied
        | WorthQueryReadDenialKind::ScopeShapeDenied
        | WorthQueryReadDenialKind::AuthoringDenied
        | WorthQueryReadDenialKind::CanonicalizationDenied
        | WorthQueryReadDenialKind::ValidationDenied
        | WorthQueryReadDenialKind::PlanningDenied => WorthQueryReadNextAction::ReviseDeclaration,
        WorthQueryReadDenialKind::ExecutionDenied => {
            WorthQueryReadNextAction::InspectOperationalFailure
        }
    }
}
