//! Road-1 configuration rendering for temporary authority-sealing repositories.

use super::repository::AuthoritySealingTestRepository;

impl AuthoritySealingTestRepository {
    pub fn minimal_config(&self) -> String {
        self.config_with_law_substrates(
            r#"
[[law_substrates]]
package = "worth-proof"
tiers = ["worth", "worthy"]
bands = ["schema", "entry", "derived", "cert", "pack"]
"#,
        )
    }

    /// Build a full road1.toml with a custom `[[law_substrates]]` body fragment.
    pub fn config_with_law_substrates(&self, law_substrates_toml: &str) -> String {
        format!(
            r#"root_manifest = "Cargo.toml"
forbidden_root_prefixes = ["cad/workspaces/"]
seed_skeletons = []

[machine_authority]
canonical_config = "tools/boundary-check/config/road1.toml"
mirrored_docs = ["cad/docs/worthy-foundations/NAMING.md"]

[naming]
bands = ["schema", "entry", "derived", "cert", "pack"]

[[naming.reserved_domains]]
tier = "worth"
band = "entry"
domains = ["adoption"]

[rule_contracts]

[rule_contracts.public_value_reachability]
package = "public-value-fixture"
crate_root = "vendor/public-value-fixture"
witness_source = "tools/boundary-check/public_value_witnesses/public_value_fixture/mod.rs"
worlds = [{{ name = "host-dev-default", target = "host", profile = "dev", default_features = true, features = [] }}]
host_timeout_ms = 30000
compilation_timeout_ms = 30000
max_output_bytes = 65536
guidance = "Expose a checked public introduction site."

[rule_contracts.query_audience]
engine_package = "worth-query"

[[rule_contracts.query_audience.audiences]]
package = "worth-query-decl"
label = "declaration"
allowed_bands = ["entry", "cert"]
guidance = "declaration artifacts and handles"

[[rule_contracts.query_audience.audiences]]
package = "worth-query-host"
label = "host"
allowed_bands = ["entry", "cert"]
guidance = "admission, lowering, and execution"

[[rule_contracts.query_audience.audiences]]
package = "worth-query-replay"
label = "replay"
allowed_bands = ["cert"]
guidance = "cert-only reconstruction and replay"

[[rule_contracts.replay_surfaces]]
label = "certification replay"
package_prefixes = ["worth-cert-replay", "worthy-cert-replay"]
cert_domains = ["replay", "reconstruction"]

[[rule_contracts.band_rules]]
source_band = "entry"
allowed_target_bands = ["schema", "resolver", "derived"]

[[born_crates]]
path = "cad/workspaces/worth-entry/crates/worth-entry-adoption"
package = "worth-entry-adoption"

[[subworkspaces]]
path = "cad/workspaces/worth-entry"
allowed_crate_prefixes = ["worth-entry-"]
member_lane = "crates/*"
{law_substrates_toml}
[legacy_reference_ratchet]
governed_roots = []
forbidden_fragments = []
snapshot = "tools/boundary-check/snapshots/legacy-references.toml"
exclude_paths = []
replacement_guidance = "Use the corresponding worth_/worth- spelling instead of the retired name."
"#
        )
    }
}
