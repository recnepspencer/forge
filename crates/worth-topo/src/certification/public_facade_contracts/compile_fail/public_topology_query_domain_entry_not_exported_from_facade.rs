use topology::facade::{
    topology_current_head_authoritative_context, topology_query_domain,
    topology_query_domain_entry, topology_query_domain_entry_checked,
    topology_query_domain_proof_root, topology_snapshot_read_only_context,
    TopologyCurrentHeadAuthoritativeContext, TopologyCurrentHeadConfiguredDomainHandle,
    TopologyCurrentHeadConfiguredDomainHandleChecked, TopologyQueryDomain,
    TopologySnapshotReadOnlyConfiguredDomainHandle,
    TopologySnapshotReadOnlyConfiguredDomainHandleChecked, TopologySnapshotReadOnlyContext,
};

fn main() {
    let _ = topology_query_domain;
    let _ = topology_current_head_authoritative_context;
    let _ = topology_snapshot_read_only_context;
    let _ = topology_query_domain_entry;
    let _ = topology_query_domain_entry_checked;
    let _ = topology_query_domain_proof_root;
    let _: Option<TopologyQueryDomain> = None;
    let _: Option<TopologyCurrentHeadAuthoritativeContext> = None;
    let _: Option<TopologySnapshotReadOnlyContext> = None;
    let _: Option<TopologyCurrentHeadConfiguredDomainHandle> = None;
    let _: Option<TopologyCurrentHeadConfiguredDomainHandleChecked> = None;
    let _: Option<TopologySnapshotReadOnlyConfiguredDomainHandle> = None;
    let _: Option<TopologySnapshotReadOnlyConfiguredDomainHandleChecked> = None;
}
