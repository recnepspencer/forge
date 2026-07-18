use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::execution::{CargoArtifactSemanticIdentity, ObservedCargoArtifact};

use super::CiPartitionEvidence;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiCompilerArtifactObservation {
    pub attempt_identity: String,
    pub unit_identity: String,
    pub artifact: ObservedCargoArtifact,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiCompilationAudit {
    pub freshly_compiled_artifacts: usize,
    pub reused_artifact_observations: usize,
    pub explained_semantic_duplicates: Vec<CiExplainedCompilationDuplication>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiExplainedCompilationDuplication {
    pub artifact: CargoArtifactSemanticIdentity,
    pub explanations: BTreeSet<CiCompilationDifference>,
    pub evidence_identities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum CiCompilationDifference {
    OperatingSystem,
    BuildProfile,
    FeatureLane,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ExactCompilationEquivalence {
    operating_system: String,
    profile_identity: String,
    features: Vec<String>,
}

#[derive(Debug)]
struct FreshCompilation<'a> {
    evidence: &'a CiPartitionEvidence,
    observation: &'a CiCompilerArtifactObservation,
}

pub(super) fn audit_compilation(
    promoted: &[&CiPartitionEvidence],
) -> Result<CiCompilationAudit, Vec<String>> {
    let mut by_semantic: BTreeMap<CargoArtifactSemanticIdentity, Vec<FreshCompilation<'_>>> =
        BTreeMap::new();
    let mut reused_artifact_observations = 0;
    for evidence in promoted {
        for observation in &evidence.compiler_artifacts {
            if observation.artifact.fresh {
                reused_artifact_observations += 1;
            } else {
                by_semantic
                    .entry(observation.artifact.semantic_identity())
                    .or_default()
                    .push(FreshCompilation {
                        evidence,
                        observation,
                    });
            }
        }
    }
    let freshly_compiled_artifacts = by_semantic.values().map(Vec::len).sum();
    let mut denials = Vec::new();
    let mut explained_semantic_duplicates = Vec::new();
    for (semantic, compilations) in by_semantic {
        if compilations.len() < 2 {
            continue;
        }
        let mut by_equivalence: BTreeMap<ExactCompilationEquivalence, Vec<&FreshCompilation<'_>>> =
            BTreeMap::new();
        for compilation in &compilations {
            by_equivalence
                .entry(ExactCompilationEquivalence {
                    operating_system: compilation.evidence.operating_system.clone(),
                    profile_identity: compilation.observation.artifact.profile_identity.clone(),
                    features: compilation.observation.artifact.features.clone(),
                })
                .or_default()
                .push(compilation);
        }
        for (equivalence, duplicates) in &by_equivalence {
            if duplicates.len() > 1 {
                denials.push(format!(
                    "{}::{} was freshly compiled {} times under equivalent OS/profile/features ({}/{}/{:?}) in evidence [{}]",
                    semantic.canonical_package,
                    semantic.target_name,
                    duplicates.len(),
                    equivalence.operating_system,
                    equivalence.profile_identity,
                    equivalence.features,
                    duplicates
                        .iter()
                        .map(|duplicate| duplicate.evidence.evidence_identity.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
        if by_equivalence
            .values()
            .all(|duplicates| duplicates.len() == 1)
        {
            explained_semantic_duplicates.push(explain(semantic, &compilations));
        }
    }
    if denials.is_empty() {
        explained_semantic_duplicates.sort_by(|left, right| left.artifact.cmp(&right.artifact));
        Ok(CiCompilationAudit {
            freshly_compiled_artifacts,
            reused_artifact_observations,
            explained_semantic_duplicates,
        })
    } else {
        denials.sort();
        Err(denials)
    }
}

fn explain(
    artifact: CargoArtifactSemanticIdentity,
    compilations: &[FreshCompilation<'_>],
) -> CiExplainedCompilationDuplication {
    let operating_systems: BTreeSet<_> = compilations
        .iter()
        .map(|compilation| compilation.evidence.operating_system.as_str())
        .collect();
    let profiles: BTreeSet<_> = compilations
        .iter()
        .map(|compilation| compilation.observation.artifact.profile_identity.as_str())
        .collect();
    let feature_lanes: BTreeSet<_> = compilations
        .iter()
        .map(|compilation| &compilation.observation.artifact.features)
        .collect();
    let mut explanations = BTreeSet::new();
    if operating_systems.len() > 1 {
        explanations.insert(CiCompilationDifference::OperatingSystem);
    }
    if profiles.len() > 1 {
        explanations.insert(CiCompilationDifference::BuildProfile);
    }
    if feature_lanes.len() > 1 {
        explanations.insert(CiCompilationDifference::FeatureLane);
    }
    CiExplainedCompilationDuplication {
        artifact,
        explanations,
        evidence_identities: compilations
            .iter()
            .map(|compilation| compilation.evidence.evidence_identity.clone())
            .collect(),
    }
}
