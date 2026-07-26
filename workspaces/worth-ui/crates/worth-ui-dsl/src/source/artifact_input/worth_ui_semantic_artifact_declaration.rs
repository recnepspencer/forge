use crate::{
    UiDslAspectName, UiDslPostureToken, UiDslSemanticFamily, UiDslSemanticKey,
    UiDslStructuralToken, UiDslSupportToken,
};

/// Authored semantic meaning that must pass through the DSL compiler before it
/// can participate in a sealed package.
///
/// This value carries no provenance, seal, protocol, or runtime authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticArtifactDeclaration {
    key: UiDslSemanticKey,
    family: UiDslSemanticFamily,
    published_aspects: Vec<UiDslAspectName>,
    consumed_aspects: Vec<UiDslAspectName>,
    structural_tokens: Vec<UiDslStructuralToken>,
    posture_tokens: Vec<UiDslPostureToken>,
    support_tokens: Vec<UiDslSupportToken>,
}

impl WorthUiSemanticArtifactDeclaration {
    pub fn new(key: UiDslSemanticKey, family: UiDslSemanticFamily) -> Self {
        Self {
            key,
            family,
            published_aspects: Vec::new(),
            consumed_aspects: Vec::new(),
            structural_tokens: Vec::new(),
            posture_tokens: Vec::new(),
            support_tokens: Vec::new(),
        }
    }

    pub fn with_published_aspect(mut self, aspect: UiDslAspectName) -> Self {
        self.published_aspects.push(aspect);
        self
    }

    pub fn with_consumed_aspect(mut self, aspect: UiDslAspectName) -> Self {
        self.consumed_aspects.push(aspect);
        self
    }

    pub fn with_structural_token(mut self, token: UiDslStructuralToken) -> Self {
        self.structural_tokens.push(token);
        self
    }

    pub fn with_posture_token(mut self, token: UiDslPostureToken) -> Self {
        self.posture_tokens.push(token);
        self
    }

    pub fn with_support_token(mut self, token: UiDslSupportToken) -> Self {
        self.support_tokens.push(token);
        self
    }

    pub fn key(&self) -> &UiDslSemanticKey {
        &self.key
    }

    pub fn family(&self) -> UiDslSemanticFamily {
        self.family
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

    pub(crate) fn canonicalize(mut self) -> Self {
        canonicalize_set(&mut self.published_aspects);
        canonicalize_set(&mut self.consumed_aspects);
        canonicalize_set(&mut self.structural_tokens);
        canonicalize_set(&mut self.posture_tokens);
        canonicalize_set(&mut self.support_tokens);
        self
    }

    pub(crate) fn fold_source_revision(&self, digest: &mut u64) {
        fold_text(digest, self.key.as_str());
        fold_text(digest, self.family.as_str());
        fold_values(digest, &self.published_aspects);
        fold_values(digest, &self.consumed_aspects);
        fold_values(digest, &self.structural_tokens);
        fold_values(digest, &self.posture_tokens);
        fold_values(digest, &self.support_tokens);
    }
}

fn canonicalize_set<T: Ord>(values: &mut Vec<T>) {
    values.sort();
    values.dedup();
}

fn fold_values<T: SemanticText>(digest: &mut u64, values: &[T]) {
    fold_u64(digest, values.len() as u64);
    for value in values {
        fold_text(digest, value.semantic_text());
    }
}

fn fold_text(digest: &mut u64, text: &str) {
    fold_u64(digest, text.len() as u64);
    for byte in text.as_bytes() {
        *digest ^= u64::from(*byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

fn fold_u64(digest: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *digest ^= u64::from(byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

trait SemanticText {
    fn semantic_text(&self) -> &str;
}

macro_rules! semantic_text {
    ($type:ty) => {
        impl SemanticText for $type {
            fn semantic_text(&self) -> &str {
                self.as_str()
            }
        }
    };
}

semantic_text!(UiDslAspectName);
semantic_text!(UiDslStructuralToken);
semantic_text!(UiDslPostureToken);
semantic_text!(UiDslSupportToken);
