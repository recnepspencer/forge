use forge_query::facade::{
    ForgeQueryAspectMutationOperation, ForgeQueryGraphObligationOperatingWorldSelector,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial,
    ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};

use crate::query_native_runtime_boundary::TopologyNativeQueryRowField;

pub const TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION: &str =
    "TopologyPrimitiveConstructionBirth";
pub(crate) const TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_LAYOUT_VIOLATION_COLLECTION: &str =
    "TopologyPrimitiveConstructionBirthLayoutViolation";

pub(super) fn topology_primitive_construction_birth_touch_descriptor(
) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial> {
    ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
        ForgeQueryMutationFamily::Insert,
        None,
        construction_birth_operations(),
        construction_birth_touches(),
    )
}

pub fn topology_primitive_construction_birth_graph_obligation_registration(
    support_lane: ForgeQueryGraphObligationSupportLane,
    operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::advisory_obligation(
        ForgeQueryGraphObligationRuleIdentity::new(
            "worth-topo.primitive-construction",
            "primitive-construction-birth-compose.graph-obligation",
            "v1",
        )
        .expect("primitive construction birth graph obligation identity is static and non-empty"),
        ForgeQueryGraphTouchSelector::declared_mutation_collection(
            TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
            ForgeQueryMutationFamily::Insert,
            construction_birth_operations(),
            construction_birth_touches(),
        )
        .expect("primitive construction birth selector is static and non-empty"),
        operating_world_selector,
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
        support_lane,
    ))
}

pub(crate) fn topology_primitive_construction_birth_layout_violation_registration(
    support_lane: ForgeQueryGraphObligationSupportLane,
    operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new(
            "worth-topo.primitive-construction",
            "primitive-construction-birth-layout-violation.graph-obligation",
            "v1",
        )
        .expect(
            "primitive construction layout violation graph obligation identity is static and non-empty",
        ),
        ForgeQueryGraphTouchSelector::collection(
            TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_LAYOUT_VIOLATION_COLLECTION,
        )
        .expect("primitive construction layout violation selector is static and non-empty"),
        operating_world_selector,
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::unsupported(
        support_lane,
    ))
}

fn construction_birth_operations() -> Vec<ForgeQueryAspectMutationOperation> {
    construction_birth_touches()
        .into_iter()
        .map(ForgeQueryAspectMutationOperation::set)
        .collect()
}

fn construction_birth_touches() -> Vec<forge_query::facade::ForgeQueryAspectTouch> {
    [
        TopologyNativeQueryRowField::TopologyKind,
        TopologyNativeQueryRowField::TopologyStructure,
        TopologyNativeQueryRowField::NamingPersistentName,
    ]
    .into_iter()
    .map(TopologyNativeQueryRowField::touch)
    .collect()
}
