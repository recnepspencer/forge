use std::any::Any;
use std::collections::{BTreeSet, HashMap};
use std::sync::Mutex;

use crate::data::host_computed::{admit_or_error, HostComputedApiFamily};
use crate::data::trace::{RuntimeArtifactHot, RuntimeArtifactState, RuntimeArtifactWarm};
use crate::facade::{AspectMask, NodeEvaluationResult, NodeId, NodeState, SignalError};
use crate::logic::prepared::PreparedEvaluation;

use super::{SignalApp, DEFAULT_ASPECT};

impl SignalApp {
    pub(in crate::easy) fn ensure_evaluated(
        &mut self,
        node: NodeId,
    ) -> Result<BTreeSet<NodeId>, SignalError> {
        if !self.computed.contains_key(&node) && !self.pending_input_versions.contains_key(&node) {
            return Ok(BTreeSet::new());
        }
        let staged_values = Mutex::new(HashMap::<NodeId, Box<dyn Any + Send + Sync>>::new());
        let meaningful_nodes = Mutex::new(BTreeSet::new());
        let max_passes = self.graph.active_node_count().saturating_add(1);
        let mut settled = false;
        for _ in 0..max_passes {
            let mut scheduled_tasks = 0_u32;
            for target in self.requested_dependency_order(node)? {
                let plan = self.graph.build_evaluation_plan(
                    &[target],
                    crate::logic::evaluation::EvaluationRequestMode::Default,
                )?;
                scheduled_tasks = scheduled_tasks.saturating_add(plan.summary.task_count);
                self.execute_easy_plan(&plan, &staged_values, &meaningful_nodes)?;
            }
            if scheduled_tasks == 0 {
                settled = true;
                break;
            }
        }
        if !settled {
            return Err(SignalError::internal(
                "easy path dependency settlement did not converge",
            ));
        }
        for (node, value) in mutex_value(staged_values, "staged value")? {
            self.values.insert(node, value);
        }
        self.pending_input_versions.retain(|node, candidate| {
            self.graph
                .node_aspect_version(*node)
                .map_or(true, |committed| committed != *candidate)
        });
        mutex_value(meaningful_nodes, "meaningful-change")
    }

    fn requested_dependency_order(&self, target: NodeId) -> Result<Vec<NodeId>, SignalError> {
        let mut discovered = BTreeSet::new();
        let mut order = Vec::new();
        let mut stack = vec![(target, false)];
        while let Some((node, expanded)) = stack.pop() {
            if expanded {
                order.push(node);
                continue;
            }
            if !discovered.insert(node) {
                continue;
            }
            stack.push((node, true));
            let dependencies = self.graph.dependencies_of(node)?;
            for dependency in dependencies.iter().rev() {
                stack.push((dependency.source(), false));
            }
        }
        Ok(order)
    }

    fn execute_easy_plan(
        &mut self,
        plan: &crate::logic::planner::EvaluationPlan,
        staged_values: &Mutex<HashMap<NodeId, Box<dyn Any + Send + Sync>>>,
        meaningful_nodes: &Mutex<BTreeSet<NodeId>>,
    ) -> Result<(), SignalError> {
        let computed = &self.computed;
        let pending = &self.pending_input_versions;
        let values = &self.values;
        self.graph
            .execute_prepared_plan_with_precompute(plan, &|current, view| {
                if let Some(computed) = computed.get(&current) {
                    let version = view
                        .graph()
                        .node_aspect_version(current)?
                        .get(DEFAULT_ASPECT);
                    let dependencies = view.graph().dependencies_of(current)?.to_vec();
                    let staged = staged_values
                        .lock()
                        .map_err(|_| mutex_error("staged value"))?;
                    let (value, prepared, meaningful) =
                        computed.precompute(current, values, &staged, &dependencies, version)?;
                    drop(staged);
                    staged_values
                        .lock()
                        .map_err(|_| mutex_error("staged value"))?
                        .insert(current, value);
                    if meaningful {
                        meaningful_nodes
                            .lock()
                            .map_err(|_| mutex_error("meaningful-change"))?
                            .insert(current);
                    }
                    Ok(prepared)
                } else if let Some(version) = pending.get(&current).copied() {
                    meaningful_nodes
                        .lock()
                        .map_err(|_| mutex_error("meaningful-change"))?
                        .insert(current);
                    Ok(PreparedEvaluation::from_result(
                        NodeEvaluationResult::from_version(version),
                    ))
                } else {
                    Ok(PreparedEvaluation::validated_clean())
                }
            })?;
        Ok(())
    }

    pub(super) fn seed_computed_if_possible(&mut self, node: NodeId) -> Result<(), SignalError> {
        let Some(computed) = self.computed.get(&node) else {
            return Ok(());
        };
        let dependencies = self.graph.dependencies_of(node)?.to_vec();
        let Ok((value, prepared, _)) =
            computed.precompute(node, &self.values, &HashMap::new(), &dependencies, 0)
        else {
            return Ok(());
        };
        let Ok(admitted) = admit_or_error(
            HostComputedApiFamily::EasyClosure,
            node,
            &dependencies,
            prepared,
            self.graph.telemetry_mut(),
        ) else {
            return Ok(());
        };
        let (prepared, reads, patch) = admitted.into_parts();
        self.values.insert(node, value);
        let mut snapshot = crate::data::dependency::DependencySnapshot::empty();
        for dependency in reads.dependencies() {
            let version = self
                .graph
                .node_aspect_version(dependency.source())?
                .get(dependency.aspect());
            snapshot.record(
                dependency.source(),
                dependency.aspect(),
                version,
                dependency.scope_ref().cloned(),
            );
        }
        {
            let mut entry = self.graph.get_entry_mut(node)?;
            entry.set_aspect_version(prepared.result.aspect_version);
            entry.set_state(NodeState::Clean);
            entry.set_dirty_aspects(AspectMask::EMPTY);
            entry.clear_dirty_partition_scopes();
            entry.set_runtime_artifact_state(Some(easy_seed_runtime_artifact_state(
                prepared.result.aspect_version,
                patch.next_dependencies().len() as u32,
                true,
            )));
        }
        self.graph
            .set_dependencies(node, reads.dependencies().iter().cloned())?;
        self.graph.set_dep_snapshot(node, snapshot)?;
        self.graph.transition_node_clean(node)
    }
}

fn mutex_value<T>(mutex: Mutex<T>, label: &str) -> Result<T, SignalError> {
    mutex.into_inner().map_err(|_| mutex_error(label))
}

fn mutex_error(label: &str) -> SignalError {
    SignalError::internal(format!("easy path {label} mutex poisoned"))
}

pub(super) fn easy_seed_runtime_artifact_state(
    version: crate::facade::AspectVersion,
    dependency_count: u32,
    recomputed: bool,
) -> RuntimeArtifactState {
    let mut hot = RuntimeArtifactHot::default();
    hot.dependency_count = dependency_count;
    hot.recomputed = recomputed;
    hot.output_hash = crate::data::core_profile::StableHashValue::from(version.slots()[0]);
    let mut warm = RuntimeArtifactWarm::default();
    warm.memoized_origin = crate::data::output::MemoizedResultOrigin::DirectCompute;
    RuntimeArtifactState::new(hot, warm)
}
