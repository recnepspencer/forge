use std::path::Path;

use serde::Deserialize;
use worth_store::physical_runtime::PhysicalWorkMutantLocalization;

use super::{hash_file, MUTATION_EVIDENCE_REPORT_SCHEMA};
use crate::mutation_campaign::{
    catalog::ControlledMutation,
    evidence::{self, MutationObservation},
    source_inventory::{self, MutationSourceBinding},
    MutationCampaignScope,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PublishedMutationEvidenceReport {
    schema: String,
    scope: MutationCampaignScope,
    source: MutationSourceBinding,
    observations: Vec<MutationObservation>,
}

#[derive(Deserialize)]
struct PublishedMutationEvidenceReportHeader {
    schema: String,
}

pub(super) fn load_evidence(
    report: &Path,
    workspace: &Path,
    expected_scope: MutationCampaignScope,
) -> Result<Vec<PhysicalWorkMutantLocalization>, String> {
    let report = report.canonicalize().map_err(|error| {
        format!(
            "cannot locate mutation report {}: {error}",
            report.display()
        )
    })?;
    let bytes = std::fs::read(&report)
        .map_err(|error| format!("cannot read mutation report {}: {error}", report.display()))?;
    let header: PublishedMutationEvidenceReportHeader = serde_json::from_slice(&bytes)
        .map_err(|error| format!("cannot decode mutation report header: {error}"))?;
    if header.schema != MUTATION_EVIDENCE_REPORT_SCHEMA {
        return Err(format!(
            "unsupported mutation report schema `{}`",
            header.schema
        ));
    }
    let published: PublishedMutationEvidenceReport = serde_json::from_slice(&bytes)
        .map_err(|error| format!("cannot decode mutation report: {error}"))?;
    debug_assert_eq!(published.schema, MUTATION_EVIDENCE_REPORT_SCHEMA);
    if published.scope != expected_scope {
        return Err(format!(
            "mutation report scope `{}` does not satisfy required scope `{}`",
            published.scope.label(),
            expected_scope.label(),
        ));
    }
    let expected = expected_scope.mutations();
    validate_shape(&published.observations, expected_scope, expected)?;
    validate_campaign_source(&published.source, workspace)?;
    published
        .observations
        .into_iter()
        .zip(expected)
        .map(|(observation, expected)| validate_observation(observation, expected, workspace))
        .collect()
}

fn validate_campaign_source(
    expected: &MutationSourceBinding,
    workspace: &Path,
) -> Result<(), String> {
    let current = source_inventory::bind(workspace)?;
    require_campaign_source(expected, &current)
}

fn require_campaign_source(
    expected: &MutationSourceBinding,
    current: &MutationSourceBinding,
) -> Result<(), String> {
    if expected != current {
        return Err("mutation campaign source is stale".into());
    }
    Ok(())
}

fn validate_shape(
    observations: &[MutationObservation],
    scope: MutationCampaignScope,
    expected: &[ControlledMutation],
) -> Result<(), String> {
    if observations.len() != expected.len() {
        return Err(format!(
            "{} mutation report requires {} observations, found {}",
            scope.label(),
            expected.len(),
            observations.len()
        ));
    }
    for (observation, mutation) in observations.iter().zip(expected) {
        if observation.id != mutation.id {
            return Err(format!(
                "{} mutation report expected mutant {}, found {}",
                scope.label(),
                mutation.id,
                observation.id
            ));
        }
    }
    Ok(())
}

fn validate_observation(
    observation: MutationObservation,
    expected: &ControlledMutation,
    workspace: &Path,
) -> Result<PhysicalWorkMutantLocalization, String> {
    validate_declared_binding(&observation, expected)?;
    let source = workspace.join(expected.source);
    let source_text = std::fs::read_to_string(&source)
        .map_err(|error| format!("cannot read mutant {} source: {error}", expected.id))?;
    if expected.source_occurrences(&source_text) != 1 {
        return Err(format!(
            "mutant {} no longer binds exactly once in current source",
            expected.id
        ));
    }
    if hash_file(&source)? != observation.source_sha256 {
        return Err(format!("mutant {} source is stale", expected.id));
    }
    if observation.source_sha256 == observation.mutant_sha256 {
        return Err(format!("mutant {} made no source change", expected.id));
    }
    let encoded = evidence::encode(&observation)?;
    evidence::decode_physical_work_localization(&encoded)
}

fn validate_declared_binding(
    observation: &MutationObservation,
    expected: &ControlledMutation,
) -> Result<(), String> {
    if observation.source_binding != expected.source {
        return Err(format!("mutant {} source binding changed", expected.id));
    }
    if observation.expected_failing_predicate != expected.predicate
        || observation.actual_failing_predicate != expected.predicate
    {
        return Err(format!("mutant {} predicate binding changed", expected.id));
    }
    if observation.profile_binding != "test" || observation.scenario_binding != expected.selector {
        return Err(format!("mutant {} execution binding changed", expected.id));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        load_evidence, require_campaign_source, MutationCampaignScope, MutationSourceBinding,
        MUTATION_EVIDENCE_REPORT_SCHEMA,
    };

    #[test]
    fn report_requires_the_complete_physical_work_campaign() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("mutants.json");
        std::fs::write(
            &report,
            format!(
                r#"{{"schema":"{MUTATION_EVIDENCE_REPORT_SCHEMA}","scope":"physical-work","source":{{"binding":"worth.store.controlled-mutation-source-closure.v3","sha256":"{}"}},"observations":[]}}"#,
                "11".repeat(32)
            ),
        )
        .unwrap();
        let error = load_evidence(
            &report,
            temporary.path(),
            MutationCampaignScope::PhysicalWork,
        )
        .unwrap_err();
        let expected = crate::mutation_campaign::catalog::physical_work_mutations().len();
        assert!(
            error.contains(&format!("requires {expected} observations")),
            "{error}"
        );
    }

    #[test]
    fn unsupported_schema_is_classified_before_body_decoding() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("mutants.json");
        std::fs::write(
            &report,
            br#"{"schema":"worth.store.c5_1.mutation-evidence.v2","observations":[]}"#,
        )
        .unwrap();

        let error = load_evidence(
            &report,
            temporary.path(),
            MutationCampaignScope::PhysicalWork,
        )
        .unwrap_err();

        assert!(
            error.contains("unsupported mutation report schema"),
            "{error}"
        );
        assert!(!error.contains("missing field"), "{error}");
    }

    #[test]
    fn report_scope_must_match_the_consuming_courtroom() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("mutants.json");
        std::fs::write(
            &report,
            format!(
                r#"{{"schema":"{MUTATION_EVIDENCE_REPORT_SCHEMA}","scope":"physical-work","source":{{"binding":"worth.store.controlled-mutation-source-closure.v3","sha256":"{}"}},"observations":[]}}"#,
                "11".repeat(32)
            ),
        )
        .unwrap();

        let error = load_evidence(
            &report,
            temporary.path(),
            MutationCampaignScope::BoundedResidency,
        )
        .unwrap_err();

        assert!(error.contains("does not satisfy required scope"), "{error}");
        assert!(error.contains("bounded-residency"), "{error}");
    }

    #[test]
    fn report_reader_rejects_same_length_source_closure_drift() {
        let expected = MutationSourceBinding {
            binding: "worth.store.controlled-mutation-source-closure.v3".into(),
            sha256: "11".repeat(32),
        };
        let current = MutationSourceBinding {
            binding: expected.binding.clone(),
            sha256: "22".repeat(32),
        };

        assert_eq!(
            require_campaign_source(&expected, &current).unwrap_err(),
            "mutation campaign source is stale"
        );
    }
}
