use crate::{
    AdmittedReplicationSource, ObservedReplicationProgress, PublishedReplication,
    ReplicationDeliveryKind, ReplicationDuplicateDelivery, ReplicationPeerProgress,
    ReplicationPublicationReadiness,
};
use worth_store_authority::StoreCurrentAuthorityIdentity;
use worth_store_security::StoreSecurityScopeIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicationAdmissionStage {
    SourceAdmitted,
    ProgressObserved,
    PublicationReady,
    Duplicate,
    Published,
    PeerProgress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicationAdmissionObservation {
    stage: ReplicationAdmissionStage,
    peer_id: String,
    source_epoch: u64,
    lineage: String,
    replay_digest: String,
    first_lsn: u64,
    last_lsn: u64,
    delivery_kind: Option<ReplicationDeliveryKind>,
    current_authority: StoreCurrentAuthorityIdentity,
    security_scope: StoreSecurityScopeIdentity,
}

pub trait ObserveReplicationAdmission: private::Sealed {
    fn observe_replication_admission(&self) -> ReplicationAdmissionObservation;
}

fn observe_source(
    source: &AdmittedReplicationSource,
    stage: ReplicationAdmissionStage,
    delivery_kind: Option<ReplicationDeliveryKind>,
) -> ReplicationAdmissionObservation {
    let replay = source.replay_identity();
    ReplicationAdmissionObservation {
        stage,
        peer_id: source.peer_id().as_str().to_owned(),
        source_epoch: source.source_epoch().get(),
        lineage: source.lineage().as_str().to_owned(),
        replay_digest: replay.digest().to_owned(),
        first_lsn: replay.first_lsn(),
        last_lsn: replay.last_lsn(),
        delivery_kind,
        current_authority: source.current_authority(),
        security_scope: source.security_scope().admitted().identity(),
    }
}

impl ObserveReplicationAdmission for AdmittedReplicationSource {
    fn observe_replication_admission(&self) -> ReplicationAdmissionObservation {
        observe_source(self, ReplicationAdmissionStage::SourceAdmitted, None)
    }
}

impl ObserveReplicationAdmission for ReplicationPublicationReadiness {
    fn observe_replication_admission(&self) -> ReplicationAdmissionObservation {
        observe_source(
            self.source(),
            ReplicationAdmissionStage::PublicationReady,
            Some(self.delivery_kind()),
        )
    }
}

impl ObserveReplicationAdmission for ObservedReplicationProgress {
    fn observe_replication_admission(&self) -> ReplicationAdmissionObservation {
        observe_source(
            self.source(),
            ReplicationAdmissionStage::ProgressObserved,
            Some(self.delivery_kind()),
        )
    }
}

impl ObserveReplicationAdmission for PublishedReplication {
    fn observe_replication_admission(&self) -> ReplicationAdmissionObservation {
        observe_source(
            self.source(),
            ReplicationAdmissionStage::Published,
            Some(self.delivery_kind()),
        )
    }
}

impl ObserveReplicationAdmission for ReplicationDuplicateDelivery {
    fn observe_replication_admission(&self) -> ReplicationAdmissionObservation {
        let replay = self.replay_identity();
        ReplicationAdmissionObservation {
            stage: ReplicationAdmissionStage::Duplicate,
            peer_id: self.peer_id().as_str().to_owned(),
            source_epoch: self.source_epoch().get(),
            lineage: self.lineage().as_str().to_owned(),
            replay_digest: replay.digest().to_owned(),
            first_lsn: replay.first_lsn(),
            last_lsn: replay.last_lsn(),
            delivery_kind: None,
            current_authority: self.current_authority(),
            security_scope: self.security_scope(),
        }
    }
}

impl ObserveReplicationAdmission for ReplicationPeerProgress {
    fn observe_replication_admission(&self) -> ReplicationAdmissionObservation {
        let replay = self.replay_identity();
        ReplicationAdmissionObservation {
            stage: ReplicationAdmissionStage::PeerProgress,
            peer_id: self.peer_id().as_str().to_owned(),
            source_epoch: self.source_epoch().get(),
            lineage: self.lineage().as_str().to_owned(),
            replay_digest: replay.digest().to_owned(),
            first_lsn: replay.first_lsn(),
            last_lsn: replay.last_lsn(),
            delivery_kind: None,
            current_authority: self.current_authority(),
            security_scope: self.security_scope(),
        }
    }
}

impl ReplicationAdmissionObservation {
    pub const fn stage(&self) -> ReplicationAdmissionStage {
        self.stage
    }

    pub fn peer_id(&self) -> &str {
        &self.peer_id
    }

    pub const fn source_epoch(&self) -> u64 {
        self.source_epoch
    }

    pub fn lineage(&self) -> &str {
        &self.lineage
    }

    pub fn replay_digest(&self) -> &str {
        &self.replay_digest
    }

    pub const fn first_lsn(&self) -> u64 {
        self.first_lsn
    }

    pub const fn last_lsn(&self) -> u64 {
        self.last_lsn
    }

    pub const fn delivery_kind(&self) -> Option<ReplicationDeliveryKind> {
        self.delivery_kind
    }

    pub const fn current_authority(&self) -> StoreCurrentAuthorityIdentity {
        self.current_authority
    }

    pub const fn security_scope(&self) -> StoreSecurityScopeIdentity {
        self.security_scope
    }
}

mod private {
    pub trait Sealed {}

    impl Sealed for crate::AdmittedReplicationSource {}
    impl Sealed for crate::ReplicationPublicationReadiness {}
    impl Sealed for crate::ObservedReplicationProgress {}
    impl Sealed for crate::PublishedReplication {}
    impl Sealed for crate::ReplicationDuplicateDelivery {}
    impl Sealed for crate::ReplicationPeerProgress {}
}
