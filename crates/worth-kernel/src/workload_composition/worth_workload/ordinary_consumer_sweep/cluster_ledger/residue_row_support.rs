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

use super::{
    WorthWorkloadOrdinaryConsumerClusterKind, WorthWorkloadOrdinaryConsumerClusterRowDisposition,
    WorthWorkloadOrdinaryConsumerSweepResidueRow,
};
use crate::workload_composition::{
    planner_owned_routing::{
        PublicCloseoutConsumerResidueDisposition, PublicCloseoutConsumerResidueOwner,
        PublicCloseoutConsumerResidueRow,
    },
    ConflictBatchAdmissionDisposition, ConflictBatchAdmissionInventoryRow,
    ConflictBatchAdmissionOwner,
};

impl WorthWorkloadOrdinaryConsumerSweepResidueRow {
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
                PublicCloseoutConsumerResidueDisposition::QueryGap => {
                    WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap
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
        EvidenceLookupPublicCloseoutResidueOwner::ForgeQuery => "forge-query",
    }
}

fn spatial_owner_label(owner: SpatialConsumerResidueOwner) -> &'static str {
    match owner {
        SpatialConsumerResidueOwner::WorthSpatial => "worth-spatial",
    }
}
