use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::{
    export_resolution, external_resolution,
    facade_exports::module_exports,
    private_imports, record_declaration,
    source_layout::{ModuleGraph, SourceModule},
    source_owner, CanonicalType, ExportedSurface, FacadeFamily,
};

pub(super) struct ExportCollection<'a> {
    pub(super) root: &'a Path,
    pub(super) source_root: &'a Path,
    pub(super) family: &'a FacadeFamily,
    pub(super) graph: &'a ModuleGraph,
    pub(super) exported_types: &'a mut BTreeMap<CanonicalType, BTreeSet<String>>,
    pub(super) delivered: &'a mut BTreeSet<(String, String)>,
}

impl ExportCollection<'_> {
    pub(super) fn collect(
        &mut self,
        module: &SourceModule,
        namespace: &[String],
        exports: Vec<ExportedSurface>,
    ) -> Result<(), String> {
        for export in exports {
            external_resolution::reject_crate_namespace_alias(&export)?;
            private_imports::reject_private_import_alias(module, self.graph, &export)?;
            if export.glob {
                for (name, declaration) in
                    export_resolution::resolve_glob(module, self.graph, self.source_root, &export)?
                {
                    self.record(qualified(namespace, &name), declaration)?;
                }
                continue;
            }
            let declarations =
                export_resolution::resolve_export(module, self.graph, self.source_root, &export)?;
            let name = qualified(namespace, &export.export_name);
            if !declarations.is_empty() {
                for declaration in declarations {
                    self.record(name.clone(), declaration)?;
                }
                continue;
            }
            let externals = external_resolution::resolve(self.root, &export)?;
            if externals.is_empty() {
                self.delivered
                    .insert((name, format!("external/{}", export.prefix.join("/"))));
                continue;
            }
            for external in externals {
                reject_external_module_alias(&external.declaration)?;
                record_declaration(
                    name.clone(),
                    external.declaration,
                    &self.root.join(external.family.source_root),
                    external.family,
                    self.exported_types,
                    self.delivered,
                )?;
            }
        }
        Ok(())
    }

    pub(super) fn collect_public_namespaces(&mut self) -> Result<(), String> {
        let modules = self
            .graph
            .modules()
            .filter(|module| module.publicly_reachable && !module.logical.is_empty())
            .cloned()
            .collect::<Vec<_>>();
        for module in modules {
            let namespace = module.logical.join("::");
            let owner = source_owner(&module.path, self.source_root, self.family)?;
            self.delivered.insert((namespace, owner));
            self.collect(
                &module,
                &module.logical,
                module_exports(self.graph, &module)?,
            )?;
        }
        Ok(())
    }

    fn record(
        &mut self,
        name: String,
        declaration: export_resolution::ResolvedDeclaration,
    ) -> Result<(), String> {
        if declaration.is_module {
            return Err(format!(
                "unsupported module re-export alias {name}; namespace projection is not provable"
            ));
        }
        record_declaration(
            name,
            declaration,
            self.source_root,
            self.family,
            self.exported_types,
            self.delivered,
        )
    }
}

fn reject_external_module_alias(
    declaration: &export_resolution::ResolvedDeclaration,
) -> Result<(), String> {
    if declaration.is_module {
        return Err("unsupported external module re-export alias; cross-family namespace projection is not provable".to_owned());
    }
    Ok(())
}

fn qualified(namespace: &[String], name: &str) -> String {
    if namespace.is_empty() {
        name.to_owned()
    } else {
        format!("{}::{name}", namespace.join("::"))
    }
}
