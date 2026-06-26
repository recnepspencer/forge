use forge_query::facade::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch,
    ForgeQueryGraphObligationRegistrationDenial, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchLifecycleFamily,
    ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};
use schema::facade::platform::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};

use crate::topology_operators::{
    TopologyGraphLifecyclePosture, TopologyTouchedAspect, TopologyTouchedScope,
    TOPOLOGY_OPERATOR_RELATION_COLLECTION,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorthTopologyTouchedApplicability {
    required_aspects: Vec<TopologyTouchedAspect>,
    required_scopes: Vec<TopologyTouchedScope>,
    lifecycle_posture: TopologyGraphLifecyclePosture,
}

impl WorthTopologyTouchedApplicability {
    pub(in crate::validator_invariant_catalog) fn from_parts(
        required_aspects: impl IntoIterator<Item = TopologyTouchedAspect>,
        required_scopes: impl IntoIterator<Item = TopologyTouchedScope>,
        lifecycle_posture: TopologyGraphLifecyclePosture,
    ) -> Self {
        Self {
            required_aspects: sorted_unique(required_aspects),
            required_scopes: sorted_unique(required_scopes),
            lifecycle_posture,
        }
    }

    pub fn required_aspects(&self) -> &[TopologyTouchedAspect] {
        &self.required_aspects
    }

    pub fn required_scopes(&self) -> &[TopologyTouchedScope] {
        &self.required_scopes
    }

    pub const fn lifecycle_posture(&self) -> TopologyGraphLifecyclePosture {
        self.lifecycle_posture
    }

    pub fn query_touch_selector(
        &self,
    ) -> Result<ForgeQueryGraphTouchSelector, ForgeQueryGraphObligationRegistrationDenial> {
        ForgeQueryGraphTouchSelector::declared_mutation_collection(
            TOPOLOGY_OPERATOR_RELATION_COLLECTION,
            mutation_family_for_lifecycle(self.lifecycle_posture),
            self.required_aspects
                .iter()
                .copied()
                .map(|aspect| ForgeQueryAspectMutationOperation::set(query_aspect_touch(aspect))),
            self.required_aspects
                .iter()
                .copied()
                .map(query_aspect_touch),
        )
    }

    pub fn query_touch_descriptor(
        &self,
    ) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial> {
        ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            TOPOLOGY_OPERATOR_RELATION_COLLECTION,
            mutation_family_for_lifecycle(self.lifecycle_posture),
            Some(lifecycle_family_for_lifecycle(self.lifecycle_posture)),
            self.required_aspects
                .iter()
                .copied()
                .map(|aspect| ForgeQueryAspectMutationOperation::set(query_aspect_touch(aspect))),
            self.required_aspects
                .iter()
                .copied()
                .map(query_aspect_touch),
        )
    }

    pub fn digest_part(&self) -> String {
        let aspects = self
            .required_aspects
            .iter()
            .map(|aspect| aspect.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let scopes = self
            .required_scopes
            .iter()
            .map(|scope| scope.as_str())
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "applicability:{}:{}:{}",
            self.lifecycle_posture.as_str(),
            aspects,
            scopes
        )
    }
}

fn sorted_unique<T: Ord>(values: impl IntoIterator<Item = T>) -> Vec<T> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn mutation_family_for_lifecycle(
    lifecycle: TopologyGraphLifecyclePosture,
) -> ForgeQueryMutationFamily {
    match lifecycle {
        TopologyGraphLifecyclePosture::EntityCreation
        | TopologyGraphLifecyclePosture::ExistingRelationCreate => ForgeQueryMutationFamily::Insert,
        TopologyGraphLifecyclePosture::EntityRetirement
        | TopologyGraphLifecyclePosture::ExistingRelationRemoval => {
            ForgeQueryMutationFamily::Delete
        }
        TopologyGraphLifecyclePosture::ExistingRelationRetarget => ForgeQueryMutationFamily::Update,
    }
}

fn lifecycle_family_for_lifecycle(
    lifecycle: TopologyGraphLifecyclePosture,
) -> ForgeQueryGraphTouchLifecycleFamily {
    match lifecycle {
        TopologyGraphLifecyclePosture::EntityCreation
        | TopologyGraphLifecyclePosture::ExistingRelationCreate => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetFollowup
        }
        TopologyGraphLifecyclePosture::EntityRetirement
        | TopologyGraphLifecyclePosture::ExistingRelationRemoval => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetirement
        }
        TopologyGraphLifecyclePosture::ExistingRelationRetarget => {
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetarget
        }
    }
}

fn query_aspect_touch(aspect: TopologyTouchedAspect) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::whole_aspect(schema_aspect_for_touched_aspect(aspect).aspect_key())
}

fn schema_aspect_for_touched_aspect(aspect: TopologyTouchedAspect) -> Aspect {
    match aspect {
        TopologyTouchedAspect::TopologyStructure => Aspect::Topology(TopologyAspect::Structure),
        TopologyTouchedAspect::TopologyOwnership => Aspect::Topology(TopologyAspect::Ownership),
        TopologyTouchedAspect::TopologyBoundary => Aspect::Topology(TopologyAspect::Boundary),
        TopologyTouchedAspect::TopologyRadial => Aspect::Topology(TopologyAspect::Radial),
        TopologyTouchedAspect::GeometryBinding => Aspect::Geometry(GeometryAspect::Binding),
        TopologyTouchedAspect::GeometryEmbedding => Aspect::Geometry(GeometryAspect::Embedding),
        TopologyTouchedAspect::GeometryProvenance => Aspect::Geometry(GeometryAspect::Provenance),
        TopologyTouchedAspect::GeometryApproximation => {
            Aspect::Geometry(GeometryAspect::Approximation)
        }
        TopologyTouchedAspect::GeometryUvAnchoring => Aspect::Geometry(GeometryAspect::UvAnchoring),
        TopologyTouchedAspect::GeometryCarrier => Aspect::Geometry(GeometryAspect::Carrier),
        TopologyTouchedAspect::GeometryPrecision => Aspect::Geometry(GeometryAspect::Precision),
        TopologyTouchedAspect::GeometryFallback => Aspect::Geometry(GeometryAspect::Fallback),
        TopologyTouchedAspect::LineageProvenance => Aspect::Lineage(LineageAspect::Provenance),
        TopologyTouchedAspect::NamingPersistentName => Aspect::Naming(NamingAspect::PersistentName),
        TopologyTouchedAspect::DiagnosticsDecisions => {
            Aspect::Diagnostics(DiagnosticsAspect::Decisions)
        }
        TopologyTouchedAspect::DiagnosticsInterpretations => {
            Aspect::Diagnostics(DiagnosticsAspect::Interpretations)
        }
    }
}
