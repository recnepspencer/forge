use forge_store_recovery_physics::RecoveryLayoutReadmissionIdentity;

use crate::{
    LayoutCorruptionClassification, LayoutCoverageWitness, MaterializationStateClass,
    PhysicalArtifactFamily,
};

use super::classification::LayoutCorruptionClass;
use super::quarantine::LayoutQuarantineWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedCorruptionState {
    family: PhysicalArtifactFamily,
    state: MaterializationStateClass,
}

impl UnsupportedCorruptionState {
    pub(super) const fn new(
        family: PhysicalArtifactFamily,
        state: MaterializationStateClass,
    ) -> Self {
        Self { family, state }
    }
    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family
    }
    pub const fn state(&self) -> MaterializationStateClass {
        self.state
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OfflineReadmissionRequirement {
    pub(super) family: PhysicalArtifactFamily,
    pub(super) identity: RecoveryLayoutReadmissionIdentity,
}

impl OfflineReadmissionRequirement {
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
pub struct QuarantineReadmissionRequirement {
    pub(super) quarantine: LayoutQuarantineWitness,
    pub(super) identity: RecoveryLayoutReadmissionIdentity,
}

impl QuarantineReadmissionRequirement {
    pub(super) const fn new(
        quarantine: LayoutQuarantineWitness,
        identity: RecoveryLayoutReadmissionIdentity,
    ) -> Self {
        Self {
            quarantine,
            identity,
        }
    }
    pub const fn quarantine(&self) -> &LayoutQuarantineWitness {
        &self.quarantine
    }
    pub const fn identity(&self) -> &RecoveryLayoutReadmissionIdentity {
        &self.identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportReadmissionRequirement {
    pub(super) family: PhysicalArtifactFamily,
    pub(super) identity: RecoveryLayoutReadmissionIdentity,
}

impl ImportReadmissionRequirement {
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

#[derive(Debug, PartialEq, Eq)]
enum LayoutCorruptionCase {
    Clean(LayoutCoverageWitness),
    NotFound(PhysicalArtifactFamily),
    Unsupported(UnsupportedCorruptionState),
    StaleBinding(LayoutCoverageWitness),
    RebuildRequired(LayoutCorruptionClassification),
    Quarantined(LayoutQuarantineWitness),
    QuarantineReadmissionRequired(QuarantineReadmissionRequirement),
    OfflineReadmissionRequired(OfflineReadmissionRequirement),
    ImportReadmissionRequired(ImportReadmissionRequirement),
    MigrationRequired(PhysicalArtifactFamily),
}

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutCorruptionOutcome {
    case: LayoutCorruptionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutCorruptionView<'a> {
    Clean(&'a LayoutCoverageWitness),
    NotFound(&'a PhysicalArtifactFamily),
    Unsupported(&'a UnsupportedCorruptionState),
    StaleBinding(&'a LayoutCoverageWitness),
    RebuildRequired(&'a LayoutCorruptionClassification),
    Quarantined(&'a LayoutQuarantineWitness),
    QuarantineReadmissionRequired(&'a QuarantineReadmissionRequirement),
    OfflineReadmissionRequired(&'a OfflineReadmissionRequirement),
    ImportReadmissionRequired(&'a ImportReadmissionRequirement),
    MigrationRequired(&'a PhysicalArtifactFamily),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CorruptionClassificationCaseId(&'static str);

impl CorruptionClassificationCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

const CORRUPTION_CLASSIFICATION_CASES: [CorruptionClassificationCaseId; 10] = [
    CorruptionClassificationCaseId("layout.integrity.classification.clean"),
    CorruptionClassificationCaseId("layout.integrity.classification.not_found"),
    CorruptionClassificationCaseId("layout.integrity.classification.unsupported"),
    CorruptionClassificationCaseId("layout.integrity.classification.stale_binding"),
    CorruptionClassificationCaseId("layout.integrity.classification.rebuild_required"),
    CorruptionClassificationCaseId("layout.integrity.classification.quarantined"),
    CorruptionClassificationCaseId(
        "layout.integrity.classification.quarantine_readmission_required",
    ),
    CorruptionClassificationCaseId("layout.integrity.classification.offline_readmission_required"),
    CorruptionClassificationCaseId("layout.integrity.classification.import_readmission_required"),
    CorruptionClassificationCaseId("layout.integrity.classification.migration_required"),
];

pub fn corruption_classification_cases() -> impl Iterator<Item = CorruptionClassificationCaseId> {
    CORRUPTION_CLASSIFICATION_CASES.into_iter()
}

impl LayoutCorruptionOutcome {
    pub(crate) fn clean(value: LayoutCoverageWitness) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::Clean(value))
    }

    pub(crate) fn not_found(value: PhysicalArtifactFamily) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::NotFound(value))
    }

    pub(crate) fn unsupported(value: UnsupportedCorruptionState) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::Unsupported(value))
    }

    pub(crate) fn stale_binding(value: LayoutCoverageWitness) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::StaleBinding(value))
    }

    pub(crate) fn rebuild_required(value: LayoutCorruptionClassification) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::RebuildRequired(value))
    }

    pub(crate) fn quarantined(value: LayoutQuarantineWitness) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::Quarantined(value))
    }

    pub(crate) fn quarantine_readmission_required(value: QuarantineReadmissionRequirement) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::QuarantineReadmissionRequired(value))
    }

    pub(crate) fn offline_readmission_required(value: OfflineReadmissionRequirement) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::OfflineReadmissionRequired(value))
    }

    pub(crate) fn import_readmission_required(value: ImportReadmissionRequirement) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::ImportReadmissionRequired(value))
    }

    pub(crate) fn migration_required(value: PhysicalArtifactFamily) -> Self {
        Self::from_owner_payload(LayoutCorruptionCase::MigrationRequired(value))
    }

    fn from_owner_payload(case: LayoutCorruptionCase) -> Self {
        Self { case }
    }

    pub fn view(&self) -> LayoutCorruptionView<'_> {
        match &self.case {
            LayoutCorruptionCase::Clean(value) => LayoutCorruptionView::Clean(value),
            LayoutCorruptionCase::NotFound(value) => LayoutCorruptionView::NotFound(value),
            LayoutCorruptionCase::Unsupported(value) => LayoutCorruptionView::Unsupported(value),
            LayoutCorruptionCase::StaleBinding(value) => LayoutCorruptionView::StaleBinding(value),
            LayoutCorruptionCase::RebuildRequired(value) => {
                LayoutCorruptionView::RebuildRequired(value)
            }
            LayoutCorruptionCase::Quarantined(value) => LayoutCorruptionView::Quarantined(value),
            LayoutCorruptionCase::QuarantineReadmissionRequired(value) => {
                LayoutCorruptionView::QuarantineReadmissionRequired(value)
            }
            LayoutCorruptionCase::OfflineReadmissionRequired(value) => {
                LayoutCorruptionView::OfflineReadmissionRequired(value)
            }
            LayoutCorruptionCase::ImportReadmissionRequired(value) => {
                LayoutCorruptionView::ImportReadmissionRequired(value)
            }
            LayoutCorruptionCase::MigrationRequired(value) => {
                LayoutCorruptionView::MigrationRequired(value)
            }
        }
    }

    fn into_owner_payload(self) -> LayoutCorruptionCase {
        self.case
    }
}

impl LayoutCorruptionOutcome {
    pub fn class(&self) -> LayoutCorruptionClass {
        match self.view() {
            LayoutCorruptionView::Clean(_) => LayoutCorruptionClass::Clean,
            LayoutCorruptionView::NotFound(_) => LayoutCorruptionClass::NotFound,
            LayoutCorruptionView::Unsupported(_) => LayoutCorruptionClass::Unsupported,
            LayoutCorruptionView::StaleBinding(_) => LayoutCorruptionClass::StaleBinding,
            LayoutCorruptionView::RebuildRequired(_) => {
                LayoutCorruptionClass::DerivedProjectionCorruption
            }
            LayoutCorruptionView::Quarantined(_) => {
                LayoutCorruptionClass::AuthoritativeArtifactCorruption
            }
            LayoutCorruptionView::QuarantineReadmissionRequired(_)
            | LayoutCorruptionView::OfflineReadmissionRequired(_)
            | LayoutCorruptionView::ImportReadmissionRequired(_) => {
                LayoutCorruptionClass::ReadmissionRequired
            }
            LayoutCorruptionView::MigrationRequired(_) => LayoutCorruptionClass::MigrationRequired,
        }
    }

    pub fn case_id(&self) -> CorruptionClassificationCaseId {
        match &self.case {
            LayoutCorruptionCase::Clean(_) => CORRUPTION_CLASSIFICATION_CASES[0],
            LayoutCorruptionCase::NotFound(_) => CORRUPTION_CLASSIFICATION_CASES[1],
            LayoutCorruptionCase::Unsupported(_) => CORRUPTION_CLASSIFICATION_CASES[2],
            LayoutCorruptionCase::StaleBinding(_) => CORRUPTION_CLASSIFICATION_CASES[3],
            LayoutCorruptionCase::RebuildRequired(_) => CORRUPTION_CLASSIFICATION_CASES[4],
            LayoutCorruptionCase::Quarantined(_) => CORRUPTION_CLASSIFICATION_CASES[5],
            LayoutCorruptionCase::QuarantineReadmissionRequired(_) => {
                CORRUPTION_CLASSIFICATION_CASES[6]
            }
            LayoutCorruptionCase::OfflineReadmissionRequired(_) => {
                CORRUPTION_CLASSIFICATION_CASES[7]
            }
            LayoutCorruptionCase::ImportReadmissionRequired(_) => {
                CORRUPTION_CLASSIFICATION_CASES[8]
            }
            LayoutCorruptionCase::MigrationRequired(_) => CORRUPTION_CLASSIFICATION_CASES[9],
        }
    }

    pub(super) fn into_quarantined(self) -> Result<LayoutQuarantineWitness, Self> {
        match self.into_owner_payload() {
            LayoutCorruptionCase::Quarantined(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }

    pub fn into_quarantine_readmission_requirement(
        self,
    ) -> Result<QuarantineReadmissionRequirement, Self> {
        match self.into_owner_payload() {
            LayoutCorruptionCase::QuarantineReadmissionRequired(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }

    pub fn into_offline_readmission_requirement(
        self,
    ) -> Result<OfflineReadmissionRequirement, Self> {
        match self.into_owner_payload() {
            LayoutCorruptionCase::OfflineReadmissionRequired(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }

    pub fn into_import_readmission_requirement(self) -> Result<ImportReadmissionRequirement, Self> {
        match self.into_owner_payload() {
            LayoutCorruptionCase::ImportReadmissionRequired(value) => Ok(value),
            case => Err(Self::from_owner_payload(case)),
        }
    }
}
