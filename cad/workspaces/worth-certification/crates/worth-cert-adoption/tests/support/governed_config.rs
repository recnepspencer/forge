pub const CONFIG: &str = r#"root_manifest = "Cargo.toml"
forbidden_root_prefixes = ["cad/workspaces/"]
seed_skeletons = []
[machine_authority]
canonical_config = "tools/boundary-check/config/road1.toml"
mirrored_docs = ["cad/docs/worthy-foundations/NAMING.md"]
[naming]
bands = ["entry", "cert"]
[[naming.reserved_domains]]
tier = "worth"
band = "entry"
domains = ["adoption"]
[rule_contracts]
[rule_contracts.query_audience]
engine_package = "worth-query"
[[rule_contracts.query_audience.audiences]]
package = "worth-query-replay"
label = "replay"
allowed_bands = ["cert"]
guidance = "cert-only reconstruction and replay"
[[rule_contracts.replay_surfaces]]
label = "certification replay"
package_prefixes = ["worth-query-replay"]
cert_domains = ["replay"]
[[rule_contracts.band_rules]]
source_band = "entry"
allowed_target_bands = []
[[born_crates]]
path = "cad/workspaces/worth-entry/crates/worth-entry-adoption"
package = "worth-entry-adoption"
[[subworkspaces]]
path = "cad/workspaces/worth-entry"
allowed_crate_prefixes = ["worth-entry-"]
member_lane = "crates/*"
[[law_substrates]]
package = "worth-proof"
tiers = ["worth", "worthy"]
bands = ["entry", "cert"]
[legacy_reference_ratchet]
governed_roots = []
forbidden_fragments = []
snapshot = "tools/boundary-check/snapshots/legacy-references.toml"
exclude_paths = []
replacement_guidance = "use worth spelling"
"#;
