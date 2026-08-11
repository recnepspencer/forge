use std::collections::{BTreeMap, BTreeSet};

use super::super::documents::{read_repository_document, split_csv, API_INVENTORY};
use super::CutoverRow;
use crate::workspace_root;

pub(super) fn assert_direct_consumer_contract(row: &CutoverRow) {
    if !matches!(
        row.responsibility.as_str(),
        "direct-recovery-physics-import-cutover" | "deleted-recovery-evidence-consumer"
    ) {
        return;
    }
    let package = row.path.split('/').nth(1).expect("crate or tool package");
    if row.disposition != "delete" {
        assert_eq!(row.destination_owner, direct_consumer_owner(package));
    }
    let dispositions = api_dispositions().expect("read C.8 API dispositions");
    let imported = imported_physics_surfaces(&row.path).expect("parse recovery-physics imports");
    for surface in &imported {
        assert!(
            dispositions.contains_key(surface),
            "unknown recovery-physics surface `{surface}` in {}",
            row.path
        );
    }
    let known = imported
        .iter()
        .filter_map(|surface| dispositions.get(surface))
        .map(String::as_str)
        .collect::<Vec<_>>();
    if !known.is_empty() && known.iter().all(|disposition| *disposition == "delete") {
        assert_eq!(
            row.disposition, "delete",
            "deleted-only consumer {} must be deleted",
            row.path
        );
        assert_eq!(row.destination_owner, "none");
        assert_ne!(row.deletion_phase, "preserve");
    }
}

fn direct_consumer_owner(package: &str) -> &'static str {
    match package {
        "store-test-runner" => "store-test-runner/c8-boundary-gate",
        "worth-store" => "worth-store/recovery-construction",
        "worth-store-aspect-native" => "worth-store-aspect-native/canonical-recovery-basis",
        "worth-store-blob-chunks" => "worth-store-blob-chunks/recovery-records",
        "worth-store-certification" | "worth-store-physical-certification" => {
            "worth-store-physical-certification/c8-fresh-process-recovery"
        }
        "worth-store-contracts" => "worth-store-contracts/durable-artifact-classification",
        "worth-store-formal-models" => "worth-store-formal-models/recovery-protocols",
        "worth-store-layout-indexes" => "worth-store-layout-indexes/recovery-layout-admission",
        "worth-store-lsm-authority" => "worth-store-lsm-authority/replay-source",
        "worth-store-offline-verifier" => "worth-store-offline-verifier/c8-recovery-observation",
        "worth-store-operations" => "worth-store-operations/recovery-workflows",
        "worth-store-physical-isolation" => "worth-store-physical-isolation/recovery-interlocks",
        "worth-store-replication" => "worth-store-replication/recovery-admission",
        "worth-store-recovery-runtime" => "worth-store-recovery-runtime/orchestration",
        "worth-store-test-support" => "worth-store-test-support/c8-recovery-fixtures",
        other => panic!("unowned C.8 direct consumer package `{other}`"),
    }
}

fn api_dispositions() -> Result<BTreeMap<String, String>, String> {
    let document = read_repository_document(API_INVENTORY)?;
    document
        .lines()
        .skip(1)
        .filter(|line| line.starts_with("current,") || line.starts_with("current-certification,"))
        .map(|line| {
            let columns = split_csv(line, 6)?;
            Ok((columns[1].to_owned(), columns[3].to_owned()))
        })
        .collect()
}

pub(super) fn imported_physics_surfaces(path: &str) -> Result<BTreeSet<String>, String> {
    if !path.ends_with(".rs") {
        return Ok(BTreeSet::new());
    }
    let source = std::fs::read_to_string(workspace_root().join(path))
        .map_err(|error| format!("cannot read {path}: {error}"))?;
    physics_surfaces(&source, path)
}
fn physics_surfaces(source: &str, label: &str) -> Result<BTreeSet<String>, String> {
    let file = syn::parse_file(source).map_err(|error| format!("cannot parse {label}: {error}"))?;
    let aliases = physics_aliases(&file);
    let mut collector = PhysicsReferenceCollector::new(aliases);
    syn::visit::Visit::visit_file(&mut collector, &file);
    collector.finish()
}

#[test]
fn syntax_references_reject_globs_comments_and_alias_bypasses() {
    let comment = "// worth_store_recovery_physics::Unknown\nfn local() {}";
    assert!(physics_surfaces(comment, "comment mutant")
        .expect("parse comment mutant")
        .is_empty());
    assert!(physics_surfaces("use worth_store_recovery_physics::*;", "glob mutant").is_err());
    let qualified = "fn local() { let _ = worth_store_recovery_physics::Unknown::value(); }";
    assert_eq!(
        physics_surfaces(qualified, "qualified mutant").expect("parse qualified mutant"),
        BTreeSet::from(["Unknown".to_owned()])
    );
    let alias = "use worth_store_recovery_physics as physics; fn local() { let _ = physics::RecoveryRedoPlan; }";
    assert_eq!(
        physics_surfaces(alias, "alias mutant").expect("parse alias mutant"),
        BTreeSet::from(["RecoveryRedoPlan".to_owned()])
    );
    let local_use = "fn local() { use worth_store_recovery_physics::RecoveryRedoPlan; let _ = RecoveryRedoPlan; }";
    assert_eq!(
        physics_surfaces(local_use, "block use mutant").expect("parse block use mutant"),
        BTreeSet::from(["RecoveryRedoPlan".to_owned()])
    );
    let unknown =
        "mod nested { use worth_store_recovery_physics as p; fn local() { let _ = p::Unknown; } }";
    assert_eq!(
        physics_surfaces(unknown, "nested alias mutant").expect("parse nested alias mutant"),
        BTreeSet::from(["Unknown".to_owned()])
    );
    let grouped = "use worth_store_recovery_physics::{self as physics, RecoveryRedoPlan}; fn local() { let _ = physics::RecoveryRedoPlan; }";
    assert_eq!(
        physics_surfaces(grouped, "grouped alias mutant").expect("parse grouped alias mutant"),
        BTreeSet::from(["RecoveryRedoPlan".to_owned()])
    );
}

fn physics_aliases(file: &syn::File) -> BTreeSet<String> {
    let mut collector = PhysicsAliasCollector::default();
    syn::visit::Visit::visit_file(&mut collector, file);
    let mut aliases = BTreeSet::from(["worth_store_recovery_physics".to_owned()]);
    loop {
        let before = aliases.len();
        for (source, alias) in &collector.renames {
            if aliases.contains(source) {
                aliases.insert(alias.clone());
            }
        }
        if aliases.len() == before {
            return aliases;
        }
    }
}

#[derive(Default)]
struct PhysicsAliasCollector {
    renames: Vec<(String, String)>,
}

impl<'ast> syn::visit::Visit<'ast> for PhysicsAliasCollector {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        collect_use_renames(&item.tree, &mut Vec::new(), &mut self.renames);
        syn::visit::visit_item_use(self, item);
    }

    fn visit_item_extern_crate(&mut self, item: &'ast syn::ItemExternCrate) {
        if let Some((_, alias)) = &item.rename {
            self.renames
                .push((item.ident.to_string(), alias.to_string()));
        }
    }
}

fn collect_use_renames(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    renames: &mut Vec<(String, String)>,
) {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_use_renames(&path.tree, prefix, renames);
            prefix.pop();
        }
        syn::UseTree::Rename(rename) => {
            let source = if rename.ident == "self" {
                prefix.last().cloned()
            } else {
                Some(rename.ident.to_string())
            };
            if let Some(source) = source {
                renames.push((source, rename.rename.to_string()));
            }
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_renames(item, prefix, renames);
            }
        }
        syn::UseTree::Name(_) | syn::UseTree::Glob(_) => {}
    }
}

struct PhysicsReferenceCollector {
    aliases: BTreeSet<String>,
    surfaces: BTreeSet<String>,
    error: Option<String>,
}

impl PhysicsReferenceCollector {
    fn new(aliases: BTreeSet<String>) -> Self {
        Self {
            aliases,
            surfaces: BTreeSet::new(),
            error: None,
        }
    }

    fn finish(self) -> Result<BTreeSet<String>, String> {
        self.error.map_or(Ok(self.surfaces), Err)
    }

    fn collect_use(&mut self, tree: &syn::UseTree) {
        let syn::UseTree::Path(root) = tree else {
            return;
        };
        if self.aliases.contains(&root.ident.to_string()) {
            if let Err(error) = collect_import_names(&root.tree, &mut self.surfaces) {
                self.error = Some(error);
            }
        }
    }
}

impl<'ast> syn::visit::Visit<'ast> for PhysicsReferenceCollector {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        self.collect_use(&item.tree);
        syn::visit::visit_item_use(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path
            .segments
            .first()
            .is_some_and(|segment| self.aliases.contains(&segment.ident.to_string()))
        {
            if let Some(surface) = path.segments.iter().nth(1) {
                self.surfaces.insert(surface.ident.to_string());
            }
        }
        syn::visit::visit_path(self, path);
    }
}

fn collect_import_names(tree: &syn::UseTree, names: &mut BTreeSet<String>) -> Result<(), String> {
    match tree {
        syn::UseTree::Path(path) => {
            names.insert(path.ident.to_string());
        }
        syn::UseTree::Name(name) => {
            if name.ident != "self" {
                names.insert(name.ident.to_string());
            }
        }
        syn::UseTree::Rename(rename) => {
            if rename.ident != "self" {
                names.insert(rename.ident.to_string());
            }
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_import_names(item, names)?;
            }
        }
        syn::UseTree::Glob(_) => {
            return Err("glob recovery-physics import cannot be reconciled".into())
        }
    }
    Ok(())
}
