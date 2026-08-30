mod syntax;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use quote::ToTokens;
use sha2::{Digest, Sha256};

use syntax::{references, validate_macros};

const WORLD_PREFIX: &[&str] = &["tests", "domains", "fintech", "world"];
const ORACLE_PREFIX: &[&str] = &[
    "tests",
    "domains",
    "fintech",
    "certification",
    "invalidation",
    "locality_expectation",
];
pub(super) const SAFE_MACROS: &[&str] = &[
    "assert",
    "assert_eq",
    "debug_assert_eq",
    "format",
    "matches",
    "panic",
    "vec",
];

pub(super) struct PureClosure {
    pub(super) files: BTreeMap<PathBuf, String>,
    pub(super) owners: BTreeSet<String>,
}

pub(super) struct PureGraphPolicy<'a> {
    pub(super) allowed: &'a [&'a str],
    pub(super) reexports: &'a BTreeMap<String, WorldReexport>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum WorldReexport {
    Unconditional(String),
    Conditional,
}

impl WorldReexport {
    pub(super) fn unconditional_origin(&self) -> Option<&str> {
        match self {
            Self::Unconditional(origin) => Some(origin),
            Self::Conditional => None,
        }
    }
}

pub(super) fn module_closure(
    root: &Path,
    root_source: Option<&str>,
) -> Result<BTreeMap<PathBuf, String>, String> {
    let mut pending = vec![(root.to_path_buf(), root_source.map(str::to_owned))];
    let mut closure = BTreeMap::new();
    while let Some((path, override_source)) = pending.pop() {
        if closure.contains_key(&path) {
            continue;
        }
        let source = override_source.unwrap_or_else(|| read(&path));
        let file = syn::parse_file(&source)
            .map_err(|error| format!("{} no longer parses: {error}", path.display()))?;
        for item in &file.items {
            let syn::Item::Mod(module) = item else {
                continue;
            };
            if cfg_test_only(&module.attrs) {
                continue;
            }
            if module.content.is_some() || !module.attrs.is_empty() {
                return Err(format!(
                    "{} contains inline, attributed, or generated module {}",
                    path.display(),
                    module.ident
                ));
            }
            pending.push((resolve_module(&path, &module.ident.to_string())?, None));
        }
        validate_macros(&path, &file)?;
        closure.insert(path, source);
    }
    Ok(closure)
}

pub(super) fn validate_oracle_imports(
    manifest: &Path,
    files: &BTreeMap<PathBuf, String>,
    reexports: &BTreeMap<String, WorldReexport>,
) -> Result<BTreeSet<String>, String> {
    let mut owners = BTreeSet::new();
    for (path, source) in files {
        for reference in references(manifest, path, source)? {
            if starts_with_segments(&reference, ORACLE_PREFIX) {
                continue;
            }
            let Some(owner) = world_owner(&reference, reexports)? else {
                return Err(format!(
                    "oracle source {} reaches runtime path {}",
                    path.display(),
                    reference.join("::")
                ));
            };
            owners.insert(owner);
        }
    }
    Ok(owners)
}

pub(super) fn resolved_pure_closure(
    manifest: &Path,
    facade_path: &str,
    roots: &BTreeSet<String>,
    policy: PureGraphPolicy<'_>,
) -> Result<PureClosure, String> {
    let mut owners = roots.clone();
    let facade = manifest.join(facade_path);
    let mut files = BTreeMap::from([(facade.clone(), read(&facade))]);
    loop {
        let mut discovered = owners.clone();
        for owner in &owners {
            ensure_allowed(owner, policy.allowed)?;
            let root = manifest
                .join("src/tests/domains/fintech/world")
                .join(format!("{owner}.rs"));
            for (path, source) in module_closure(&root, None)? {
                for reference in references(manifest, &path, &source)? {
                    if let Some(owner) = world_owner(&reference, policy.reexports)? {
                        ensure_allowed(&owner, policy.allowed)?;
                        discovered.insert(owner);
                    } else if reference.first().is_some_and(|segment| segment == "tests") {
                        return Err(format!(
                            "pure source {} reaches non-world path {}",
                            path.display(),
                            reference.join("::")
                        ));
                    }
                }
                files.insert(path, source);
            }
        }
        if discovered == owners {
            break;
        }
        owners = discovered;
    }
    Ok(PureClosure { files, owners })
}

pub(super) fn validate_pure_source_mutation(
    manifest: &Path,
    path: &Path,
    source: &str,
    policy: PureGraphPolicy<'_>,
) -> Result<(), String> {
    let file = syn::parse_file(source).map_err(|error| error.to_string())?;
    validate_macros(path, &file)?;
    for reference in references(manifest, path, source)? {
        if let Some(owner) = world_owner(&reference, policy.reexports)? {
            ensure_allowed(&owner, policy.allowed)?;
        }
    }
    Ok(())
}

pub(super) fn visible_reexports(source: &str) -> Result<BTreeMap<String, WorldReexport>, String> {
    let file = syn::parse_file(source).map_err(|error| error.to_string())?;
    let mut exports = BTreeMap::new();
    for item in file.items {
        let syn::Item::Use(item) = item else {
            continue;
        };
        let mut item_exports = BTreeMap::new();
        collect_reexports(&item.tree, &mut Vec::new(), &mut item_exports)?;
        for (symbol, origin) in item_exports {
            let reexport = if item.attrs.is_empty() {
                WorldReexport::Unconditional(origin)
            } else {
                WorldReexport::Conditional
            };
            insert_world_reexport(&mut exports, symbol, reexport)?;
        }
    }
    Ok(exports)
}

pub(super) fn closure_digest(manifest: &Path, closure: &BTreeMap<PathBuf, String>) -> String {
    let mut hasher = Sha256::new();
    for (path, source) in closure {
        let relative = path.strip_prefix(manifest).unwrap_or(path);
        hasher.update(relative.to_string_lossy().replace('\\', "/").as_bytes());
        let file = syn::parse_file(source).expect("sealed dependency source must parse");
        hasher.update(file.to_token_stream().to_string().as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

fn world_owner(
    reference: &[String],
    reexports: &BTreeMap<String, WorldReexport>,
) -> Result<Option<String>, String> {
    if !starts_with_segments(reference, WORLD_PREFIX) {
        return Ok(None);
    }
    let Some(first) = reference.get(WORLD_PREFIX.len()) else {
        return Ok(None);
    };
    let world = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/tests/domains/fintech/world");
    if world.join(format!("{first}.rs")).is_file() || world.join(first).join("mod.rs").is_file() {
        return Ok(Some(first.clone()));
    }
    let reexport = reexports
        .get(first)
        .ok_or_else(|| format!("world facade does not resolve symbol {first}"))?;
    let Some(origin) = reexport.unconditional_origin() else {
        return Err(format!(
            "conditional world symbol {first} is not oracle authority"
        ));
    };
    if origin.starts_with("crate::") {
        return Err(format!(
            "world symbol {first} aliases runtime authority {origin}"
        ));
    }
    Ok(Some(
        origin
            .split("::")
            .next()
            .expect("reexport origin must have an owner")
            .to_owned(),
    ))
}

fn ensure_allowed(owner: &str, allowed: &[&str]) -> Result<(), String> {
    if allowed.contains(&owner) {
        Ok(())
    } else {
        Err(format!("non-meaning financial owner {owner}"))
    }
}

fn resolve_module(parent: &Path, name: &str) -> Result<PathBuf, String> {
    let directory = if parent.file_name().is_some_and(|file| file == "mod.rs") {
        parent.parent().expect("mod.rs parent").to_path_buf()
    } else {
        parent
            .parent()
            .expect("module parent")
            .join(parent.file_stem().expect("module stem"))
    };
    for candidate in [
        directory.join(format!("{name}.rs")),
        directory.join(name).join("mod.rs"),
    ] {
        if candidate.is_file() {
            return Ok(candidate);
        }
    }
    Err(format!(
        "module {name} declared by {} has no source",
        parent.display()
    ))
}

fn cfg_test_only(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        attribute.path().is_ident("cfg")
            && attribute.meta.to_token_stream().to_string() == "cfg (test)"
    })
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn collect_reexports(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    exports: &mut BTreeMap<String, String>,
) -> Result<(), String> {
    match tree {
        syn::UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            collect_reexports(&path.tree, prefix, exports)?;
            prefix.pop();
        }
        syn::UseTree::Name(name) => insert_export(
            exports,
            name.ident.to_string(),
            joined_origin(prefix, &name.ident.to_string()),
        )?,
        syn::UseTree::Rename(rename) => insert_export(
            exports,
            rename.rename.to_string(),
            joined_origin(prefix, &rename.ident.to_string()),
        )?,
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_reexports(item, prefix, exports)?;
            }
        }
        syn::UseTree::Glob(_) => return Err("world facade glob reexport is opaque".to_owned()),
    }
    Ok(())
}

fn joined_origin(prefix: &[String], leaf: &str) -> String {
    prefix
        .iter()
        .cloned()
        .chain([leaf.to_owned()])
        .collect::<Vec<_>>()
        .join("::")
}

fn insert_export(
    exports: &mut BTreeMap<String, String>,
    symbol: String,
    origin: String,
) -> Result<(), String> {
    if exports.insert(symbol.clone(), origin).is_some() {
        return Err(format!("duplicate world reexport {symbol}"));
    }
    Ok(())
}

fn insert_world_reexport(
    exports: &mut BTreeMap<String, WorldReexport>,
    symbol: String,
    reexport: WorldReexport,
) -> Result<(), String> {
    if exports.insert(symbol.clone(), reexport).is_some() {
        return Err(format!("duplicate world reexport {symbol}"));
    }
    Ok(())
}

fn starts_with_segments(reference: &[String], expected: &[&str]) -> bool {
    reference.len() >= expected.len()
        && reference
            .iter()
            .zip(expected)
            .all(|(actual, expected)| actual == expected)
}
