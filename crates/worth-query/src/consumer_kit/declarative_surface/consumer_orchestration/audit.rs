use std::collections::{BTreeMap, BTreeSet};

use super::model::{
    WorthQueryConsumerOrchestrationAudit, WorthQueryConsumerOrchestrationError,
    WorthQueryConsumerOrchestrationFinding, WorthQueryConsumerOrchestrationSite,
};
use super::syntax::{consumer_function_observations, ConsumerFunctionObservation};
use crate::consumer_kit::declarative_surface::WorthQueryDeclarativeSurfaceSource;

pub fn audit_consumer_orchestration_sources(
    sources: &[WorthQueryDeclarativeSurfaceSource],
) -> Result<WorthQueryConsumerOrchestrationAudit, WorthQueryConsumerOrchestrationError> {
    let mut functions = Vec::new();
    for source in sources {
        let observations = consumer_function_observations(source.text()).map_err(|error| {
            WorthQueryConsumerOrchestrationError::invalid_rust_source(source.path(), error)
        })?;
        functions.extend(
            observations
                .into_iter()
                .map(|observation| ObservedConsumerFunction::new(source.path(), observation)),
        );
    }
    let name_index = function_name_index(&functions);
    let mut transitive_phases = functions
        .iter()
        .map(|function| function.observation.direct_phases.clone())
        .collect::<Vec<_>>();

    loop {
        let prior = transitive_phases.clone();
        for (function_index, function) in functions.iter().enumerate() {
            for called_name in &function.observation.called_functions {
                for called_index in
                    resolved_called_indices(function, called_name, &functions, &name_index)
                {
                    transitive_phases[function_index].extend(prior[called_index].iter().copied());
                }
            }
        }
        if transitive_phases == prior {
            break;
        }
    }

    let flagged = transitive_phases
        .iter()
        .enumerate()
        .filter_map(|(index, phases)| (phases.len() >= 2).then_some(index))
        .collect::<BTreeSet<_>>();
    let called_by_flagged = flagged
        .iter()
        .flat_map(|index| {
            functions[*index]
                .observation
                .called_functions
                .iter()
                .flat_map(|name| {
                    resolved_called_indices(&functions[*index], name, &functions, &name_index)
                })
                .collect::<Vec<_>>()
        })
        .collect::<BTreeSet<_>>();
    let mut findings = flagged
        .difference(&called_by_flagged)
        .map(|index| {
            let function = &functions[*index];
            WorthQueryConsumerOrchestrationFinding::new(
                WorthQueryConsumerOrchestrationSite::new(
                    &function.path,
                    function.observation.line,
                    function.observation.column,
                    &function.observation.name,
                ),
                transitive_phases[*index].iter().copied().collect(),
            )
        })
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| left.site().cmp(right.site()));
    Ok(WorthQueryConsumerOrchestrationAudit::new(
        functions.len(),
        findings,
    ))
}

struct ObservedConsumerFunction {
    path: String,
    observation: ConsumerFunctionObservation,
}

impl ObservedConsumerFunction {
    fn new(path: &str, observation: ConsumerFunctionObservation) -> Self {
        Self {
            path: path.to_string(),
            observation,
        }
    }
}

fn function_name_index(functions: &[ObservedConsumerFunction]) -> BTreeMap<String, Vec<usize>> {
    let mut index = BTreeMap::<String, Vec<usize>>::new();
    for (function_index, function) in functions.iter().enumerate() {
        let full_name = &function.observation.name;
        let short_name = full_name.rsplit("::").next().unwrap_or(full_name);
        index
            .entry(short_name.to_string())
            .or_default()
            .push(function_index);
    }
    index
}

fn resolved_called_indices(
    caller: &ObservedConsumerFunction,
    called_name: &str,
    functions: &[ObservedConsumerFunction],
    name_index: &BTreeMap<String, Vec<usize>>,
) -> Vec<usize> {
    let Some(candidates) = name_index.get(called_name) else {
        return Vec::new();
    };
    let same_source = candidates
        .iter()
        .copied()
        .filter(|index| functions[*index].path == caller.path)
        .collect::<Vec<_>>();
    if !same_source.is_empty() {
        return same_source;
    }
    if candidates.len() == 1 {
        return candidates.clone();
    }
    Vec::new()
}
