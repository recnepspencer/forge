use crate::snapshot::{
    validate_snapshot_read_result_contract, AdmittedSnapshotContext, BridgeSnapshotReadError,
    BridgeSnapshotToken, PlannedTruthViewPacket, TruthSnapshotIdentity, TruthSnapshotReader,
    ValidatedSnapshotReadPacketResult,
};

pub struct TruthViewObservationReader {
    snapshot: AdmittedSnapshotContext<Box<dyn TruthSnapshotReader>>,
}

impl TruthViewObservationReader {
    pub(crate) fn new(snapshot: AdmittedSnapshotContext<Box<dyn TruthSnapshotReader>>) -> Self {
        Self { snapshot }
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.snapshot.snapshot_identity()
    }

    pub fn read_packet(
        &self,
        request: &crate::snapshot::SnapshotReadPacket,
    ) -> Result<crate::snapshot::SnapshotReadPacketResult, BridgeSnapshotReadError> {
        self.snapshot.read_packet(request)
    }
}

pub struct MaterializedTruthViewObservation {
    planned: PlannedTruthViewPacket,
    snapshot_token: BridgeSnapshotToken,
    materialization_path: crate::diagnostics::BridgeHistoricalMaterializationPath,
    snapshot_reader: TruthViewObservationReader,
}

impl MaterializedTruthViewObservation {
    pub(crate) fn new(
        planned: PlannedTruthViewPacket,
        snapshot_token: BridgeSnapshotToken,
        materialization_path: crate::diagnostics::BridgeHistoricalMaterializationPath,
        snapshot: AdmittedSnapshotContext<Box<dyn TruthSnapshotReader>>,
    ) -> Self {
        Self {
            planned,
            snapshot_token,
            materialization_path,
            snapshot_reader: TruthViewObservationReader::new(snapshot),
        }
    }

    pub fn planned(&self) -> &PlannedTruthViewPacket {
        &self.planned
    }

    pub fn authority_basis(&self) -> &crate::snapshot::BridgeTruthViewAuthorityBasis {
        self.planned.authority_basis()
    }

    pub fn snapshot_token(&self) -> &BridgeSnapshotToken {
        &self.snapshot_token
    }

    pub fn materialization_path(&self) -> crate::diagnostics::BridgeHistoricalMaterializationPath {
        self.materialization_path
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.snapshot_reader.snapshot_identity()
    }

    pub fn read_packet(&self) -> &crate::snapshot::SnapshotReadPacket {
        self.planned.read_packet()
    }

    pub fn read_planned_packet(&self) -> Result<ValidatedSnapshotReadPacketResult, BridgeSnapshotReadError> {
        let read_result = self.snapshot_reader.read_packet(self.planned.read_packet())?;
        if read_result.snapshot_identity() != self.snapshot_reader.snapshot_identity() {
            return Err(BridgeSnapshotReadError::new(format!(
                "Truth-view observation read returned `{}` but materialized snapshot authority was `{}`.",
                read_result.snapshot_identity().as_str(),
                self.snapshot_reader.snapshot_identity().as_str()
            )));
        }

        validate_snapshot_read_result_contract(self.planned.read_packet(), read_result)
    }
}
