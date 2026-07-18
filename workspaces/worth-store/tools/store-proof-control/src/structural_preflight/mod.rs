mod evaluator;
mod execution;
mod freshness;
mod inputs;
#[cfg(test)]
mod integrity_tests;
mod plan;
#[cfg(test)]
mod plan_tests;
mod predicate_evaluation;
mod repository_failure;
mod residue;
#[cfg(test)]
mod tests;
mod tool_execution;
mod version_probe;

use std::path::{Path, PathBuf};

pub(crate) use freshness::compare_to_plan;
pub use freshness::require_fresh;
pub use worth_store_test_support::structural_preflight::{
    PreflightEvidenceFreshness, PreflightEvidenceIdentity, StructuralPredicate,
    StructuralPredicateFailure, StructuralPreflightEvidence, StructuralPreflightProfile,
    StructuralPreflightRequest,
};

use crate::selection::StoreProofMode;
use crate::ValidatedProofInventory;

pub(crate) use repository_failure::RepositoryPredicateFailure;

#[derive(Debug)]
pub struct StructuralPreflightProduct {
    pub evidence: StructuralPreflightEvidence,
    pub bundle_path: PathBuf,
}

pub(crate) fn execute(
    forge_root: &Path,
    store_root: &Path,
    mode: StoreProofMode,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&RepositoryPredicateFailure>,
) -> Result<StructuralPreflightProduct, String> {
    let request = request_for_mode(mode)?;
    let plan = plan::build(forge_root, request)?;
    let evidence = execution::execute(forge_root, plan, inventory, validation_failure)?;
    evidence
        .validate_integrity()
        .map_err(|denial| denial.to_string())?;
    let bundle_path = bundle_path(store_root, &evidence.evidence_identity);
    crate::evidence::write_immutable_json(&bundle_path, &evidence)?;
    Ok(StructuralPreflightProduct {
        evidence,
        bundle_path,
    })
}

pub(crate) fn consume(
    forge_root: &Path,
    mode: StoreProofMode,
    bundle_path: &Path,
) -> Result<StructuralPreflightProduct, String> {
    let evidence: StructuralPreflightEvidence = crate::evidence::read_json(bundle_path)?;
    evidence
        .validate_integrity()
        .map_err(|denial| denial.to_string())?;
    if bundle_path.file_stem().and_then(|value| value.to_str())
        != Some(evidence.evidence_identity.0.as_str())
    {
        return Err(format!(
            "preflight bundle path does not carry evidence identity {}: {}",
            evidence.evidence_identity.0,
            bundle_path.display()
        ));
    }
    require_compatible_predicates(mode, &evidence)?;
    let failures = evidence.failures();
    if !failures.is_empty() {
        return Err(format!(
            "preflight bundle contains failed predicates: {}",
            failures
                .iter()
                .map(|failure| format!("{:?}/{}", failure.predicate, failure.failure_code))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    match require_fresh(forge_root, &evidence)? {
        PreflightEvidenceFreshness::Fresh { .. } => Ok(StructuralPreflightProduct {
            evidence,
            bundle_path: bundle_path.to_path_buf(),
        }),
        PreflightEvidenceFreshness::Stale { failures } => Err(format!(
            "preflight bundle is stale: {}",
            failures
                .iter()
                .map(|failure| format!(
                    "{:?}/{} invalidated {:?}",
                    failure.predicate, failure.failure_code, failure.invalidated_inputs
                ))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn require_compatible_predicates(
    mode: StoreProofMode,
    evidence: &StructuralPreflightEvidence,
) -> Result<(), String> {
    let required = request_for_mode(mode)?;
    let observed = evidence
        .plan
        .request
        .predicates
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let missing = required
        .predicates
        .iter()
        .filter(|predicate| !observed.contains(predicate))
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "preflight bundle is incompatible with {}: missing {missing:?}",
            mode_identity(mode)
        ))
    }
}

const fn mode_identity(mode: StoreProofMode) -> &'static str {
    match mode {
        StoreProofMode::Owner => "store-owner",
        StoreProofMode::Smoke => "store-smoke",
        StoreProofMode::Ui => "store-ui",
        StoreProofMode::Ci => "store-ci",
        StoreProofMode::Soak => "store-soak",
        StoreProofMode::Release => "store-release",
        StoreProofMode::Hardware => "store-hardware",
    }
}

pub fn forge_root(store_root: &Path) -> Result<PathBuf, String> {
    store_root
        .ancestors()
        .find(|candidate| candidate.join("tools/boundary-check/Cargo.toml").is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| format!("could not locate Forge root above {}", store_root.display()))
}

pub fn bundle_path(store_root: &Path, identity: &PreflightEvidenceIdentity) -> PathBuf {
    store_root
        .join(".store-proof/evidence/preflight")
        .join(format!("{}.json", identity.0))
}

fn request_for_mode(mode: StoreProofMode) -> Result<StructuralPreflightRequest, String> {
    use StructuralPredicate as Predicate;
    let (profile, predicates) = match mode {
        StoreProofMode::Ui => (
            StructuralPreflightProfile::Ui,
            vec![
                Predicate::Boundary,
                Predicate::Feature,
                Predicate::Dependency,
                Predicate::Naming,
                Predicate::AdmittedResidue,
            ],
        ),
        StoreProofMode::Ci
        | StoreProofMode::Soak
        | StoreProofMode::Release
        | StoreProofMode::Hardware => (
            StructuralPreflightProfile::Complete,
            vec![
                Predicate::Boundary,
                Predicate::AgentContext,
                Predicate::Inventory,
                Predicate::Preservation,
                Predicate::Feature,
                Predicate::Dependency,
                Predicate::LineCap,
                Predicate::Naming,
                Predicate::AdmittedResidue,
            ],
        ),
        StoreProofMode::Owner | StoreProofMode::Smoke => (
            StructuralPreflightProfile::DeveloperSmoke,
            vec![
                Predicate::Inventory,
                Predicate::Preservation,
                Predicate::Dependency,
                Predicate::AdmittedResidue,
            ],
        ),
    };
    StructuralPreflightRequest::new(profile, predicates)
}
