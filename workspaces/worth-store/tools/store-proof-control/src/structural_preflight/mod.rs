pub mod dependency_boundary;
mod execution;
mod freshness;
mod inputs;
mod plan;
mod residue;
#[cfg(test)]
mod tests;
mod tool_execution;

use std::path::{Path, PathBuf};

pub use freshness::require_fresh;
pub use worth_store_test_support::structural_preflight::{
    PreflightEvidenceFreshness, PreflightEvidenceIdentity, StructuralPredicate,
    StructuralPredicateFailure, StructuralPreflightEvidence, StructuralPreflightProfile,
    StructuralPreflightRequest,
};

use crate::selection::StoreProofMode;
use crate::ValidatedProofInventory;

#[derive(Debug)]
pub struct StructuralPreflightProduct {
    pub evidence: StructuralPreflightEvidence,
    pub bundle_path: PathBuf,
}

pub fn execute(
    forge_root: &Path,
    store_root: &Path,
    mode: StoreProofMode,
    inventory: Option<&ValidatedProofInventory>,
    validation_failure: Option<&str>,
) -> Result<StructuralPreflightProduct, String> {
    let request = request_for_mode(mode)?;
    let plan = plan::build(forge_root, request)?;
    let evidence = execution::execute(
        forge_root,
        store_root,
        plan,
        inventory,
        validation_failure,
    )?;
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

pub fn forge_root(store_root: &Path) -> Result<PathBuf, String> {
    store_root
        .ancestors()
        .find(|candidate| candidate.join("tools/boundary-check/Cargo.toml").is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            format!(
                "could not locate Forge root above {}",
                store_root.display()
            )
        })
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
