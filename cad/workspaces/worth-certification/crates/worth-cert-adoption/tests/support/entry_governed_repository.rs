use std::fs;
use std::path::Path;

use crate::corpus_contract::{Corpus, EntryDependency, Specimen};
use crate::governed_config::CONFIG;
use crate::synthetic_repository_filesystem::{recreate, write};

const ROOT_CONSTITUTION: &str = "[workspace]\nresolver = \"2\"\nexclude = [\"cad/workspaces/*\"]\nmembers = []\n\n[workspace.metadata.worth_topology]\nrole = \"thin_orchestrator\"\nroad1_subworkspaces = [\"cad/workspaces/worth-entry\"]\nforbidden_member_prefixes = [\"cad/workspaces/\"]\nboundary_check_manifest = \"tools/boundary-check/Cargo.toml\"\nboundary_check_config = \"tools/boundary-check/config/road1.toml\"\n";
const ENTRY_WORKSPACE: &str = "[workspace]\nresolver = \"2\"\nmembers = [\"crates/worth-entry-adoption\"]\n\n[workspace.metadata.worth_topology]\nrole = \"road1_subworkspace\"\nconstitutional_lane = \"worth-entry\"\nmember_lane = \"crates/*\"\nallowed_crate_prefixes = [\"worth-entry-\"]\n";

pub fn assemble(root: &Path) {
    recreate(root);
    write_root_constitution(root);
    write_machine_authority(root);
    write_governed_entry_consumer(root);
    write_proof_provider(root);
    write_replay_facade_provider(root);
}

fn write_root_constitution(root: &Path) {
    write(root, "Cargo.toml", ROOT_CONSTITUTION);
    write(
        root,
        "cad/docs/worthy-foundations/NAMING.md",
        "# fixture naming\n",
    );
}

fn write_machine_authority(root: &Path) {
    write(
        root,
        "tools/boundary-check/Cargo.toml",
        "[package]\nname = \"fixture-boundary-check\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
    write(
        root,
        "tools/boundary-check/snapshots/legacy-references.toml",
        "schema_version = 1\nreferences = []\n",
    );
    write(root, "tools/boundary-check/config/road1.toml", CONFIG);
}

fn write_governed_entry_consumer(root: &Path) {
    write(
        root,
        "cad/workspaces/worth-entry/Cargo.toml",
        ENTRY_WORKSPACE,
    );
    write(root, "cad/workspaces/worth-entry/README.md", "# fixture\n");
    write(
        root,
        "cad/workspaces/worth-entry/crates/worth-entry-adoption/AGENT_CONTEXT.md",
        "# fixture\n",
    );
    write(
        root,
        "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/lib.rs",
        "pub mod facade;\nmod legal_control;\n",
    );
    write(
        root,
        "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/legal_control.rs",
        "pub struct LegalControl;\n",
    );
    write(
        root,
        "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/facade.rs",
        "pub use crate::legal_control::LegalControl;\n",
    );
    write(
        root,
        "cad/workspaces/worth-entry/crates/worth-entry-adoption/Cargo.toml",
        "[package]\nname = \"worth-entry-adoption\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    );
}

fn write_proof_provider(root: &Path) {
    write(
        root,
        "vendor/worth-proof/Cargo.toml",
        "[package]\nname = \"worth-proof\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n",
    );
    write(
        root,
        "vendor/worth-proof/src/lib.rs",
        "pub trait AuthorityMarker: 'static {}\n",
    );
}

fn write_replay_facade_provider(root: &Path) {
    write(
        root,
        "crates/worth-query/Cargo.toml",
        "[package]\nname = \"worth-query\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n",
    );
    write(
        root,
        "crates/worth-query/src/lib.rs",
        "pub mod facade { pub mod foundation { pub struct ScopedReplayBasis; } }\n",
    );
    write(root, "crates/worth-query-replay/Cargo.toml", "[package]\nname = \"worth-query-replay\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nworth-query = { path = \"../worth-query\" }\n[workspace]\n");
    write(
        root,
        "crates/worth-query-replay/src/lib.rs",
        "pub mod facade;\n",
    );
    write(
        root,
        "crates/worth-query-replay/src/facade.rs",
        "pub use worth_query::facade::foundation::ScopedReplayBasis;\n",
    );
}

pub fn install_hostile(
    root: &Path,
    corpus: &Corpus,
    specimen: &Specimen,
    dependency: EntryDependency,
) {
    let source = fs::read_to_string(corpus.specimen_path(specimen)).expect("read specimen");
    write(
        root,
        "cad/workspaces/worth-entry/crates/worth-entry-adoption/src/lib.rs",
        &format!("pub mod facade;\nmod legal_control;\n{source}\n"),
    );
    let dependency = match dependency {
        EntryDependency::Replay => {
            "worth-query-replay = { path = \"../../../../../crates/worth-query-replay\" }"
        }
        EntryDependency::Proof => "worth-proof = { path = \"../../../../../vendor/worth-proof\" }",
    };
    write(root, "cad/workspaces/worth-entry/crates/worth-entry-adoption/Cargo.toml", &format!("[package]\nname = \"worth-entry-adoption\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n{dependency}\n"));
}
