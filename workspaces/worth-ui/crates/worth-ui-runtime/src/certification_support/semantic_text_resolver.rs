//! Qualified-layout resolver owned by semantic-text certification worlds.

pub struct UiCertificationQualifiedTextResolver {
    layout: std::sync::Arc<worth_ui_text::UiQualifiedTextLayout>,
}

pub fn semantic_text_layout_resolver_for_certification() -> UiCertificationQualifiedTextResolver {
    UiCertificationQualifiedTextResolver {
        layout: crate::mounting::qualified_text_test_support::inert_qualified_layout("ONLINE"),
    }
}

impl worth_ui_host_contract::UiMountedQualifiedTextResolver
    for UiCertificationQualifiedTextResolver
{
    fn resolve(
        &self,
        identity: worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
    ) -> Option<worth_ui_host_contract::UiQualifiedTextLayoutView<'_>> {
        (self.layout.identity() == identity).then(|| self.layout.view())
    }
}
