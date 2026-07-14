use syn::visit::Visit;

use super::WorthQueryDeclarativeSurfaceSourceSite;

pub(super) fn public_phase_surface_sites(
    source_path: &str,
    source_text: &str,
) -> Result<Vec<WorthQueryDeclarativeSurfaceSourceSite>, syn::Error> {
    let syntax = syn::parse_file(source_text)?;
    let mut collector = PublicPhaseSurfaceCollector::new(source_path);
    collector.visit_file(&syntax);
    Ok(collector.sites)
}

struct PublicPhaseSurfaceCollector<'a> {
    source_path: &'a str,
    sites: Vec<WorthQueryDeclarativeSurfaceSourceSite>,
    owner_stack: Vec<String>,
}

impl<'a> PublicPhaseSurfaceCollector<'a> {
    fn new(source_path: &'a str) -> Self {
        Self {
            source_path,
            sites: Vec::new(),
            owner_stack: Vec::new(),
        }
    }

    fn record(&mut self, signature: &syn::Signature) {
        let function_name = signature.ident.to_string();
        if is_phase_surface(&function_name) {
            let start = signature.ident.span().start();
            let site = match self.owner_stack.last() {
                Some(owner) => WorthQueryDeclarativeSurfaceSourceSite::method(
                    self.source_path,
                    start.line,
                    owner,
                    &function_name,
                ),
                None => WorthQueryDeclarativeSurfaceSourceSite::new(
                    self.source_path,
                    start.line,
                    &function_name,
                ),
            };
            self.sites.push(site);
        }
    }
}

impl<'ast> Visit<'ast> for PublicPhaseSurfaceCollector<'_> {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        if is_public(&node.vis) {
            self.record(&node.sig);
        }
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if is_public(&node.vis) {
            self.record(&node.sig);
        }
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        let owner = type_name(&node.self_ty).unwrap_or_else(|| "impl".to_string());
        self.owner_stack.push(owner);
        syn::visit::visit_item_impl(self, node);
        self.owner_stack.pop();
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        if is_public(&node.vis) {
            self.owner_stack.push(node.ident.to_string());
            syn::visit::visit_item_trait(self, node);
            self.owner_stack.pop();
        }
    }

    fn visit_trait_item_fn(&mut self, node: &'ast syn::TraitItemFn) {
        if !self.owner_stack.is_empty() {
            self.record(&node.sig);
        }
    }

    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        if is_public(&node.vis) {
            let mut exports = Vec::new();
            collect_public_exports(&node.tree, &mut exports);
            for (exported_name, span) in exports {
                if exported_name == "*" || is_phase_surface(&exported_name) {
                    let start = span.start();
                    self.sites.push(WorthQueryDeclarativeSurfaceSourceSite::new(
                        self.source_path,
                        start.line,
                        &exported_name,
                    ));
                }
            }
        }
    }
}

fn is_public(visibility: &syn::Visibility) -> bool {
    matches!(visibility, syn::Visibility::Public(_))
}

fn type_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

fn collect_public_exports(tree: &syn::UseTree, exports: &mut Vec<(String, proc_macro2::Span)>) {
    match tree {
        syn::UseTree::Path(path) => collect_public_exports(&path.tree, exports),
        syn::UseTree::Name(name) => exports.push((name.ident.to_string(), name.ident.span())),
        syn::UseTree::Rename(rename) => {
            exports.push((rename.rename.to_string(), rename.rename.span()));
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_public_exports(item, exports);
            }
        }
        syn::UseTree::Glob(glob) => exports.push(("*".to_string(), glob.star_token.span)),
    }
}

fn is_phase_surface(function_name: &str) -> bool {
    if (function_name.ends_with("_count") && !function_name.starts_with("declare_"))
        || function_name.ends_with("_digest")
        || function_name.ends_with("_identity")
        || function_name.ends_with("_kind")
        || function_name.ends_with("_report")
    {
        return false;
    }
    matches!(
        function_name,
        "declare"
            | "run"
            | "open"
            | "close"
            | "using"
            | "current"
            | "under_policy_tenant"
            | "with_relationship_proofs"
            | "install_program"
            | "materialization_metadata_from_resolved"
            | "scoped_observation_basis_for_preview_binding"
    ) || [
        "compose_",
        "declare_",
        "bind_",
        "canonicalize_",
        "validate_",
        "resolve_",
        "lower_",
        "define_",
        "execute_",
        "explain_",
        "admit_",
        "plan_",
        "inspect",
        "orchestrate_",
    ]
    .iter()
    .any(|prefix| function_name.starts_with(prefix))
}
