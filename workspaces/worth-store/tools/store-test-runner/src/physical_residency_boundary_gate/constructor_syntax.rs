use syn::{
    visit::{self, Visit},
    Attribute, Expr, ExprCall, ExprPath, ImplItemType, Item, ItemImpl, ItemType, ItemUse, Macro,
    TraitItemType, Type, TypePath, UseTree,
};

#[derive(Clone, Copy)]
pub(super) struct ConstructorSpec {
    pub(super) owner: &'static str,
    pub(super) method: &'static str,
}

pub(super) fn semantic_identifier(ident: &proc_macro2::Ident) -> String {
    let rendered = ident.to_string();
    rendered.strip_prefix("r#").unwrap_or(&rendered).to_owned()
}

pub(super) fn identifier_is(ident: &proc_macro2::Ident, expected: &str) -> bool {
    semantic_identifier(ident) == expected
}

pub(super) fn path_is_ident(path: &syn::Path, expected: &str) -> bool {
    path.segments.len() == 1 && identifier_is(&path.segments[0].ident, expected)
}

pub(super) fn constructor_calls(
    source: &str,
    specs: &[ConstructorSpec],
) -> Result<Vec<String>, String> {
    let syntax = syn::parse_file(source).or_else(|file_error| {
        syn::parse_file(&format!(
            "fn __controlled_constructor_site() {{ {source} }}"
        ))
        .map_err(|_| file_error)
    });
    let syntax = syntax.map_err(|error| format!("invalid Rust source: {error}"))?;
    let mut visitor = ConstructorVisitor {
        specs,
        calls: Vec::new(),
        denial: None,
        impl_owner: None,
    };
    visitor.visit_file(&syntax);
    match visitor.denial {
        Some(denial) => Err(denial),
        None => Ok(visitor.calls),
    }
}

struct ConstructorVisitor<'a> {
    specs: &'a [ConstructorSpec],
    calls: Vec<String>,
    denial: Option<String>,
    impl_owner: Option<String>,
}

impl ConstructorVisitor<'_> {
    fn governed_call(&self, path: &ExprPath) -> Option<ConstructorSpec> {
        let segments: Vec<_> = path.path.segments.iter().collect();
        let method = semantic_identifier(&segments.last()?.ident);
        let owner = if path.qself.is_some() {
            type_path_leaf(&path.qself.as_ref()?.ty)?
        } else if segments.len() >= 2 {
            semantic_identifier(&segments[segments.len() - 2].ident)
        } else {
            return None;
        };
        let owner = if owner == "Self" {
            self.impl_owner.as_deref()?
        } else {
            owner.as_str()
        };
        self.specs
            .iter()
            .copied()
            .find(|spec| spec.owner == owner && spec.method == method)
    }

    fn reject_aliases(&mut self, tree: &UseTree, prefix: &mut Vec<String>) {
        match tree {
            UseTree::Rename(rename) => {
                let source = if identifier_is(&rename.ident, "self") {
                    prefix
                        .last()
                        .cloned()
                        .unwrap_or_else(|| semantic_identifier(&rename.ident))
                } else {
                    semantic_identifier(&rename.ident)
                };
                let alias = semantic_identifier(&rename.rename);
                if source != alias
                    && self
                        .specs
                        .iter()
                        .any(|spec| spec.owner == source || spec.owner == alias)
                {
                    self.denial = Some(format!(
                        "governed constructor owner alias {source} as {alias} is not exact"
                    ));
                }
            }
            UseTree::Path(path) => {
                prefix.push(semantic_identifier(&path.ident));
                self.reject_aliases(&path.tree, prefix);
                prefix.pop();
            }
            UseTree::Group(group) => {
                for item in &group.items {
                    self.reject_aliases(item, prefix);
                }
            }
            UseTree::Name(_) | UseTree::Glob(_) => {}
        }
    }
}

impl<'ast> Visit<'ast> for ConstructorVisitor<'_> {
    fn visit_item(&mut self, node: &'ast Item) {
        if item_attributes(node).is_some_and(exact_cfg_test)
            || matches!(node, Item::Fn(function) if exact_test_function(&function.attrs))
        {
            return;
        }
        visit::visit_item(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(path) = node.func.as_ref() {
            if let Some(spec) = self.governed_call(path) {
                self.calls.push(format!("{}::{}", spec.owner, spec.method));
                for argument in &node.args {
                    self.visit_expr(argument);
                }
                return;
            }
            if path.qself.is_some()
                && path.path.segments.last().is_some_and(|segment| {
                    self.specs
                        .iter()
                        .any(|spec| identifier_is(&segment.ident, spec.method))
                })
            {
                self.denial = Some(format!(
                    "qualified constructor projection {} cannot prove its governed owner",
                    path.path.segments.last().expect("checked segment").ident
                ));
            }
        }
        visit::visit_expr_call(self, node);
    }

    fn visit_expr_path(&mut self, node: &'ast ExprPath) {
        if let Some(spec) = self.governed_call(node) {
            self.denial = Some(format!(
                "indirect governed constructor reference {}::{} is not exact",
                spec.owner, spec.method
            ));
        }
        visit::visit_expr_path(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let prior = self.impl_owner.take();
        self.impl_owner = type_path_leaf(&node.self_ty);
        visit::visit_item_impl(self, node);
        self.impl_owner = prior;
    }

    fn visit_item_type(&mut self, node: &'ast ItemType) {
        if governed_owner_in_type(&node.ty, self.specs).is_some() {
            self.denial = Some(format!(
                "governed constructor owner type alias {} is not exact",
                node.ident
            ));
        }
        visit::visit_item_type(self, node);
    }

    fn visit_impl_item_type(&mut self, node: &'ast ImplItemType) {
        if let Some(owner) = governed_owner_in_type(&node.ty, self.specs) {
            self.denial = Some(format!(
                "associated type {} cannot alias governed constructor owner {owner}",
                node.ident
            ));
        }
        visit::visit_impl_item_type(self, node);
    }

    fn visit_trait_item_type(&mut self, node: &'ast TraitItemType) {
        if node
            .default
            .as_ref()
            .and_then(|(_, ty)| governed_owner_in_type(ty, self.specs))
            .is_some()
        {
            self.denial = Some(format!(
                "associated type default {} cannot alias a governed constructor owner",
                node.ident
            ));
        }
        visit::visit_trait_item_type(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast ItemUse) {
        self.reject_aliases(&node.tree, &mut Vec::new());
        visit::visit_item_use(self, node);
    }

    fn visit_macro(&mut self, node: &'ast Macro) {
        if node
            .path
            .segments
            .last()
            .is_some_and(|segment| identifier_is(&segment.ident, "include"))
            && syn::parse2::<syn::LitStr>(node.tokens.clone()).is_err()
        {
            self.denial = Some(
                "generated or computed Rust include cannot prove constructor ownership".to_owned(),
            );
        }
        let identifiers: Vec<_> = node
            .tokens
            .clone()
            .into_iter()
            .flat_map(token_identifiers)
            .collect();
        if let Some(spec) = self
            .specs
            .iter()
            .find(|spec| identifiers.iter().any(|ident| ident == spec.owner))
        {
            self.denial = Some(format!(
                "macro-carried governed constructor owner {} is not exact",
                spec.owner
            ));
        }
        if self
            .impl_owner
            .as_deref()
            .is_some_and(|owner| self.specs.iter().any(|spec| spec.owner == owner))
        {
            self.denial = Some(
                "a macro inside a governed constructor-owner impl cannot prove expansion"
                    .to_owned(),
            );
        }
        visit::visit_macro(self, node);
    }
}

fn type_path_leaf(ty: &Type) -> Option<String> {
    let Type::Path(path) = ty else {
        return None;
    };
    path.path
        .segments
        .last()
        .map(|segment| semantic_identifier(&segment.ident))
}

fn governed_owner_in_type<'a>(ty: &Type, specs: &'a [ConstructorSpec]) -> Option<&'a str> {
    let mut visitor = GovernedTypeVisitor { specs, found: None };
    visitor.visit_type(ty);
    visitor.found
}

struct GovernedTypeVisitor<'a> {
    specs: &'a [ConstructorSpec],
    found: Option<&'a str>,
}

impl<'ast, 'a> Visit<'ast> for GovernedTypeVisitor<'a> {
    fn visit_type_path(&mut self, node: &'ast TypePath) {
        if let Some(spec) = self.specs.iter().find(|spec| {
            node.path
                .segments
                .iter()
                .any(|segment| identifier_is(&segment.ident, spec.owner))
        }) {
            self.found = Some(spec.owner);
        }
        visit::visit_type_path(self, node);
    }
}

fn token_identifiers(token: proc_macro2::TokenTree) -> Vec<String> {
    match token {
        proc_macro2::TokenTree::Ident(ident) => vec![semantic_identifier(&ident)],
        proc_macro2::TokenTree::Group(group) => group
            .stream()
            .into_iter()
            .flat_map(token_identifiers)
            .collect(),
        proc_macro2::TokenTree::Punct(_) | proc_macro2::TokenTree::Literal(_) => Vec::new(),
    }
}

fn exact_cfg_test(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        path_is_ident(attr.path(), "cfg")
            && attr
                .meta
                .require_list()
                .is_ok_and(|list| list.tokens.to_string() == "test")
    })
}

fn exact_test_function(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path()
            .segments
            .last()
            .is_some_and(|segment| identifier_is(&segment.ident, "test"))
    })
}

fn item_attributes(item: &Item) -> Option<&[Attribute]> {
    match item {
        Item::Const(item) => Some(&item.attrs),
        Item::Enum(item) => Some(&item.attrs),
        Item::ExternCrate(item) => Some(&item.attrs),
        Item::Fn(item) => Some(&item.attrs),
        Item::ForeignMod(item) => Some(&item.attrs),
        Item::Impl(item) => Some(&item.attrs),
        Item::Macro(item) => Some(&item.attrs),
        Item::Mod(item) => Some(&item.attrs),
        Item::Static(item) => Some(&item.attrs),
        Item::Struct(item) => Some(&item.attrs),
        Item::Trait(item) => Some(&item.attrs),
        Item::TraitAlias(item) => Some(&item.attrs),
        Item::Type(item) => Some(&item.attrs),
        Item::Union(item) => Some(&item.attrs),
        Item::Use(item) => Some(&item.attrs),
        Item::Verbatim(_) | _ => None,
    }
}
