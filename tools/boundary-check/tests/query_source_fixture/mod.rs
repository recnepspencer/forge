//! Temporary governed repository for Query source-law production-binary proofs.

use crate::query_audience_fixture::{
    base_config, run_boundary_check, seed_snapshots, unique_temp_root, write_file,
    write_query_stubs, write_root_shell, write_subworkspace_crate,
};
use quote::ToTokens;
use std::fs;

pub struct SourceCase<'a> {
    pub label: &'a str,
    pub workspace: &'a str,
    pub lane: &'a str,
    pub prefix: &'a str,
    pub package: &'a str,
    pub tier: &'a str,
    pub band: &'a str,
    pub domain: &'a str,
    pub dependencies: &'a str,
    pub source: &'a str,
    pub additional_sources: &'a [(&'a str, &'a str)],
    pub manifest_suffix: &'a str,
}

pub fn run_source_case(case: SourceCase<'_>) -> (bool, String) {
    let root = unique_temp_root(case.label);
    let _ = fs::remove_dir_all(&root);
    write_query_stubs(&root);
    let workspace_path = format!("cad/workspaces/{}", case.workspace);
    write_subworkspace_crate(
        &root,
        &workspace_path,
        case.lane,
        case.prefix,
        case.package,
        case.package,
        case.dependencies,
    );
    if !case.manifest_suffix.is_empty() {
        let manifest_path = root
            .join(&workspace_path)
            .join("crates")
            .join(case.package)
            .join("Cargo.toml");
        let mut manifest = fs::read_to_string(&manifest_path).expect("read case manifest");
        manifest.push_str(case.manifest_suffix);
        fs::write(&manifest_path, manifest).expect("write case manifest suffix");
    }
    for (relative, source) in case.additional_sources {
        write_file(
            &root,
            &format!("{workspace_path}/crates/{}/{}", case.package, relative),
            source,
        );
    }
    write_root_shell(
        &root,
        &base_config(
            &format!(
                "[[subworkspaces]]\npath = \"{workspace_path}\"\nallowed_crate_prefixes = [\"{}\"]\nmember_lane = \"crates/*\"\n",
                case.prefix
            ),
            &format!(
                "[[born_crates]]\npath = \"{workspace_path}/crates/{}\"\npackage = \"{}\"\n",
                case.package, case.package
            ),
            &format!(
                "[[naming.reserved_domains]]\ntier = \"{}\"\nband = \"{}\"\ndomains = [\"{}\"]\n",
                case.tier, case.band, case.domain
            ),
            &format!(
                "[[rule_contracts.band_rules]]\nsource_band = \"{}\"\nallowed_target_bands = []\n",
                case.band
            ),
        ),
    );
    let crate_root = format!("{workspace_path}/crates/{}", case.package);
    apply_case(&root, &crate_root, case.source);
    if seed_snapshots(&root).is_err() {
        write_file(
            &root,
            &format!("{crate_root}/src/lib.rs"),
            "mod test_surface;\npub mod facade;\n",
        );
        write_file(
            &root,
            &format!("{crate_root}/src/test_surface.rs"),
            "pub fn seed() {}\n",
        );
        write_file(
            &root,
            &format!("{crate_root}/src/facade.rs"),
            "// Phase 6 fixture facade: intentionally no exports.\n",
        );
        if let Err(error) = seed_snapshots(&root) {
            let _ = fs::remove_dir_all(root);
            return (false, error);
        }
        apply_case(&root, &crate_root, case.source);
    }
    let result = run_boundary_check(&root);
    let _ = fs::remove_dir_all(root);
    result
}

fn apply_case(root: &std::path::Path, crate_root: &str, source: &str) {
    if source.contains("#[macro_export]") || source.contains("pub extern crate") {
        write_file(
            root,
            &format!("{crate_root}/src/lib.rs"),
            &format!("mod test_surface;\npub mod facade;\n{source}"),
        );
    } else {
        write_file(root, &format!("{crate_root}/src/test_surface.rs"), source);
        write_file(
            root,
            &format!("{crate_root}/src/facade.rs"),
            &facade_reexports(source),
        );
    }
}

fn facade_reexports(source: &str) -> String {
    let syntax = syn::parse_file(source).expect("parse Query fixture source");
    let local_modules = syntax
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Mod(value) => Some(value.ident.to_string()),
            _ => None,
        })
        .collect::<std::collections::BTreeSet<_>>();
    let mut facade = String::from("// Phase 6 projection of the intended public test surface.\n");
    for item in syntax.items {
        match item {
            syn::Item::Use(value) if matches!(value.vis, syn::Visibility::Public(_)) => {
                let root = use_root(&value.tree);
                if root
                    .as_ref()
                    .is_some_and(|name| local_modules.contains(name))
                    || matches!(root.as_deref(), Some("self" | "super" | "crate"))
                {
                    facade.push_str(&format!(
                        "pub use crate::test_surface::{};\n",
                        value.tree.to_token_stream()
                    ));
                } else {
                    facade.push_str(&format!("{}\n", value.to_token_stream()));
                }
            }
            item => {
                if let Some(name) = public_item_name(&item) {
                    facade.push_str(&format!("pub use crate::test_surface::{name};\n"));
                }
            }
        }
    }
    facade
}

fn use_root(tree: &syn::UseTree) -> Option<String> {
    match tree {
        syn::UseTree::Path(path) => Some(path.ident.to_string()),
        syn::UseTree::Name(name) => Some(name.ident.to_string()),
        syn::UseTree::Rename(rename) => Some(rename.ident.to_string()),
        syn::UseTree::Glob(_) | syn::UseTree::Group(_) => None,
    }
}

fn public_item_name(item: &syn::Item) -> Option<&syn::Ident> {
    match item {
        syn::Item::Const(value) if matches!(value.vis, syn::Visibility::Public(_)) => {
            Some(&value.ident)
        }
        syn::Item::Enum(value) if matches!(value.vis, syn::Visibility::Public(_)) => {
            Some(&value.ident)
        }
        syn::Item::Fn(value) if matches!(value.vis, syn::Visibility::Public(_)) => {
            Some(&value.sig.ident)
        }
        syn::Item::Static(value) if matches!(value.vis, syn::Visibility::Public(_)) => {
            Some(&value.ident)
        }
        syn::Item::Struct(value) if matches!(value.vis, syn::Visibility::Public(_)) => {
            Some(&value.ident)
        }
        syn::Item::Trait(value) if matches!(value.vis, syn::Visibility::Public(_)) => {
            Some(&value.ident)
        }
        syn::Item::Type(value) if matches!(value.vis, syn::Visibility::Public(_)) => {
            Some(&value.ident)
        }
        syn::Item::Union(value) if matches!(value.vis, syn::Visibility::Public(_)) => {
            Some(&value.ident)
        }
        _ => None,
    }
}

pub fn entry_case(label: &'static str, source: &'static str) -> SourceCase<'static> {
    SourceCase {
        label,
        workspace: "worth-entry",
        lane: "worth-entry",
        prefix: "worth-entry-",
        package: "worth-entry-adoption",
        tier: "worth",
        band: "entry",
        domain: "adoption",
        dependencies: r#"worth-query-decl = { path = "../../../../../vendor/worth-query-decl" }"#,
        source,
        additional_sources: &[],
        manifest_suffix: "",
    }
}
