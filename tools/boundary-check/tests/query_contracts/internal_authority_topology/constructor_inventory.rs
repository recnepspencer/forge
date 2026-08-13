//! Exact struct-literal construction inventory for central C7 authority values.

use std::collections::BTreeSet;
use std::path::Path;

use syn::visit::Visit;

use super::root;

const PROTECTED: &[&str] = &[
    "WorthQueryApplicationAttemptAffinity",
    "WorthQueryApplicationAttemptBasis",
    "WorthQueryExactCapabilityObservationContext",
    "WorthQueryPreparedApplicationCommit",
    "WorthQueryPreparedProviderSession",
    "WorthQueryProgressedApplicationCommit",
    "WorthQueryProvisionalOverlayCleanupBinding",
    "WorthQueryRunningApplicationCommit",
    "WorthQueryRegisteredProviderAttempt",
    "WorthQueryRegisteredProviderSession",
    "WorthQueryFreshProviderAttempt",
    "WorthQueryAuthorizedProviderCommit",
    "WorthQueryProviderPlanReadmission",
    "WorthQuerySessionBoundReadsAndEffects",
    "WorthQuerySessionPrepareOutcome",
];

#[derive(Default)]
struct ConstructorVisitor {
    impl_owner: Option<String>,
    function: Option<String>,
    constructors: BTreeSet<(String, String)>,
}

impl ConstructorVisitor {
    fn terminal(path: &syn::Path) -> Option<String> {
        path.segments
            .last()
            .map(|segment| segment.ident.to_string())
    }
}

impl<'ast> Visit<'ast> for ConstructorVisitor {
    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        let prior = self.impl_owner.clone();
        self.impl_owner = match item.self_ty.as_ref() {
            syn::Type::Path(path) => Self::terminal(&path.path),
            _ => None,
        };
        syn::visit::visit_item_impl(self, item);
        self.impl_owner = prior;
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let prior = self.function.replace(item.sig.ident.to_string());
        syn::visit::visit_item_fn(self, item);
        self.function = prior;
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        let prior = self.function.replace(item.sig.ident.to_string());
        syn::visit::visit_impl_item_fn(self, item);
        self.function = prior;
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let test_only = item.attrs.iter().any(cfg_requires_test);
        if !test_only {
            syn::visit::visit_item_mod(self, item);
        }
    }

    fn visit_expr_struct(&mut self, expression: &'ast syn::ExprStruct) {
        let named = Self::terminal(&expression.path);
        let authority = match named.as_deref() {
            Some("Self") => self.impl_owner.clone(),
            _ => named,
        };
        if let Some(authority) = authority.filter(|name| PROTECTED.contains(&name.as_str())) {
            self.constructors.insert((
                self.function
                    .clone()
                    .unwrap_or_else(|| "<module>".to_owned()),
                authority,
            ));
        }
        syn::visit::visit_expr_struct(self, expression);
    }
}

fn rust_files(directory: &Path, files: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("read Query source directory") {
        let path = entry.expect("Query source entry").path();
        if path.is_dir() {
            rust_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs")
            && !path.file_stem().is_some_and(|stem| {
                let stem = stem.to_string_lossy();
                stem == "tests" || stem.ends_with("_tests")
            })
            && !path.components().any(|part| part.as_os_str() == "tests")
        {
            files.push(path);
        }
    }
}

fn cfg_requires_test(attribute: &syn::Attribute) -> bool {
    if !attribute.path().is_ident("cfg") {
        return false;
    }
    attribute
        .parse_args::<syn::Meta>()
        .is_ok_and(|predicate| meta_requires_test(&predicate))
}

fn meta_requires_test(predicate: &syn::Meta) -> bool {
    match predicate {
        syn::Meta::Path(path) => path.is_ident("test"),
        syn::Meta::List(list) if list.path.is_ident("all") => list
            .parse_args_with(
                syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|items| items.iter().any(meta_requires_test)),
        syn::Meta::List(_) | syn::Meta::NameValue(_) => false,
    }
}

fn production_constructors() -> BTreeSet<String> {
    let source_root = root().join("workspaces/worth-query/crates/worth-query-execution/src");
    let mut files = Vec::new();
    rust_files(&source_root, &mut files);
    let mut constructors = BTreeSet::new();
    for path in files {
        let source = std::fs::read_to_string(&path).expect("read Query constructor source");
        let syntax = syn::parse_file(&source).expect("Query constructor source must parse");
        let mut visitor = ConstructorVisitor::default();
        visitor.visit_file(&syntax);
        let relative = path
            .strip_prefix(&source_root)
            .expect("Query source below root")
            .to_string_lossy()
            .replace('\\', "/");
        for (function, authority) in visitor.constructors {
            constructors.insert(format!("{relative}\t{function}\t{authority}"));
        }
    }
    constructors
}

#[test]
fn central_c7_struct_literals_match_the_exact_owner_catalog() {
    let expected = BTreeSet::from([
        "domain_computation/authorization/operation_admission/capability_admission/preparation/observation/admitted_access/exact_observation.rs\tbind_capability_observation\tWorthQueryExactCapabilityObservationContext".to_owned(),
        "domain_computation/primary_graph/application_attempt/provider_execution/application_attempt_affinity.rs\tbind_live_session\tWorthQueryApplicationAttemptAffinity".to_owned(),
        "domain_computation/primary_graph/application_attempt/provider_execution/application_attempt_affinity.rs\tcapture\tWorthQueryApplicationAttemptBasis".to_owned(),
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared.rs\tprepare_authorized_application_commit\tWorthQueryPreparedApplicationCommit".to_owned(),
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running.rs\tstart_managed_application_commit\tWorthQueryRunningApplicationCommit".to_owned(),
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression.rs\tfinish\tWorthQueryProgressedApplicationCommit".to_owned(),
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression/authorized.rs\tauthorize_provider_commit\tWorthQueryAuthorizedProviderCommit".to_owned(),
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression/fresh.rs\tcompare_provider_read_set\tWorthQueryFreshProviderAttempt".to_owned(),
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression/registered.rs\tfrom_registration\tWorthQueryRegisteredProviderAttempt".to_owned(),
        "domain_computation/primary_graph/application_attempt/provider_execution/phase/prepared/running/progression/session_admission.rs\tregister\tWorthQueryRegisteredProviderSession".to_owned(),
        "domain_computation/primary_graph/provider/application_attempt_state/commit_preparation.rs\ttake_prepared_session\tWorthQueryPreparedApplicationCommit".to_owned(),
        "domain_computation/provider_session/protocol/readmission.rs\tfrom_admitted\tWorthQueryProviderPlanReadmission".to_owned(),
        "domain_computation/provider_session/protocol/readmission/prepared.rs\tprepare\tWorthQueryPreparedProviderSession".to_owned(),
        "domain_computation/provider_session/protocol/readmission/prepared/staged.rs\tbind_reads_and_effects\tWorthQuerySessionBoundReadsAndEffects".to_owned(),
        "domain_computation/provider_session/protocol/readmission/prepared/staged/prepare_outcome.rs\tprepare_for_commit\tWorthQuerySessionPrepareOutcome".to_owned(),
        "domain_computation/provider_session/provisional_attempt/provider_port.rs\tfrom_session\tWorthQueryProvisionalOverlayCleanupBinding".to_owned(),
    ]);
    assert_eq!(production_constructors(), expected);

    let mutant = syn::parse_file(
        "struct WorthQueryApplicationAttemptAffinity; fn sibling() { let _ = WorthQueryApplicationAttemptAffinity {}; }",
    )
    .expect("constructor mutant parses");
    let mut visitor = ConstructorVisitor::default();
    visitor.visit_file(&mutant);
    assert_eq!(
        visitor.constructors,
        BTreeSet::from([(
            "sibling".to_owned(),
            "WorthQueryApplicationAttemptAffinity".to_owned()
        )])
    );

    let production_cfg = syn::parse_file(
        "#[cfg(not(test))] mod latest { struct WorthQueryApplicationAttemptAffinity; fn sibling() { let _ = WorthQueryApplicationAttemptAffinity {}; } }",
    )
    .expect("production cfg mutant parses");
    let mut visitor = ConstructorVisitor::default();
    visitor.visit_file(&production_cfg);
    assert_eq!(
        visitor.constructors,
        BTreeSet::from([(
            "sibling".to_owned(),
            "WorthQueryApplicationAttemptAffinity".to_owned()
        )]),
        "cfg(not(test)) and substring-named production modules must be inventoried"
    );
}
