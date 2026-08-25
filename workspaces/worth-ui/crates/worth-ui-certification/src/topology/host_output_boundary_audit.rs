use syn::visit::Visit;
use syn::{ImplItem, Item, Signature, Visibility};

use super::WorkspaceSourceInventory;

const FACADE_ROOTS: [&str; 2] = [
    "crates/worth-ui-runtime/src/facade",
    "crates/worth-ui/src/facade",
];

const FORBIDDEN_FACADE_SYMBOLS: [&str; 7] = [
    "WorthUiSealedExecutionPlanBundle",
    "WorthUiExecutionPlanBundleDenial",
    "WorthUiExecutionPlanInputPreparer",
    "WorthUiOrdinaryLanePlanBuilder",
    "WorthUiVirtualizedDataPlanBuilder",
    "WorthUiCanvasSpatialPlanBuilder",
    "WorthUiHudPlanBuilder",
];

pub fn audit_host_output_plan_encapsulation(inventory: &WorkspaceSourceInventory) -> Vec<String> {
    let mut violations = Vec::new();
    for root in FACADE_ROOTS {
        for file in inventory.rust_files_under(root) {
            for symbol in FORBIDDEN_FACADE_SYMBOLS {
                if file.text().contains(symbol) {
                    violations.push(format!(
                        "{} exposes internal plan construction symbol `{symbol}` through a facade",
                        file.absolute_path().display()
                    ));
                }
            }
            let syntax = syn::parse_file(file.text()).unwrap_or_else(|error| {
                panic!("{} should parse: {error}", file.absolute_path().display())
            });
            if public_signature_accepts_owned_plan(&syntax) {
                violations.push(format!(
                    "{} exposes a public facade signature carrying an owned execution plan",
                    file.absolute_path().display()
                ));
            }
        }
    }
    violations.sort();
    violations.dedup();
    violations
}

fn public_signature_accepts_owned_plan(syntax: &syn::File) -> bool {
    syntax.items.iter().any(|item| match item {
        Item::Fn(function) if matches!(function.vis, Visibility::Public(_)) => {
            signature_mentions_owned_plan(&function.sig)
        }
        Item::Impl(item_impl) => item_impl.items.iter().any(|item| {
            matches!(
                item,
                ImplItem::Fn(method)
                    if matches!(method.vis, Visibility::Public(_))
                        && signature_mentions_owned_plan(&method.sig)
            )
        }),
        _ => false,
    })
}

fn signature_mentions_owned_plan(signature: &Signature) -> bool {
    let mut visitor = OwnedPlanTypeVisitor::default();
    visitor.visit_signature(signature);
    visitor.found
}

#[derive(Default)]
struct OwnedPlanTypeVisitor {
    found: bool,
}

impl<'ast> Visit<'ast> for OwnedPlanTypeVisitor {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "WorthUiExecutionPlan")
        {
            self.found = true;
        }
        syn::visit::visit_path(self, path);
    }
}
