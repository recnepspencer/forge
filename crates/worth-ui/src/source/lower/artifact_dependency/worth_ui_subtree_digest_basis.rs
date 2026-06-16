use crate::source::{
    WorthUiArtifactNode, WorthUiArtifactSubtreeDigest, WorthUiMosaicMountFacts,
    WorthUiMosaicRegionFacts, WorthUiMosaicStructureFacts, WorthUiRuntimeDependencyHook,
};

pub(super) fn subtree_digest(
    node: &WorthUiArtifactNode,
    hooks: &[WorthUiRuntimeDependencyHook],
) -> WorthUiArtifactSubtreeDigest {
    WorthUiArtifactSubtreeDigest::new(fold_text(&subtree_basis(node, hooks)))
}

fn subtree_basis(node: &WorthUiArtifactNode, hooks: &[WorthUiRuntimeDependencyHook]) -> String {
    let hook_basis = hooks
        .iter()
        .map(WorthUiRuntimeDependencyHook::digest_basis)
        .collect::<Vec<_>>()
        .join("|");
    format!("{}|runtime_hooks:[{hook_basis}]", node_basis(node))
}

fn node_basis(node: &WorthUiArtifactNode) -> String {
    match node {
        WorthUiArtifactNode::Import(node) => {
            format!("import:{}", node.target().authored_text())
        }
        WorthUiArtifactNode::Page(node) => format!(
            "page:{}|structure:{}",
            node.name_text(),
            structure_basis(node.structure())
        ),
        WorthUiArtifactNode::Component(node) => format!(
            "component:{}|structure:{}",
            node.component().id().as_str(),
            structure_basis(node.structure())
        ),
        WorthUiArtifactNode::Surface(node) => format!(
            "surface:{}|structure:{}",
            node.surface().id().as_str(),
            structure_basis(node.structure())
        ),
        WorthUiArtifactNode::Binding(node) => format!(
            "binding:{}|structure:{}",
            node.view_binding_reference().view_binding().id().as_str(),
            structure_basis(node.structure())
        ),
        WorthUiArtifactNode::Token(node) => format!(
            "token:{}|target:{}",
            node.theme_token().id().as_str(),
            node.semantics().resolved_target_theme_token().id().as_str()
        ),
    }
}

fn structure_basis(structure: &WorthUiMosaicStructureFacts) -> String {
    structure
        .root_regions()
        .iter()
        .map(region_basis)
        .collect::<Vec<_>>()
        .join("|")
}

fn region_basis(region: &WorthUiMosaicRegionFacts) -> String {
    let child_regions = region
        .child_regions()
        .iter()
        .map(region_basis)
        .collect::<Vec<_>>()
        .join("|");
    let mounts = region
        .mounts()
        .iter()
        .map(mount_basis)
        .collect::<Vec<_>>()
        .join("|");
    format!(
        "region:{}|children:[{}]|mounts:[{}]",
        region.region().id().as_str(),
        child_regions,
        mounts
    )
}

fn mount_basis(mount: &WorthUiMosaicMountFacts) -> String {
    format!("mount:{}", mount.surface().id().as_str())
}

fn fold_text(text: &str) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for byte in text.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x100_0000_01b3);
    }
    digest
}
