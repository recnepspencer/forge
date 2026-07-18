use std::collections::BTreeSet;

use crate::evidence::sha256_serialized;
use crate::selection::SelectedProofExecutionPlan;

use super::ExecutedProofRun;

impl ExecutedProofRun {
    pub fn validate_integrity(&self, plan: &SelectedProofExecutionPlan) -> Result<(), String> {
        plan.validate_integrity()?;
        let expected_schedule = super::schedule::schedule(plan)?;
        let attempted_units: BTreeSet<_> = self
            .attempts
            .iter()
            .map(|attempt| attempt.unit_index)
            .collect();
        let verdict_units: BTreeSet<_> = self
            .unit_verdicts
            .iter()
            .map(|verdict| verdict.unit_identity.as_str())
            .collect();
        let skipped_units: BTreeSet<_> = self
            .skipped_units
            .iter()
            .map(|unit| unit.unit_identity.as_str())
            .collect();
        if self.schema_version != 1
            || self.run_identity.trim().is_empty()
            || self.plan_digest != plan.plan_digest
            || self.schedule != expected_schedule
            || self.planned_units != plan.units.len()
            || self.executed_units != self.unit_verdicts.len()
            || self.passed_units + self.failed_units != self.executed_units
            || self.executed_units + self.skipped_units.len() != self.planned_units
            || attempted_units.len() != self.executed_units
            || verdict_units.len() != self.executed_units
            || skipped_units.len() != self.skipped_units.len()
            || self.structural_preflight_evidence_identity
                != plan.structural_preflight.evidence_identity
            || self.structural_preflight_bundle_path != plan.structural_preflight.bundle_path
        {
            return Err(
                "executed proof run is not a complete execution of its sealed plan".to_owned(),
            );
        }
        for attempt in &self.attempts {
            let unit = plan.units.get(attempt.unit_index).ok_or_else(|| {
                format!(
                    "proof attempt addresses missing unit {}",
                    attempt.unit_index
                )
            })?;
            let expected_identity = sha256_serialized(&(
                "worth-store-proof-run-attempt-v1",
                &plan.plan_digest,
                &self.run_identity,
                attempt.unit_index,
                attempt.ordinal,
                unit,
            ))?;
            let (program, arguments) = unit.command_line(plan.request.mode());
            let expected_command: Vec<_> = std::iter::once(program).chain(arguments).collect();
            if attempt.attempt_identity != expected_identity
                || attempt.plan_digest != plan.plan_digest
                || attempt.unit_identity != unit.identity()
                || attempt.command != expected_command
            {
                return Err(format!(
                    "proof attempt {} is not bound to its sealed execution unit",
                    attempt.attempt_identity
                ));
            }
        }
        let expected_behavior = if self.failed_units == 0 && self.skipped_units.is_empty() {
            "passed"
        } else if self
            .unit_verdicts
            .iter()
            .any(|verdict| verdict.behavioral_verdict == "flaky-indeterminate")
        {
            "indeterminate"
        } else {
            "failed"
        };
        if self.behavioral_verdict != expected_behavior
            || self.expected_identity()? != self.evidence_identity
        {
            return Err("executed proof run verdict or evidence identity was altered".to_owned());
        }
        Ok(())
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub(super) fn seal(&mut self) -> Result<(), String> {
        self.evidence_identity = self.expected_identity()?;
        Ok(())
    }

    fn expected_identity(&self) -> Result<String, String> {
        let mut basis = self.clone();
        basis.evidence_identity.clear();
        sha256_serialized(&basis)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::Path;

    use crate::selection::{
        RepositoryIdentity, StoreProofMode, StoreProofRequest, StoreProofSelection,
        StructuralPreflightReference,
    };

    use super::*;

    #[test]
    fn plan_and_run_content_changes_invalidate_their_seals() {
        let plan = plan();
        plan.validate_integrity().unwrap();
        let mut run = empty_run(&plan);
        run.seal().unwrap();
        run.validate_integrity(&plan).unwrap();

        run.behavioral_verdict = "failed".to_owned();
        assert!(run.validate_integrity(&plan).is_err());

        let mut changed_plan = plan;
        changed_plan.maximum_concurrency += 1;
        assert!(changed_plan.validate_integrity().is_err());
    }

    fn plan() -> SelectedProofExecutionPlan {
        let request = StoreProofRequest::new(
            StoreProofMode::Ci,
            None,
            Some("structural-preflight".to_owned()),
            None,
            None,
            true,
        );
        SelectedProofExecutionPlan::lower(
            Path::new("."),
            request,
            StoreProofSelection {
                included_products: Vec::new(),
                included_packages: Vec::new(),
                excluded_packages: BTreeMap::new(),
                included_targets: Vec::new(),
                excluded_targets: BTreeMap::new(),
                included_case_responsibilities: BTreeMap::new(),
                included_fixtures: Vec::new(),
                excluded_fixtures: BTreeMap::new(),
                included_suites: Vec::new(),
                excluded_suites: BTreeMap::new(),
                feature_lanes: Vec::new(),
                build_profiles: Vec::new(),
                subprocess_probes: Vec::new(),
            },
            Vec::new(),
            None,
            BTreeMap::new(),
            RepositoryIdentity {
                source_revision: "a".repeat(40),
                source_tree_digest: "b".repeat(64),
                lockfile_digest: "c".repeat(64),
                rustc_identity: "rustc test".to_owned(),
                operating_system: "test".to_owned(),
                architecture: "test".to_owned(),
            },
            StructuralPreflightReference::synthetic_for_selection(Path::new(".")),
            None,
        )
        .unwrap()
    }

    fn empty_run(plan: &SelectedProofExecutionPlan) -> ExecutedProofRun {
        ExecutedProofRun {
            schema_version: 1,
            evidence_identity: String::new(),
            plan_digest: plan.plan_digest.clone(),
            schedule: super::super::schedule::schedule(plan).unwrap(),
            run_identity: "run".to_owned(),
            run_started_unix_millis: 1,
            planned_units: 0,
            executed_units: 0,
            passed_units: 0,
            failed_units: 0,
            skipped_units: Vec::new(),
            behavioral_verdict: "passed".to_owned(),
            failed_unit: None,
            unit_verdicts: Vec::new(),
            attempts: Vec::new(),
            observed_cost: super::super::ObservedProofRunCost {
                target_roots: Vec::new(),
                cargo_processes_launched: 0,
                test_or_check_processes_requested: 0,
                declared_subprocess_evidence: 0,
                externally_observed_processes: 0,
                externally_observed_compilers: 0,
                externally_observed_linkers: 0,
                peak_observed_descendants: 0,
                observer_authorities: Vec::new(),
                cargo_compiler_artifact_messages: 0,
                freshly_compiled_cargo_artifacts: 0,
                reused_cargo_artifacts: 0,
                linked_executable_artifacts: Vec::new(),
                compiler_process_observation: String::new(),
                linker_process_observation: String::new(),
                child_process_observation: String::new(),
            },
            structural_preflight_evidence_identity: plan
                .structural_preflight
                .evidence_identity
                .clone(),
            structural_preflight_bundle_path: plan.structural_preflight.bundle_path.clone(),
        }
    }
}
