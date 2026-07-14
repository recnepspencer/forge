use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::WorthQueryWorkspace;

use super::{
    WorthQueryDomainHandleDenial, WorthQueryDomainHandleDenialKind,
    WorthQueryInstalledDomainAuthorityWitness,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledDomainCapabilityKind {
    Read,
    Projection,
    Inspection,
    LiveOpen,
    LiveRead,
    LiveDelivery,
    LiveObservation,
    LiveCheckpoint,
    LiveResume,
    LiveClose,
    Mutation,
    Workflow,
}

impl WorthQueryInstalledDomainCapabilityKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Projection => "projection",
            Self::Inspection => "inspection",
            Self::LiveOpen => "live-open",
            Self::LiveRead => "live-read",
            Self::LiveDelivery => "live-delivery",
            Self::LiveObservation => "live-observation",
            Self::LiveCheckpoint => "live-checkpoint",
            Self::LiveResume => "live-resume",
            Self::LiveClose => "live-close",
            Self::Mutation => "mutation",
            Self::Workflow => "workflow",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledDomainExecutionDriftKind {
    DomainNotInstalled,
    ForeignRuntime,
    StaleInstallation,
    PackageMeaningChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledDomainExecutionNextAction {
    InstallDomainPackage,
    RebindInstalledDomain,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainExecutionDrift {
    kind: WorthQueryInstalledDomainExecutionDriftKind,
    next_action: WorthQueryInstalledDomainExecutionNextAction,
}

impl WorthQueryInstalledDomainExecutionDrift {
    pub(crate) fn validate<D: 'static>(
        witness: &WorthQueryInstalledDomainAuthorityWitness,
        workspace: &WorthQueryWorkspace,
    ) -> Result<(), Self> {
        workspace
            .validate_installed_domain_witness::<D>(witness)
            .map_err(Self::from_handle_denial)
    }

    fn from_handle_denial(denial: WorthQueryDomainHandleDenial) -> Self {
        let (kind, next_action) = match denial.kind() {
            WorthQueryDomainHandleDenialKind::DomainNotInstalled => (
                WorthQueryInstalledDomainExecutionDriftKind::DomainNotInstalled,
                WorthQueryInstalledDomainExecutionNextAction::InstallDomainPackage,
            ),
            WorthQueryDomainHandleDenialKind::ForeignRuntime => (
                WorthQueryInstalledDomainExecutionDriftKind::ForeignRuntime,
                WorthQueryInstalledDomainExecutionNextAction::RebindInstalledDomain,
            ),
            WorthQueryDomainHandleDenialKind::StaleInstallationGeneration => (
                WorthQueryInstalledDomainExecutionDriftKind::StaleInstallation,
                WorthQueryInstalledDomainExecutionNextAction::RebindInstalledDomain,
            ),
            WorthQueryDomainHandleDenialKind::PackageIdentityChanged => (
                WorthQueryInstalledDomainExecutionDriftKind::PackageMeaningChanged,
                WorthQueryInstalledDomainExecutionNextAction::RebindInstalledDomain,
            ),
        };
        Self { kind, next_action }
    }

    pub fn kind(&self) -> WorthQueryInstalledDomainExecutionDriftKind {
        self.kind
    }

    pub fn next_action(&self) -> WorthQueryInstalledDomainExecutionNextAction {
        self.next_action
    }
}

#[derive(Debug)]
pub struct WorthQueryInstalledDomainCapabilityStop<S> {
    installed_authority: WorthQueryInstalledDomainAuthorityWitness,
    capability: WorthQueryInstalledDomainCapabilityKind,
    declaration_identity: WorthQueryEvidenceIdentity,
    stop: S,
}

impl<S> WorthQueryInstalledDomainCapabilityStop<S> {
    pub(crate) fn new(
        installed_authority: WorthQueryInstalledDomainAuthorityWitness,
        capability: WorthQueryInstalledDomainCapabilityKind,
        declaration_identity: WorthQueryEvidenceIdentity,
        stop: S,
    ) -> Self {
        Self {
            installed_authority,
            capability,
            declaration_identity,
            stop,
        }
    }

    pub fn installed_authority(&self) -> &WorthQueryInstalledDomainAuthorityWitness {
        &self.installed_authority
    }

    pub fn capability(&self) -> WorthQueryInstalledDomainCapabilityKind {
        self.capability
    }

    pub fn declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.declaration_identity
    }

    pub fn stop(&self) -> &S {
        &self.stop
    }

    pub fn into_stop(self) -> S {
        self.stop
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainExecutionReceipt {
    installed_authority: WorthQueryInstalledDomainAuthorityWitness,
    capability: WorthQueryInstalledDomainCapabilityKind,
    declaration_identity: WorthQueryEvidenceIdentity,
    basis_identity: WorthQueryEvidenceIdentity,
    operational_identity: WorthQueryEvidenceIdentity,
    receipt_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryInstalledDomainExecutionReceipt {
    pub(crate) fn new(
        installed_authority: WorthQueryInstalledDomainAuthorityWitness,
        capability: WorthQueryInstalledDomainCapabilityKind,
        declaration_identity: WorthQueryEvidenceIdentity,
        basis_identity: WorthQueryEvidenceIdentity,
        operational_identity: WorthQueryEvidenceIdentity,
    ) -> Self {
        let receipt_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecution)
                .field_shape(
                    WorthQueryEvidenceTag::new("capability"),
                    capability.as_str(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("installed_authority"),
                    installed_authority.witness_identity(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("declaration"),
                    &declaration_identity,
                )
                .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), &basis_identity)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("operational"),
                    &operational_identity,
                )
                .seal();
        Self {
            installed_authority,
            capability,
            declaration_identity,
            basis_identity,
            operational_identity,
            receipt_identity,
        }
    }

    pub(crate) fn label_identity(role: &'static str, value: &str) -> WorthQueryEvidenceIdentity {
        worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecution)
            .field_shape(WorthQueryEvidenceTag::new("role"), role)
            .field_value(WorthQueryEvidenceTag::new("query_minted_value"), value)
            .seal()
    }

    pub(crate) fn derive(
        &self,
        capability: WorthQueryInstalledDomainCapabilityKind,
        operational_identity: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self::new(
            self.installed_authority.clone(),
            capability,
            self.declaration_identity.clone(),
            self.basis_identity.clone(),
            operational_identity,
        )
    }

    pub fn installed_authority(&self) -> &WorthQueryInstalledDomainAuthorityWitness {
        &self.installed_authority
    }

    pub fn capability(&self) -> WorthQueryInstalledDomainCapabilityKind {
        self.capability
    }

    pub fn declaration_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.declaration_identity
    }

    pub fn basis_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn operational_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.operational_identity
    }

    pub fn receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.receipt_identity
    }
}
