use crate::snapshot::{BridgeTruthViewSelector, TruthSnapshotIdentity};
use crate::source::{
    BridgeSourceCapability, BridgeSourceCapabilitySet, SourceDeclaration, SourceDeclarationIdentity,
};

pub(in crate::builder::tests) fn source_declaration(
    declaration_id: &str,
    snapshot_id: &str,
    capabilities: Vec<BridgeSourceCapability>,
) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::new(declaration_id),
        BridgeTruthViewSelector::committed_snapshot(
            crate::input::envelope::TruthBranchIdentity::new("main"),
            TruthSnapshotIdentity::new(snapshot_id),
        ),
        BridgeSourceCapabilitySet::new(capabilities),
    )
}
