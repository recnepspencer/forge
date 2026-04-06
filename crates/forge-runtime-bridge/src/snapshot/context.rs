use crate::snapshot::{
    BridgeSnapshotReadError, SnapshotReadPacket, SnapshotReadPacketResult, TruthSnapshotIdentity,
};

pub trait TruthSnapshotReader: Send + Sync + 'static {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity;

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError>;
}

impl<T> TruthSnapshotReader for Box<T>
where
    T: TruthSnapshotReader + ?Sized,
{
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        (**self).snapshot_identity()
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        (**self).read_packet(request)
    }
}

#[derive(Debug)]
pub struct BridgeSnapshotContext<R: TruthSnapshotReader> {
    snapshot: R,
    snapshot_identity: TruthSnapshotIdentity,
}

#[derive(Debug)]
pub struct AdmittedSnapshotContext<R: TruthSnapshotReader> {
    bound: BridgeSnapshotContext<R>,
}

impl<R: TruthSnapshotReader> BridgeSnapshotContext<R> {
    pub(crate) fn bind(snapshot: R) -> Self {
        let snapshot_identity = snapshot.snapshot_identity();
        Self {
            snapshot,
            snapshot_identity,
        }
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn reader(&self) -> &R {
        &self.snapshot
    }

    pub fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        self.snapshot.read_packet(request)
    }
}

impl<R: TruthSnapshotReader> AdmittedSnapshotContext<R> {
    pub(crate) fn admit_for(
        snapshot: BridgeSnapshotContext<R>,
        planned_identity: &TruthSnapshotIdentity,
    ) -> Result<Self, TruthSnapshotIdentity> {
        if snapshot.snapshot_identity() != planned_identity {
            return Err(snapshot.snapshot_identity().clone());
        }

        Ok(Self { bound: snapshot })
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.bound.snapshot_identity()
    }

    pub fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
        self.bound.read_packet(request)
    }
}

#[cfg(test)]
mod tests {
    use crate::snapshot::{
        BridgeSnapshotContext, BridgeSnapshotReadError, SnapshotReadPacket, SnapshotReadPacketResult,
        TruthSnapshotIdentity, TruthSnapshotReader,
    };

    struct StaticReader;

    impl TruthSnapshotReader for StaticReader {
        fn snapshot_identity(&self) -> TruthSnapshotIdentity {
            TruthSnapshotIdentity::new("snapshot-a")
        }

        fn read_packet(
            &self,
            _request: &SnapshotReadPacket,
        ) -> Result<SnapshotReadPacketResult, BridgeSnapshotReadError> {
            Ok(SnapshotReadPacketResult::new(
                TruthSnapshotIdentity::new("snapshot-a"),
                vec![],
            ))
        }
    }

    #[test]
    fn context_binds_snapshot_identity_from_reader() {
        let context = BridgeSnapshotContext::bind(StaticReader);

        assert_eq!(context.snapshot_identity().as_str(), "snapshot-a");
    }
}
