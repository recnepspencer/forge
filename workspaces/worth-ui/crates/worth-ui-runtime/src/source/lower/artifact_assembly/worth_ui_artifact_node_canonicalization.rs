use crate::source::WorthUiIdentitySeededArtifactInputNode;

pub(crate) fn worth_ui_canonical_node_sort_key(
    node: &WorthUiIdentitySeededArtifactInputNode,
) -> (u8, String, String) {
    match node {
        WorthUiIdentitySeededArtifactInputNode::Import(node) => (
            0,
            node.target().authored_text().to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
        WorthUiIdentitySeededArtifactInputNode::Component(node) => (
            1,
            node.component().id().as_str().to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
        WorthUiIdentitySeededArtifactInputNode::Surface(node) => (
            2,
            node.surface().id().as_str().to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
        WorthUiIdentitySeededArtifactInputNode::Binding(node) => (
            3,
            node.view_binding_reference()
                .view_binding()
                .id()
                .as_str()
                .to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
        WorthUiIdentitySeededArtifactInputNode::Token(node) => (
            4,
            node.theme_token().id().as_str().to_owned(),
            node.identity_seed().basis().to_owned(),
        ),
    }
}

pub(crate) fn worth_ui_canonical_node_key(node: &WorthUiIdentitySeededArtifactInputNode) -> String {
    match node {
        WorthUiIdentitySeededArtifactInputNode::Import(node) => {
            format!(
                "import:{}:{}",
                node.target().authored_text(),
                node.identity_seed().basis()
            )
        }
        WorthUiIdentitySeededArtifactInputNode::Component(node) => format!(
            "component:{}:{}",
            node.component().id().as_str(),
            node.identity_seed().basis()
        ),
        WorthUiIdentitySeededArtifactInputNode::Surface(node) => format!(
            "surface:{}:{}",
            node.surface().id().as_str(),
            node.identity_seed().basis()
        ),
        WorthUiIdentitySeededArtifactInputNode::Binding(node) => format!(
            "binding:{}:{}",
            node.view_binding_reference().view_binding().id().as_str(),
            node.identity_seed().basis()
        ),
        WorthUiIdentitySeededArtifactInputNode::Token(node) => format!(
            "token:{}:{}",
            node.theme_token().id().as_str(),
            node.identity_seed().basis()
        ),
    }
}

pub(crate) fn worth_ui_semantic_locus(node: &WorthUiIdentitySeededArtifactInputNode) -> String {
    match node {
        WorthUiIdentitySeededArtifactInputNode::Import(node) => {
            format!("import:{}", node.target().authored_text())
        }
        WorthUiIdentitySeededArtifactInputNode::Component(node) => {
            format!("component:{}", node.component().id().as_str())
        }
        WorthUiIdentitySeededArtifactInputNode::Surface(node) => {
            format!("surface:{}", node.surface().id().as_str())
        }
        WorthUiIdentitySeededArtifactInputNode::Binding(node) => format!(
            "binding:{}",
            node.view_binding_reference().view_binding().id().as_str()
        ),
        WorthUiIdentitySeededArtifactInputNode::Token(node) => {
            format!("token:{}", node.theme_token().id().as_str())
        }
    }
}
