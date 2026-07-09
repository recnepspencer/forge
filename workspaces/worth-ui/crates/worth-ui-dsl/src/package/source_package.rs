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

    pub fn with_semantic_artifact_spec(mut self, semantic_spec: UiDslSemanticArtifactSpec) -> Self {
        let receipt = self.admit_semantic_artifact(semantic_spec);
        self.admitted_declarations.push(receipt);
        rebuild_source_artifact_generations(&mut self.admitted_declarations);
        self
    }

    pub fn admit_semantic_artifact(
        &self,
        semantic_spec: UiDslSemanticArtifactSpec,
    ) -> UiDslLoweringReceipt {
        let semantic_artifact = semantic_spec.into_artifact();
        let semantic_input_digest = semantic_input_digest(&semantic_artifact);
        let source_artifact_generation = package_source_artifact_generation(
            &self.admitted_declarations,
            semantic_artifact.provenance(),
            semantic_input_digest,
        );

        UiDslLoweringReceipt::new(
            semantic_artifact.clone(),
            semantic_input_digest,
            source_artifact_generation,
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
    let source_artifact_generation =
        source_artifact_generation(semantic_artifact.provenance(), [semantic_input_digest]);

    UiDslLoweringReceipt::new(
        semantic_artifact.clone(),
        semantic_input_digest,
        source_artifact_generation,
        semantic_artifact.provenance().clone(),
    )
}

fn rebuild_source_artifact_generations(receipts: &mut [UiDslLoweringReceipt]) {
    let generations = receipts
        .iter()
        .map(|receipt| {
            (
                source_artifact_key(receipt.source_provenance()),
                receipt.semantic_input_digest(),
            )
        })
        .fold(
            std::collections::BTreeMap::new(),
            |mut groups, (key, digest)| {
                groups.entry(key).or_insert_with(Vec::new).push(digest);
                groups
            },
        );

    for receipt in receipts {
        let key = source_artifact_key(receipt.source_provenance());
        let digests = generations
            .get(&key)
            .expect("grouped source generation should exist for every lowering receipt");
        *receipt = UiDslLoweringReceipt::new(
            receipt.semantic_artifact().clone(),
            receipt.semantic_input_digest(),
            source_artifact_generation(receipt.source_provenance(), digests.iter().copied()),
            receipt.source_provenance().clone(),
        );
    }
}

fn package_source_artifact_generation(
    admitted_declarations: &[UiDslLoweringReceipt],
    provenance: &UiDslSourceProvenance,
    incoming_semantic_input_digest: u64,
) -> u64 {
    let artifact_key = source_artifact_key(provenance);
    let semantic_input_digests = admitted_declarations
        .iter()
        .filter(|receipt| source_artifact_key(receipt.source_provenance()) == artifact_key)
        .map(UiDslLoweringReceipt::semantic_input_digest)
        .chain(std::iter::once(incoming_semantic_input_digest));

    source_artifact_generation(provenance, semantic_input_digests)
}

fn source_artifact_generation(
    provenance: &UiDslSourceProvenance,
    semantic_input_digests: impl IntoIterator<Item = u64>,
) -> u64 {
    semantic_input_digests.into_iter().fold(
        stable_text_digest(source_artifact_key(provenance).as_str()),
        |digest, semantic_input_digest| digest.rotate_left(9) ^ semantic_input_digest,
    )
}

fn source_artifact_key(provenance: &UiDslSourceProvenance) -> String {
    match provenance {
        UiDslSourceProvenance::FileAuthored { module_path, .. } => {
            format!("file:{module_path}")
        }
        UiDslSourceProvenance::RustAuthored { module_path, .. } => {
            format!("rust:{module_path}")
        }
    }
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
    let mut canonical = values
        .iter()
        .map(DslDigestText::digest_text)
        .collect::<Vec<_>>();
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

#[cfg(test)]
mod tests {
    use super::WorthUiDslPackage;
    use crate::{
        UiDslSemanticArtifactSpec, UiDslSemanticFamily, UiDslSemanticKey, UiDslSourceProvenance,
        UiDslStructuralToken,
    };

    #[test]
    fn admitted_semantic_artifact_uses_package_authoritative_source_generation() {
        let package = WorthUiDslPackage::named("worth-ui.dsl.package.authoritative-generation")
            .with_semantic_artifact_spec(spec("ui.workflow.editor", "control:workflow", 0));
        let admitted =
            package.admit_semantic_artifact(spec("ui.workflow.sidebar", "control:sidebar", 1));
        let authoritative = package
            .clone()
            .with_semantic_artifact_spec(spec("ui.workflow.sidebar", "control:sidebar", 1))
            .admitted_declarations()[1]
            .clone();

        assert_eq!(
            admitted.source_artifact_generation(),
            authoritative.source_artifact_generation()
        );
    }

    fn spec(
        semantic_key: &str,
        structural_token: &str,
        declaration_index: usize,
    ) -> UiDslSemanticArtifactSpec {
        UiDslSemanticArtifactSpec::new(
            UiDslSemanticKey::new(semantic_key),
            UiDslSemanticFamily::Control,
            UiDslSourceProvenance::file_authored("app/source_package.wui", declaration_index),
        )
        .with_structural_token(UiDslStructuralToken::new(structural_token))
    }
}
