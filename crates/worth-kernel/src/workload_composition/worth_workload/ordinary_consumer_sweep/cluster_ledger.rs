use crate::workload_composition::{
    ConflictBatchAdmissionCertificationPosture,
    ConflictBatchAdmissionInventoryRow,
};

mod residue_row_support;
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthWorkloadOrdinaryConsumerClusterKind {
    TopologyDerived,
    SpatialDerived,
    QueryBacked,
    RetainedReplay,
    PublicCloseout,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily {
    TopologyDerivedMaterializationConsumers,
    EvidenceLookupIndexProductConsumers,
    QueryBackedProjectionAndLowerRuntimeConsumers,
    RetainedReplayProductConsumers,
    PublicCloseoutAndReadModelConsumers,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthWorkloadOrdinaryConsumerClusterRowDisposition {
    MigratedOrdinaryConsumer,
    DeletedDirectHelper,
    CappedResidue,
    QueryGap,
    CertificationOnlyDeniedAsOrdinaryProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadOrdinaryConsumerSweepResidueRow {
    cluster_kind: WorthWorkloadOrdinaryConsumerClusterKind,
    source_path: String,
    surface_name: String,
    owner: String,
    disposition: WorthWorkloadOrdinaryConsumerClusterRowDisposition,
    blocker: String,
    removal_trigger: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadOrdinaryConsumerClusterLedger {
    cluster_kind: WorthWorkloadOrdinaryConsumerClusterKind,
    blocked_follow_on_family: WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily,
    proof_basis_digests: Vec<String>,
    rows: Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow>,
}

impl WorthWorkloadOrdinaryConsumerClusterKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyDerived => "topology-derived",
            Self::SpatialDerived => "spatial-derived",
            Self::QueryBacked => "query-backed",
            Self::RetainedReplay => "retained-replay",
            Self::PublicCloseout => "public-closeout",
        }
    }
}

impl WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyDerivedMaterializationConsumers => {
                "topology-derived-materialization-consumers"
            }
            Self::EvidenceLookupIndexProductConsumers => "evidence-lookup-index-product-consumers",
            Self::QueryBackedProjectionAndLowerRuntimeConsumers => {
                "query-backed-projection-and-lower-runtime-consumers"
            }
            Self::RetainedReplayProductConsumers => "retained-replay-product-consumers",
            Self::PublicCloseoutAndReadModelConsumers => "public-closeout-and-read-model-consumers",
        }
    }
}

impl WorthWorkloadOrdinaryConsumerClusterRowDisposition {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigratedOrdinaryConsumer => "migrated-ordinary-consumer",
            Self::DeletedDirectHelper => "deleted-direct-helper",
            Self::CappedResidue => "capped-residue",
            Self::QueryGap => "query-gap",
            Self::CertificationOnlyDeniedAsOrdinaryProof => {
                "certification-only-denied-as-ordinary-proof"
            }
        }
    }
}

impl WorthWorkloadOrdinaryConsumerSweepResidueRow {
    fn new(
        cluster_kind: WorthWorkloadOrdinaryConsumerClusterKind,
        source_path: impl Into<String>,
        surface_name: impl Into<String>,
        owner: impl Into<String>,
        disposition: WorthWorkloadOrdinaryConsumerClusterRowDisposition,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self {
            cluster_kind,
            source_path: source_path.into(),
            surface_name: surface_name.into(),
            owner: owner.into(),
            disposition,
            blocker: blocker.into(),
            removal_trigger: removal_trigger.into(),
        }
    }

    pub(crate) fn ordinary_migrated(
        cluster_kind: WorthWorkloadOrdinaryConsumerClusterKind,
        source_path: impl Into<String>,
        surface_name: impl Into<String>,
        owner: impl Into<String>,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self::new(
            cluster_kind,
            source_path,
            surface_name,
            owner,
            WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer,
            blocker,
            removal_trigger,
        )
    }

    pub(crate) fn with_disposition(
        cluster_kind: WorthWorkloadOrdinaryConsumerClusterKind,
        source_path: impl Into<String>,
        surface_name: impl Into<String>,
        owner: impl Into<String>,
        disposition: WorthWorkloadOrdinaryConsumerClusterRowDisposition,
        blocker: impl Into<String>,
        removal_trigger: impl Into<String>,
    ) -> Self {
        Self::new(
            cluster_kind,
            source_path,
            surface_name,
            owner,
            disposition,
            blocker,
            removal_trigger,
        )
    }

    pub const fn cluster_kind(&self) -> WorthWorkloadOrdinaryConsumerClusterKind {
        self.cluster_kind
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub const fn disposition(&self) -> WorthWorkloadOrdinaryConsumerClusterRowDisposition {
        self.disposition
    }

    pub fn blocker(&self) -> &str {
        &self.blocker
    }

    pub fn removal_trigger(&self) -> &str {
        &self.removal_trigger
    }
}

impl WorthWorkloadOrdinaryConsumerClusterLedger {
    pub(crate) fn new(
        cluster_kind: WorthWorkloadOrdinaryConsumerClusterKind,
        blocked_follow_on_family: WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily,
        mut proof_basis_digests: Vec<String>,
        mut rows: Vec<WorthWorkloadOrdinaryConsumerSweepResidueRow>,
    ) -> Self {
        proof_basis_digests.sort();
        proof_basis_digests.dedup();
        rows.sort_by(|left, right| left.surface_name.cmp(&right.surface_name));
        Self {
            cluster_kind,
            blocked_follow_on_family,
            proof_basis_digests,
            rows,
        }
    }

    pub const fn cluster_kind(&self) -> WorthWorkloadOrdinaryConsumerClusterKind {
        self.cluster_kind
    }

    pub fn proof_basis_digests(&self) -> &[String] {
        &self.proof_basis_digests
    }

    pub const fn blocked_follow_on_family(
        &self,
    ) -> WorthWorkloadOrdinaryConsumerBlockedFollowOnFamily {
        self.blocked_follow_on_family
    }

    pub fn rows(&self) -> &[WorthWorkloadOrdinaryConsumerSweepResidueRow] {
        &self.rows
    }

    pub fn migrated_count(&self) -> usize {
        self.count_rows(
            WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer,
        )
    }

    pub fn deleted_count(&self) -> usize {
        self.count_rows(WorthWorkloadOrdinaryConsumerClusterRowDisposition::DeletedDirectHelper)
    }

    pub fn capped_residue_count(&self) -> usize {
        self.count_rows(WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue)
    }

    pub fn query_gap_count(&self) -> usize {
        self.count_rows(WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap)
    }

    pub fn certification_only_count(&self) -> usize {
        self.count_rows(
            WorthWorkloadOrdinaryConsumerClusterRowDisposition::CertificationOnlyDeniedAsOrdinaryProof,
        )
    }

    fn count_rows(&self, disposition: WorthWorkloadOrdinaryConsumerClusterRowDisposition) -> usize {
        self.rows
            .iter()
            .filter(|row| row.disposition() == disposition)
            .count()
    }
}

pub(crate) fn row_is_non_ordinary_residue(row: &ConflictBatchAdmissionInventoryRow) -> bool {
    row.certification_posture()
        != ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable
}
