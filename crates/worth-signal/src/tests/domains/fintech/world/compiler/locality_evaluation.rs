use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use crate::data::aspect::AspectVersion;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::output::NodeEvaluationResult;
use crate::logic::context::EvaluationContext;
use crate::logic::evaluation::EvaluationOutput;

use super::super::{
    FinancialLocalityDefinition, FinancialLocalityFormula, FinancialLocalityOutput,
    LocalitySemanticOutputId,
};
use super::locality_topology::partition_subscription;
use super::topology::signal_aspect;

pub(super) struct LocalityEvaluationProgram {
    outputs_by_node: BTreeMap<NodeId, FinancialLocalityOutput>,
    handles: BTreeMap<LocalitySemanticOutputId, NodeId>,
    financial_values: BTreeMap<LocalitySemanticOutputId, i64>,
    aspect_version: u64,
    evaluated_outputs: Arc<Mutex<BTreeSet<LocalitySemanticOutputId>>>,
}

impl LocalityEvaluationProgram {
    pub(super) fn new(
        definition: &FinancialLocalityDefinition,
        handles: &BTreeMap<LocalitySemanticOutputId, NodeId>,
        financial_values: &BTreeMap<LocalitySemanticOutputId, i64>,
        aspect_version: u64,
    ) -> Self {
        Self {
            outputs_by_node: definition
                .outputs()
                .iter()
                .map(|output| (handles[&output.id], output.clone()))
                .collect(),
            handles: handles.clone(),
            financial_values: financial_values.clone(),
            aspect_version,
            evaluated_outputs: Arc::new(Mutex::new(BTreeSet::new())),
        }
    }

    pub(super) fn evaluate(
        &self,
        view: &mut EvaluationContext<'_, ()>,
    ) -> Result<EvaluationOutput, SignalError> {
        let output = self.outputs_by_node.get(&view.node()).ok_or_else(|| {
            SignalError::invalid_input("locality evaluator received an unknown node")
        })?;
        self.evaluated_outputs
            .lock()
            .expect("locality evaluation identity lock poisoned")
            .insert(output.id);
        for dependency in &output.dependencies {
            let source = self.handles[&dependency.producer];
            match dependency.edge_scope {
                None => {
                    let _ = view.read_aspect_version(source, signal_aspect(dependency.aspect))?;
                }
                Some(scope) => {
                    let _ = view.read_partitioned_aspect_version(
                        source,
                        signal_aspect(dependency.aspect),
                        partition_subscription(scope),
                    )?;
                }
            }
        }
        Ok(view.finish(self.result_for(output)))
    }

    fn result_for(&self, output: &FinancialLocalityOutput) -> NodeEvaluationResult {
        let version = output
            .produced_aspects
            .iter()
            .fold(AspectVersion::zero(), |version, aspect| {
                version.with(signal_aspect(*aspect), self.aspect_version)
            });
        NodeEvaluationResult::from_version(version).with_output_identity(format!(
            "financial-locality:{:?}:{:?}:{}:{}",
            output.owner,
            output.role,
            output.id.ordinal(),
            self.financial_values[&output.id]
        ))
    }

    pub(super) fn evaluated_outputs(&self) -> BTreeSet<LocalitySemanticOutputId> {
        self.evaluated_outputs
            .lock()
            .expect("locality evaluation identity lock poisoned")
            .clone()
    }
}

pub(super) fn runtime_baseline_values(
    definition: &FinancialLocalityDefinition,
) -> Result<BTreeMap<LocalitySemanticOutputId, i64>, SignalError> {
    let mut values = BTreeMap::new();
    for output in definition.outputs() {
        let value = match output.formula {
            FinancialLocalityFormula::MarketSource { baseline_value, .. } => baseline_value,
            FinancialLocalityFormula::StableControl { retained_value } => retained_value,
            FinancialLocalityFormula::LinearDependency {
                multiplier_micros,
                basis_value,
            } => linear_value(output, &values, multiplier_micros, basis_value)?,
        };
        values.insert(output.id, value);
    }
    Ok(values)
}

pub(super) fn runtime_shocked_values(
    definition: &FinancialLocalityDefinition,
    baseline: &BTreeMap<LocalitySemanticOutputId, i64>,
) -> Result<BTreeMap<LocalitySemanticOutputId, i64>, SignalError> {
    let mutation = definition.mutation();
    let mut shocked = BTreeMap::new();
    for output in definition.outputs() {
        let value = match output.formula {
            FinancialLocalityFormula::MarketSource {
                baseline_value,
                mutation_delta,
            } if output.id == mutation.producer => baseline_value
                .checked_add(mutation_delta)
                .ok_or_else(|| SignalError::internal("locality source shock overflow"))?,
            FinancialLocalityFormula::MarketSource { baseline_value, .. } => baseline_value,
            FinancialLocalityFormula::StableControl { retained_value } => retained_value,
            FinancialLocalityFormula::LinearDependency {
                multiplier_micros,
                basis_value,
            } => shocked_linear_value(
                output,
                ShockEvaluationContext {
                    baseline,
                    shocked: &shocked,
                    mutation,
                },
                multiplier_micros,
                basis_value,
            )?,
        };
        shocked.insert(output.id, value);
    }
    Ok(shocked)
}

struct ShockEvaluationContext<'a> {
    baseline: &'a BTreeMap<LocalitySemanticOutputId, i64>,
    shocked: &'a BTreeMap<LocalitySemanticOutputId, i64>,
    mutation: super::super::FinancialLocalityMutation,
}

fn shocked_linear_value(
    output: &FinancialLocalityOutput,
    context: ShockEvaluationContext<'_>,
    multiplier_micros: i64,
    basis_value: i64,
) -> Result<i64, SignalError> {
    let inputs = output
        .dependencies
        .iter()
        .map(|dependency| {
            let values = if dependency.producer == context.mutation.producer
                && (dependency.aspect != context.mutation.aspect
                    || !scopes_overlap(dependency.edge_scope, context.mutation.scope))
            {
                context.baseline
            } else {
                context.shocked
            };
            values.get(&dependency.producer).copied().ok_or_else(|| {
                SignalError::internal("locality shock dependency is not topological")
            })
        })
        .sum::<Result<i64, SignalError>>()?;
    inputs
        .checked_mul(multiplier_micros)
        .and_then(|value| value.checked_div(1_000_000))
        .and_then(|value| value.checked_add(basis_value))
        .ok_or_else(|| SignalError::internal("locality shocked financial formula overflow"))
}

fn scopes_overlap(
    left: Option<super::super::LocalityScope>,
    right: Option<super::super::LocalityScope>,
) -> bool {
    match (left, right) {
        (None, _) | (_, None) => true,
        (Some(left), Some(right)) if left.region != right.region => false,
        (Some(left), Some(right)) => {
            left.detail.is_none() || right.detail.is_none() || left.detail == right.detail
        }
    }
}

fn linear_value(
    output: &FinancialLocalityOutput,
    values: &BTreeMap<LocalitySemanticOutputId, i64>,
    multiplier_micros: i64,
    basis_value: i64,
) -> Result<i64, SignalError> {
    let inputs = output
        .dependencies
        .iter()
        .map(|dependency| {
            values.get(&dependency.producer).copied().ok_or_else(|| {
                SignalError::internal("locality formula dependency is not topological")
            })
        })
        .sum::<Result<i64, SignalError>>()?;
    inputs
        .checked_mul(multiplier_micros)
        .and_then(|value| value.checked_div(1_000_000))
        .and_then(|value| value.checked_add(basis_value))
        .ok_or_else(|| SignalError::internal("locality financial formula overflow"))
}
