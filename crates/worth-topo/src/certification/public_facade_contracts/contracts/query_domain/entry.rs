fn _topology_query_domain_entry_contracts() {
    let _: fn() -> TopologyQueryDomain = topology_query_domain;
    let _: fn() -> TopologyCurrentHeadAuthoritativeContext =
        topology_current_head_authoritative_context;
    let _: fn() -> TopologySnapshotReadOnlyContext = topology_snapshot_read_only_context;
    let _: fn(
        &ForgeQueryApplicationFacade,
    ) -> forge_query::facade::ForgeQueryDomainEntryRoot<TopologyQueryDomain> =
        topology_query_domain_entry;
    let _: fn(
        &ForgeQueryApplicationFacade,
    ) -> forge_query::facade::ForgeQueryDomainEntryChecked<TopologyQueryDomain> =
        topology_query_domain_entry_checked;
    let _: fn(
        &ForgeQueryApplicationFacade,
    ) -> forge_query::facade::ForgeQueryDomainEntryProofRoot<TopologyQueryDomain> =
        topology_query_domain_proof_root;
    let _: Option<TopologyCurrentHeadConfiguredDomainHandle> = None;
    let _: Option<TopologyCurrentHeadConfiguredDomainHandleChecked> = None;
    let _: Option<TopologySnapshotReadOnlyConfiguredDomainHandle> = None;
    let _: Option<TopologySnapshotReadOnlyConfiguredDomainHandleChecked> = None;
}
