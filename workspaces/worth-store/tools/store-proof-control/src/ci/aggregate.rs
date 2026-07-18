use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::evidence::sha256_serialized;
use crate::execution::ExecutedProofRun;
use crate::selection::SelectedProofExecutionPlan;
use crate::ValidatedProofInventory;

use super::{required_lanes, CiCacheIdentity, CiShardPlan, RequiredCiLane};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiPartitionEvidence {
    pub schema_version: u32,
    pub evidence_identity: String,
    pub partition: String,
    pub operating_system: String,
    pub source_identity: String,
    pub plan_digest: String,
    pub run_identity: String,
    pub cache_identity: CiCacheIdentity,
    pub shard_plan: Option<CiShardPlan>,
    pub behavioral_verdict: String,
    pub attempt_identities: Vec<String>,
    pub external_observer_authorities: Vec<String>,
    pub formal_tool_receipts: Vec<String>,
    pub closeout_eligible: bool,
    pub observed_unix_millis: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct MissingCiProofPartition {
    pub partition: String,
    pub operating_system: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiCertificationAggregate {
    pub schema_version: u32,
    pub aggregate_identity: String,
    pub source_identity: String,
    pub required_lanes: BTreeSet<RequiredCiLane>,
    pub evidence_history: Vec<CiLaneEvidenceHistory>,
    pub promoted_evidence: Vec<CiPromotedLaneEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiLaneEvidenceHistory {
    pub lane: RequiredCiLane,
    pub evidence_identities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiPromotedLaneEvidence {
    pub lane: RequiredCiLane,
    pub evidence_identities: Vec<String>,
}

impl CiPartitionEvidence {
    pub fn from_run(
        workspace_root: &Path,
        partition: &str,
        plan: &SelectedProofExecutionPlan,
        run: &ExecutedProofRun,
        shard_plan: Option<CiShardPlan>,
    ) -> Result<Self, String> {
        let cache_identity = CiCacheIdentity::from_plan(workspace_root, partition, plan)?;
        let source_identity = source_identity(plan)?;
        let external_observer_authorities = run.observed_cost.observer_authorities.clone();
        let structural_preflight = partition == "structural-preflight" && plan.units.is_empty();
        let formal_tool_receipts: Vec<_> = run
            .attempts
            .iter()
            .filter_map(|attempt| attempt.formal_tool_evidence.as_ref())
            .map(|evidence| evidence.receipt_sha256.clone())
            .collect();
        let formal_evidence_complete =
            partition != "formal-external" || !formal_tool_receipts.is_empty();
        let closeout_eligible = run.behavioral_verdict == "passed"
            && formal_evidence_complete
            && (structural_preflight
                || external_observer_authorities
                    .iter()
                    .any(|authority| authority == "independent-observer-process"));
        let mut evidence = Self {
            schema_version: 1,
            evidence_identity: String::new(),
            partition: partition.to_owned(),
            operating_system: plan.repository.operating_system.clone(),
            source_identity,
            plan_digest: plan.plan_digest.clone(),
            run_identity: run.run_identity.clone(),
            cache_identity,
            shard_plan,
            behavioral_verdict: run.behavioral_verdict.clone(),
            attempt_identities: run
                .attempts
                .iter()
                .map(|attempt| attempt.attempt_identity.clone())
                .collect(),
            external_observer_authorities,
            formal_tool_receipts,
            closeout_eligible,
            observed_unix_millis: run.run_started_unix_millis,
        };
        evidence.evidence_identity = sha256_serialized(&evidence)?;
        Ok(evidence)
    }

    pub fn output_path(&self, workspace_root: &Path) -> PathBuf {
        workspace_root
            .join(".store-proof/evidence/ci/partitions")
            .join(&self.source_identity)
            .join(&self.partition)
            .join(&self.operating_system)
            .join(format!("{}.json", self.evidence_identity))
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported CI partition evidence schema {}",
                self.schema_version
            ));
        }
        self.cache_identity.validate()?;
        if self.cache_identity.partition != self.partition
            || self.cache_identity.operating_system != self.operating_system
        {
            return Err("CI cache identity belongs to a different partition or OS lane".to_owned());
        }
        let mut basis = self.clone();
        basis.evidence_identity.clear();
        if sha256_serialized(&basis)? != self.evidence_identity {
            return Err("CI partition evidence identity does not match its contents".to_owned());
        }
        let structural_preflight =
            self.partition == "structural-preflight" && self.attempt_identities.is_empty();
        if self.closeout_eligible
            && (self.behavioral_verdict != "passed"
                || (!structural_preflight
                    && !self
                        .external_observer_authorities
                        .iter()
                        .any(|authority| authority == "independent-observer-process")))
        {
            return Err(
                "CI evidence claims closure without behavior and observer proof".to_owned(),
            );
        }
        if self.closeout_eligible
            && self.partition == "formal-external"
            && self.formal_tool_receipts.is_empty()
        {
            return Err("formal CI evidence claims closure without a TLC receipt".to_owned());
        }
        Ok(())
    }
}

impl CiCertificationAggregate {
    pub fn certify(
        inventory: &ValidatedProofInventory,
        evidence: &[CiPartitionEvidence],
    ) -> Result<Self, Vec<MissingCiProofPartition>> {
        let required_lanes = required_lanes(inventory);
        let invalid: Vec<_> = evidence
            .iter()
            .filter_map(|bundle| {
                bundle
                    .validate()
                    .err()
                    .map(|reason| MissingCiProofPartition {
                        partition: bundle.partition.clone(),
                        operating_system: bundle.operating_system.clone(),
                        reason,
                    })
            })
            .collect();
        if !invalid.is_empty() {
            return Err(invalid);
        }
        let source_identities: BTreeSet<_> = evidence
            .iter()
            .map(|bundle| bundle.source_identity.as_str())
            .collect();
        if source_identities.len() != 1 {
            return Err(vec![MissingCiProofPartition {
                partition: "aggregate".to_owned(),
                operating_system: "all".to_owned(),
                reason: "partition evidence does not share one source identity".to_owned(),
            }]);
        }
        let source_identity = source_identities
            .first()
            .map(|identity| (*identity).to_owned())
            .unwrap_or_default();
        let mut by_lane: BTreeMap<RequiredCiLane, Vec<&CiPartitionEvidence>> = BTreeMap::new();
        for bundle in evidence {
            let lane = RequiredCiLane {
                partition: bundle.partition.clone(),
                operating_system: bundle.operating_system.clone(),
            };
            by_lane.entry(lane).or_default().push(bundle);
        }
        let mut missing = Vec::new();
        let mut history = BTreeMap::new();
        let mut promoted = BTreeMap::new();
        for lane in &required_lanes {
            let mut bundles = by_lane.get(lane).cloned().unwrap_or_default();
            bundles.sort_by_key(|bundle| bundle.observed_unix_millis);
            history.insert(
                lane.clone(),
                bundles
                    .iter()
                    .map(|bundle| bundle.evidence_identity.clone())
                    .collect(),
            );
            match promoted_lane_evidence(&bundles) {
                Ok(evidence_identities) => {
                    promoted.insert(lane.clone(), evidence_identities);
                }
                Err(reason) => missing.push(MissingCiProofPartition {
                    partition: lane.partition.clone(),
                    operating_system: lane.operating_system.clone(),
                    reason,
                }),
            }
        }
        if !missing.is_empty() {
            return Err(missing);
        }
        let mut aggregate = Self {
            schema_version: 1,
            aggregate_identity: String::new(),
            source_identity,
            required_lanes,
            evidence_history: history
                .into_iter()
                .map(|(lane, evidence_identities)| CiLaneEvidenceHistory {
                    lane,
                    evidence_identities,
                })
                .collect(),
            promoted_evidence: promoted
                .into_iter()
                .map(|(lane, evidence_identities)| CiPromotedLaneEvidence {
                    lane,
                    evidence_identities,
                })
                .collect(),
        };
        aggregate.aggregate_identity = sha256_serialized(&aggregate).map_err(|reason| {
            vec![MissingCiProofPartition {
                partition: "aggregate".to_owned(),
                operating_system: "all".to_owned(),
                reason,
            }]
        })?;
        Ok(aggregate)
    }

    pub fn output_path(&self, workspace_root: &Path) -> PathBuf {
        workspace_root
            .join(".store-proof/evidence/ci/aggregates")
            .join(&self.source_identity)
            .join(format!("{}.json", self.aggregate_identity))
    }
}

pub fn read_partition_evidence(root: &Path) -> Result<Vec<CiPartitionEvidence>, String> {
    if !root.is_dir() {
        return Err(format!(
            "CI evidence root is not a directory: {}",
            root.display()
        ));
    }
    let mut pending = vec![root.to_path_buf()];
    let mut paths = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("could not inspect {}: {error}", directory.display()))?
        {
            let entry = entry.map_err(|error| {
                format!(
                    "could not inspect entry under {}: {error}",
                    directory.display()
                )
            })?;
            let file_type = entry.file_type().map_err(|error| {
                format!("could not classify {}: {error}", entry.path().display())
            })?;
            if file_type.is_symlink() {
                return Err(format!(
                    "CI evidence traversal denies symlink {}",
                    entry.path().display()
                ));
            }
            if file_type.is_dir() {
                pending.push(entry.path());
            } else if file_type.is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("json")
            {
                paths.push(entry.path());
            }
        }
    }
    paths.sort();
    paths
        .into_iter()
        .map(|path| crate::evidence::read_json(&path))
        .collect()
}

fn promoted_lane_evidence(bundles: &[&CiPartitionEvidence]) -> Result<Vec<String>, String> {
    if bundles.is_empty() {
        return Err("required lane has no evidence".to_owned());
    }
    let shard_plans: Vec<_> = bundles
        .iter()
        .filter_map(|bundle| bundle.shard_plan.as_ref())
        .collect();
    if shard_plans.is_empty() {
        return bundles
            .iter()
            .rev()
            .find(|bundle| bundle.closeout_eligible)
            .map(|bundle| vec![bundle.evidence_identity.clone()])
            .ok_or_else(|| "required lane has no closeout-eligible attempt".to_owned());
    }
    if shard_plans.len() != bundles.len() {
        return Err("CI lane mixes sharded and unsharded evidence".to_owned());
    }
    let plan_identities: BTreeSet<_> = shard_plans
        .iter()
        .map(|plan| plan.plan_identity.as_str())
        .collect();
    if plan_identities.len() != 1 {
        return Err("CI lane mixes incompatible shard plans".to_owned());
    }
    let shard_count = shard_plans[0].shard_count;
    let mut promoted = Vec::new();
    for shard_index in 0..shard_count {
        let candidate = bundles.iter().rev().find(|bundle| {
            bundle.closeout_eligible
                && bundle
                    .shard_plan
                    .as_ref()
                    .is_some_and(|plan| plan.selected_shard == shard_index)
        });
        match candidate {
            Some(candidate) => promoted.push(candidate.evidence_identity.clone()),
            None => return Err(format!("CI lane is missing successful shard {shard_index}")),
        }
    }
    Ok(promoted)
}

fn source_identity(plan: &SelectedProofExecutionPlan) -> Result<String, String> {
    sha256_serialized(&(
        "worth-store-ci-source-v1",
        &plan.repository.source_revision,
        &plan.repository.source_tree_digest,
        &plan.repository.lockfile_digest,
    ))
}
