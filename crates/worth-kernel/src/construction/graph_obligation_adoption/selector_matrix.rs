use forge_query::facade::consumer_kit::ForgeQueryGraphObligationInMemoryTestWorkspace;
use forge_query::facade::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchReadVerb, ForgeQueryMutationFamily,
};
use topology::facade::TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION;

use super::catalog::{
    primitive_construction_birth_touch_descriptor, primitive_construction_graph_obligation_catalog,
    PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_OPERATION, PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_PATH,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionGraphObligationSelectorPrecisionRow {
    label: &'static str,
    descriptor_digest: String,
    expected_selected_count: usize,
    selected_count: usize,
}

struct PrimitiveConstructionGraphObligationSelectorPrecisionCase {
    label: &'static str,
    expected_selected_count: usize,
    descriptor: ForgeQueryGraphTouchDescriptor,
}

impl PrimitiveConstructionGraphObligationSelectorPrecisionRow {
    pub(crate) fn label(&self) -> &'static str {
        self.label
    }

    pub(crate) fn descriptor_digest(&self) -> &str {
        &self.descriptor_digest
    }

    pub(crate) fn expected_selected_count(&self) -> usize {
        self.expected_selected_count
    }

    pub(crate) fn selected_count(&self) -> usize {
        self.selected_count
    }
}

pub(crate) fn primitive_construction_graph_obligation_selector_precision_matrix() -> Result<
    Vec<PrimitiveConstructionGraphObligationSelectorPrecisionRow>,
    ForgeQueryGraphTouchDescriptorDenial,
> {
    let workspace = ForgeQueryGraphObligationInMemoryTestWorkspace::from_registrations(
        primitive_construction_graph_obligation_catalog().registrations(),
    )
    .expect("in-memory primitive construction registration workspace");
    let operating_world =
        ForgeQueryGraphObligationOperatingWorldDescriptor::configured_domain_handle();

    selector_precision_descriptors()?
        .into_iter()
        .map(|precision_case| {
            let proof = workspace.prove_selection(&precision_case.descriptor, &operating_world);
            Ok(PrimitiveConstructionGraphObligationSelectorPrecisionRow {
                label: precision_case.label,
                descriptor_digest: precision_case.descriptor.descriptor_digest().to_string(),
                expected_selected_count: precision_case.expected_selected_count,
                selected_count: proof.selected_obligation_count(),
            })
        })
        .collect()
}

fn selector_precision_descriptors() -> Result<
    Vec<PrimitiveConstructionGraphObligationSelectorPrecisionCase>,
    ForgeQueryGraphTouchDescriptorDenial,
> {
    Ok(vec![
        PrimitiveConstructionGraphObligationSelectorPrecisionCase {
            label: "unrelated-collection",
            expected_selected_count: 0,
            descriptor: ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
                "unrelated.kernel.collection",
                ForgeQueryMutationFamily::Insert,
                None,
                ["insert:unrelated"],
                ["unrelated.path"],
            )?,
        },
        PrimitiveConstructionGraphObligationSelectorPrecisionCase {
            label: "wrong-mutation-family",
            expected_selected_count: 1,
            descriptor: ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
                TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
                ForgeQueryMutationFamily::Update,
                None,
                [PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_OPERATION],
                [PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_PATH],
            )?,
        },
        PrimitiveConstructionGraphObligationSelectorPrecisionCase {
            label: "wrong-aspect-operation",
            expected_selected_count: 1,
            descriptor: ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
                TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
                ForgeQueryMutationFamily::Insert,
                None,
                ["insert:primitive-construction.unrelated"],
                [PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_PATH],
            )?,
        },
        PrimitiveConstructionGraphObligationSelectorPrecisionCase {
            label: "wrong-aspect-path",
            expected_selected_count: 1,
            descriptor: ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
                TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
                ForgeQueryMutationFamily::Insert,
                None,
                [PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_OPERATION],
                ["primitive-construction.unrelated"],
            )?,
        },
        PrimitiveConstructionGraphObligationSelectorPrecisionCase {
            label: "read-not-mutation",
            expected_selected_count: 1,
            descriptor: ForgeQueryGraphTouchDescriptor::read_family(
                TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
                [ForgeQueryGraphTouchReadVerb::ObservesCollection],
            )?,
        },
        PrimitiveConstructionGraphObligationSelectorPrecisionCase {
            label: "positive-control",
            expected_selected_count: 1,
            descriptor: primitive_construction_birth_touch_descriptor()?,
        },
    ])
}
