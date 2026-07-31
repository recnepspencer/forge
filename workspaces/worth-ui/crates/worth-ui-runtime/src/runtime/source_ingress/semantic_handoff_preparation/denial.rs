use super::WorthUiSemanticHandoffEvidence;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSemanticHandoffPreparationStop {
    UnsupportedProtocol,
    CapabilityResolution,
    RuntimeStructuralAdmission,
    DeclarationProjection,
    IntentDeclaration,
    BindingAdmission,
    IdentitySeeding,
    CanonicalAssembly,
}

/// Typed runtime-owned stop after DSL sealing and before candidate mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticHandoffPreparationDenial {
    handoff: WorthUiSemanticHandoffEvidence,
    stop: WorthUiSemanticHandoffPreparationStop,
}

impl WorthUiSemanticHandoffPreparationDenial {
    pub(super) fn new(
        handoff: WorthUiSemanticHandoffEvidence,
        stop: WorthUiSemanticHandoffPreparationStop,
    ) -> Self {
        Self { handoff, stop }
    }

    pub fn handoff(&self) -> &WorthUiSemanticHandoffEvidence {
        &self.handoff
    }

    pub fn stop(&self) -> WorthUiSemanticHandoffPreparationStop {
        self.stop
    }
}
