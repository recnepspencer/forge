use crate::snapshot::{
    validate_snapshot_read_result_contract, AdmittedSnapshotContext, BridgeSnapshotReadError,
    BridgeSnapshotToken, PlannedTruthViewPacket, TruthSnapshotIdentity, TruthSnapshotReader,
    ValidatedSnapshotReadPacketResult,
};

pub struct MaterializedTruthViewObservation {
    planned: PlannedTruthViewPacket,
    snapshot_token: BridgeSnapshotToken,
    snapshot: AdmittedSnapshotContext<Box<dyn TruthSnapshotReader>>,
}

impl MaterializedTruthViewObservation {
    pub(crate) fn new(
        planned: PlannedTruthViewPacket,
        snapshot_token: BridgeSnapshotToken,
        snapshot: AdmittedSnapshotContext<Box<dyn TruthSnapshotReader>>,
    ) -> Self {
        Self {
            planned,
            snapshot_token,
            snapshot,
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

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.snapshot.snapshot_identity()
    }

    pub fn read_packet(&self) -> &crate::snapshot::SnapshotReadPacket {
        self.planned.read_packet()
    }

    pub fn read_planned_packet(&self) -> Result<ValidatedSnapshotReadPacketResult, BridgeSnapshotReadError> {
        let read_result = self.snapshot.read_packet(self.planned.read_packet())?;
        if read_result.snapshot_identity() != self.snapshot.snapshot_identity() {
            return Err(BridgeSnapshotReadError::new(format!(
                "Truth-view observation read returned `{}` but materialized snapshot authority was `{}`.",
                read_result.snapshot_identity().as_str(),
                self.snapshot.snapshot_identity().as_str()
            )));
        }

        validate_snapshot_read_result_contract(self.planned.read_packet(), read_result)
    }
}
