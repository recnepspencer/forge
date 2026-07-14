use std::collections::BTreeSet;

use syn::spanned::Spanned;
use syn::visit::Visit;

use super::model::WorthQueryConsumerOrchestrationPhase as Phase;

#[derive(Clone, Debug)]
pub(super) struct ConsumerFunctionObservation {
    pub(super) name: String,
    pub(super) line: usize,
    pub(super) column: usize,
    pub(super) direct_phases: BTreeSet<Phase>,
    pub(super) called_functions: BTreeSet<String>,
}

pub(super) fn consumer_function_observations(
    source: &str,
) -> Result<Vec<ConsumerFunctionObservation>, syn::Error> {
    let syntax = syn::parse_file(source)?;
    let mut collector = FunctionCollector::default();
    collector.visit_file(&syntax);
    Ok(collector.functions)
}

#[derive(Default)]
struct FunctionCollector {
    functions: Vec<ConsumerFunctionObservation>,
    type_stack: Vec<String>,
}

impl FunctionCollector {
    fn observe_function(
        &mut self,
        name: String,
        signature_span: proc_macro2::Span,
        block: &syn::Block,
    ) {
        let mut calls = CallCollector::default();
        calls.visit_block(block);
        let start = signature_span.start();
        self.functions.push(ConsumerFunctionObservation {
            name,
            line: start.line,
            column: start.column + 1,
            direct_phases: calls.phases,
            called_functions: calls.local_calls,
        });
    }
}

impl<'ast> Visit<'ast> for FunctionCollector {
    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        let type_name = type_name(&node.self_ty).unwrap_or_else(|| "impl".to_string());
        self.type_stack.push(type_name);
        syn::visit::visit_item_impl(self, node);
        self.type_stack.pop();
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.observe_function(node.sig.ident.to_string(), node.sig.span(), &node.block);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        let owner = self.type_stack.last().map(String::as_str).unwrap_or("impl");
        self.observe_function(
            format!("{owner}::{}", node.sig.ident),
            node.sig.span(),
            &node.block,
        );
    }
}

#[derive(Default)]
struct CallCollector {
    phases: BTreeSet<Phase>,
    local_calls: BTreeSet<String>,
}

impl CallCollector {
    fn observe(&mut self, call_name: &str, query_qualified: bool) {
        if let Some(phase) = query_phase(call_name, query_qualified) {
            self.phases.insert(phase);
        } else {
            self.local_calls.insert(call_name.to_string());
        }
    }
}

impl<'ast> Visit<'ast> for CallCollector {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = node.func.as_ref() {
            if let Some(segment) = path.path.segments.last() {
                let qualified = path.path.segments.iter().any(|part| {
                    matches!(
                        part.ident.to_string().as_str(),
                        "worth_query" | "query" | "facade"
                    )
                });
                self.observe(&segment.ident.to_string(), qualified);
            }
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        self.observe(&node.method.to_string(), false);
        syn::visit::visit_expr_method_call(self, node);
    }
}

fn query_phase(name: &str, query_qualified: bool) -> Option<Phase> {
    let distinctive = name.contains("query")
        || name.contains("declaration_entry")
        || name.contains("read_family")
        || name.contains("graph_read")
        || name.contains("projection_consumption")
        || name.contains("historical_evaluation")
        || name.contains("correspondence_evidence")
        || name.contains("preview_session");
    if !query_qualified && !distinctive {
        return None;
    }
    if name.starts_with("canonical") || name.contains("canonicaliz") {
        Some(Phase::Canonicalize)
    } else if name.starts_with("bind") || name.contains("_binding") {
        Some(Phase::Bind)
    } else if name.starts_with("validat") {
        Some(Phase::Validate)
    } else if name.starts_with("admit") {
        Some(Phase::Admit)
    } else if name.starts_with("plan") {
        Some(Phase::Plan)
    } else if name.starts_with("lower") {
        Some(Phase::Lower)
    } else if name.starts_with("execute") || name == "run" || name == "open" {
        Some(Phase::Execute)
    } else if name.starts_with("orchestrate")
        || name.contains("receipt")
        || name.contains("outcome")
    {
        Some(Phase::AssembleOutcome)
    } else if name.starts_with("inspect") {
        Some(Phase::Inspect)
    } else {
        None
    }
}

fn type_name(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(path) => path.path.segments.last().map(|part| part.ident.to_string()),
        _ => None,
    }
}
