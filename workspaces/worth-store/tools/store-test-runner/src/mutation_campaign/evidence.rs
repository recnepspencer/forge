use serde::{Deserialize, Serialize};

const ORDINARY_EXECUTION_LIMIT_MS: u64 = 180_000;
const NESTED_EXECUTABLE_COLD_LIMIT_MS: u64 = 300_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum MutationExecutionClass {
    Ordinary,
    NestedExecutableCold,
}

impl MutationExecutionClass {
    pub(super) const fn limit_ms(self) -> u64 {
        match self {
            Self::Ordinary => ORDINARY_EXECUTION_LIMIT_MS,
            Self::NestedExecutableCold => NESTED_EXECUTABLE_COLD_LIMIT_MS,
        }
    }

    pub(super) const fn limit(self) -> std::time::Duration {
        std::time::Duration::from_millis(self.limit_ms())
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct MutationExecutionEvidence {
    class: MutationExecutionClass,
    elapsed_ms: u64,
    budget_ms: u64,
}

impl MutationExecutionEvidence {
    pub(super) fn bind(
        class: MutationExecutionClass,
        elapsed: std::time::Duration,
    ) -> Result<Self, String> {
        let elapsed_ms = elapsed.as_millis().try_into().unwrap_or(u64::MAX);
        let evidence = Self {
            class,
            elapsed_ms,
            budget_ms: class.limit_ms(),
        };
        evidence.validate()?;
        Ok(evidence)
    }

    fn validate(&self) -> Result<(), String> {
        if self.budget_ms != self.class.limit_ms() {
            return Err("mutation execution budget does not match its cost class".into());
        }
        if self.elapsed_ms > self.budget_ms {
            return Err("mutation execution exceeded its declared budget".into());
        }
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
    pub(crate) execution: MutationExecutionEvidence,
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
    observation.execution.validate()?;
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
    use super::{
        decode_physical_work_localization, MutationExecutionClass, MutationExecutionEvidence,
    };

    #[test]
    fn emitted_mutation_schema_retains_complete_physical_work_provenance() {
        let encoded = format!(
            r#"{{"id":15,"source_binding":"causal.rs","source_sha256":"{source}","mutant_sha256":"{mutant}","binary_binding":"proof.exe","binary_sha256":"{binary}","profile_binding":"test","scenario_binding":"trace","expected_failing_predicate":"settlement","actual_failing_predicate":"settlement","localization":"trace.rs:68","execution":{{"class":"ordinary","elapsed_ms":12,"budget_ms":180000}}}}"#,
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

    #[test]
    fn nested_executable_cases_retain_distinct_cold_build_headroom() {
        if MutationExecutionClass::NestedExecutableCold.limit_ms() != 300_000 {
            panic!("MUTANT_PREDICATE:c7-nested-case-cost-class-collapsed");
        }
        assert_eq!(MutationExecutionClass::Ordinary.limit_ms(), 180_000);
        assert!(MutationExecutionEvidence::bind(
            MutationExecutionClass::NestedExecutableCold,
            std::time::Duration::from_millis(240_000),
        )
        .is_ok());
    }

    #[test]
    fn execution_evidence_rejects_substituted_and_exceeded_budgets() {
        let mut substituted = MutationExecutionEvidence::bind(
            MutationExecutionClass::Ordinary,
            std::time::Duration::from_millis(10),
        )
        .unwrap();
        substituted.budget_ms += 1;
        assert!(substituted.validate().is_err());

        let exceeded = MutationExecutionEvidence {
            class: MutationExecutionClass::Ordinary,
            elapsed_ms: 180_001,
            budget_ms: 180_000,
        };
        assert!(exceeded.validate().is_err());
    }

    #[test]
    fn execution_evidence_requires_its_exact_closed_schema() {
        let missing_budget = r#"{"class":"ordinary","elapsed_ms":12}"#;
        let unknown_field =
            r#"{"class":"ordinary","elapsed_ms":12,"budget_ms":180000,"surplus_ms":1}"#;

        assert!(serde_json::from_str::<MutationExecutionEvidence>(missing_budget).is_err());
        assert!(serde_json::from_str::<MutationExecutionEvidence>(unknown_field).is_err());
    }
}
