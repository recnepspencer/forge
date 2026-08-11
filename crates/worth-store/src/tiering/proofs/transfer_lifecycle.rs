use serde::Serialize;

use super::classification::{PlacementExecutionOrigin, TierResidenceClass};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierTransferIntent {
    artifact_key: String,
    source_residence: TierResidenceClass,
    target_residence: TierResidenceClass,
    execution_origin: PlacementExecutionOrigin,
}

impl TierTransferIntent {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        source_residence: TierResidenceClass,
        target_residence: TierResidenceClass,
        execution_origin: PlacementExecutionOrigin,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            source_residence,
            target_residence,
            execution_origin,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn source_residence(&self) -> TierResidenceClass {
        self.source_residence
    }

    pub fn target_residence(&self) -> TierResidenceClass {
        self.target_residence
    }

    pub fn execution_origin(&self) -> PlacementExecutionOrigin {
        self.execution_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TransferredTierReplica {
    intent: TierTransferIntent,
    replica_locator: String,
}

impl TransferredTierReplica {
    pub(crate) fn new(intent: TierTransferIntent, replica_locator: impl Into<String>) -> Self {
        Self {
            intent,
            replica_locator: replica_locator.into(),
        }
    }

    pub fn intent(&self) -> &TierTransferIntent {
        &self.intent
    }

    pub fn replica_locator(&self) -> &str {
        &self.replica_locator
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VerifiedTierReplica {
    transferred_replica: TransferredTierReplica,
    verification_label: String,
}

impl VerifiedTierReplica {
    pub(crate) fn new(
        transferred_replica: TransferredTierReplica,
        verification_label: impl Into<String>,
    ) -> Self {
        Self {
            transferred_replica,
            verification_label: verification_label.into(),
        }
    }

    pub fn transferred_replica(&self) -> &TransferredTierReplica {
        &self.transferred_replica
    }

    pub fn verification_label(&self) -> &str {
        &self.verification_label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TierCutoverWitness {
    artifact_key: String,
    canonical_residence: TierResidenceClass,
}

impl TierCutoverWitness {
    pub(crate) fn new(
        artifact_key: impl Into<String>,
        canonical_residence: TierResidenceClass,
    ) -> Self {
        Self {
            artifact_key: artifact_key.into(),
            canonical_residence,
        }
    }

    pub fn artifact_key(&self) -> &str {
        &self.artifact_key
    }

    pub fn canonical_residence(&self) -> TierResidenceClass {
        self.canonical_residence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RetiredTierReplica {
    cutover_witness: TierCutoverWitness,
    retired_locator: String,
}

impl RetiredTierReplica {
    pub(crate) fn new(
        cutover_witness: TierCutoverWitness,
        retired_locator: impl Into<String>,
    ) -> Self {
        Self {
            cutover_witness,
            retired_locator: retired_locator.into(),
        }
    }

    pub fn cutover_witness(&self) -> &TierCutoverWitness {
        &self.cutover_witness
    }

    pub fn retired_locator(&self) -> &str {
        &self.retired_locator
    }
}
