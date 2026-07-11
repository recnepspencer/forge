use forge_store_recovery_physics::RecoveryLayoutReadmissionIdentity;

use crate::{
    LayoutCorruptionClassification, PhysicalArtifactFamily, S8LayoutCoverageWitness,
    S8MaterializationStateClass,
};

use super::classification::{S8LayoutCorruptionClass, S8LayoutReadmissionSource};
use super::denial::S8CorruptionDenial;
use super::quarantine::S8LayoutQuarantineWitness;
use super::readmission::S8LayoutReadmissionWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8UnsupportedCorruptionState {
    family: PhysicalArtifactFamily,
    state: S8MaterializationStateClass,
}

impl S8UnsupportedCorruptionState {
    pub(super) const fn new(
        family: PhysicalArtifactFamily,
        state: S8MaterializationStateClass,
    ) -> Self {
        Self { family, state }
    }
    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family
    }
    pub const fn state(&self) -> S8MaterializationStateClass {
        self.state
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8ReadmissionRequirement {
    pub(super) family: PhysicalArtifactFamily,
    pub(super) identity: RecoveryLayoutReadmissionIdentity,
}

impl S8ReadmissionRequirement {
    pub(super) const fn new(
        family: PhysicalArtifactFamily,
        identity: RecoveryLayoutReadmissionIdentity,
    ) -> Self {
        Self { family, identity }
    }
    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family
    }
    pub const fn identity(&self) -> &RecoveryLayoutReadmissionIdentity {
        &self.identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8QuarantineReadmissionRequirement {
    pub(super) quarantine: S8LayoutQuarantineWitness,
    pub(super) identity: RecoveryLayoutReadmissionIdentity,
}

impl S8QuarantineReadmissionRequirement {
    pub(super) const fn new(
        quarantine: S8LayoutQuarantineWitness,
        identity: RecoveryLayoutReadmissionIdentity,
    ) -> Self {
        Self {
            quarantine,
            identity,
        }
    }
    pub const fn quarantine(&self) -> &S8LayoutQuarantineWitness {
        &self.quarantine
    }
    pub const fn identity(&self) -> &RecoveryLayoutReadmissionIdentity {
        &self.identity
    }
}

#[derive(Debug, PartialEq, Eq)]
enum S8LayoutCorruptionCase {
    Clean(S8LayoutCoverageWitness),
    NotFound(PhysicalArtifactFamily),
    Unsupported(S8UnsupportedCorruptionState),
    StaleBinding(S8LayoutCoverageWitness),
    RebuildRequired(LayoutCorruptionClassification),
    Quarantined(S8LayoutQuarantineWitness),
    QuarantineReadmissionRequired(S8QuarantineReadmissionRequirement),
    OfflineReadmissionRequired(S8ReadmissionRequirement),
    ImportReadmissionRequired(S8ReadmissionRequirement),
    MigrationRequired(PhysicalArtifactFamily),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8LayoutCorruptionOutcome {
    case: S8LayoutCorruptionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutCorruptionView<'a> {
    Clean(&'a S8LayoutCoverageWitness),
    NotFound(&'a PhysicalArtifactFamily),
    Unsupported(&'a S8UnsupportedCorruptionState),
    StaleBinding(&'a S8LayoutCoverageWitness),
    RebuildRequired(&'a LayoutCorruptionClassification),
    Quarantined(&'a S8LayoutQuarantineWitness),
    QuarantineReadmissionRequired(&'a S8QuarantineReadmissionRequirement),
    OfflineReadmissionRequired(&'a S8ReadmissionRequirement),
    ImportReadmissionRequired(&'a S8ReadmissionRequirement),
    MigrationRequired(&'a PhysicalArtifactFamily),
}

impl S8LayoutCorruptionOutcome {
    pub(crate) fn clean(value: S8LayoutCoverageWitness) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::Clean(value))
    }

    pub(crate) fn not_found(value: PhysicalArtifactFamily) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::NotFound(value))
    }

    pub(crate) fn unsupported(value: S8UnsupportedCorruptionState) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::Unsupported(value))
    }

    pub(crate) fn stale_binding(value: S8LayoutCoverageWitness) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::StaleBinding(value))
    }

    pub(crate) fn rebuild_required(value: LayoutCorruptionClassification) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::RebuildRequired(value))
    }

    pub(crate) fn quarantined(value: S8LayoutQuarantineWitness) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::Quarantined(value))
    }

    pub(crate) fn quarantine_readmission_required(
        value: S8QuarantineReadmissionRequirement,
    ) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::QuarantineReadmissionRequired(value))
    }

    pub(crate) fn offline_readmission_required(value: S8ReadmissionRequirement) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::OfflineReadmissionRequired(value))
    }

    pub(crate) fn import_readmission_required(value: S8ReadmissionRequirement) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::ImportReadmissionRequired(value))
    }

    pub(crate) fn migration_required(value: PhysicalArtifactFamily) -> Self {
        Self::from_owner_payload(S8LayoutCorruptionCase::MigrationRequired(value))
    }

    fn from_owner_payload(case: S8LayoutCorruptionCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8LayoutCorruptionView<'_> {
        match &self.case {
            S8LayoutCorruptionCase::Clean(value) => S8LayoutCorruptionView::Clean(value),
            S8LayoutCorruptionCase::NotFound(value) => S8LayoutCorruptionView::NotFound(value),
            S8LayoutCorruptionCase::Unsupported(value) => {
                S8LayoutCorruptionView::Unsupported(value)
            }
            S8LayoutCorruptionCase::StaleBinding(value) => {
                S8LayoutCorruptionView::StaleBinding(value)
            }
            S8LayoutCorruptionCase::RebuildRequired(value) => {
                S8LayoutCorruptionView::RebuildRequired(value)
            }
            S8LayoutCorruptionCase::Quarantined(value) => {
                S8LayoutCorruptionView::Quarantined(value)
            }
            S8LayoutCorruptionCase::QuarantineReadmissionRequired(value) => {
                S8LayoutCorruptionView::QuarantineReadmissionRequired(value)
            }
            S8LayoutCorruptionCase::OfflineReadmissionRequired(value) => {
                S8LayoutCorruptionView::OfflineReadmissionRequired(value)
            }
            S8LayoutCorruptionCase::ImportReadmissionRequired(value) => {
                S8LayoutCorruptionView::ImportReadmissionRequired(value)
            }
            S8LayoutCorruptionCase::MigrationRequired(value) => {
                S8LayoutCorruptionView::MigrationRequired(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8LayoutCorruptionCase {
        self.case
    }
}

impl S8LayoutCorruptionOutcome {
    pub fn class(&self) -> S8LayoutCorruptionClass {
        match self.view() {
            S8LayoutCorruptionView::Clean(_) => S8LayoutCorruptionClass::Clean,
            S8LayoutCorruptionView::NotFound(_) => S8LayoutCorruptionClass::NotFound,
            S8LayoutCorruptionView::Unsupported(_) => S8LayoutCorruptionClass::Unsupported,
            S8LayoutCorruptionView::StaleBinding(_) => S8LayoutCorruptionClass::StaleBinding,
            S8LayoutCorruptionView::RebuildRequired(_) => {
                S8LayoutCorruptionClass::DerivedProjectionCorruption
            }
            S8LayoutCorruptionView::Quarantined(_) => {
                S8LayoutCorruptionClass::AuthoritativeArtifactCorruption
            }
            S8LayoutCorruptionView::QuarantineReadmissionRequired(_)
            | S8LayoutCorruptionView::OfflineReadmissionRequired(_)
            | S8LayoutCorruptionView::ImportReadmissionRequired(_) => {
                S8LayoutCorruptionClass::ReadmissionRequired
            }
            S8LayoutCorruptionView::MigrationRequired(_) => {
                S8LayoutCorruptionClass::MigrationRequired
            }
        }
    }

    pub(super) fn into_quarantined(self) -> Result<S8LayoutQuarantineWitness, Self> {
        match self.into_owner_payload() {
            S8LayoutCorruptionCase::Quarantined(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }

    pub(super) fn into_readmission_requirement(self) -> Result<S8RequiredReadmission, Self> {
        match self.into_owner_payload() {
            S8LayoutCorruptionCase::QuarantineReadmissionRequired(value) => {
                Ok(S8RequiredReadmission::Quarantine(value))
            }
            S8LayoutCorruptionCase::OfflineReadmissionRequired(value) => {
                Ok(S8RequiredReadmission::Offline(value))
            }
            S8LayoutCorruptionCase::ImportReadmissionRequired(value) => {
                Ok(S8RequiredReadmission::Import(value))
            }
            case => Err(Self::from_owner_payload(case)),
        }
    }
}

pub(super) enum S8RequiredReadmission {
    Quarantine(S8QuarantineReadmissionRequirement),
    Offline(S8ReadmissionRequirement),
    Import(S8ReadmissionRequirement),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8ReadmissionDenied {
    source: S8LayoutReadmissionSource,
    denial: S8CorruptionDenial,
}

impl S8ReadmissionDenied {
    const fn new(source: S8LayoutReadmissionSource, denial: S8CorruptionDenial) -> Self {
        Self { source, denial }
    }
    pub const fn source(&self) -> S8LayoutReadmissionSource {
        self.source
    }
    pub const fn denial(&self) -> &S8CorruptionDenial {
        &self.denial
    }
}

#[derive(Debug, PartialEq, Eq)]
enum S8LayoutReadmissionCase {
    QuarantineReadmitted(S8LayoutReadmissionWitness),
    OfflineReadmitted(S8LayoutReadmissionWitness),
    ImportReadmitted(S8LayoutReadmissionWitness),
    QuarantineDenied(S8ReadmissionDenied),
    OfflineDenied(S8ReadmissionDenied),
    ImportDenied(S8ReadmissionDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8LayoutReadmissionOutcome {
    case: S8LayoutReadmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutReadmissionView<'a> {
    QuarantineReadmitted(&'a S8LayoutReadmissionWitness),
    OfflineReadmitted(&'a S8LayoutReadmissionWitness),
    ImportReadmitted(&'a S8LayoutReadmissionWitness),
    QuarantineDenied(&'a S8ReadmissionDenied),
    OfflineDenied(&'a S8ReadmissionDenied),
    ImportDenied(&'a S8ReadmissionDenied),
}

impl S8LayoutReadmissionOutcome {
    pub(crate) fn quarantine_readmitted(value: S8LayoutReadmissionWitness) -> Self {
        Self::from_owner_payload(S8LayoutReadmissionCase::QuarantineReadmitted(value))
    }

    pub(crate) fn offline_readmitted(value: S8LayoutReadmissionWitness) -> Self {
        Self::from_owner_payload(S8LayoutReadmissionCase::OfflineReadmitted(value))
    }

    pub(crate) fn import_readmitted(value: S8LayoutReadmissionWitness) -> Self {
        Self::from_owner_payload(S8LayoutReadmissionCase::ImportReadmitted(value))
    }

    pub(crate) fn quarantine_denied(value: S8ReadmissionDenied) -> Self {
        Self::from_owner_payload(S8LayoutReadmissionCase::QuarantineDenied(value))
    }

    pub(crate) fn offline_denied(value: S8ReadmissionDenied) -> Self {
        Self::from_owner_payload(S8LayoutReadmissionCase::OfflineDenied(value))
    }

    pub(crate) fn import_denied(value: S8ReadmissionDenied) -> Self {
        Self::from_owner_payload(S8LayoutReadmissionCase::ImportDenied(value))
    }

    fn from_owner_payload(case: S8LayoutReadmissionCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8LayoutReadmissionView<'_> {
        match &self.case {
            S8LayoutReadmissionCase::QuarantineReadmitted(value) => {
                S8LayoutReadmissionView::QuarantineReadmitted(value)
            }
            S8LayoutReadmissionCase::OfflineReadmitted(value) => {
                S8LayoutReadmissionView::OfflineReadmitted(value)
            }
            S8LayoutReadmissionCase::ImportReadmitted(value) => {
                S8LayoutReadmissionView::ImportReadmitted(value)
            }
            S8LayoutReadmissionCase::QuarantineDenied(value) => {
                S8LayoutReadmissionView::QuarantineDenied(value)
            }
            S8LayoutReadmissionCase::OfflineDenied(value) => {
                S8LayoutReadmissionView::OfflineDenied(value)
            }
            S8LayoutReadmissionCase::ImportDenied(value) => {
                S8LayoutReadmissionView::ImportDenied(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8LayoutReadmissionCase {
        self.case
    }
}

impl S8LayoutReadmissionOutcome {
    pub(super) fn readmitted(witness: S8LayoutReadmissionWitness) -> Self {
        match witness.source() {
            S8LayoutReadmissionSource::QuarantineRecovery => Self::quarantine_readmitted(witness),
            S8LayoutReadmissionSource::OfflineRecoveryEvidence => Self::offline_readmitted(witness),
            S8LayoutReadmissionSource::TerminalImport => Self::import_readmitted(witness),
        }
    }

    pub(super) fn denied(source: S8LayoutReadmissionSource, denial: S8CorruptionDenial) -> Self {
        let denied = S8ReadmissionDenied::new(source, denial);
        match source {
            S8LayoutReadmissionSource::QuarantineRecovery => Self::quarantine_denied(denied),
            S8LayoutReadmissionSource::OfflineRecoveryEvidence => Self::offline_denied(denied),
            S8LayoutReadmissionSource::TerminalImport => Self::import_denied(denied),
        }
    }

    pub fn source(&self) -> Option<S8LayoutReadmissionSource> {
        Some(match self.view() {
            S8LayoutReadmissionView::QuarantineReadmitted(_)
            | S8LayoutReadmissionView::QuarantineDenied(_) => {
                S8LayoutReadmissionSource::QuarantineRecovery
            }
            S8LayoutReadmissionView::OfflineReadmitted(_)
            | S8LayoutReadmissionView::OfflineDenied(_) => {
                S8LayoutReadmissionSource::OfflineRecoveryEvidence
            }
            S8LayoutReadmissionView::ImportReadmitted(_)
            | S8LayoutReadmissionView::ImportDenied(_) => S8LayoutReadmissionSource::TerminalImport,
        })
    }
}
