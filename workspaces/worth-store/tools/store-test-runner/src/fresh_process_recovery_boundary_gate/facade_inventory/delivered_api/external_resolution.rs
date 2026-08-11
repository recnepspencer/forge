use std::path::Path;

use super::{
    export_resolution, facade_exports, source_layout::ModuleGraph, ExportedSurface, FacadeFamily,
    FAMILIES,
};

pub(super) struct ExternalResolution {
    pub(super) declaration: export_resolution::ResolvedDeclaration,
    pub(super) family: &'static FacadeFamily,
}

pub(super) fn resolve(
    root: &Path,
    export: &ExportedSurface,
) -> Result<Vec<ExternalResolution>, String> {
    let Some(external_family) = family(export) else {
        return Ok(Vec::new());
    };
    let facade = root.join(external_family.facade);
    let source_root = root.join(external_family.source_root);
    let graph = ModuleGraph::build(&source_root)?;
    let facade_module = graph.module_for_file(&facade).ok_or_else(|| {
        format!(
            "external facade {} is not production reachable",
            facade.display()
        )
    })?;
    for candidate in facade_exports(&facade)? {
        if candidate.export_name != export.source_name || candidate.glob {
            continue;
        }
        let declarations =
            export_resolution::resolve_export(facade_module, &graph, &source_root, &candidate)?;
        if !declarations.is_empty() {
            return Ok(declarations
                .into_iter()
                .map(|declaration| ExternalResolution {
                    declaration,
                    family: external_family,
                })
                .collect());
        }
        let resolutions = resolve(root, &candidate)?;
        if !resolutions.is_empty() {
            return Ok(resolutions);
        }
    }
    let direct_modules = export_resolution::declarations_named_in_module(
        facade_module,
        &graph,
        &export.source_name,
    )?;
    if !direct_modules.is_empty() {
        return Ok(direct_modules
            .into_iter()
            .map(|declaration| ExternalResolution {
                declaration,
                family: external_family,
            })
            .collect());
    }
    Ok(Vec::new())
}

pub(super) fn reject_crate_namespace_alias(export: &ExportedSurface) -> Result<(), String> {
    if export.prefix.is_empty() && family_name(&export.source_name).is_some() {
        return Err(format!(
            "unsupported contractual dependency crate namespace re-export {}; namespace projection is not provable",
            export.source_name
        ));
    }
    Ok(())
}

fn family(export: &ExportedSurface) -> Option<&'static FacadeFamily> {
    if export.prefix.len() != 1 {
        return None;
    }
    family_name(export.prefix.first()?.as_str())
}

fn family_name(name: &str) -> Option<&'static FacadeFamily> {
    match name {
        "worth_store_physical_backend" => Some(&FAMILIES[1]),
        "worth_store" => Some(&FAMILIES[2]),
        "worth_store_physical_format" => Some(&FAMILIES[3]),
        "worth_store_wal" => Some(&FAMILIES[4]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_dependency_contractual_facade_has_external_provenance() {
        let root = surface(&["worth_store_physical_backend"]);
        assert!(family(&root).is_some());

        let bypass = surface(&["worth_store_physical_backend", "decoy"]);
        assert!(family(&bypass).is_none());
    }

    fn surface(prefix: &[&str]) -> ExportedSurface {
        ExportedSurface {
            prefix: prefix.iter().map(|part| (*part).to_owned()).collect(),
            source_name: "Twin".to_owned(),
            export_name: "Twin".to_owned(),
            direct: false,
            glob: false,
        }
    }
}
