use std::fs;
use std::path::Path;

use crate::corpus_contract::{Corpus, Specimen};
use crate::synthetic_repository_filesystem::{recreate, write};

struct AudienceRepository<'a> {
    band: &'a str,
    facade: &'a str,
    item: &'a str,
    package: String,
    workspace: String,
    crate_root: String,
}

pub fn assemble(root: &Path, band: &str, facade: &str, item: &str) {
    recreate(root);
    let repository = AudienceRepository::new(band, facade, item);
    repository.write_root_constitution(root);
    repository.write_machine_authority(root);
    repository.write_governed_consumer(root);
    repository.write_query_facade_provider(root);
}

impl<'a> AudienceRepository<'a> {
    fn new(band: &'a str, facade: &'a str, item: &'a str) -> Self {
        let package = format!("worth-{band}-adoption");
        let workspace = format!("cad/workspaces/worth-{band}");
        let crate_root = format!("{workspace}/crates/{package}");
        Self {
            band,
            facade,
            item,
            package,
            workspace,
            crate_root,
        }
    }

    fn write_root_constitution(&self, root: &Path) {
        write(root, "Cargo.toml", &format!("[workspace]\nresolver = \"2\"\nexclude = [\"cad/workspaces/*\"]\nmembers = []\n\n[workspace.metadata.worth_topology]\nrole = \"thin_orchestrator\"\nroad1_subworkspaces = [\"{}\"]\nforbidden_member_prefixes = [\"cad/workspaces/\"]\nboundary_check_manifest = \"tools/boundary-check/Cargo.toml\"\nboundary_check_config = \"tools/boundary-check/config/road1.toml\"\n", self.workspace));
        write(
            root,
            "cad/docs/worthy-foundations/NAMING.md",
            "# fixture naming\n",
        );
    }

    fn write_machine_authority(&self, root: &Path) {
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
        write(
            root,
            "tools/boundary-check/config/road1.toml",
            &machine_authority_config(self.band, self.facade),
        );
    }

    fn write_governed_consumer(&self, root: &Path) {
        write(root, &format!("{}/Cargo.toml", self.workspace), &format!("[workspace]\nresolver = \"2\"\nmembers = [\"crates/{}\"]\n\n[workspace.metadata.worth_topology]\nrole = \"road1_subworkspace\"\nconstitutional_lane = \"worth-{}\"\nmember_lane = \"crates/*\"\nallowed_crate_prefixes = [\"worth-{}-\"]\n", self.package, self.band, self.band));
        write(
            root,
            &format!("{}/README.md", self.workspace),
            "# fixture\n",
        );
        write(
            root,
            &format!("{}/AGENT_CONTEXT.md", self.crate_root),
            "# fixture\n",
        );
        write(
            root,
            &format!("{}/src/lib.rs", self.crate_root),
            "pub mod facade;\nmod legal_control;\n",
        );
        write(
            root,
            &format!("{}/src/legal_control.rs", self.crate_root),
            "pub struct LegalControl;\n",
        );
        write(
            root,
            &format!("{}/src/facade.rs", self.crate_root),
            "pub use crate::legal_control::LegalControl;\n",
        );
        write(
            root,
            &format!("{}/Cargo.toml", self.crate_root),
            &format!(
                "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
                self.package
            ),
        );
    }

    fn write_query_facade_provider(&self, root: &Path) {
        write(
        root,
        "crates/worth-query/Cargo.toml",
        "[package]\nname = \"worth-query\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[workspace]\n",
    );
        write(
            root,
            "crates/worth-query/src/lib.rs",
            &format!("pub struct {};\n", self.item),
        );
        write(root, &format!("crates/{}/Cargo.toml", self.facade), &format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[dependencies]\nworth-query = {{ path = \"../worth-query\" }}\n[workspace]\n", self.facade));
        write(
            root,
            &format!("crates/{}/src/lib.rs", self.facade),
            "pub mod facade;\n",
        );
        write(
            root,
            &format!("crates/{}/src/facade.rs", self.facade),
            &format!("pub use worth_query::{};\n", self.item),
        );
    }
}

pub fn install_hostile(
    root: &Path,
    corpus: &Corpus,
    specimen: &Specimen,
    band: &str,
    facade: &str,
) {
    let crate_root = format!("cad/workspaces/worth-{band}/crates/worth-{band}-adoption");
    let source = fs::read_to_string(corpus.specimen_path(specimen)).expect("read specimen");
    write(
        root,
        &format!("{crate_root}/src/lib.rs"),
        &format!("pub mod facade;\nmod legal_control;\n{source}\n"),
    );
    write(root, &format!("{crate_root}/Cargo.toml"), &format!("[package]\nname = \"worth-{band}-adoption\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n{facade} = {{ path = \"../../../../../crates/{facade}\" }}\n"));
}

fn machine_authority_config(band: &str, facade: &str) -> String {
    format!(
        r#"root_manifest = "Cargo.toml"
forbidden_root_prefixes = ["cad/workspaces/"]
seed_skeletons = []
[machine_authority]
canonical_config = "tools/boundary-check/config/road1.toml"
mirrored_docs = ["cad/docs/worthy-foundations/NAMING.md"]
[naming]
bands = ["{band}", "entry", "cert"]
[[naming.reserved_domains]]
tier = "worth"
band = "{band}"
domains = ["adoption"]
[rule_contracts]
replay_surfaces = []
[rule_contracts.query_audience]
engine_package = "worth-query"
[[rule_contracts.query_audience.audiences]]
package = "{facade}"
label = "hostile facade pair"
allowed_bands = ["entry", "cert"]
guidance = "entry or cert only"
[[rule_contracts.band_rules]]
source_band = "{band}"
allowed_target_bands = []
[[born_crates]]
path = "cad/workspaces/worth-{band}/crates/worth-{band}-adoption"
package = "worth-{band}-adoption"
[[subworkspaces]]
path = "cad/workspaces/worth-{band}"
allowed_crate_prefixes = ["worth-{band}-"]
member_lane = "crates/*"
[[law_substrates]]
package = "worth-proof"
tiers = ["worth", "worthy"]
bands = ["{band}", "entry", "cert"]
[legacy_reference_ratchet]
governed_roots = []
forbidden_fragments = []
snapshot = "tools/boundary-check/snapshots/legacy-references.toml"
exclude_paths = []
replacement_guidance = "use worth spelling"
"#
    )
}
