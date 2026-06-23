use forge_query::facade::consumer_kit::ForgeQueryGraphObligationInMemoryTestWorkspace;
use forge_query::facade::{
    ForgeQueryGraphObligationOperatingWorldDescriptor, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchReadVerb, ForgeQueryMutationFamily,
};
use topology::facade::TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION;

use crate::query_obligation_selection::selection_substrate::{
    QuerySelectorPrecisionPosture, QuerySelectorPrecisionReport,
};

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
    touch_lookup_key_count: usize,
    operating_world_lookup_key_count: usize,
    attempted_bucket_lookup_count: usize,
    matched_bucket_count: usize,
    candidate_registration_count: usize,
    deduplicated_candidate_count: usize,
    denied_row_count: usize,
    residue_row_count: usize,
    registration_full_scan_count: usize,
    precision_posture: QuerySelectorPrecisionPosture,
    precision_report_digest: String,
    precision_counters_digest: String,
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

    pub(crate) fn touch_lookup_key_count(&self) -> usize {
        self.touch_lookup_key_count
    }

    pub(crate) fn operating_world_lookup_key_count(&self) -> usize {
        self.operating_world_lookup_key_count
    }

    pub(crate) fn attempted_bucket_lookup_count(&self) -> usize {
        self.attempted_bucket_lookup_count
    }

    pub(crate) fn matched_bucket_count(&self) -> usize {
        self.matched_bucket_count
    }

    pub(crate) fn candidate_registration_count(&self) -> usize {
        self.candidate_registration_count
    }

    pub(crate) fn deduplicated_candidate_count(&self) -> usize {
        self.deduplicated_candidate_count
    }

    pub(crate) fn denied_row_count(&self) -> usize {
        self.denied_row_count
    }

    pub(crate) fn residue_row_count(&self) -> usize {
        self.residue_row_count
    }

    pub(crate) fn registration_full_scan_count(&self) -> usize {
        self.registration_full_scan_count
    }

    pub(crate) fn precision_posture(&self) -> QuerySelectorPrecisionPosture {
        self.precision_posture
    }

    pub(crate) fn precision_report_digest(&self) -> &str {
        &self.precision_report_digest
    }

    pub(crate) fn precision_counters_digest(&self) -> &str {
        &self.precision_counters_digest
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
            let counters = proof.selection_counters();
            let selected_count = proof.selected_obligation_count();
            let precision_report = QuerySelectorPrecisionReport::from_counter_only_certification(
                counters,
                selected_count,
            );
            Ok(PrimitiveConstructionGraphObligationSelectorPrecisionRow {
                label: precision_case.label,
                descriptor_digest: precision_case.descriptor.descriptor_digest().to_string(),
                expected_selected_count: precision_case.expected_selected_count,
                selected_count,
                touch_lookup_key_count: counters.touch_lookup_key_count(),
                operating_world_lookup_key_count: counters.operating_world_lookup_key_count(),
                attempted_bucket_lookup_count: counters.attempted_bucket_lookup_count(),
                matched_bucket_count: counters.matched_bucket_count(),
                candidate_registration_count: counters.candidate_registration_count(),
                deduplicated_candidate_count: counters.deduplicated_candidate_count(),
                denied_row_count: counters
                    .candidate_registration_count()
                    .saturating_sub(selected_count),
                residue_row_count: 0,
                registration_full_scan_count: counters.registration_full_scan_count(),
                precision_posture: precision_report.posture(),
                precision_report_digest: precision_report.report_digest().to_string(),
                precision_counters_digest: precision_report.counters_digest().to_string(),
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
            expected_selected_count: 0,
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
            expected_selected_count: 0,
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
            expected_selected_count: 0,
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
            expected_selected_count: 0,
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
