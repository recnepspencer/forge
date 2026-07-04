use topology::facade::{QueryBackedConsumerResidueOwner, TopologyConsumerResidueOwner};
use worth_spatial::facade::evidence_lookup_public_closeout::EvidenceLookupPublicCloseoutResidueOwner;
use worth_spatial::facade::spatial_compiled_product_consumer_cutover::SpatialConsumerResidueOwner;

use super::cluster_ledger::WorthWorkloadOrdinaryConsumerClusterRowDisposition;
use crate::workload_composition::planner_owned_routing::PublicCloseoutConsumerResidueOwner;
use crate::workload_composition::{ConflictBatchAdmissionDisposition, ConflictBatchAdmissionOwner};

pub(crate) fn inventory_owner_label(owner: ConflictBatchAdmissionOwner) -> &'static str {
    match owner {
        ConflictBatchAdmissionOwner::WorthKernel => "worth-kernel",
        ConflictBatchAdmissionOwner::WorthTopo => "worth-topo",
        ConflictBatchAdmissionOwner::WorthSpatial => "worth-spatial",
        ConflictBatchAdmissionOwner::ForgeQuery => "forge-query",
    }
}

pub(crate) fn disposition_label(disposition: ConflictBatchAdmissionDisposition) -> &'static str {
    match disposition {
        ConflictBatchAdmissionDisposition::Migrate => WorthWorkloadOrdinaryConsumerClusterRowDisposition::MigratedOrdinaryConsumer.as_str(),
        ConflictBatchAdmissionDisposition::Delete => WorthWorkloadOrdinaryConsumerClusterRowDisposition::DeletedDirectHelper.as_str(),
        ConflictBatchAdmissionDisposition::Cap => WorthWorkloadOrdinaryConsumerClusterRowDisposition::CappedResidue.as_str(),
        ConflictBatchAdmissionDisposition::CertificationOnly => WorthWorkloadOrdinaryConsumerClusterRowDisposition::CertificationOnlyDeniedAsOrdinaryProof.as_str(),
        ConflictBatchAdmissionDisposition::QueryGap => WorthWorkloadOrdinaryConsumerClusterRowDisposition::QueryGap.as_str(),
    }
}

pub(crate) fn topology_owner_label(owner: TopologyConsumerResidueOwner) -> &'static str {
    match owner {
        TopologyConsumerResidueOwner::WorthTopo => "worth-topo",
        TopologyConsumerResidueOwner::ForgeQuery => "forge-query",
    }
}

pub(crate) fn query_backed_owner_label(owner: QueryBackedConsumerResidueOwner) -> &'static str {
    match owner {
        QueryBackedConsumerResidueOwner::WorthTopo => "worth-topo",
        QueryBackedConsumerResidueOwner::ForgeQuery => "forge-query",
    }
}

pub(crate) fn public_closeout_owner_label(
    owner: PublicCloseoutConsumerResidueOwner,
) -> &'static str {
    match owner {
        PublicCloseoutConsumerResidueOwner::WorthKernel => "worth-kernel",
        PublicCloseoutConsumerResidueOwner::WorthTopo => "worth-topo",
        PublicCloseoutConsumerResidueOwner::WorthSpatial => "worth-spatial",
        PublicCloseoutConsumerResidueOwner::ForgeQuery => "forge-query",
    }
}

pub(crate) fn evidence_lookup_public_owner_label(
    owner: EvidenceLookupPublicCloseoutResidueOwner,
) -> &'static str {
    match owner {
        EvidenceLookupPublicCloseoutResidueOwner::WorthSpatial => "worth-spatial",
        EvidenceLookupPublicCloseoutResidueOwner::WorthTopo => "worth-topo",
        EvidenceLookupPublicCloseoutResidueOwner::ForgeQuery => "forge-query",
    }
}

pub(crate) fn spatial_owner_label(owner: SpatialConsumerResidueOwner) -> &'static str {
    match owner {
        SpatialConsumerResidueOwner::WorthSpatial => "worth-spatial",
    }
}
