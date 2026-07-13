use crate::runtime::{
    WorthQueryReadDenialKind, WorthQueryReadResult, WorthQueryRuntimeError,
    WorthQueryRuntimeMissingComponent, WorthQueryStopClass,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryReadNextAction {
    ReviseDeclaration,
    SupplyFreshBasis,
    ConfigureRuntime(WorthQueryRuntimeMissingComponent),
    SelectSupportedCapability,
    ResolveDomainInvariant,
    InspectOperationalFailure,
}

#[derive(Debug)]
pub struct WorthQueryReadStop {
    next_action: WorthQueryReadNextAction,
    source: WorthQueryRuntimeError,
}

impl WorthQueryReadStop {
    pub fn next_action(&self) -> WorthQueryReadNextAction {
        self.next_action
    }

    pub fn source(&self) -> &WorthQueryRuntimeError {
        &self.source
    }

    pub(crate) fn new(source: WorthQueryRuntimeError) -> Self {
        let next_action = classify_next_action(&source);
        Self {
            next_action,
            source,
        }
    }
}

#[derive(Debug)]
pub enum WorthQueryReadOutcome {
    Completed(WorthQueryReadResult),
    Stopped(WorthQueryReadStop),
}

impl WorthQueryReadOutcome {
    pub fn completed(&self) -> Option<&WorthQueryReadResult> {
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

    pub fn into_result(self) -> Result<WorthQueryReadResult, WorthQueryReadStop> {
        match self {
            Self::Completed(result) => Ok(result),
            Self::Stopped(stop) => Err(stop),
        }
    }
}

fn classify_next_action(error: &WorthQueryRuntimeError) -> WorthQueryReadNextAction {
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
