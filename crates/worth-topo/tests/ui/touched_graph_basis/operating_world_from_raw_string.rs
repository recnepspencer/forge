use forge_query::facade::ForgeQuerySessionLabel;
use topology::facade::{
    TopologyTouchedOperatingWorld, TopologyTouchedOperatingWorldIdentityDigest,
};

fn main() {
    let label =
        ForgeQuerySessionLabel::scoped_strs("copied-query-branch", ["raw-string"]).unwrap();
    let digest = TopologyTouchedOperatingWorldIdentityDigest::from_query_evidence_identity(
        label.identity_digest(),
    );
    let _world = TopologyTouchedOperatingWorld::branch(digest);
}
