use std::collections::BTreeSet;
use std::path::Path;

use syn::visit::{self, Visit};
use syn::{
    FnArg, ImplItemFn, ItemFn, ItemUse, Pat, ReturnType, Signature, Type, UseTree, Visibility,
};

use crate::topology::WorkspaceSourceInventory;

const RUNTIME_ROOT: &str = "crates/worth-ui-runtime/src";
const DSL_ROOT: &str = "crates/worth-ui-dsl/src";
const DIRECT_SEMANTIC_SPEC_CONSTRUCTION: &str = "UiDslSemanticArtifactSpec::new(";
const RUNTIME_BOOTSTRAP_OWNER: &str =
    "crates/worth-ui-runtime/src/declaration/artifact/ui_declaration_lowering.rs";
const FORBIDDEN_DSL_DEPENDENCIES: &[&str] = &[
    "worth-query",
    "worth-ui",
    "worth-ui-host-contract",
    "worth-ui-host-egui",
    "worth-ui-inspection",
    "worth-ui-query-binding",
    "worth-ui-runtime",
];

pub(super) fn audit(inventory: &WorkspaceSourceInventory) -> Result<(), String> {
    for source in inventory.rust_files_under(RUNTIME_ROOT) {
        reject_runtime_language_owner(source.relative_path(), source.text())?;
        reject_direct_runtime_semantic_spec_construction(source.relative_path(), source.text())?;
    }
    for source in inventory.rust_files_under(DSL_ROOT) {
        reject_dsl_runtime_authority_owner(source.relative_path(), source.text())?;
    }
    let manifest = inventory.text("crates/worth-ui-dsl/Cargo.toml");
    reject_dsl_manifest_dependencies(manifest)
}

pub(super) fn reject_direct_runtime_semantic_spec_construction(
    path: &Path,
    source: &str,
) -> Result<(), String> {
    let normalized = path.to_string_lossy().replace('\\', "/");
    if normalized.contains("/tests/")
        || normalized.contains("/certification_support/")
        || normalized.ends_with("_tests.rs")
        || normalized.ends_with("_test_support.rs")
        || normalized.ends_with("/tests.rs")
        || normalized.ends_with("/test_support.rs")
    {
        return Ok(());
    }
    let production = source.split("#[cfg(test)]").next().unwrap_or(source);
    let observed = production
        .matches(DIRECT_SEMANTIC_SPEC_CONSTRUCTION)
        .count();
    let expected = usize::from(normalized == RUNTIME_BOOTSTRAP_OWNER);
    if observed != expected {
        return Err(format!(
            "{normalized} contains {observed} direct DSL semantic-spec constructions; expected {expected}"
        ));
    }
    if normalized == RUNTIME_BOOTSTRAP_OWNER {
        for witness in [
            "pub(crate) fn lower_runtime_bootstrap()",
            "UiDslSourceProvenance::rust_authored(\"worth-ui.runtime.bootstrap\", 0)",
            "UiDslSemanticKey::new(\"worth_ui.runtime.bootstrap.product_root\")",
            "UiDslStructuralToken::new(\"page:product-root\")",
            "UiDslPostureToken::new(\"world:authoritative\")",
            "UiDslSupportToken::new(\"support:runtime-bootstrap\")",
        ] {
            if !source.contains(witness) {
                return Err(format!(
                    "{normalized} drifted from the invariant runtime bootstrap `{witness}`"
                ));
            }
        }
    }
    Ok(())
}

pub(super) fn reject_dsl_runtime_authority_owner(path: &Path, source: &str) -> Result<(), String> {
    let syntax = syn::parse_file(source).map_err(|error| {
        format!(
            "{} should parse for inverse ownership audit: {error}",
            path.display()
        )
    })?;
    let mut visitor = DslRuntimeAuthorityVisitor::default();
    visitor.visit_file(&syntax);
    if visitor.findings.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} violates runtime authority ownership: {}",
            path.display(),
            visitor.findings.into_iter().collect::<Vec<_>>().join(", ")
        ))
    }
}

#[derive(Default)]
struct DslRuntimeAuthorityVisitor {
    findings: BTreeSet<String>,
}

impl Visit<'_> for DslRuntimeAuthorityVisitor {
    fn visit_ident(&mut self, identifier: &syn::Ident) {
        let identifier = identifier.to_string();
        if runtime_authority_identifier(&identifier) {
            self.findings.insert(identifier);
        }
    }
}

fn runtime_authority_identifier(identifier: &str) -> bool {
    (identifier.starts_with("Ui") || identifier.starts_with("WorthUi"))
        && [
            "ActiveApplicationSession",
            "AllocationCatalog",
            "ApplicationPublication",
            "CanvasSpatialPlan",
            "FrameworkTurn",
            "HostAdapterSessionAuthority",
            "HostSession",
            "Mounted",
            "OrdinaryPlan",
            "PreparedApplicationGeneration",
            "RealtimePlan",
            "VirtualizedPlan",
        ]
        .iter()
        .any(|fragment| identifier.contains(fragment))
}

pub(super) fn reject_runtime_language_owner(path: &Path, source: &str) -> Result<(), String> {
    let syntax = syn::parse_file(source).map_err(|error| {
        format!(
            "{} should parse for ownership audit: {error}",
            path.display()
        )
    })?;
    let mut visitor = RuntimeLanguageOwnerVisitor::default();
    visitor.visit_file(&syntax);
    if visitor.findings.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{} violates DSL language ownership: {}",
            path.display(),
            visitor.findings.into_iter().collect::<Vec<_>>().join("; ")
        ))
    }
}

pub(super) fn reject_dsl_manifest_dependencies(source: &str) -> Result<(), String> {
    let manifest = source
        .parse::<toml::Value>()
        .map_err(|error| format!("DSL manifest should parse: {error}"))?;
    let mut forbidden = BTreeSet::new();
    for table_name in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let Some(table) = manifest.get(table_name).and_then(toml::Value::as_table) else {
            continue;
        };
        for dependency in table.keys() {
            if FORBIDDEN_DSL_DEPENDENCIES.contains(&dependency.as_str()) {
                forbidden.insert(dependency.clone());
            }
        }
    }
    if forbidden.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "worth-ui-dsl imports forbidden authority dependencies: {}",
            forbidden.into_iter().collect::<Vec<_>>().join(", ")
        ))
    }
}

#[derive(Default)]
struct RuntimeLanguageOwnerVisitor {
    findings: BTreeSet<String>,
}

impl Visit<'_> for RuntimeLanguageOwnerVisitor {
    fn visit_item_fn(&mut self, function: &ItemFn) {
        self.inspect_function(&function.sig, &function.block);
        visit::visit_item_fn(self, function);
    }

    fn visit_impl_item_fn(&mut self, function: &ImplItemFn) {
        self.inspect_function(&function.sig, &function.block);
        visit::visit_impl_item_fn(self, function);
    }

    fn visit_item_use(&mut self, item_use: &ItemUse) {
        if matches!(item_use.vis, Visibility::Public(_)) {
            for terminal in dsl_forwarding_terminals(&item_use.tree) {
                self.findings.insert(format!(
                    "public runtime forwarding export `{terminal}` from worth_ui_dsl"
                ));
            }
        }
        visit::visit_item_use(self, item_use);
    }
}

impl RuntimeLanguageOwnerVisitor {
    fn inspect_function(&mut self, signature: &Signature, body: &syn::Block) {
        let name = signature.ident.to_string();
        let input = function_input_shape(signature);
        let output = return_type_shape(&signature.output);
        let mut methods = MethodNameVisitor::default();
        methods.visit_block(body);

        if semantic_verb(&name)
            && input.has_text_source
            && methods
                .names
                .iter()
                .any(|method| lexical_text_method(method))
        {
            self.findings
                .insert(format!("`{name}` performs authored-text lexical work"));
        }
        if semantic_verb(&name)
            && input
                .type_identifiers
                .iter()
                .any(|identifier| syntax_shape(identifier))
            && output.iter().any(|identifier| semantic_shape(identifier))
        {
            self.findings.insert(format!(
                "`{name}` lowers a syntax shape into semantic meaning"
            ));
        }
    }
}

#[derive(Default)]
struct FunctionInputShape {
    has_text_source: bool,
    type_identifiers: BTreeSet<String>,
}

fn function_input_shape(signature: &Signature) -> FunctionInputShape {
    let mut shape = FunctionInputShape::default();
    for input in &signature.inputs {
        let FnArg::Typed(argument) = input else {
            continue;
        };
        let mut identifiers = BTreeSet::new();
        collect_type_identifiers(&argument.ty, &mut identifiers);
        let parameter_name = match argument.pat.as_ref() {
            Pat::Ident(identifier) => identifier.ident.to_string(),
            _ => String::new(),
        };
        let text_type = identifiers
            .iter()
            .any(|identifier| matches!(identifier.as_str(), "str" | "String" | "u8"));
        let text_name = ["source", "text", "bytes", "units", "tokens"]
            .iter()
            .any(|fragment| parameter_name.to_ascii_lowercase().contains(fragment));
        shape.has_text_source |= text_type && text_name;
        shape.type_identifiers.extend(identifiers);
    }
    shape
}

fn return_type_shape(output: &ReturnType) -> BTreeSet<String> {
    let mut identifiers = BTreeSet::new();
    if let ReturnType::Type(_, ty) = output {
        collect_type_identifiers(ty, &mut identifiers);
    }
    identifiers
}

fn collect_type_identifiers(ty: &Type, identifiers: &mut BTreeSet<String>) {
    struct TypeVisitor<'a>(&'a mut BTreeSet<String>);
    impl Visit<'_> for TypeVisitor<'_> {
        fn visit_path(&mut self, path: &syn::Path) {
            self.0.extend(
                path.segments
                    .iter()
                    .map(|segment| segment.ident.to_string()),
            );
            visit::visit_path(self, path);
        }
    }
    TypeVisitor(identifiers).visit_type(ty);
}

#[derive(Default)]
struct MethodNameVisitor {
    names: BTreeSet<String>,
}

impl Visit<'_> for MethodNameVisitor {
    fn visit_expr_method_call(&mut self, call: &syn::ExprMethodCall) {
        self.names.insert(call.method.to_string());
        visit::visit_expr_method_call(self, call);
    }
}

fn semantic_verb(name: &str) -> bool {
    [
        "compile",
        "decode",
        "lex",
        "lower",
        "normalize",
        "parse",
        "tokenize",
    ]
    .iter()
    .any(|verb| name.to_ascii_lowercase().contains(verb))
}

fn lexical_text_method(name: &str) -> bool {
    matches!(
        name,
        "bytes"
            | "char_indices"
            | "chars"
            | "lines"
            | "split"
            | "split_ascii_whitespace"
            | "split_whitespace"
    )
}

fn syntax_shape(identifier: &str) -> bool {
    ["Ast", "Parsed", "Syntax", "Token"]
        .iter()
        .any(|fragment| identifier.contains(fragment))
}

fn semantic_shape(identifier: &str) -> bool {
    ["Declaration", "Package", "Semantic"]
        .iter()
        .any(|fragment| identifier.contains(fragment))
}

fn dsl_forwarding_terminals(tree: &UseTree) -> Vec<String> {
    match tree {
        UseTree::Path(path) if path.ident == "worth_ui_dsl" => use_tree_terminals(&path.tree),
        UseTree::Group(group) => group
            .items
            .iter()
            .flat_map(dsl_forwarding_terminals)
            .collect(),
        UseTree::Path(_) | UseTree::Name(_) | UseTree::Rename(_) | UseTree::Glob(_) => Vec::new(),
    }
}

fn use_tree_terminals(tree: &UseTree) -> Vec<String> {
    match tree {
        UseTree::Name(name) => vec![name.ident.to_string()],
        UseTree::Rename(rename) => vec![rename.rename.to_string()],
        UseTree::Path(path) => use_tree_terminals(&path.tree),
        UseTree::Group(group) => group.items.iter().flat_map(use_tree_terminals).collect(),
        UseTree::Glob(_) => vec!["*".to_owned()],
    }
}
