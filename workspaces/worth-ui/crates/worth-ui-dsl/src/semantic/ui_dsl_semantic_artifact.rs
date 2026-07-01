use crate::semantic::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslSourceProvenance, UiDslStructuralToken, UiDslSupportToken,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDslSemanticArtifact {
    key: UiDslSemanticKey,
    family: UiDslSemanticFamily,
    provenance: UiDslSourceProvenance,
    published_aspects: Vec<UiDslAspectName>,
    consumed_aspects: Vec<UiDslAspectName>,
    structural_tokens: Vec<UiDslStructuralToken>,
    posture_tokens: Vec<UiDslPostureToken>,
    support_tokens: Vec<UiDslSupportToken>,
    authored_comments: Vec<String>,
    formatting_profile: Option<String>,
    parser_local_id: Option<String>,
    diagnostic_label: Option<String>,
    renderer_label: Option<String>,
}

impl UiDslSemanticArtifact {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        key: UiDslSemanticKey,
        family: UiDslSemanticFamily,
        provenance: UiDslSourceProvenance,
        published_aspects: Vec<UiDslAspectName>,
        consumed_aspects: Vec<UiDslAspectName>,
        structural_tokens: Vec<UiDslStructuralToken>,
        posture_tokens: Vec<UiDslPostureToken>,
        support_tokens: Vec<UiDslSupportToken>,
        authored_comments: Vec<String>,
        formatting_profile: Option<String>,
        parser_local_id: Option<String>,
        diagnostic_label: Option<String>,
        renderer_label: Option<String>,
    ) -> Self {
        Self {
            key,
            family,
            provenance,
            published_aspects,
            consumed_aspects,
            structural_tokens,
            posture_tokens,
            support_tokens,
            authored_comments,
            formatting_profile,
            parser_local_id,
            diagnostic_label,
            renderer_label,
        }
    }

    pub fn key(&self) -> &UiDslSemanticKey {
        &self.key
    }

    pub fn family(&self) -> UiDslSemanticFamily {
        self.family
    }

    pub fn provenance(&self) -> &UiDslSourceProvenance {
        &self.provenance
    }

    pub fn published_aspects(&self) -> &[UiDslAspectName] {
        &self.published_aspects
    }

    pub fn consumed_aspects(&self) -> &[UiDslAspectName] {
        &self.consumed_aspects
    }

    pub fn structural_tokens(&self) -> &[UiDslStructuralToken] {
        &self.structural_tokens
    }

    pub fn posture_tokens(&self) -> &[UiDslPostureToken] {
        &self.posture_tokens
    }

    pub fn support_tokens(&self) -> &[UiDslSupportToken] {
        &self.support_tokens
    }
}
