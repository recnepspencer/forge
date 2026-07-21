use worth_query::facade::{domain, foundation};

fn main() {
    let raw = foundation::WorthQueryEntityIdentity::from_relational_record(
        foundation::RelationalBridgeRecordIdentityParts::entity(1, 1, 0),
    );
    let _forged = domain::InstalledIdentityEvolutionOutcome::singular_successor(
        raw.clone(),
        raw,
    );
}
