use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationConsumerKitError,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationSelectorCoverageDeclaration, ForgeQueryGraphObligationSupportPin,
};
use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationOperatingWorldSelector,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportMatrix, ForgeQueryGraphObligationSupportMatrixRow,
    ForgeQueryGraphObligationSupportStatus, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};
use topology::facade::{
    topology_primitive_construction_birth_graph_obligation_registration,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
};

use crate::construction::request::{PrimitiveConstructionFamily, PRIMITIVE_CONSTRUCTION_FAMILIES};

pub(crate) const PRIMITIVE_CONSTRUCTION_GRAPH_OBLIGATION_FAMILY: &str =
    "worth-kernel.primitive-construction";
pub(crate) const PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_OPERATION: &str = "set:topology.kind";
pub(crate) const PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_PATH: &str = "topology.kind";

#[derive(Clone, Debug)]
pub(crate) struct PrimitiveConstructionGraphObligationCatalog {
    rows: Vec<PrimitiveConstructionGraphObligationCatalogRow>,
}

#[derive(Clone, Debug)]
pub(crate) struct PrimitiveConstructionGraphObligationCatalogRow {
    family: PrimitiveConstructionFamily,
    descriptor_source: &'static str,
    registration: ForgeQueryGraphObligationRegistration,
}

impl PrimitiveConstructionGraphObligationCatalog {
    pub(crate) fn current() -> Self {
        Self {
            rows: PRIMITIVE_CONSTRUCTION_FAMILIES
                .into_iter()
                .map(PrimitiveConstructionGraphObligationCatalogRow::covered_compose_birth)
                .collect(),
        }
    }

    pub(crate) fn rows(&self) -> &[PrimitiveConstructionGraphObligationCatalogRow] {
        &self.rows
    }

    pub(crate) fn registrations(&self) -> Vec<ForgeQueryGraphObligationRegistration> {
        self.rows
            .iter()
            .map(|row| row.registration().clone())
            .collect()
    }
}

impl PrimitiveConstructionGraphObligationCatalogRow {
    fn covered_compose_birth(family: PrimitiveConstructionFamily) -> Self {
        Self {
            family,
            descriptor_source: "worth-topo primitive construction birth compose execution",
            registration: topology_primitive_construction_birth_graph_obligation_registration(
                ForgeQueryGraphObligationSupportLane::GraphComposition,
                ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
            ),
        }
    }

    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub(crate) fn descriptor_source(&self) -> &'static str {
        self.descriptor_source
    }

    pub(crate) fn registration(&self) -> &ForgeQueryGraphObligationRegistration {
        &self.registration
    }

    pub(crate) fn touch_selector(&self) -> &ForgeQueryGraphTouchSelector {
        self.registration.touch_selector()
    }
}

pub(crate) fn primitive_construction_graph_obligation_catalog(
) -> PrimitiveConstructionGraphObligationCatalog {
    PrimitiveConstructionGraphObligationCatalog::current()
}

pub(crate) fn primitive_construction_birth_touch_descriptor(
) -> Result<ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenial> {
    ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
        ForgeQueryMutationFamily::Insert,
        None,
        [
            PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_OPERATION,
            "set:topology.structure",
            "set:naming.persistent_name",
        ],
        [
            PRIMITIVE_CONSTRUCTION_BIRTH_ASPECT_PATH,
            "topology.structure",
            "naming.persistent_name",
        ],
    )
}

pub(crate) fn primitive_construction_graph_obligation_registration_declaration() -> Result<
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationConsumerKitError,
> {
    ForgeQueryGraphObligationConsumerRegistrationDeclaration::for_runtime_family(
        PRIMITIVE_CONSTRUCTION_GRAPH_OBLIGATION_FAMILY,
        primitive_construction_graph_obligation_catalog().registrations(),
    )
}

pub(crate) fn primitive_construction_graph_obligation_selector_coverage(
) -> ForgeQueryGraphObligationSelectorCoverageDeclaration {
    ForgeQueryGraphObligationSelectorCoverageDeclaration::required(
        primitive_construction_graph_obligation_catalog()
            .rows()
            .iter()
            .map(|row| (row.family().as_str(), row.touch_selector().clone())),
    )
}

pub(crate) fn primitive_construction_graph_obligation_support_pin(
) -> ForgeQueryGraphObligationSupportPin {
    ForgeQueryGraphObligationSupportPin::new([(
        ForgeQueryGraphObligationKind::AdvisoryObligation,
        ForgeQueryGraphObligationSupportLane::GraphComposition,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )])
}

pub(crate) fn primitive_construction_graph_obligation_support_matrix(
) -> ForgeQueryGraphObligationSupportMatrix {
    let mut rows = ForgeQueryGraphObligationKind::ALL
        .into_iter()
        .map(|kind| {
            ForgeQueryGraphObligationSupportMatrixRow::new(
                kind,
                ForgeQueryGraphObligationSupportLane::AssemblyIndexSelection,
                ForgeQueryGraphObligationSupportStatus::Supported,
            )
        })
        .collect::<Vec<_>>();
    rows.extend(ForgeQueryGraphObligationKind::ALL.into_iter().map(|kind| {
        ForgeQueryGraphObligationSupportMatrixRow::new(
            kind,
            ForgeQueryGraphObligationSupportLane::GraphComposition,
            primitive_construction_birth_lane_status(kind),
        )
    }));
    rows.extend(ForgeQueryGraphObligationKind::ALL.into_iter().map(|kind| {
        ForgeQueryGraphObligationSupportMatrixRow::new(
            kind,
            ForgeQueryGraphObligationSupportLane::WorthKernelPhaseChain,
            primitive_construction_phase_chain_lane_status(kind),
        )
    }));
    ForgeQueryGraphObligationSupportMatrix::new(rows)
}

fn primitive_construction_birth_lane_status(
    kind: ForgeQueryGraphObligationKind,
) -> ForgeQueryGraphObligationSupportStatus {
    match kind {
        ForgeQueryGraphObligationKind::AdvisoryObligation => {
            ForgeQueryGraphObligationSupportStatus::Supported
        }
        _ => ForgeQueryGraphObligationSupportStatus::NotApplicable,
    }
}

fn primitive_construction_phase_chain_lane_status(
    kind: ForgeQueryGraphObligationKind,
) -> ForgeQueryGraphObligationSupportStatus {
    match kind {
        ForgeQueryGraphObligationKind::AdvisoryObligation => {
            ForgeQueryGraphObligationSupportStatus::DeferredToBackstop
        }
        _ => ForgeQueryGraphObligationSupportStatus::NotApplicable,
    }
}
