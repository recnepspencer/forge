use crate::snapshot::BridgeTruthViewSelector;
use crate::source::{
    BridgeSourceCapability, BridgeSourceCapabilitySet, SourceDeclaration, SourceDeclarationIdentity,
};

pub(in crate::facade::tests) fn registered_source(
    id: &str,
    selector: BridgeTruthViewSelector,
    capabilities: Vec<BridgeSourceCapability>,
) -> SourceDeclaration {
    SourceDeclaration::new(
        SourceDeclarationIdentity::admit_bridge_owned(id),
        selector,
        BridgeSourceCapabilitySet::new(capabilities),
    )
}
