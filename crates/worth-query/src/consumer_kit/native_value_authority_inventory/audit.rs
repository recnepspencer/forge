use syn::visit::{self, Visit};

use super::{
    worth_query_native_value_authority_rows, WorthQueryNativeValueAuthorityAudit,
    WorthQueryNativeValueFinding, WorthQueryNativeValueFindingKind, WorthQueryNativeValueSource,
    WorthQueryNativeValueSourceSite,
};

pub fn audit_native_value_authority_sources(
    sources: &[WorthQueryNativeValueSource],
) -> WorthQueryNativeValueAuthorityAudit {
    let mut observed = Vec::new();
    let mut findings = Vec::new();

    for source in sources {
        match syn::parse_file(source.text()) {
            Ok(file) => {
                let mut visitor = NativeValueAuthorityVisitor::new(source.path());
                visitor.visit_file(&file);
                observed.extend(visitor.sites);
            }
            Err(error) => findings.push(WorthQueryNativeValueFinding::new(
                WorthQueryNativeValueFindingKind::InvalidRustSource,
                WorthQueryNativeValueSourceSite::new(
                    source.path(),
                    error.span().start().line,
                    "<invalid-rust-source>",
                ),
            )),
        }
    }

    let rows = worth_query_native_value_authority_rows();
    for site in &observed {
        if !rows.iter().any(|row| row_matches_site(row, site)) {
            findings.push(WorthQueryNativeValueFinding::new(
                WorthQueryNativeValueFindingKind::UnclassifiedAuthority,
                site.clone(),
            ));
        }
    }
    for row in rows {
        require_site(
            row.defining_path(),
            row.symbol(),
            WorthQueryNativeValueFindingKind::MissingClassifiedAuthority,
            &observed,
            &mut findings,
        );
        for exporting_path in row.exporting_paths() {
            require_site(
                exporting_path,
                row.symbol(),
                WorthQueryNativeValueFindingKind::MissingFacadeExport,
                &observed,
                &mut findings,
            );
        }
    }

    findings.sort();
    WorthQueryNativeValueAuthorityAudit::new(observed.len(), findings)
}

fn require_site(
    path: &str,
    symbol: &str,
    kind: WorthQueryNativeValueFindingKind,
    observed: &[WorthQueryNativeValueSourceSite],
    findings: &mut Vec<WorthQueryNativeValueFinding>,
) {
    if observed
        .iter()
        .any(|site| site.path() == path && site.symbol() == symbol)
    {
        return;
    }
    findings.push(WorthQueryNativeValueFinding::new(
        kind,
        WorthQueryNativeValueSourceSite::new(path, 0, symbol),
    ));
}

fn row_matches_site(
    row: &super::WorthQueryNativeValueAuthorityRow,
    site: &WorthQueryNativeValueSourceSite,
) -> bool {
    row.symbol() == site.symbol()
        && (row.defining_path() == site.path() || row.exporting_paths().contains(&site.path()))
}

struct NativeValueAuthorityVisitor<'a> {
    path: &'a str,
    current_impl: Option<String>,
    sites: Vec<WorthQueryNativeValueSourceSite>,
}

impl<'a> NativeValueAuthorityVisitor<'a> {
    fn new(path: &'a str) -> Self {
        Self {
            path,
            current_impl: None,
            sites: Vec::new(),
        }
    }

    fn record(&mut self, span: proc_macro2::Span, symbol: impl Into<String>) {
        self.sites.push(WorthQueryNativeValueSourceSite::new(
            self.path,
            span.start().line,
            symbol,
        ));
    }
}

impl<'ast> Visit<'ast> for NativeValueAuthorityVisitor<'_> {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if item.attrs.iter().any(is_test_cfg) {
            return;
        }
        visit::visit_item_mod(self, item);
    }

    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        if is_public(&item.vis) && is_candidate_type(&item.ident.to_string(), item) {
            self.record(item.ident.span(), item.ident.to_string());
        }
        visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
        if is_public(&item.vis) && is_candidate_type(&item.ident.to_string(), item) {
            self.record(item.ident.span(), item.ident.to_string());
        }
        visit::visit_item_enum(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        if is_candidate_function(item) {
            self.record(item.sig.ident.span(), item.sig.ident.to_string());
        }
        visit::visit_item_fn(self, item);
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        let prior = self.current_impl.replace(type_name(&item.self_ty));
        visit::visit_item_impl(self, item);
        self.current_impl = prior;
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        let owner = self
            .current_impl
            .clone()
            .unwrap_or_else(|| "<unknown>".to_string());
        if is_public(&item.vis) && is_proof_bypass_constructor(&owner, item) {
            self.record(
                item.sig.ident.span(),
                format!("{owner}::{}", item.sig.ident),
            );
        }
        if is_debug_native_identity_impl(item) {
            self.record(
                item.sig.ident.span(),
                format!("{owner}::{}", item.sig.ident),
            );
        }
        visit::visit_impl_item_fn(self, item);
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        if is_facade_path(self.path) && is_public(&item.vis) {
            collect_use_names(&item.tree, &mut |ident| {
                if is_registered_symbol(&ident.to_string())
                    || is_suspicious_name(&ident.to_string())
                {
                    self.record(ident.span(), ident.to_string());
                }
            });
        }
        visit::visit_item_use(self, item);
    }
}

fn is_candidate_type(name: &str, item: impl quote::ToTokens) -> bool {
    if is_registered_symbol(name) || is_suspicious_name(name) {
        return true;
    }
    let tokens = item.into_token_stream().to_string();
    mentions_native_value_authority(&tokens)
}

fn is_candidate_function(item: &syn::ItemFn) -> bool {
    let name = item.sig.ident.to_string();
    if is_registered_symbol(&name) {
        return true;
    }
    let body = quote::ToTokens::to_token_stream(&item.block).to_string();
    let output = quote::ToTokens::to_token_stream(&item.sig.output).to_string();
    let encodes_native_value = contains_identifier(&body, "AspectValue")
        && (output.contains("String") || name.contains("digest") || name.contains("identity"));
    let scalar_only_bridge = contains_identifier(&body, "ContractValidatedAspectValueView")
        && body.contains(":: Scalar")
        && body.contains(":: Struct");
    encodes_native_value || scalar_only_bridge || is_debug_native_identity(&item.sig, &body)
}

fn is_debug_native_identity_impl(item: &syn::ImplItemFn) -> bool {
    is_debug_native_identity(
        &item.sig,
        &quote::ToTokens::to_token_stream(&item.block).to_string(),
    )
}

fn is_debug_native_identity(signature: &syn::Signature, body: &str) -> bool {
    let name = signature.ident.to_string();
    let identity_role = ["digest", "identity", "basis", "encode", "canonical"]
        .iter()
        .any(|role| name.contains(role));
    let native_input = signature.inputs.iter().any(|input| {
        let syn::FnArg::Typed(input) = input else {
            return false;
        };
        let input = quote::ToTokens::to_token_stream(&input.ty).to_string();
        contains_identifier(&input, "AspectValue")
            || contains_identifier(&input, "StructAspectValue")
    });
    identity_role && native_input && (body.contains(":?") || body.contains(":#?"))
}

fn mentions_native_value_authority(tokens: &str) -> bool {
    [
        "AspectValue",
        "StructAspectValue",
        "ContractValidatedAspectValue",
        "WorthQueryPredicateOperand",
        "WorthQueryUnrefinedLiveShape",
    ]
    .iter()
    .any(|needle| contains_identifier(tokens, needle))
}

fn contains_identifier(tokens: &str, expected: &str) -> bool {
    tokens
        .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
        .any(|token| token == expected)
}

fn is_registered_symbol(symbol: &str) -> bool {
    worth_query_native_value_authority_rows()
        .iter()
        .any(|row| row.symbol() == symbol)
}

fn is_suspicious_name(name: &str) -> bool {
    name.contains("PredicateValue")
        || name.contains("NativeAspectValue")
        || name.contains("AspectValueCarrier")
        || name.contains("AspectFieldKind")
        || name.ends_with("NativeRow")
}

fn is_proof_bypass_constructor(owner: &str, item: &syn::ImplItemFn) -> bool {
    (owner.contains("Admitted") || owner.contains("Validated"))
        && item.sig.ident.to_string().starts_with("from_")
        && item.sig.inputs.iter().any(|input| {
            let syn::FnArg::Typed(input) = input else {
                return false;
            };
            contains_identifier(
                &quote::ToTokens::to_token_stream(&input.ty).to_string(),
                "AspectValue",
            )
        })
}

fn type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(path) => path.path.segments.last().map_or_else(
            || "<unknown>".to_string(),
            |segment| segment.ident.to_string(),
        ),
        _ => "<unknown>".to_string(),
    }
}

fn is_facade_path(path: &str) -> bool {
    path == "src/facade.rs" || path.starts_with("src/facade/")
}

fn is_public(visibility: &syn::Visibility) -> bool {
    matches!(visibility, syn::Visibility::Public(_))
}

fn is_test_cfg(attribute: &syn::Attribute) -> bool {
    attribute.path().is_ident("cfg")
        && matches!(&attribute.meta, syn::Meta::List(list) if list.tokens.to_string() == "test")
}

fn collect_use_names(tree: &syn::UseTree, record: &mut impl FnMut(&syn::Ident)) {
    match tree {
        syn::UseTree::Name(name) => record(&name.ident),
        syn::UseTree::Rename(rename) => record(&rename.rename),
        syn::UseTree::Path(path) => collect_use_names(&path.tree, record),
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_use_names(item, record);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}
