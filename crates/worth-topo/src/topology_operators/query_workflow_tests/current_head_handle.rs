use forge_query::facade::ForgeQueryApplicationFacade;

use crate::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
    TopologyCurrentHeadConfiguredDomainHandle,
};

pub(super) fn current_head_handle() -> TopologyCurrentHeadConfiguredDomainHandle {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    topology_query_domain_entry(&query)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .expect("current-head topology context should validate")
        .admit()
        .expect("current-head topology context should admit")
}
