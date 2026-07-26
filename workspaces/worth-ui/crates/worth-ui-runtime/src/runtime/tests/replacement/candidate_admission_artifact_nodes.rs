use super::*;

pub(super) fn import_node(
    module_id: &WorthUiSourceModuleId,
    node_index: usize,
    target: &str,
) -> WorthUiArtifactNode {
    WorthUiArtifactNode::Import(WorthUiArtifactImportNode::new(
        WorthUiArtifactHandle::Import(WorthUiArtifactImportHandle::new(
            module_id.clone(),
            node_index,
        )),
        crate::source::test_compilation::semantic_import(target)
            .target()
            .clone(),
        0,
        WorthUiArtifactIdentitySeed::structural_fallback(format!(
            "module:{}|import:{}",
            module_id.as_str(),
            target
        )),
        WorthUiDurableStateEligibility::Ineligible {
            reason: WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
        },
    ))
}

pub(super) fn module_id(path: &str) -> WorthUiSourceModuleId {
    WorthUiSourceModuleId::from_relative_path(Path::new(path)).expect("valid module id")
}
