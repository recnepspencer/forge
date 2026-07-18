mod boundary_policy;
mod cache_identity;
mod classification_denials;
mod discovery_boundary_fixture;
mod discovery_preservation;
mod failure_localization_fixture;
mod feature_graph_fixture;
mod owner_closure_fixture;
mod proof_selection;
mod replacement_parity;
mod scratch_workspace;
mod shared_codegen_fixture;
mod suite_topology;

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::classification::{
    classify_from_authority, validate, ClassifiedInventory, PostBaselineProofAuthority,
};
use crate::discovery::discover_workspace;
use crate::evidence::read_json;
use crate::preservation::{semantic_authority_from_ledger, ProofPreservationLedger};

fn current_inventory(root: &Path) -> &'static crate::ValidatedProofInventory {
    static INVENTORY: OnceLock<crate::ValidatedProofInventory> = OnceLock::new();
    INVENTORY.get_or_init(|| {
        let baseline: ClassifiedInventory =
            read_json(&root.join("test-control/pre-cleanup/classified-proof-inventory.json"))
                .unwrap();
        let ledger: ProofPreservationLedger =
            read_json(&root.join("test-control/pre-cleanup/proof-preservation-ledger.json"))
                .unwrap();
        let authority = semantic_authority_from_ledger(&baseline, &ledger).unwrap();
        let post_baseline: PostBaselineProofAuthority =
            read_json(&root.join("test-control/post-baseline-proof-authority.json")).unwrap();
        let discovered = discover_workspace(root, false).unwrap();
        validate(classify_from_authority(discovered, &authority, &post_baseline).unwrap()).unwrap()
    })
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap()
        .to_path_buf()
}

fn selection_preflight(root: &Path) -> crate::selection::StructuralPreflightReference {
    crate::selection::StructuralPreflightReference::synthetic_for_selection(root)
}
