use std::collections::BTreeMap;

use syn::visit::{self, Visit};

use super::model::{
    WorthQueryDomainAuthorityFinding, WorthQueryDomainAuthorityFindingKind,
    WorthQueryDomainAuthorityInventoryAudit, WorthQueryDomainAuthoritySource,
    WorthQueryDomainAuthoritySourceSite,
};
use super::registry::worth_query_domain_authority_inventory_rows;

pub fn current_domain_authority_inventory_audit() -> WorthQueryDomainAuthorityInventoryAudit {
    audit_domain_authority_sources(&current_sources())
}

pub fn audit_domain_authority_sources(
    sources: &[WorthQueryDomainAuthoritySource],
) -> WorthQueryDomainAuthorityInventoryAudit {
    let mut observed = Vec::new();
    let mut findings = Vec::new();

    for source in sources {
        match syn::parse_file(source.text()) {
            Ok(file) => {
                let mut visitor = DomainAuthorityVisitor::new(source.path());
                visitor.visit_file(&file);
                observed.extend(visitor.sites);
            }
            Err(error) => findings.push(WorthQueryDomainAuthorityFinding::new(
                WorthQueryDomainAuthorityFindingKind::InvalidRustSource,
                WorthQueryDomainAuthoritySourceSite::new(
                    source.path(),
                    error.span().start().line,
                    "<invalid-rust-source>",
                ),
            )),
        }
    }

    let rows = worth_query_domain_authority_inventory_rows();
    let mut observed_counts = BTreeMap::<(String, String), usize>::new();
    for site in &observed {
        *observed_counts
            .entry((site.path().to_string(), site.symbol().to_string()))
            .or_default() += 1;
        if is_physical_adapter_site(site) {
            continue;
        }
        if !rows.iter().any(|row| row_matches_site(row, site)) {
            findings.push(WorthQueryDomainAuthorityFinding::new(
                WorthQueryDomainAuthorityFindingKind::UnclassifiedSemanticAuthority,
                site.clone(),
            ));
        }
    }

    for ((path, symbol), count) in &observed_counts {
        if *count > 1 {
            findings.push(WorthQueryDomainAuthorityFinding::new(
                WorthQueryDomainAuthorityFindingKind::DuplicateClassifiedAuthority,
                WorthQueryDomainAuthoritySourceSite::new(path, 0, symbol),
            ));
        }
    }

    for row in rows {
        require_site(row.defining_path(), row.symbol(), &observed, &mut findings);
        if let Some(exporting_path) = row.exporting_path() {
            if exporting_path != row.defining_path() {
                require_site(exporting_path, row.symbol(), &observed, &mut findings);
            }
        }
    }

    findings.sort_by(|left, right| left.site().cmp(right.site()));
    WorthQueryDomainAuthorityInventoryAudit::new(observed.len(), findings)
}

fn require_site(
    path: &str,
    symbol: &str,
    observed: &[WorthQueryDomainAuthoritySourceSite],
    findings: &mut Vec<WorthQueryDomainAuthorityFinding>,
) {
    if observed
        .iter()
        .any(|site| site.path() == path && site.symbol() == symbol)
    {
        return;
    }
    findings.push(WorthQueryDomainAuthorityFinding::new(
        WorthQueryDomainAuthorityFindingKind::MissingClassifiedAuthority,
        WorthQueryDomainAuthoritySourceSite::new(path, 0, symbol),
    ));
}

fn row_matches_site(
    row: &super::WorthQueryDomainAuthorityInventoryRow,
    site: &WorthQueryDomainAuthoritySourceSite,
) -> bool {
    row.symbol() == site.symbol()
        && (row.defining_path() == site.path() || row.exporting_path() == Some(site.path()))
}

fn is_physical_adapter_site(site: &WorthQueryDomainAuthoritySourceSite) -> bool {
    site.symbol().contains("Adapter")
        && (site.path().contains("runtime/backend/adapter_contracts")
            || site.path().contains("physical_boundary"))
}

struct DomainAuthorityVisitor<'a> {
    path: &'a str,
    current_impl: Option<String>,
    sites: Vec<WorthQueryDomainAuthoritySourceSite>,
}

impl<'a> DomainAuthorityVisitor<'a> {
    fn new(path: &'a str) -> Self {
        Self {
            path,
            current_impl: None,
            sites: Vec::new(),
        }
    }

    fn record(&mut self, line: usize, symbol: impl Into<String>) {
        self.sites.push(WorthQueryDomainAuthoritySourceSite::new(
            self.path, line, symbol,
        ));
    }
}

impl<'ast> Visit<'ast> for DomainAuthorityVisitor<'_> {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        if is_public(&item.vis)
            && is_candidate_function(&item.sig.ident.to_string(), &item.sig.output)
        {
            self.record(
                item.sig.ident.span().start().line,
                item.sig.ident.to_string(),
            );
        }
        visit::visit_item_fn(self, item);
    }

    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        let name = item.ident.to_string();
        if is_public(&item.vis) && is_candidate_type(&name) {
            self.record(item.ident.span().start().line, name);
        }
        visit::visit_item_struct(self, item);
    }

    fn visit_item_trait(&mut self, item: &'ast syn::ItemTrait) {
        let name = item.ident.to_string();
        if is_public(&item.vis) && is_candidate_type(&name) {
            self.record(item.ident.span().start().line, name);
        }
        visit::visit_item_trait(self, item);
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        let prior = self.current_impl.replace(type_name(&item.self_ty));
        visit::visit_item_impl(self, item);
        self.current_impl = prior;
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        let name = item.sig.ident.to_string();
        let owner = self.current_impl.as_deref().unwrap_or("<unknown>");
        if is_public(&item.vis) && is_candidate_method(owner, &name, &item.sig.output) {
            self.record(
                item.sig.ident.span().start().line,
                format!("{owner}::{name}"),
            );
        }
        visit::visit_impl_item_fn(self, item);
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        if is_public(&item.vis) {
            collect_candidate_use_names(&item.tree, &mut |ident| {
                self.record(ident.span().start().line, ident.to_string());
            });
        }
        visit::visit_item_use(self, item);
    }
}

fn is_public(visibility: &syn::Visibility) -> bool {
    matches!(visibility, syn::Visibility::Public(_))
}

fn is_candidate_type(name: &str) -> bool {
    name.ends_with("OperationRegistry") || name.contains("DomainAdapter")
}

fn is_candidate_function(name: &str, output: &syn::ReturnType) -> bool {
    name.starts_with("materialize_")
        || name == "worth_query_domain"
        || name.contains("with_operation_registry")
        || return_type_mentions(output, "WorthQueryDomainContributionSurface")
}

fn is_candidate_method(owner: &str, name: &str, output: &syn::ReturnType) -> bool {
    (owner == "WorthQueryApplicationFacade"
        && (name == "domain"
            || name == "domain_checked"
            || name == "domain_proof_root"
            || name == "domain_entry_support_snapshot"))
        || (owner == "WorthQueryRuntimeBuilder"
            && (name.contains("invariant") || name.contains("graph_obligation")))
        || (owner == "WorthQueryGraphReadOperationRegistry"
            && matches!(
                name,
                "empty"
                    | "define"
                    | "admit"
                    | "with_registration"
                    | "admit_registration"
                    | "with_required_capability_for_relations"
                    | "with_unsupported_shape_for_relations"
                    | "with_unsupported_shape_for_operation"
                    | "registrations"
            ))
        || is_candidate_function(name, output)
}

fn return_type_mentions(output: &syn::ReturnType, expected: &str) -> bool {
    match output {
        syn::ReturnType::Default => false,
        syn::ReturnType::Type(_, ty) => type_mentions(ty, expected),
    }
}

fn type_mentions(ty: &syn::Type, expected: &str) -> bool {
    match ty {
        syn::Type::Path(path) => path.path.segments.iter().any(|segment| {
            segment.ident == expected
                || match &segment.arguments {
                    syn::PathArguments::AngleBracketed(arguments) => arguments.args.iter().any(|arg| {
                        matches!(arg, syn::GenericArgument::Type(inner) if type_mentions(inner, expected))
                    }),
                    _ => false,
                }
        }),
        syn::Type::Reference(reference) => type_mentions(&reference.elem, expected),
        syn::Type::Tuple(tuple) => tuple.elems.iter().any(|inner| type_mentions(inner, expected)),
        _ => false,
    }
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

fn collect_candidate_use_names(tree: &syn::UseTree, record: &mut impl FnMut(&syn::Ident)) {
    match tree {
        syn::UseTree::Name(name) if is_candidate_export(&name.ident.to_string()) => {
            record(&name.ident)
        }
        syn::UseTree::Rename(rename) if is_candidate_export(&rename.rename.to_string()) => {
            record(&rename.rename)
        }
        syn::UseTree::Path(path) => collect_candidate_use_names(&path.tree, record),
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_candidate_use_names(item, record);
            }
        }
        _ => {}
    }
}

fn is_candidate_export(name: &str) -> bool {
    name.starts_with("materialize_")
        || name == "prepare_admitted_domain_capability_contribution_for_materialization"
        || name == "worth_query_domain"
        || name == "WorthQueryGraphReadOperationRegistry"
}

macro_rules! source {
    ($path:literal, $include:literal) => {
        WorthQueryDomainAuthoritySource::new($path, include_str!($include))
    };
}

fn current_sources() -> Vec<WorthQueryDomainAuthoritySource> {
    vec![
        source!(
            "src/application/capability/facade.rs",
            "../../application/capability/facade.rs"
        ),
        source!("src/runtime/builder.rs", "../../runtime/builder.rs"),
        source!(
            "src/runtime/graph_read_access/operation_resolution/registry.rs",
            "../../runtime/graph_read_access/operation_resolution/registry.rs"
        ),
        source!(
            "src/runtime/graph_read_access/explanation_api.rs",
            "../../runtime/graph_read_access/explanation_api.rs"
        ),
        source!(
            "src/domain_capabilities/dx/common/root.rs",
            "../../domain_capabilities/dx/common/root.rs"
        ),
        source!(
            "src/runtime/backend/adapter_contracts.rs",
            "../../runtime/backend/adapter_contracts.rs"
        ),
        source!(
            "src/facade/exports_runtime_capabilities.rs",
            "../../facade/exports_runtime_capabilities.rs"
        ),
        source!(
            "src/facade/exports_runtime_core.rs",
            "../../facade/exports_runtime_core.rs"
        ),
    ]
}
