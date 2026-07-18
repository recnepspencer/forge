use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::evidence::sha256_serialized;
use crate::selection::{ProofExecutionUnit, SelectedProofExecutionPlan};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofExecutionSchedule {
    pub schedule_digest: String,
    pub plan_digest: String,
    pub maximum_concurrency: usize,
    pub waves: Vec<ProofExecutionWave>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofExecutionWave {
    pub ordinal: usize,
    pub unit_indices: Vec<usize>,
}

#[derive(Serialize)]
struct ScheduleDigestBasis<'a> {
    plan_digest: &'a str,
    maximum_concurrency: usize,
    waves: &'a [ProofExecutionWave],
}

pub fn schedule(plan: &SelectedProofExecutionPlan) -> Result<ProofExecutionSchedule, String> {
    if plan.maximum_concurrency == 0 {
        return Err("execution plan declares zero maximum concurrency".to_owned());
    }
    let identities = indexed_identities(&plan.units)?;
    validate_dependencies(&plan.units, &identities)?;
    let mut remaining: BTreeSet<_> = (0..plan.units.len()).collect();
    let mut scheduled = BTreeSet::new();
    let mut waves = Vec::new();
    while !remaining.is_empty() {
        let ready: Vec<_> = remaining
            .iter()
            .copied()
            .filter(|index| dependencies_satisfied(&plan.units[*index], &identities, &scheduled))
            .collect();
        if ready.is_empty() {
            return Err("execution dependency graph contains a cycle".to_owned());
        }
        let mut selected: Vec<usize> = Vec::new();
        for index in ready {
            if selected.len() == plan.maximum_concurrency {
                break;
            }
            if selected.iter().all(|other| {
                !plan.units[index]
                    .resources
                    .conflicts_with(&plan.units[*other].resources)
            }) {
                selected.push(index);
            }
        }
        if selected.is_empty() {
            return Err(
                "ready execution units could not be assigned an isolation-safe wave".to_owned(),
            );
        }
        for index in &selected {
            remaining.remove(index);
            scheduled.insert(*index);
        }
        waves.push(ProofExecutionWave {
            ordinal: waves.len(),
            unit_indices: selected,
        });
    }
    let schedule_digest = sha256_serialized(&ScheduleDigestBasis {
        plan_digest: &plan.plan_digest,
        maximum_concurrency: plan.maximum_concurrency,
        waves: &waves,
    })?;
    Ok(ProofExecutionSchedule {
        schedule_digest,
        plan_digest: plan.plan_digest.clone(),
        maximum_concurrency: plan.maximum_concurrency,
        waves,
    })
}

fn indexed_identities(units: &[ProofExecutionUnit]) -> Result<BTreeMap<String, usize>, String> {
    let mut identities = BTreeMap::new();
    for (index, unit) in units.iter().enumerate() {
        let identity = unit.identity();
        if identities.insert(identity.clone(), index).is_some() {
            return Err(format!(
                "execution plan duplicates unit identity {identity}"
            ));
        }
    }
    Ok(identities)
}

fn validate_dependencies(
    units: &[ProofExecutionUnit],
    identities: &BTreeMap<String, usize>,
) -> Result<(), String> {
    for unit in units {
        let identity = unit.identity();
        let mut unique = BTreeSet::new();
        for dependency in &unit.dependencies {
            if dependency == &identity {
                return Err(format!("execution unit {identity} depends on itself"));
            }
            if !identities.contains_key(dependency) {
                return Err(format!(
                    "execution unit {identity} has missing dependency {dependency}"
                ));
            }
            if !unique.insert(dependency) {
                return Err(format!(
                    "execution unit {identity} duplicates dependency {dependency}"
                ));
            }
        }
    }
    Ok(())
}

fn dependencies_satisfied(
    unit: &ProofExecutionUnit,
    identities: &BTreeMap<String, usize>,
    scheduled: &BTreeSet<usize>,
) -> bool {
    unit.dependencies
        .iter()
        .all(|dependency| scheduled.contains(&identities[dependency]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selection::{
        ProofExecutionUnit, StoreProofMode, StoreProofRequest, StructuralPreflightReference,
    };

    fn unit(package: &str) -> ProofExecutionUnit {
        let mut unit = ProofExecutionUnit::feature_compatibility(package.to_owned(), Vec::new());
        unit.resources.target_root = format!("target/{package}");
        unit
    }

    fn plan(mut units: Vec<ProofExecutionUnit>) -> SelectedProofExecutionPlan {
        let root = std::path::Path::new(".");
        let request = StoreProofRequest::new(StoreProofMode::Ci, None, None, None, None, true);
        units.sort();
        let selection = crate::selection::StoreProofSelection {
            included_products: vec!["store-ci:test".to_owned()],
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
        };
        SelectedProofExecutionPlan::lower(
            root,
            request,
            selection,
            units,
            None,
            BTreeMap::new(),
            crate::selection::RepositoryIdentity {
                source_revision: "revision".to_owned(),
                source_tree_digest: "tree".to_owned(),
                lockfile_digest: "lock".to_owned(),
                rustc_identity: "rustc".to_owned(),
                operating_system: "test".to_owned(),
                architecture: "test".to_owned(),
            },
            StructuralPreflightReference::synthetic_for_selection(root),
        )
        .unwrap()
    }

    #[test]
    fn schedule_is_deterministic_and_parallel_only_for_disjoint_resources() {
        let mut proof_plan = plan(vec![unit("a"), unit("b"), unit("c")]);
        proof_plan.maximum_concurrency = 2;
        let first = schedule(&proof_plan).unwrap();
        let second = schedule(&proof_plan).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.waves[0].unit_indices.len(), 2);

        proof_plan.units[1].resources.target_root =
            proof_plan.units[0].resources.target_root.clone();
        let serialized = schedule(&proof_plan).unwrap();
        assert!(serialized
            .waves
            .iter()
            .all(|wave| wave.unit_indices.len() <= 2));
        assert!(!serialized
            .waves
            .iter()
            .any(|wave| { wave.unit_indices.contains(&0) && wave.unit_indices.contains(&1) }));
    }

    #[test]
    fn malformed_dependency_graph_denies_before_execution() {
        let mut missing = unit("missing");
        missing.dependencies.push("absent::unit::all".to_owned());
        assert!(schedule(&plan(vec![missing]))
            .unwrap_err()
            .contains("missing dependency"));

        let mut left = unit("left");
        let mut right = unit("right");
        left.dependencies.push(right.identity());
        right.dependencies.push(left.identity());
        assert!(schedule(&plan(vec![left, right]))
            .unwrap_err()
            .contains("cycle"));
    }
}
