use crate::planning::ExecutionPlanBundle;
use crate::validation::ValidatedQueryBundle;
use forge_foundational::facade::AspectKey;

use super::grouped_binding::QueryResultBindingProof;
use super::grouped_policy::{GroupedDeltaAdmissionPolicy, GroupedReplayDeliveryPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedBaselineMaterializationContract {
    grouping_aspect: AspectKey,
    identity_binding: QueryResultBindingProof,
    grouping_binding: QueryResultBindingProof,
    grouped_binding_width: usize,
}

impl GroupedBaselineMaterializationContract {
    pub fn grouping_aspect(&self) -> &str {
        self.grouping_aspect.as_str()
    }

    pub fn native_grouping_aspect_key(&self) -> &AspectKey {
        &self.grouping_aspect
    }

    pub fn identity_binding_index(&self) -> usize {
        self.identity_binding.binding_index()
    }

    pub fn grouping_binding_index(&self) -> usize {
        self.grouping_binding.binding_index()
    }

    pub fn identity_binding(&self) -> &QueryResultBindingProof {
        &self.identity_binding
    }

    pub fn grouping_binding(&self) -> &QueryResultBindingProof {
        &self.grouping_binding
    }

    pub fn grouped_binding_width(&self) -> usize {
        self.grouped_binding_width
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedViewPlanningArtifact {
    grouping_aspect: AspectKey,
    identity_binding: QueryResultBindingProof,
    grouping_binding: QueryResultBindingProof,
    grouped_binding_width: usize,
    grouped_projection_width: usize,
    traversal_count: usize,
    ordering_count: usize,
    predicate_count: usize,
    fallback: crate::planning::FallbackDisposition,
    baseline_materialization: GroupedBaselineMaterializationContract,
    replay_delivery_posture: GroupedReplayDeliveryPosture,
    grouped_delta_policy: GroupedDeltaAdmissionPolicy,
}

impl GroupedViewPlanningArtifact {
    pub fn grouping_aspect(&self) -> &str {
        self.grouping_aspect.as_str()
    }

    pub fn native_grouping_aspect_key(&self) -> &AspectKey {
        &self.grouping_aspect
    }

    pub fn identity_binding_index(&self) -> usize {
        self.identity_binding.binding_index()
    }

    pub fn grouping_binding_index(&self) -> usize {
        self.grouping_binding.binding_index()
    }

    pub fn identity_binding(&self) -> &QueryResultBindingProof {
        &self.identity_binding
    }

    pub fn grouping_binding(&self) -> &QueryResultBindingProof {
        &self.grouping_binding
    }

    pub fn grouped_binding_width(&self) -> usize {
        self.grouped_binding_width
    }

    pub fn grouped_projection_width(&self) -> usize {
        self.grouped_projection_width
    }

    pub fn traversal_count(&self) -> usize {
        self.traversal_count
    }

    pub fn ordering_count(&self) -> usize {
        self.ordering_count
    }

    pub fn predicate_count(&self) -> usize {
        self.predicate_count
    }

    pub fn fallback(&self) -> &crate::planning::FallbackDisposition {
        &self.fallback
    }

    pub fn baseline_materialization(&self) -> &GroupedBaselineMaterializationContract {
        &self.baseline_materialization
    }

    pub fn replay_delivery_posture(&self) -> &GroupedReplayDeliveryPosture {
        &self.replay_delivery_posture
    }

    pub fn grouped_delta_policy(&self) -> &GroupedDeltaAdmissionPolicy {
        &self.grouped_delta_policy
    }

    pub(crate) fn derive(
        validated_view: &ValidatedQueryBundle,
        execution_plan: &ExecutionPlanBundle,
        grouping_aspect: &AspectKey,
    ) -> Option<Self> {
        let identity_binding = validated_view
            .result_shape()
            .bindings()
            .iter()
            .enumerate()
            .find(|(_, binding)| {
                binding.source_aspect() == "identity" && binding.source_field() == "id"
            })
            .map(|(binding_index, binding)| {
                QueryResultBindingProof::new(
                    binding.source_aspect(),
                    binding.source_field(),
                    binding_index,
                )
            })??;
        let grouping_binding = validated_view
            .result_shape()
            .bindings()
            .iter()
            .enumerate()
            .find(|(_, binding)| binding.source_aspect() == grouping_aspect.as_str())
            .map(|(binding_index, binding)| {
                QueryResultBindingProof::new(
                    binding.source_aspect(),
                    binding.source_field(),
                    binding_index,
                )
            })??;

        let grouped_binding_width = validated_view.result_shape().bindings().len();
        let grouped_projection_width = execution_plan.query().projection_count();
        let traversal_count = execution_plan.query().traversal_count();
        let ordering_count = execution_plan.query().ordering_count();
        let predicate_count = execution_plan.query().predicate_count();
        let fallback = execution_plan.query().fallback().clone();

        let seed = Self {
            grouping_aspect: grouping_aspect.clone(),
            identity_binding: identity_binding.clone(),
            grouping_binding: grouping_binding.clone(),
            grouped_binding_width,
            grouped_projection_width,
            traversal_count,
            ordering_count,
            predicate_count,
            fallback,
            baseline_materialization: GroupedBaselineMaterializationContract {
                grouping_aspect: grouping_aspect.clone(),
                identity_binding,
                grouping_binding,
                grouped_binding_width,
            },
            replay_delivery_posture: GroupedReplayDeliveryPosture::grouped_committed(),
            grouped_delta_policy: GroupedDeltaAdmissionPolicy::refresh_deferred_debt(),
        };
        let grouped_delta_policy = GroupedDeltaAdmissionPolicy::derive_from_grouped_planning(&seed);

        Some(Self {
            grouped_delta_policy,
            ..seed
        })
    }
}
