use topology::facade::{
    QueryBackedConsumerResidueDisposition, QueryBackedConsumerResidueOwner,
    QueryBackedConsumerResidueRow, TopologyConsumerResidueDisposition,
    TopologyConsumerResidueOwner, TopologyConsumerResidueRow,
};
use worth_spatial::facade::evidence_lookup_public_closeout::{
    EvidenceLookupPublicCloseoutResidueDisposition, EvidenceLookupPublicCloseoutResidueOwner,
    EvidenceLookupPublicCloseoutResidueRow,
};
use worth_spatial::facade::spatial_compiled_product_consumer_cutover::{
    SpatialConsumerResidueDisposition, SpatialConsumerResidueOwner, SpatialConsumerResidueRow,
};

use crate::workload_composition::{
    public_closeout::{
        PublicCloseoutConsumerResidueDisposition, PublicCloseoutConsumerResidueOwner,
        PublicCloseoutConsumerResidueRow,
    },
    ConflictBatchAdmissionCertificationPosture, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionOwner,
};

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

    pub(crate) fn from_inventory_row(
        cluster_kind: WorthWorkloadOrdinaryConsumerClusterKind,
        row: &ConflictBatchAdmissionInventoryRow,
    ) -> Self {
        let disposition = match row.disposition() {
            ConflictBatchAdmissionDisposition::Migrate => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer
            }
            ConflictBatchAdmissionDisposition::Delete => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::DeletedDirectHelper
            }
            ConflictBatchAdmissionDisposition::Cap => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
            }
            ConflictBatchAdmissionDisposition::CertificationOnly => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::CertificationOnlyDeniedAsOrdinaryProof
            }
            ConflictBatchAdmissionDisposition::QueryGap => {
                WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap
            }
        };
        Self::new(
            cluster_kind,
            row.source_path(),
            row.surface_name(),
            inventory_owner_label(row.owner()),
            disposition,
            row.blocker(),
            row.removal_trigger(),
        )
    }

    pub(crate) fn from_topology_residue_row(row: &TopologyConsumerResidueRow) -> Self {
        Self::new(
            WorthWorkloadOrdinaryConsumerClusterKind::TopologyDerived,
            row.source_path(),
            row.current_surface(),
            topology_owner_label(row.owner()),
            match row.disposition() {
                TopologyConsumerResidueDisposition::ExplicitResidue => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
                }
                TopologyConsumerResidueDisposition::QueryGap => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap
                }
                TopologyConsumerResidueDisposition::AuthoritativeOrdinaryConsumer => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer
                }
            },
            row.blocker(),
            row.removal_trigger(),
        )
    }

    pub(crate) fn from_query_backed_residue_row(row: &QueryBackedConsumerResidueRow) -> Self {
        Self::new(
            WorthWorkloadOrdinaryConsumerClusterKind::QueryBacked,
            row.source_path(),
            row.current_surface(),
            query_backed_owner_label(row.owner()),
            match row.disposition() {
                QueryBackedConsumerResidueDisposition::ExplicitResidue => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
                }
                QueryBackedConsumerResidueDisposition::QueryGap => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap
                }
            },
            row.blocker(),
            row.removal_trigger(),
        )
    }

    pub(crate) fn from_public_closeout_residue_row(row: &PublicCloseoutConsumerResidueRow) -> Self {
        Self::new(
            WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
            row.source_path(),
            row.current_surface(),
            public_closeout_owner_label(row.owner()),
            match row.disposition() {
                PublicCloseoutConsumerResidueDisposition::ExplicitResidue => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
                }
            },
            row.blocker(),
            row.removal_trigger(),
        )
    }

    pub(crate) fn from_evidence_lookup_public_closeout_residue_row(
        row: &EvidenceLookupPublicCloseoutResidueRow,
    ) -> Self {
        Self::new(
            WorthWorkloadOrdinaryConsumerClusterKind::PublicCloseout,
            row.source_path(),
            row.current_surface(),
            evidence_lookup_owner_label(row.owner()),
            match row.disposition() {
                EvidenceLookupPublicCloseoutResidueDisposition::ExplicitResidue => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
                }
                EvidenceLookupPublicCloseoutResidueDisposition::QueryGap => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap
                }
            },
            row.blocker(),
            row.removal_trigger(),
        )
    }

    pub(crate) fn from_spatial_residue_row(row: &SpatialConsumerResidueRow) -> Self {
        Self::new(
            WorthWorkloadOrdinaryConsumerClusterKind::SpatialDerived,
            row.source_path(),
            row.current_surface(),
            spatial_owner_label(row.owner()),
            match row.disposition() {
                SpatialConsumerResidueDisposition::ExplicitResidue => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue
                }
                SpatialConsumerResidueDisposition::CertificationOnly => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::CertificationOnlyDeniedAsOrdinaryProof
                }
                SpatialConsumerResidueDisposition::AuthoritativeOrdinaryConsumer => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer
                }
            },
            row.blocker(),
            row.removal_trigger(),
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

fn inventory_owner_label(owner: ConflictBatchAdmissionOwner) -> &'static str {
    match owner {
        ConflictBatchAdmissionOwner::WorthKernel => "worth-kernel",
        ConflictBatchAdmissionOwner::WorthTopo => "worth-topo",
        ConflictBatchAdmissionOwner::WorthSpatial => "worth-spatial",
        ConflictBatchAdmissionOwner::ForgeQuery => "forge-query",
    }
}

fn topology_owner_label(owner: TopologyConsumerResidueOwner) -> &'static str {
    owner.as_str()
}

fn query_backed_owner_label(owner: QueryBackedConsumerResidueOwner) -> &'static str {
    match owner {
        QueryBackedConsumerResidueOwner::WorthTopo => "worth-topo",
        QueryBackedConsumerResidueOwner::ForgeQuery => "forge-query",
    }
}

fn public_closeout_owner_label(owner: PublicCloseoutConsumerResidueOwner) -> &'static str {
    match owner {
        PublicCloseoutConsumerResidueOwner::WorthKernel => "worth-kernel",
        PublicCloseoutConsumerResidueOwner::WorthTopo => "worth-topo",
        PublicCloseoutConsumerResidueOwner::WorthSpatial => "worth-spatial",
        PublicCloseoutConsumerResidueOwner::ForgeQuery => "forge-query",
    }
}

fn evidence_lookup_owner_label(owner: EvidenceLookupPublicCloseoutResidueOwner) -> &'static str {
    match owner {
        EvidenceLookupPublicCloseoutResidueOwner::WorthSpatial => "worth-spatial",
        EvidenceLookupPublicCloseoutResidueOwner::WorthTopo => "worth-topo",
    }
}

fn spatial_owner_label(owner: SpatialConsumerResidueOwner) -> &'static str {
    match owner {
        SpatialConsumerResidueOwner::WorthSpatial => "worth-spatial",
    }
}
