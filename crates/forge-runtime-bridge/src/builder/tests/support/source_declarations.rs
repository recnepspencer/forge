use crate::snapshot::BridgeTruthViewSelector;
use crate::source::{
    BridgeSourceCapability, BridgeSourceCapabilitySet, SourceDeclaration, SourceDeclarationIdentity,
};
use crate::truth_identity_fixtures::{truth_branch, truth_snapshot};

pub(in crate::builder::tests) fn source_declaration(
    declaration_id: &str,
    snapshot_id: &str,
    capabilities: Vec<BridgeSourceCapability>,
) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::new(declaration_id),
        BridgeTruthViewSelector::committed_snapshot(
            truth_branch("main"),
            truth_snapshot(snapshot_fixture_id(snapshot_id), 1),
        ),
        BridgeSourceCapabilitySet::new(capabilities),
    )
}

fn snapshot_fixture_id(snapshot_id: &str) -> u64 {
    snapshot_id
        .bytes()
        .fold(1_u64, |acc, byte| acc.wrapping_mul(31) + u64::from(byte))
}
