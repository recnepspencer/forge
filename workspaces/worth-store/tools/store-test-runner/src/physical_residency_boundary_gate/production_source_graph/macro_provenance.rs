use syn::{
    visit::{self, Visit},
    Item, ItemMacro, ItemUse, Macro, UseTree,
};

use super::super::constructor_syntax::identifier_is;

pub(super) fn macro_path_is_include(path: &syn::Path) -> bool {
    path.segments
        .last()
        .is_some_and(|segment| identifier_is(&segment.ident, "include"))
}

pub(super) fn use_renames_include(tree: &UseTree) -> bool {
    match tree {
        UseTree::Rename(rename) => identifier_is(&rename.ident, "include"),
        UseTree::Path(path) => use_renames_include(&path.tree),
        UseTree::Group(group) => group.items.iter().any(use_renames_include),
        UseTree::Glob(_) | UseTree::Name(_) => false,
    }
}

pub(super) fn unproved_source_expansion(tokens: &proc_macro2::TokenStream) -> Option<&'static str> {
    for token in tokens.clone() {
        match token {
            proc_macro2::TokenTree::Ident(ident) if ident == "mod" => {
                return Some("module");
            }
            proc_macro2::TokenTree::Ident(ident) if identifier_is(&ident, "include") => {
                return Some("include");
            }
            proc_macro2::TokenTree::Group(group) => {
                if let Some(expansion) = unproved_source_expansion(&group.stream()) {
                    return Some(expansion);
                }
            }
            proc_macro2::TokenTree::Ident(_)
            | proc_macro2::TokenTree::Punct(_)
            | proc_macro2::TokenTree::Literal(_) => {}
        }
    }
    None
}

pub(super) fn external_declarative_source_expansion(
    source: &str,
) -> Result<ExternalSourceEvidence, String> {
    let syntax = syn::parse_file(source)
        .map_err(|error| format!("cannot parse external macro source: {error}"))?;
    let mut visitor = ExternalMacroVisitor {
        denial: None,
        literal_includes: Vec::new(),
    };
    visitor.visit_file(&syntax);
    Ok(ExternalSourceEvidence {
        denial: visitor.denial,
        literal_includes: visitor.literal_includes,
    })
}

pub(super) struct ExternalSourceEvidence {
    pub(super) denial: Option<&'static str>,
    pub(super) literal_includes: Vec<String>,
}

struct ExternalMacroVisitor {
    denial: Option<&'static str>,
    literal_includes: Vec<String>,
}

impl<'ast> Visit<'ast> for ExternalMacroVisitor {
    fn visit_item(&mut self, node: &'ast Item) {
        if super::item_attributes(node)
            .is_some_and(|attrs| super::non_test_reachable(attrs).is_ok_and(|reachable| !reachable))
        {
            return;
        }
        visit::visit_item(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        if use_renames_include(&node.tree) {
            self.denial = Some("include macro alias");
            return;
        }
        visit::visit_item_use(self, node);
    }

    fn visit_item_macro(&mut self, node: &'ast ItemMacro) {
        if node.mac.path.is_ident("macro_rules") {
            self.denial = unproved_source_expansion(&node.mac.tokens).or(self.denial);
        }
        visit::visit_item_macro(self, node);
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        if macro_path_is_include(&node.path) {
            match syn::parse2::<syn::LitStr>(node.tokens.clone()) {
                Ok(path) => self.literal_includes.push(path.value()),
                Err(_) => {
                    self.denial = Some("computed external include");
                    return;
                }
            }
        }
        visit::visit_macro(self, node);
    }
}
