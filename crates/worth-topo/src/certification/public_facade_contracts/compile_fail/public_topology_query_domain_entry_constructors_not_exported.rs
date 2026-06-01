use topology::query_domain::{
    TopologyCurrentHeadAuthoritativeContext, TopologyQueryDomain,
    TopologySnapshotReadOnlyContext,
};

fn main() {
    let _ = TopologyQueryDomain { _sealed: () };
    let _ = TopologyCurrentHeadAuthoritativeContext { _sealed: () };
    let _ = TopologySnapshotReadOnlyContext { _sealed: () };
}
