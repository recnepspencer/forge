use forge_store_recovery_physics::RecoveryLayoutReadmissionIdentity;

use crate::production_transition::define_owner_outcome;
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

define_owner_outcome!(
    pub S8LayoutCorruptionOutcome,
    pub S8LayoutCorruptionView,
    S8LayoutCorruptionCase,
    CorruptionQuarantine,
    ClassifyCorruption,
    [
        clean => Clean(S8LayoutCoverageWitness): Unclassified => Classify => Clean,
        not_found => NotFound(PhysicalArtifactFamily): Unclassified => Classify => NotFound,
        unsupported => Unsupported(S8UnsupportedCorruptionState): Unclassified => Classify => Unsupported,
        stale_binding => StaleBinding(S8LayoutCoverageWitness): Unclassified => Classify => Stale,
        rebuild_required => RebuildRequired(LayoutCorruptionClassification): Unclassified => Classify => RebuildRequired,
        quarantined => Quarantined(S8LayoutQuarantineWitness): Unclassified => Quarantine => Quarantined,
        quarantine_readmission_required => QuarantineReadmissionRequired(S8QuarantineReadmissionRequirement): Quarantined => RequireRebind => QuarantineReadmissionRequired,
        offline_readmission_required => OfflineReadmissionRequired(S8ReadmissionRequirement): Unclassified => Classify => OfflineEvidenceReadmissionRequired,
        import_readmission_required => ImportReadmissionRequired(S8ReadmissionRequirement): Unclassified => Classify => TerminalImportReadmissionRequired,
        migration_required => MigrationRequired(PhysicalArtifactFamily): Unclassified => Classify => MigrationRequired
    ]
);

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

define_owner_outcome!(
    pub S8LayoutReadmissionOutcome,
    pub S8LayoutReadmissionView,
    S8LayoutReadmissionCase,
    CorruptionQuarantine,
    ReadmitCorruptionEvidence,
    [
        quarantine_readmitted => QuarantineReadmitted(S8LayoutReadmissionWitness): QuarantineReadmissionRequired => Readmit => Readmitted,
        offline_readmitted => OfflineReadmitted(S8LayoutReadmissionWitness): OfflineEvidenceReadmissionRequired => Readmit => Readmitted,
        import_readmitted => ImportReadmitted(S8LayoutReadmissionWitness): TerminalImportReadmissionRequired => Readmit => Readmitted,
        quarantine_denied => QuarantineDenied(S8ReadmissionDenied): QuarantineReadmissionRequired => Deny => Denied,
        offline_denied => OfflineDenied(S8ReadmissionDenied): OfflineEvidenceReadmissionRequired => Deny => Denied,
        import_denied => ImportDenied(S8ReadmissionDenied): TerminalImportReadmissionRequired => Deny => Denied
    ]
);

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
