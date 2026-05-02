use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeAuthoritativeMutationEvidenceSupport {
    carry_forward_sections: Vec<String>,
    existing_truth_binding_families: Vec<String>,
    symbolic_target_reference_families: Vec<String>,
    naming_mutation_families: Vec<String>,
    continuity_mutation_families: Vec<String>,
    aggregate_evidence_sections: Vec<String>,
    support_digest: String,
}

impl BridgeAuthoritativeMutationEvidenceSupport {
    pub fn standard() -> Self {
        let carry_forward_sections = vec![
            "declared-resolved-target-evidence".to_string(),
            "batch-session-causality-provenance".to_string(),
            "existing-truth-binding".to_string(),
            "same-batch-symbolic-target-reference".to_string(),
            "naming-mutation-evidence".to_string(),
            "continuity-mutation-evidence".to_string(),
            "replay-safe-request-receipt-digests".to_string(),
        ];
        let existing_truth_binding_families = vec![
            "direct_entity_identity".to_string(),
            "direct_relation_identity".to_string(),
        ];
        let symbolic_target_reference_families = vec!["same_batch_declared_target".to_string()];
        let naming_mutation_families = vec![
            "attach_new_target".to_string(),
            "attach_existing_target".to_string(),
            "rebind_target".to_string(),
            "remove".to_string(),
        ];
        let continuity_mutation_families = vec![
            "rebind_existing_target".to_string(),
            "split_existing_target".to_string(),
        ];
        let aggregate_evidence_sections = vec![
            "aggregate_existing_truth_binding_digest".to_string(),
            "aggregate_symbolic_target_reference_digest".to_string(),
            "aggregate_naming_mutation_digest".to_string(),
            "aggregate_continuity_mutation_digest".to_string(),
            "aggregate_causality_digest".to_string(),
            "aggregate_provenance_digest".to_string(),
        ];
        let mut parts = vec!["bridge_authoritative_mutation_evidence_support_v1".to_string()];
        parts.extend(
            carry_forward_sections
                .iter()
                .map(|item| format!("carry-forward:{item}")),
        );
        parts.extend(
            existing_truth_binding_families
                .iter()
                .map(|item| format!("existing-binding:{item}")),
        );
        parts.extend(
            symbolic_target_reference_families
                .iter()
                .map(|item| format!("symbolic-target:{item}")),
        );
        parts.extend(
            naming_mutation_families
                .iter()
                .map(|item| format!("naming:{item}")),
        );
        parts.extend(
            continuity_mutation_families
                .iter()
                .map(|item| format!("continuity:{item}")),
        );
        parts.extend(
            aggregate_evidence_sections
                .iter()
                .map(|item| format!("aggregate:{item}")),
        );
        let support_digest = hash_parts("bridge-authoritative-mutation-evidence-support", &parts);
        Self {
            carry_forward_sections,
            existing_truth_binding_families,
            symbolic_target_reference_families,
            naming_mutation_families,
            continuity_mutation_families,
            aggregate_evidence_sections,
            support_digest,
        }
    }

    pub fn carry_forward_sections(&self) -> &[String] {
        &self.carry_forward_sections
    }

    pub fn existing_truth_binding_families(&self) -> &[String] {
        &self.existing_truth_binding_families
    }

    pub fn symbolic_target_reference_families(&self) -> &[String] {
        &self.symbolic_target_reference_families
    }

    pub fn naming_mutation_families(&self) -> &[String] {
        &self.naming_mutation_families
    }

    pub fn continuity_mutation_families(&self) -> &[String] {
        &self.continuity_mutation_families
    }

    pub fn aggregate_evidence_sections(&self) -> &[String] {
        &self.aggregate_evidence_sections
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeAuthoritativeMutationEvidenceCloseout {
    support_digest: String,
    safe_to_build_now: Vec<String>,
    must_not_assume_yet: Vec<String>,
    required_verification_commands: Vec<String>,
    closeout_digest: String,
}

impl BridgeAuthoritativeMutationEvidenceCloseout {
    pub fn derive(support: &BridgeAuthoritativeMutationEvidenceSupport) -> Self {
        let safe_to_build_now = vec![
            "bridge writeback artifacts can carry target, causality, provenance, naming, and continuity evidence into one Query-facing contract".to_string(),
            "batch/session authority bundles preserve aggregate existing-binding, symbolic-target, naming, continuity, causality, and provenance digests".to_string(),
            "replay-safe request and receipt digests remain part of the carry-forward story for admitted authority sessions".to_string(),
        ];
        let must_not_assume_yet = vec![
            "bridge authority evidence closes durable restart, temporal, or async authority-mutation semantics".to_string(),
            "unsupported existing-truth binding, symbolic-target, naming, or continuity families remain fail-closed until explicitly admitted".to_string(),
            "downstream domains may reconstruct dropped causality or provenance after the bridge boundary".to_string(),
        ];
        let required_verification_commands = vec![
            "cargo fmt -p forge-runtime-bridge".to_string(),
            "cargo check -p forge-runtime-bridge --tests".to_string(),
            "cargo test -p forge-runtime-bridge".to_string(),
            "cargo test --manifest-path crates/forge-runtime-bridge/Cargo.toml --test phase_boundaries_compile_fail".to_string(),
            "git diff --check".to_string(),
        ];
        let mut parts = vec![
            "bridge_authoritative_mutation_evidence_closeout_v1".to_string(),
            format!("support:{}", support.support_digest()),
        ];
        parts.extend(safe_to_build_now.iter().map(|item| format!("safe:{item}")));
        parts.extend(
            must_not_assume_yet
                .iter()
                .map(|item| format!("deferred:{item}")),
        );
        parts.extend(
            required_verification_commands
                .iter()
                .map(|item| format!("verify:{item}")),
        );
        let closeout_digest = hash_parts("bridge-authoritative-mutation-evidence-closeout", &parts);
        Self {
            support_digest: support.support_digest().to_string(),
            safe_to_build_now,
            must_not_assume_yet,
            required_verification_commands,
            closeout_digest,
        }
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }

    pub fn safe_to_build_now(&self) -> &[String] {
        &self.safe_to_build_now
    }

    pub fn must_not_assume_yet(&self) -> &[String] {
        &self.must_not_assume_yet
    }

    pub fn required_verification_commands(&self) -> &[String] {
        &self.required_verification_commands
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

fn hash_parts(label: &str, parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(label.as_bytes());
    for part in parts {
        hasher.update(part.as_bytes());
    }
    let digest = hasher.finalize();
    format!("{label}:sha256:{digest:x}")
}
