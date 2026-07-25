use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct MutationObservation {
    pub(crate) id: u8,
    pub(crate) source_binding: String,
    pub(crate) source_sha256: String,
    pub(crate) mutant_sha256: String,
    pub(crate) binary_binding: String,
    pub(crate) binary_sha256: String,
    pub(crate) profile_binding: String,
    pub(crate) scenario_binding: String,
    pub(crate) expected_failing_predicate: String,
    pub(crate) actual_failing_predicate: String,
    pub(crate) localization: String,
}

pub(super) fn encode(observation: &MutationObservation) -> Result<String, String> {
    serde_json::to_string(observation)
        .map_err(|error| format!("cannot encode mutation evidence: {error}"))
}

#[cfg(feature = "physical-work-evidence")]
pub(crate) fn decode_physical_work_localization(
    encoded: &str,
) -> Result<worth_store::physical_runtime::PhysicalWorkMutantLocalization, String> {
    use worth_store::physical_runtime::{
        PhysicalWorkMutantBinding, PhysicalWorkMutantExecutionContext,
        PhysicalWorkMutantLocalization, PhysicalWorkMutantOutcome, PhysicalWorkMutantSubject,
        PhysicalWorkSourceBinding,
    };

    let observation: MutationObservation = serde_json::from_str(encoded)
        .map_err(|error| format!("cannot decode mutation evidence: {error}"))?;
    if observation.expected_failing_predicate != observation.actual_failing_predicate {
        return Err("mutation evidence predicate binding is inconsistent".into());
    }
    let subject = PhysicalWorkMutantSubject::new(
        u16::from(observation.id),
        observation.actual_failing_predicate,
        observation.source_binding,
    )
    .map_err(binding_denial)?;
    let execution = PhysicalWorkMutantExecutionContext::new(
        observation.profile_binding,
        observation.scenario_binding,
    )
    .map_err(binding_denial)?;
    let source_digest = digest(&observation.source_sha256)?;
    let mutant_digest = digest(&observation.mutant_sha256)?;
    let binary_digest = digest(&observation.binary_sha256)?;
    let binary = PhysicalWorkSourceBinding::new(observation.binary_binding, binary_digest)
        .map_err(binding_denial)?;
    let binding =
        PhysicalWorkMutantBinding::new(subject, source_digest, mutant_digest, binary, execution);
    PhysicalWorkMutantLocalization::new(
        binding,
        PhysicalWorkMutantOutcome::new(true, observation.localization),
    )
    .map_err(binding_denial)
}

#[cfg(feature = "physical-work-evidence")]
fn digest(
    encoded: &str,
) -> Result<worth_store::physical_runtime::PhysicalWorkEvidenceDigest, String> {
    if encoded.len() != 64 || !encoded.is_ascii() {
        return Err("mutation digest must be 64 hexadecimal characters".into());
    }
    let mut bytes = [0_u8; 32];
    for (index, slot) in bytes.iter_mut().enumerate() {
        let offset = index * 2;
        *slot = u8::from_str_radix(&encoded[offset..offset + 2], 16)
            .map_err(|_| "mutation digest contains non-hexadecimal data".to_owned())?;
    }
    worth_store::physical_runtime::PhysicalWorkEvidenceDigest::new(bytes)
        .ok_or_else(|| "mutation digest cannot be all zero".to_owned())
}

#[cfg(feature = "physical-work-evidence")]
fn binding_denial(
    denial: worth_store::physical_runtime::PhysicalWorkEvidenceBindingDenial,
) -> String {
    format!("mutation evidence binding denied: {denial:?}")
}

#[cfg(all(test, feature = "physical-work-evidence"))]
mod tests {
    use super::decode_physical_work_localization;

    #[test]
    fn emitted_mutation_schema_retains_complete_physical_work_provenance() {
        let encoded = format!(
            r#"{{"id":15,"source_binding":"causal.rs","source_sha256":"{source}","mutant_sha256":"{mutant}","binary_binding":"proof.exe","binary_sha256":"{binary}","profile_binding":"test","scenario_binding":"trace","expected_failing_predicate":"settlement","actual_failing_predicate":"settlement","localization":"trace.rs:68"}}"#,
            source = "11".repeat(32),
            mutant = "22".repeat(32),
            binary = "33".repeat(32),
        );
        let localization = decode_physical_work_localization(&encoded).unwrap();
        assert_eq!(localization.identity(), 15);
        assert_eq!(localization.predicate(), "settlement");
        assert_eq!(localization.binding().execution().scenario(), "trace");
        assert_eq!(localization.binding().binary().path(), "proof.exe");
        assert!(localization.killed());
    }
}
