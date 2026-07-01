use crate::{
    UiDslAspectName, UiDslLoweringReceipt, UiDslPostureToken, UiDslSemanticArtifact,
    UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
    UiDslStructuralToken, UiDslSupportToken,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDslPackage {
    package_name: String,
    admitted_declarations: Vec<UiDslLoweringReceipt>,
}

impl WorthUiDslPackage {
    pub fn empty() -> Self {
        Self::named("worth-ui.dsl.empty")
    }

    pub fn named(package_name: impl Into<String>) -> Self {
        Self {
            package_name: package_name.into(),
            admitted_declarations: Vec::new(),
        }
    }

    pub fn package_name(&self) -> &str {
        &self.package_name
    }

    pub fn admitted_declarations(&self) -> &[UiDslLoweringReceipt] {
        &self.admitted_declarations
    }

    pub fn runtime_lowering_receipts(&self) -> Vec<UiDslLoweringReceipt> {
        let mut receipts = Vec::with_capacity(self.admitted_declarations.len() + 1);
        receipts.push(runtime_bootstrap_receipt());
        receipts.extend(self.admitted_declarations.iter().cloned());
        receipts
    }

    pub fn with_semantic_artifact_spec(
        mut self,
        semantic_spec: UiDslSemanticArtifactSpec,
    ) -> Self {
        let receipt = self.admit_semantic_artifact(semantic_spec);
        self.admitted_declarations.push(receipt);
        self
    }

    pub fn admit_semantic_artifact(
        &self,
        semantic_spec: UiDslSemanticArtifactSpec,
    ) -> UiDslLoweringReceipt {
        let semantic_artifact = semantic_spec.into_artifact();
        let semantic_input_digest = semantic_input_digest(&semantic_artifact);

        UiDslLoweringReceipt::new(
            semantic_artifact.clone(),
            semantic_input_digest,
            semantic_artifact.provenance().clone(),
        )
    }
}

fn runtime_bootstrap_receipt() -> UiDslLoweringReceipt {
    let semantic_artifact = UiDslSemanticArtifactSpec::new(
        UiDslSemanticKey::new("worth_ui.runtime.bootstrap.product_root"),
        UiDslSemanticFamily::Page,
        UiDslSourceProvenance::rust_authored("worth-ui.runtime.bootstrap", 0),
    )
    .with_published_aspect(UiDslAspectName::new("structure.product-root"))
    .with_structural_token(UiDslStructuralToken::new("page:product-root"))
    .with_posture_token(UiDslPostureToken::new("world:authoritative"))
    .with_support_token(UiDslSupportToken::new("support:runtime-bootstrap"))
    .into_artifact();
    let semantic_input_digest = semantic_input_digest(&semantic_artifact);

    UiDslLoweringReceipt::new(
        semantic_artifact.clone(),
        semantic_input_digest,
        semantic_artifact.provenance().clone(),
    )
}

fn semantic_input_digest(semantic_artifact: &UiDslSemanticArtifact) -> u64 {
    stable_text_digest(semantic_artifact.key().as_str())
        ^ stable_text_digest(semantic_artifact.family().as_str()).rotate_left(7)
        ^ digest_string_slice(semantic_artifact.published_aspects()).rotate_left(17)
        ^ digest_string_slice(semantic_artifact.consumed_aspects()).rotate_left(23)
        ^ digest_string_slice(semantic_artifact.structural_tokens()).rotate_left(31)
        ^ digest_string_slice(semantic_artifact.posture_tokens()).rotate_left(41)
        ^ digest_string_slice(semantic_artifact.support_tokens()).rotate_left(53)
}

fn digest_string_slice<T>(values: &[T]) -> u64
where
    T: DslDigestText,
{
    let mut canonical = values.iter().map(DslDigestText::digest_text).collect::<Vec<_>>();
    canonical.sort();
    canonical.dedup();

    canonical
        .iter()
        .fold(0x9E37_79B9_7F4A_7C15, |digest, value| {
            digest.rotate_left(5) ^ stable_text_digest(value)
        })
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}

trait DslDigestText {
    fn digest_text(&self) -> &str;
}

impl DslDigestText for UiDslAspectName {
    fn digest_text(&self) -> &str {
        self.as_str()
    }
}

impl DslDigestText for UiDslStructuralToken {
    fn digest_text(&self) -> &str {
        self.as_str()
    }
}

impl DslDigestText for UiDslPostureToken {
    fn digest_text(&self) -> &str {
        self.as_str()
    }
}

impl DslDigestText for UiDslSupportToken {
    fn digest_text(&self) -> &str {
        self.as_str()
    }
}
