use super::crate_modules::{is_public_visibility, ModuleGraph};
use super::module_path_resolution::expand_resolved_use_tree;
use super::public_reachability::{
    item_is_public_declaration, item_name, Reachability, ReachableItemKey,
};
use std::collections::{BTreeSet, VecDeque};
use syn::Item;

pub(super) fn promote_item(
    graph: &ModuleGraph,
    target_module: &[String],
    target_name: &str,
    reachability: &mut Reachability,
    public_modules: &mut BTreeSet<Vec<String>>,
    queue: &mut VecDeque<Vec<String>>,
) {
    let mut visited = BTreeSet::new();
    let mut context = PromotionContext {
        graph,
        reachability,
        public_modules,
        queue,
        visited: &mut visited,
    };
    context.promote(target_module, target_name);
}

struct PromotionContext<'a> {
    graph: &'a ModuleGraph,
    reachability: &'a mut Reachability,
    public_modules: &'a mut BTreeSet<Vec<String>>,
    queue: &'a mut VecDeque<Vec<String>>,
    visited: &'a mut BTreeSet<(Vec<String>, String)>,
}

impl PromotionContext<'_> {
    fn promote(&mut self, target_module: &[String], target_name: &str) {
        if !self
            .visited
            .insert((target_module.to_vec(), target_name.to_owned()))
        {
            return;
        }
        let Some(node) = self.graph.modules.get(target_module) else {
            return;
        };
        for item in &node.items {
            self.promote_matching_item(target_module, target_name, item);
        }
    }

    fn promote_matching_item(&mut self, module: &[String], name: &str, item: &Item) {
        match item {
            Item::Mod(item_mod)
                if item_mod.ident == name && is_public_visibility(&item_mod.vis) =>
            {
                let mut child = module.to_vec();
                child.push(name.to_owned());
                if self.public_modules.insert(child.clone()) {
                    self.queue.push_back(child);
                }
            }
            other
                if item_name(other).as_deref() == Some(name)
                    && item_is_public_declaration(other) =>
            {
                self.reachability.items.insert(ReachableItemKey {
                    module_path: module.to_vec(),
                    item_name: name.to_owned(),
                });
            }
            Item::Use(item_use) if is_public_visibility(&item_use.vis) => {
                for (target, target_name, export_name) in
                    expand_resolved_use_tree(self.graph, module, &item_use.tree)
                {
                    if export_name == name && target_name != "*" {
                        self.promote(&target, &target_name);
                    } else if export_name == "*" && target_name == "*" {
                        self.promote(&target, name);
                    }
                }
            }
            _ => {}
        }
    }
}
