//! Parse the one module body selected by each exact Cargo production world.

use super::crate_modules::{is_public_visibility, GovernedCrate, ModuleGraph, ModuleNode};
use super::module_source::{
    child_module_dir, directory_after_loading_file, path_attribute_dir,
    resolve_selected_child_source,
};
use super::production_world::ProductionWorld;
use crate::cargo_graph::normalize_path;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use syn::{Item, ItemMod};

pub(super) fn parse(
    governed: &GovernedCrate,
    world: &ProductionWorld,
) -> Result<ModuleGraph, String> {
    let source = super::library_target::resolve_lib_source_path(&governed.crate_root)?;
    let module_dir = source.parent().unwrap_or_else(|| Path::new("."));
    let mut parser = WorldModuleParser {
        crate_root: &governed.crate_root,
        world,
        modules: BTreeMap::new(),
    };
    parser.parse_file(&source, module_dir, Vec::new(), true)?;
    Ok(ModuleGraph {
        modules: parser.modules,
    })
}

struct WorldModuleParser<'a> {
    crate_root: &'a Path,
    world: &'a ProductionWorld,
    modules: BTreeMap<Vec<String>, ModuleNode>,
}

struct ParsedModuleBody<'a> {
    source: &'a Path,
    module_dir: &'a Path,
    module_path: Vec<String>,
    public_from_parent: bool,
    items: Vec<Item>,
}

impl WorldModuleParser<'_> {
    fn parse_file(
        &mut self,
        source: &Path,
        module_dir: &Path,
        module_path: Vec<String>,
        public_from_parent: bool,
    ) -> Result<(), String> {
        let text = fs::read_to_string(source)
            .map_err(|error| format!("read {}: {error}", source.display()))?;
        let file = syn::parse_file(&text)
            .map_err(|error| format!("parse {}: {error}", source.display()))?;
        self.parse_items(ParsedModuleBody {
            source,
            module_dir,
            module_path,
            public_from_parent,
            items: file.items,
        })
    }

    fn parse_items(&mut self, body: ParsedModuleBody<'_>) -> Result<(), String> {
        let ParsedModuleBody {
            source,
            module_dir,
            module_path,
            public_from_parent,
            items,
        } = body;
        self.insert_node(source, &module_path, public_from_parent, items.clone())?;
        let mut children = BTreeMap::<String, Vec<ItemMod>>::new();
        for item_mod in items.into_iter().filter_map(|item| match item {
            Item::Mod(item_mod) if self.world.includes(&item_mod.attrs) => Some(item_mod),
            _ => None,
        }) {
            children
                .entry(item_mod.ident.to_string())
                .or_default()
                .push(item_mod);
        }
        for (name, declarations) in children {
            if declarations.len() != 1 {
                return Err(format!(
                    "Cargo world `{}` selects multiple declarations for module `{}`",
                    self.world.name,
                    child_display(&module_path, &name)
                ));
            }
            self.load_child(
                source,
                module_dir,
                &module_path,
                declarations.into_iter().next().expect("one declaration"),
            )?;
        }
        Ok(())
    }

    fn insert_node(
        &mut self,
        source: &Path,
        module_path: &[String],
        public_from_parent: bool,
        items: Vec<Item>,
    ) -> Result<(), String> {
        let relative = normalize_path(
            source
                .strip_prefix(self.crate_root)
                .map_err(|error| format!("strip crate root from {}: {error}", source.display()))?,
        );
        let node = ModuleNode {
            relative_source: relative,
            public_from_parent,
            items,
        };
        if self.modules.insert(module_path.to_vec(), node).is_some() {
            return Err(format!(
                "one Cargo world selected multiple bodies for module `{}`",
                super::crate_modules::module_path_display(module_path)
            ));
        }
        Ok(())
    }

    fn load_child(
        &mut self,
        parent_source: &Path,
        parent_dir: &Path,
        parent_path: &[String],
        item_mod: ItemMod,
    ) -> Result<(), String> {
        let name = item_mod.ident.to_string();
        let mut child_path = parent_path.to_vec();
        child_path.push(name.clone());
        let public = is_public_visibility(&item_mod.vis);
        let child_dir = child_module_dir(parent_dir, &name);
        if let Some((_, items)) = item_mod.content {
            return self.parse_items(ParsedModuleBody {
                source: parent_source,
                module_dir: &child_dir,
                module_path: child_path,
                public_from_parent: public,
                items,
            });
        }
        let attrs = self.world.project_attributes(&item_mod.attrs);
        let path_dir = path_attribute_dir(parent_source, parent_dir);
        let (source, path_selected) =
            resolve_selected_child_source(parent_dir, &path_dir, &name, &attrs)?;
        let loaded_dir = if path_selected {
            source
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf()
        } else {
            directory_after_loading_file(&source)
        };
        self.parse_file(&source, &loaded_dir, child_path, public)
    }
}

fn child_display(parent: &[String], child: &str) -> String {
    if parent.is_empty() {
        child.to_owned()
    } else {
        format!("{}::{child}", parent.join("::"))
    }
}
