//! Admit only exact compiler-owned attributes whose expansion cannot add value definitions.

use super::super::super::crate_modules::ModuleGraph;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::visit::Visit;
use syn::{Attribute, ItemMod, Path, Token};

const INERT_ATTRIBUTES: &[&str] = &[
    "allow",
    "doc",
    "macro_export",
    "must_use",
    "non_exhaustive",
    "path",
    "repr",
];
const BUILTIN_DERIVES: &[&str] = &[
    "Clone",
    "Copy",
    "Debug",
    "Default",
    "Eq",
    "Hash",
    "Ord",
    "PartialEq",
    "PartialOrd",
];

pub(super) fn verify(graph: &ModuleGraph) -> Result<(), String> {
    for (module_path, node) in &graph.modules {
        let mut verifier = AttributeVerifier {
            graph,
            module_path,
            error: None,
        };
        for item in &node.items {
            verifier.visit_item(item);
            if let Some(error) = verifier.error.take() {
                return Err(error);
            }
        }
    }
    Ok(())
}

struct AttributeVerifier<'graph> {
    graph: &'graph ModuleGraph,
    module_path: &'graph [String],
    error: Option<String>,
}

impl<'ast> Visit<'ast> for AttributeVerifier<'_> {
    fn visit_attribute(&mut self, attribute: &'ast Attribute) {
        if self.error.is_some() {
            return;
        }
        self.error = verify_attribute(self.graph, self.module_path, attribute).err();
    }

    fn visit_item_mod(&mut self, item: &'ast ItemMod) {
        for attribute in &item.attrs {
            self.visit_attribute(attribute);
        }
    }
}

fn verify_attribute(
    graph: &ModuleGraph,
    module_path: &[String],
    attribute: &Attribute,
) -> Result<(), String> {
    let path = path_text(attribute.path());
    if attribute.path().is_ident("derive") {
        return verify_derives(graph, module_path, attribute);
    }
    if attribute.path().segments.len() == 1
        && INERT_ATTRIBUTES.iter().any(|allowed| path == *allowed)
    {
        return Ok(());
    }
    Err(format!(
        "attribute `#[{path}]` is not an exact compiler-owned inert attribute; proc-macro expansion could add public values"
    ))
}

fn verify_derives(
    graph: &ModuleGraph,
    module_path: &[String],
    attribute: &Attribute,
) -> Result<(), String> {
    let derives = Punctuated::<Path, Token![,]>::parse_terminated
        .parse2(
            attribute
                .meta
                .require_list()
                .map_err(|error| format!("parse compiler derive attribute: {error}"))?
                .tokens
                .clone(),
        )
        .map_err(|error| format!("parse compiler derive list: {error}"))?;
    if derives.is_empty() {
        return Err("empty derive attribute cannot prove compiler-owned expansion".to_owned());
    }
    for derive in derives {
        let name = path_text(&derive);
        if derive.segments.len() != 1 || !BUILTIN_DERIVES.iter().any(|builtin| name == *builtin) {
            return Err(format!(
                "derive `{name}` is not an exact compiler built-in; proc-macro expansion could add public values"
            ));
        }
        super::derive_bindings::require_unbound(graph, module_path, &name)?;
    }
    Ok(())
}

fn path_text(path: &Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
