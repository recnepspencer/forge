use std::collections::BTreeMap;

use syn::visit::{self, Visit};

use super::model::{
    WorthQueryDomainAuthorityFinding, WorthQueryDomainAuthorityFindingKind,
    WorthQueryDomainAuthorityInventoryAudit, WorthQueryDomainAuthoritySource,
    WorthQueryDomainAuthoritySourceSite,
};
use super::registry::worth_query_domain_authority_inventory_rows;

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
        for exporting_path in row.exporting_paths() {
            if *exporting_path != row.defining_path() {
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
        && (row.defining_path() == site.path() || row.exporting_paths().contains(&site.path()))
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
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if item.attrs.iter().any(is_test_cfg) {
            return;
        }
        visit::visit_item_mod(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let name = item.sig.ident.to_string();
        if is_public(&item.vis)
            && ((is_effective_domain_facade_source(self.path)
                && is_candidate_function(&name, &item.sig.output))
                || is_raw_domain_authority_function(&name, &item.sig.output))
        {
            self.record(item.sig.ident.span().start().line, name);
        }
        visit::visit_item_fn(self, item);
    }

    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        let name = item.ident.to_string();
        if (is_public(&item.vis) && is_candidate_type(&name))
            || name.ends_with("OperationRegistry")
            || name.contains("DomainAdapter")
        {
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
        if is_public(&item.vis) && is_candidate_method(owner, &name) {
            self.record(
                item.sig.ident.span().start().line,
                format!("{owner}::{name}"),
            );
        }
        visit::visit_impl_item_fn(self, item);
    }

    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        if is_effective_facade_source(self.path) && is_public(&item.vis) {
            collect_candidate_use_names(&item.tree, self.path, &mut |ident| {
                self.record(ident.span().start().line, ident.to_string());
            });
        }
        visit::visit_item_use(self, item);
    }
}

fn is_test_cfg(attribute: &syn::Attribute) -> bool {
    attribute.path().is_ident("cfg")
        && matches!(&attribute.meta, syn::Meta::List(list) if list.tokens.to_string() == "test")
}

fn is_public(visibility: &syn::Visibility) -> bool {
    matches!(visibility, syn::Visibility::Public(_))
}

fn is_candidate_type(name: &str) -> bool {
    matches!(
        name,
        "WorthQueryDomainIdentityNamespace"
            | "WorthQueryDomainIdentityName"
            | "WorthQueryDomainSemanticVersion"
            | "WorthQueryDomainIdentityDeclaration"
            | "WorthQueryDomainInvariantDefinition"
            | "WorthQueryDomainGraphObligationDefinition"
            | "WorthQueryDomainGraphReadOperationDefinition"
            | "WorthQueryDomainDeclarationFamilyDefinition"
            | "WorthQueryDomainPackage"
            | "WorthQueryInstalledDomainHandle"
            | "WorthQueryInstalledDomainContributionSurface"
    ) || name.ends_with("OperationRegistry")
        || name.contains("DomainAdapter")
}

fn is_candidate_function(name: &str, output: &syn::ReturnType) -> bool {
    name.starts_with("materialize_")
        || name == "worth_query_domain"
        || name.contains("with_operation_registry")
        || return_type_mentions(output, "WorthQueryDomainContributionSurface")
}

fn is_candidate_method(owner: &str, name: &str) -> bool {
    is_package_input_method(owner, name)
        || (owner == "WorthQueryRuntimeBuilder" && name == "domain_package")
        || ((owner == "WorthQueryRuntime" || owner == "WorthQueryWorkspace") && name == "domain")
        || (owner == "WorthQueryInstalledDomainHandle"
            && matches!(
                name,
                "contributions"
                    | "contributions_in"
                    | "authority_witness"
                    | "rebind_request"
                    | "graph_read_operation"
                    | "declarations"
                    | "declarations_in"
            ))
        || (owner == "WorthQueryInstalledDomainContributionSurface"
            && matches!(
                name,
                "intent_target"
                    | "for_intent"
                    | "for_intent_target"
                    | "admitted_plan_target"
                    | "for_admitted_intent_plan"
                    | "for_admitted_plan_target"
                    | "lower_runtime_target"
                    | "for_lower_runtime_boundary_envelope"
                    | "for_lower_runtime_target"
                    | "for_lower_runtime_boundary_source"
            ))
        || (owner == "WorthQueryRuntimeBuilder"
            && (name.contains("invariant") || name.contains("graph_obligation")))
        || (owner == "WorthQueryRuntimeBuilder"
            && matches!(
                name,
                "session_graph_participation_provider"
                    | "decision_graph_participation_provider"
                    | "provisional_graph_participation_provider"
            ))
        || (owner == "WorthQueryGraphReadOperationRegistry"
            && matches!(
                name,
                "empty" | "admit" | "admit_registration" | "with_unsupported_shape_for_operation"
            ))
}

fn is_effective_facade_source(path: &str) -> bool {
    path == "src/facade.rs" || path.starts_with("src/facade/")
}

fn is_effective_domain_facade_source(path: &str) -> bool {
    matches!(
        path,
        "src/facade/exports_domain.rs" | "src/facade/exports_domain_capabilities.rs"
    ) || path.starts_with("src/facade/domain_")
}

fn is_raw_domain_authority_function(name: &str, output: &syn::ReturnType) -> bool {
    name == "worth_query_domain"
        || return_type_mentions(output, "WorthQueryDomainContributionSurface")
}

fn is_package_input_method(owner: &str, name: &str) -> bool {
    matches!(
        (owner, name),
        (
            "WorthQueryDomainIdentityNamespace"
                | "WorthQueryDomainIdentityName"
                | "WorthQueryDomainSemanticVersion"
                | "WorthQueryDomainIdentityDeclaration"
                | "WorthQueryDomainInvariantDefinition",
            "new"
        ) | (
            "WorthQueryDomainInvariantPredicate",
            "requires_outgoing_relations"
        ) | (
            "WorthQueryDomainGraphObligationDefinition",
            "new" | "with_support_posture"
        ) | (
            "WorthQueryDomainGraphReadOperationDefinition",
            "new" | "accepts_relation" | "lowers_to" | "requires_support_family"
        ) | ("WorthQueryDomainDeclarationFamilyDefinition", "from_marker")
            | (
                "WorthQueryDomainPackage",
                "declare"
                    | "requires_capability"
                    | "requires_configuration"
                    | "requires_operating_posture"
                    | "invariant"
                    | "graph_obligation"
                    | "graph_read_operation"
                    | "declaration_family"
                    | "declaration_families"
                    | "permits_contribution"
            )
    )
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

fn collect_candidate_use_names(
    tree: &syn::UseTree,
    source_path: &str,
    record: &mut impl FnMut(&syn::Ident),
) {
    match tree {
        syn::UseTree::Name(name) if is_candidate_export(source_path, &name.ident.to_string()) => {
            record(&name.ident)
        }
        syn::UseTree::Rename(rename)
            if is_candidate_export(source_path, &rename.rename.to_string()) =>
        {
            record(&rename.rename)
        }
        syn::UseTree::Path(path) => collect_candidate_use_names(&path.tree, source_path, record),
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_candidate_use_names(item, source_path, record);
            }
        }
        _ => {}
    }
}

fn is_candidate_export(source_path: &str, name: &str) -> bool {
    is_candidate_type(name)
        || (is_effective_domain_facade_source(source_path) && name.starts_with("materialize_"))
        || name == "prepare_admitted_domain_capability_contribution_for_materialization"
        || name == "worth_query_domain"
        || name == "WorthQueryGraphReadOperationRegistry"
}
