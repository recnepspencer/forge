use forge_foundational::facade::{
    AspectValue, ContractValidatedAspectArtifact, ContractValidatedAspectValueView,
    ContractValidationInput, StructAspectValue,
};

use super::SnapshotReadCorrelationId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SnapshotReadValue {
    Scalar(AspectValue),
    Struct(StructAspectValue),
}

impl SnapshotReadValue {
    pub fn scalar_value(&self) -> Option<&AspectValue> {
        match self {
            Self::Scalar(value) => Some(value),
            Self::Struct(_) => None,
        }
    }

    pub(crate) fn into_validation_input(self) -> ContractValidationInput {
        match self {
            Self::Scalar(value) => value.into(),
            Self::Struct(value) => value.into(),
        }
    }
}

impl From<AspectValue> for SnapshotReadValue {
    fn from(value: AspectValue) -> Self {
        Self::Scalar(value)
    }
}

impl From<StructAspectValue> for SnapshotReadValue {
    fn from(value: StructAspectValue) -> Self {
        Self::Struct(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadRecord {
    correlation_id: SnapshotReadCorrelationId,
    read_value: SnapshotReadValue,
}

impl SnapshotReadRecord {
    pub fn for_request(
        request: &super::SnapshotReadRequest,
        read_value: impl Into<SnapshotReadValue>,
    ) -> Self {
        Self {
            correlation_id: request.correlation_id().clone(),
            read_value: read_value.into(),
        }
    }

    pub fn correlation_id(&self) -> &SnapshotReadCorrelationId {
        &self.correlation_id
    }

    pub fn read_value(&self) -> &SnapshotReadValue {
        &self.read_value
    }

    pub fn scalar_aspect_value(&self) -> Option<&AspectValue> {
        self.read_value.scalar_value()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotReadPacketResult {
    snapshot_identity: super::TruthSnapshotIdentity,
    records: Vec<SnapshotReadRecord>,
}

impl SnapshotReadPacketResult {
    pub fn new(
        snapshot_identity: super::TruthSnapshotIdentity,
        records: Vec<SnapshotReadRecord>,
    ) -> Self {
        Self {
            snapshot_identity,
            records,
        }
    }

    pub fn snapshot_identity(&self) -> &super::TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn records(&self) -> &[SnapshotReadRecord] {
        &self.records
    }

    pub(crate) fn into_parts(self) -> (super::TruthSnapshotIdentity, Vec<SnapshotReadRecord>) {
        (self.snapshot_identity, self.records)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedSnapshotReadPacketResult {
    snapshot_identity: super::TruthSnapshotIdentity,
    records: Vec<ValidatedSnapshotReadRecord>,
}

impl ValidatedSnapshotReadPacketResult {
    pub(crate) fn validated(
        snapshot_identity: super::TruthSnapshotIdentity,
        records: Vec<ValidatedSnapshotReadRecord>,
    ) -> Self {
        Self {
            snapshot_identity,
            records,
        }
    }

    pub fn snapshot_identity(&self) -> &super::TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn records(&self) -> &[ValidatedSnapshotReadRecord] {
        &self.records
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedSnapshotReadRecord {
    correlation_id: SnapshotReadCorrelationId,
    validated_value: ContractValidatedAspectArtifact,
}

impl ValidatedSnapshotReadRecord {
    pub(crate) fn new(
        correlation_id: SnapshotReadCorrelationId,
        validated_value: ContractValidatedAspectArtifact,
    ) -> Self {
        Self {
            correlation_id,
            validated_value,
        }
    }

    pub fn correlation_id(&self) -> &SnapshotReadCorrelationId {
        &self.correlation_id
    }

    pub fn validated_value(&self) -> &ContractValidatedAspectArtifact {
        &self.validated_value
    }

    pub fn scalar_aspect_value(&self) -> Option<&AspectValue> {
        contract_validated_scalar_aspect_value(&self.validated_value)
    }
}

pub(crate) fn contract_validated_scalar_aspect_value(
    validated_value: &ContractValidatedAspectArtifact,
) -> Option<&AspectValue> {
    match validated_value.payload().view() {
        ContractValidatedAspectValueView::Scalar(value) => Some(value),
        ContractValidatedAspectValueView::Struct(_) => None,
    }
}
