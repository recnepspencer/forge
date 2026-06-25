use crate::capability::{
    standard_expression_operator_descriptor, WorthUiExpressionInputKind,
    WorthUiExpressionOperatorDescriptor,
};
use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::WorthUiLiveViewDeclarationReceipt;

use super::{WorthUiLiveViewExpressionDeclaration, WorthUiLiveViewExpressionInput};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewExpressionDenial {
    UnknownOperator {
        expression_id: String,
        operator_id: String,
    },
    InvalidArity {
        expression_id: String,
        operator_id: String,
        expected: String,
        actual: usize,
    },
    InvalidInputKind {
        expression_id: String,
        operator_id: String,
        input_index: usize,
        expected: WorthUiExpressionInputKind,
        actual: WorthUiExpressionInputKind,
    },
    UnknownBinding {
        expression_id: String,
        binding_id: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewExpressionAdmissionCounters {
    checked_expressions: usize,
    denied_expressions: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewExpressionAdmissionReport {
    denials: Vec<WorthUiLiveViewExpressionDenial>,
    counters: WorthUiLiveViewExpressionAdmissionCounters,
    denial_set_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiAdmittedLiveViewExpression {
    declaration: WorthUiLiveViewExpressionDeclaration,
    descriptor: WorthUiExpressionOperatorDescriptor,
}

pub(crate) fn admit_live_view_expression(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
) -> Result<WorthUiAdmittedLiveViewExpression, WorthUiLiveViewExpressionAdmissionReport> {
    let mut denials = Vec::new();
    let descriptor = match standard_expression_operator_descriptor(declaration.operator_id()) {
        Some(descriptor) => descriptor,
        None => {
            denials.push(WorthUiLiveViewExpressionDenial::UnknownOperator {
                expression_id: declaration.expression_id().to_owned(),
                operator_id: declaration.operator_id().as_str().to_owned(),
            });
            return Err(report(denials, 1));
        }
    };
    admit_expression_inputs(live_view, declaration, &descriptor, &mut denials);
    if denials.is_empty() {
        Ok(WorthUiAdmittedLiveViewExpression {
            declaration: declaration.clone(),
            descriptor,
        })
    } else {
        Err(report(denials, 1))
    }
}

fn admit_expression_inputs(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declaration: &WorthUiLiveViewExpressionDeclaration,
    descriptor: &WorthUiExpressionOperatorDescriptor,
    denials: &mut Vec<WorthUiLiveViewExpressionDenial>,
) {
    if !descriptor.arity().admits(declaration.inputs().len()) {
        denials.push(WorthUiLiveViewExpressionDenial::InvalidArity {
            expression_id: declaration.expression_id().to_owned(),
            operator_id: declaration.operator_id().as_str().to_owned(),
            expected: descriptor.arity().token(),
            actual: declaration.inputs().len(),
        });
    }
    for (index, input) in declaration.inputs().iter().enumerate() {
        let actual = input_kind(input);
        let expected = expected_kind(descriptor, index);
        if let Some(expected) = expected {
            if actual != expected {
                denials.push(WorthUiLiveViewExpressionDenial::InvalidInputKind {
                    expression_id: declaration.expression_id().to_owned(),
                    operator_id: declaration.operator_id().as_str().to_owned(),
                    input_index: index,
                    expected,
                    actual,
                });
            }
        }
        collect_dependency_denials(live_view, declaration.expression_id(), input, denials);
    }
}

fn expected_kind(
    descriptor: &WorthUiExpressionOperatorDescriptor,
    index: usize,
) -> Option<WorthUiExpressionInputKind> {
    let kinds = descriptor.input_kinds();
    if kinds.len() == 1 {
        kinds.first().copied()
    } else {
        kinds.get(index).copied().or_else(|| kinds.last().copied())
    }
}

fn input_kind(input: &WorthUiLiveViewExpressionInput) -> WorthUiExpressionInputKind {
    match input {
        WorthUiLiveViewExpressionInput::BindingReference(_) => {
            WorthUiExpressionInputKind::BindingReference
        }
        WorthUiLiveViewExpressionInput::BindingSet(_) => WorthUiExpressionInputKind::BindingSet,
        WorthUiLiveViewExpressionInput::TextLiteral(_) => WorthUiExpressionInputKind::TextLiteral,
        WorthUiLiveViewExpressionInput::NestedExpression(_) => {
            WorthUiExpressionInputKind::BooleanExpression
        }
    }
}

fn collect_dependency_denials(
    live_view: &WorthUiLiveViewDeclarationReceipt,
    expression_id: &str,
    input: &WorthUiLiveViewExpressionInput,
    denials: &mut Vec<WorthUiLiveViewExpressionDenial>,
) {
    match input {
        WorthUiLiveViewExpressionInput::BindingReference(binding_id) => {
            if live_view.binding(binding_id).is_none() {
                denials.push(WorthUiLiveViewExpressionDenial::UnknownBinding {
                    expression_id: expression_id.to_owned(),
                    binding_id: binding_id.to_owned(),
                });
            }
        }
        WorthUiLiveViewExpressionInput::BindingSet(binding_ids) => {
            for binding_id in binding_ids {
                if live_view.binding(binding_id).is_none() {
                    denials.push(WorthUiLiveViewExpressionDenial::UnknownBinding {
                        expression_id: expression_id.to_owned(),
                        binding_id: binding_id.to_owned(),
                    });
                }
            }
        }
        WorthUiLiveViewExpressionInput::NestedExpression(nested) => {
            if let Err(report) = admit_live_view_expression(live_view, nested) {
                denials.extend(report.denials);
            }
        }
        WorthUiLiveViewExpressionInput::TextLiteral(_) => {}
    }
}

fn report(
    mut denials: Vec<WorthUiLiveViewExpressionDenial>,
    checked_expressions: usize,
) -> WorthUiLiveViewExpressionAdmissionReport {
    denials.sort_by_key(denial_key);
    let denial_set_digest = digest_parts(denials.iter().map(denial_digest_token));
    WorthUiLiveViewExpressionAdmissionReport {
        counters: WorthUiLiveViewExpressionAdmissionCounters {
            checked_expressions,
            denied_expressions: denials.len(),
        },
        denials,
        denial_set_digest,
    }
}

fn denial_key(denial: &WorthUiLiveViewExpressionDenial) -> String {
    match denial {
        WorthUiLiveViewExpressionDenial::UnknownOperator { expression_id, .. }
        | WorthUiLiveViewExpressionDenial::InvalidArity { expression_id, .. }
        | WorthUiLiveViewExpressionDenial::InvalidInputKind { expression_id, .. }
        | WorthUiLiveViewExpressionDenial::UnknownBinding { expression_id, .. } => {
            expression_id.clone()
        }
    }
}

fn denial_digest_token(denial: &WorthUiLiveViewExpressionDenial) -> String {
    format!("{denial:?}")
}

impl WorthUiAdmittedLiveViewExpression {
    pub(crate) fn declaration(&self) -> &WorthUiLiveViewExpressionDeclaration {
        &self.declaration
    }

    pub(crate) fn descriptor(&self) -> &WorthUiExpressionOperatorDescriptor {
        &self.descriptor
    }
}

impl WorthUiLiveViewExpressionAdmissionReport {
    pub fn denials(&self) -> &[WorthUiLiveViewExpressionDenial] {
        &self.denials
    }

    pub fn counters(&self) -> &WorthUiLiveViewExpressionAdmissionCounters {
        &self.counters
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

impl WorthUiLiveViewExpressionAdmissionCounters {
    pub fn checked_expressions(&self) -> usize {
        self.checked_expressions
    }

    pub fn denied_expressions(&self) -> usize {
        self.denied_expressions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::{
        WorthUiExpressionArity, WorthUiExpressionCostPosture, WorthUiExpressionDependencyContract,
        WorthUiExpressionDiagnosticsPosture, WorthUiExpressionInputKind,
        WorthUiExpressionOperatorDescriptor, WorthUiExpressionOperatorId,
        WorthUiExpressionOutputKind,
    };
    use crate::runtime::WorthUiSemanticSliceId;

    #[test]
    fn variadic_descriptor_repeats_last_declared_input_kind() {
        let descriptor = WorthUiExpressionOperatorDescriptor::new(
            WorthUiExpressionOperatorId::new("worth.expression.test.variadic"),
            vec![
                WorthUiExpressionInputKind::BindingReference,
                WorthUiExpressionInputKind::TextLiteral,
            ],
            WorthUiExpressionOutputKind::Boolean,
            WorthUiExpressionArity::at_least(2),
            WorthUiExpressionDependencyContract::BindingReferenceAndLiteral,
            WorthUiExpressionCostPosture::SingleBindingLookup,
            WorthUiExpressionDiagnosticsPosture::DependencyReferenced,
            WorthUiSemanticSliceId::ExpressionOutput,
        );

        assert_eq!(
            expected_kind(&descriptor, 0),
            Some(WorthUiExpressionInputKind::BindingReference)
        );
        assert_eq!(
            expected_kind(&descriptor, 1),
            Some(WorthUiExpressionInputKind::TextLiteral)
        );
        assert_eq!(
            expected_kind(&descriptor, 3),
            Some(WorthUiExpressionInputKind::TextLiteral)
        );
    }
}
