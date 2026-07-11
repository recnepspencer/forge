use std::fs;
use std::path::{Path, PathBuf};

use forge_store_readiness::{FoundationalAdoptionDenial, FoundationalVocabularyAdoptionMap};
use forge_store_s0_reclassification::{
    certify_current_s0_handoff_gate_proof_evidence, S0HandoffGateProofEvidence,
    S0HandoffGateProofEvidenceDenial,
};
use forge_store_test_support::NativeStoreAspectFixture;

use crate::{
    certify_store_json_residue_inventory, StoreJsonResidueDenial, StoreJsonResidueInventory,
    StoreJsonResidueZone,
};

pub fn certify_s0_handoff_gate_proof_evidence(
) -> Result<S0HandoffGateProofEvidence, S0HandoffGateCertificationDenial> {
    let certified_current_evidence = certify_current_s0_handoff_gate_proof_evidence()?;
    let residue_inventory = certify_store_json_residue_inventory()?;
    require_current_residue_scan(&residue_inventory)?;
    require_terminal_projection_boundary(&residue_inventory)?;
    require_foundational_adoption()?;
    require_public_facade()?;
    require_native_harness()?;

    Ok(certified_current_evidence)
}

fn require_current_residue_scan(
    inventory: &StoreJsonResidueInventory,
) -> Result<(), S0HandoffGateProofEvidenceDenial> {
    if inventory.classified().is_empty() {
        return Err(S0HandoffGateProofEvidenceDenial::MissingCurrentResidueScan);
    }
    Ok(())
}

fn require_terminal_projection_boundary(
    inventory: &StoreJsonResidueInventory,
) -> Result<(), S0HandoffGateProofEvidenceDenial> {
    let terminal_boundary_count = inventory
        .classified()
        .iter()
        .filter(|classification| {
            classification.zone() == StoreJsonResidueZone::DedicatedWorkspaceTerminalBoundary
        })
        .count();
    if terminal_boundary_count == 0 {
        return Err(S0HandoffGateProofEvidenceDenial::MissingTerminalProjectionBoundary);
    }
    Ok(())
}

fn require_foundational_adoption() -> Result<(), S0HandoffGateCertificationDenial> {
    let adoption = FoundationalVocabularyAdoptionMap::s1_all_public_lanes()?;
    if adoption.rows().is_empty() {
        return Err(S0HandoffGateProofEvidenceDenial::MissingFoundationalAdoption.into());
    }
    Ok(())
}

fn require_public_facade() -> Result<(), S0HandoffGateCertificationDenial> {
    let facade = workspace_file("workspaces/forge-store/crates/forge-store/src/lib.rs")?;
    let aspect_native = facade_module_body(&facade, "aspect_native")?;
    let certification = facade_module_body(&facade, "certification")?;
    let terminal_projection = facade_module_body(&facade, "terminal_projection")?;

    require_exports(
        aspect_native,
        &[
            "StoreAspectBoundaryFact",
            "StorePhysicalBoundaryWitness",
            "StorePhysicalAuthorityWitness",
        ],
    )?;
    require_exports(
        certification,
        &[
            "certify_store_json_residue_inventory",
            "StoreJsonResidueInventory",
            "StoreJsonResidueDenial",
        ],
    )?;
    require_exports(
        terminal_projection,
        &[
            "project_store_boundary_fact_to_terminal_json",
            "StoreTerminalJsonProjection",
        ],
    )?;
    require_no_json_or_serde_export(aspect_native)?;
    require_no_json_or_serde_export(certification)?;

    Ok(())
}

fn require_native_harness() -> Result<(), S0HandoffGateCertificationDenial> {
    let segment = NativeStoreAspectFixture::segment_header("s0-handoff-harness", 11);
    let scalar = NativeStoreAspectFixture::scalar_string("s0-handoff-scalar");

    if segment.struct_value().is_none() || scalar.scalar_value().is_none() {
        return Err(S0HandoffGateCertificationDenial::NativeHarnessMissingSurface);
    }
    if segment.boundary_fact().identity() != segment.identity()
        || scalar.boundary_fact().identity() != scalar.identity()
    {
        return Err(S0HandoffGateCertificationDenial::NativeHarnessMissingSurface);
    }

    Ok(())
}

fn facade_module_body<'a>(
    facade: &'a str,
    module_name: &'static str,
) -> Result<&'a str, S0HandoffGateCertificationDenial> {
    let module_header = format!("pub mod {module_name} {{");
    let Some(module_start) = facade.find(&module_header) else {
        return Err(S0HandoffGateCertificationDenial::PublicFacadeMissingModule(
            module_name,
        ));
    };
    let body_start = module_start + module_header.len();
    let mut depth = 1usize;
    for (offset, character) in facade[body_start..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(&facade[body_start..body_start + offset]);
                }
            }
            _ => {}
        }
    }
    Err(S0HandoffGateCertificationDenial::PublicFacadeMissingModule(
        module_name,
    ))
}

fn require_exports(
    module_body: &str,
    expected_exports: &[&'static str],
) -> Result<(), S0HandoffGateCertificationDenial> {
    for expected in expected_exports {
        if !module_body.contains(expected) {
            return Err(S0HandoffGateCertificationDenial::PublicFacadeMissingExport(
                expected,
            ));
        }
    }
    Ok(())
}

fn require_no_json_or_serde_export(
    module_body: &str,
) -> Result<(), S0HandoffGateCertificationDenial> {
    if module_body.lines().any(contains_forbidden_json_or_serde) {
        return Err(S0HandoffGateCertificationDenial::PublicFacadeJsonExport);
    }
    Ok(())
}

fn contains_forbidden_json_or_serde(line: &str) -> bool {
    line.contains(&["serde", "json"].join("_"))
        || line.contains(&["json", "!"].join(""))
        || line.contains(&["Serial", "ize"].join(""))
        || line.contains(&["De", "serialize", "Owned"].join(""))
        || line.contains(&["Json", "Document"].join(""))
        || line.contains(&["json", "document"].join("_"))
}

fn workspace_file(relative: &str) -> Result<String, S0HandoffGateCertificationDenial> {
    fs::read_to_string(repo_root().join(relative))
        .map_err(|error| S0HandoffGateCertificationDenial::SourceReadFailed(error.to_string()))
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("certification crate lives under workspaces/forge-store/crates")
        .to_path_buf()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S0HandoffGateCertificationDenial {
    Residue(StoreJsonResidueDenial),
    FoundationalAdoption(FoundationalAdoptionDenial),
    GateProof(S0HandoffGateProofEvidenceDenial),
    PublicFacadeMissingModule(&'static str),
    PublicFacadeMissingExport(&'static str),
    PublicFacadeJsonExport,
    NativeHarnessMissingSurface,
    SourceReadFailed(String),
}

impl From<StoreJsonResidueDenial> for S0HandoffGateCertificationDenial {
    fn from(denial: StoreJsonResidueDenial) -> Self {
        Self::Residue(denial)
    }
}

impl From<FoundationalAdoptionDenial> for S0HandoffGateCertificationDenial {
    fn from(denial: FoundationalAdoptionDenial) -> Self {
        Self::FoundationalAdoption(denial)
    }
}

impl From<S0HandoffGateProofEvidenceDenial> for S0HandoffGateCertificationDenial {
    fn from(denial: S0HandoffGateProofEvidenceDenial) -> Self {
        Self::GateProof(denial)
    }
}
