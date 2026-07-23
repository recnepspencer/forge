use crate::projection_consumption::{ProjectionFactFieldPath, ProjectionSourceFamily};
use worth_foundational::facade::AspectValuePosture;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryNativeAccessDenialKind {
    StaleInstallationGeneration,
    RuntimeMismatch,
    AccessKeyInstallationGenerationMismatch,
    CapabilityMismatch,
    LayoutMismatch,
    RowOutOfBounds,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeAccessDenial {
    kind: WorthQueryNativeAccessDenialKind,
    field_path: ProjectionFactFieldPath,
    contract_key: worth_foundational::facade::AspectKey,
    contract_identity: worth_foundational::facade::AspectIdentity,
    contract_revision: worth_foundational::facade::AspectContractRevision,
    expected_shape: AspectValuePosture,
    absence: worth_foundational::facade::AbsenceLaw,
    source_family: ProjectionSourceFamily,
    source_identity: String,
    projection_authority: String,
    counters: super::WorthQueryNativeAccessCounters,
}

impl WorthQueryNativeAccessDenial {
    pub(crate) fn new(
        kind: WorthQueryNativeAccessDenialKind,
        key: &super::WorthQueryNativeAccessKey,
        source_family: ProjectionSourceFamily,
        source_identity: &str,
        projection_authority: &str,
        counters: super::WorthQueryNativeAccessCounters,
    ) -> Self {
        Self {
            kind,
            field_path: key.field_path().clone(),
            contract_key: key.contract_key().clone(),
            contract_identity: key.contract_identity(),
            contract_revision: key.contract_revision(),
            expected_shape: key.expected_shape(),
            absence: key.absence_posture(),
            source_family,
            source_identity: source_identity.to_string(),
            projection_authority: projection_authority.to_string(),
            counters,
        }
    }

    pub fn kind(&self) -> WorthQueryNativeAccessDenialKind {
        self.kind
    }

    pub fn field_path(&self) -> &ProjectionFactFieldPath {
        &self.field_path
    }

    pub fn contract_key(&self) -> &worth_foundational::facade::AspectKey {
        &self.contract_key
    }

    pub fn contract_revision(&self) -> worth_foundational::facade::AspectContractRevision {
        self.contract_revision
    }

    pub fn contract_identity(&self) -> worth_foundational::facade::AspectIdentity {
        self.contract_identity
    }

    pub fn expected_shape(&self) -> AspectValuePosture {
        self.expected_shape
    }

    pub fn absence_posture(&self) -> worth_foundational::facade::AbsenceLaw {
        self.absence
    }

    pub fn source_family(&self) -> ProjectionSourceFamily {
        self.source_family
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn projection_authority(&self) -> &str {
        &self.projection_authority
    }

    pub fn counters(&self) -> super::WorthQueryNativeAccessCounters {
        self.counters
    }
}
