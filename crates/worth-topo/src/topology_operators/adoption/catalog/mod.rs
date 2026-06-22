use forge_query::facade::{
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportPosture, ForgeQueryGraphTouchLifecycleFamily,
    ForgeQueryGraphTouchSelector,
};

mod catalog_row;
mod operator_touch_descriptor;
mod registration_declaration;
mod selector_coverage;
mod support_pin;

pub use catalog_row::{
    TopologyOperatorGraphObligationAdoptionStatus, TopologyOperatorGraphObligationCatalogRow,
    TopologyOperatorGraphObligationLoweringPath,
};
pub use operator_touch_descriptor::{
    topology_operator_command_batch_equivalent_touch_descriptor,
    topology_operator_relation_touch_descriptor, TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION,
    TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH,
};
pub use registration_declaration::topology_operator_graph_obligation_registration_declaration;
pub(crate) use registration_declaration::topology_operator_runtime_graph_obligation_registrations;
pub use selector_coverage::topology_operator_graph_obligation_selector_coverage;
pub use support_pin::{
    topology_operator_graph_obligation_support_matrix,
    topology_operator_graph_obligation_support_pin,
};

pub const TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY: &str = "worth-topo.operator-catalog";
pub const TOPOLOGY_OPERATOR_RELATION_COLLECTION: &str = "TopologyRelation";

#[derive(Clone, Debug)]
pub struct TopologyOperatorGraphObligationCatalog {
    rows: Vec<TopologyOperatorGraphObligationCatalogRow>,
}

impl TopologyOperatorGraphObligationCatalog {
    pub fn current() -> Self {
        Self {
            rows: current_operator_adoption_rows(),
        }
    }

    pub fn rows(&self) -> &[TopologyOperatorGraphObligationCatalogRow] {
        &self.rows
    }

    pub fn covered_rows(&self) -> impl Iterator<Item = &TopologyOperatorGraphObligationCatalogRow> {
        self.rows.iter().filter(|row| {
            row.adoption_status() == TopologyOperatorGraphObligationAdoptionStatus::Covered
        })
    }

    pub fn residue_rows(&self) -> impl Iterator<Item = &TopologyOperatorGraphObligationCatalogRow> {
        self.rows.iter().filter(|row| {
            row.adoption_status() == TopologyOperatorGraphObligationAdoptionStatus::Residue
        })
    }

    pub fn registrations(&self) -> Vec<ForgeQueryGraphObligationRegistration> {
        self.covered_rows()
            .filter_map(|row| row.registration().cloned())
            .collect()
    }

    pub fn runtime_graph_composition_registrations(
        &self,
    ) -> Vec<ForgeQueryGraphObligationRegistration> {
        self.covered_rows()
            .filter(|row| {
                row.lowering_path() == TopologyOperatorGraphObligationLoweringPath::GraphComposition
            })
            .filter_map(|row| row.registration().cloned())
            .collect()
    }
}

pub fn topology_operator_graph_obligation_catalog() -> TopologyOperatorGraphObligationCatalog {
    TopologyOperatorGraphObligationCatalog::current()
}

fn current_operator_adoption_rows() -> Vec<TopologyOperatorGraphObligationCatalogRow> {
    vec![
        TopologyOperatorGraphObligationCatalogRow::covered(
            "topology.rewire_loop_successor_program",
            "retargets TopologyRelation successor edges",
            "declaration-entry grouped rewire program",
            TopologyOperatorGraphObligationLoweringPath::GraphComposition,
            topology_rewire_loop_successor_runtime_registration(),
        ),
        TopologyOperatorGraphObligationCatalogRow::covered(
            "topology.rewire_loop_successor_program",
            "retargets TopologyRelation successor edges",
            "contribution orchestration declaration family",
            TopologyOperatorGraphObligationLoweringPath::ContributionOrchestration,
            topology_rewire_loop_successor_contribution_registration(),
        ),
        TopologyOperatorGraphObligationCatalogRow::residue(
            "topology.rehome_all_owned_half_edges_to_new_wire",
            "reassigns wire ownership for a half-edge set",
            "grouped declaration entry command-batch helper",
            TopologyOperatorGraphObligationLoweringPath::AuthoritativeCommandBatch,
            "wire-rehome-command-batch-operator",
        ),
        TopologyOperatorGraphObligationCatalogRow::residue(
            "topology.rehome_all_owned_faces_to_new_shell",
            "reassigns shell ownership for a face set",
            "grouped declaration entry command-batch helper",
            TopologyOperatorGraphObligationLoweringPath::AuthoritativeCommandBatch,
            "shell-membership-command-batch-operator",
        ),
        TopologyOperatorGraphObligationCatalogRow::residue(
            "topology.create_inner_loop_on_existing_face",
            "adds face inner-loop membership",
            "grouped declaration entry command-batch helper",
            TopologyOperatorGraphObligationLoweringPath::AuthoritativeCommandBatch,
            "face-inner-loop-command-batch-operator",
        ),
        TopologyOperatorGraphObligationCatalogRow::residue(
            "topology.scalar_mutation_fronts",
            "scalar topology mutation fronts",
            "scalar declaration entry operators",
            TopologyOperatorGraphObligationLoweringPath::ScalarMutation,
            "scalar-topology-mutation-fronts",
        ),
        TopologyOperatorGraphObligationCatalogRow::residue(
            "topology.reference_integrity.milestone_one",
            "reference integrity commit and graph-composition backstops",
            "relational custom invariant registrations",
            TopologyOperatorGraphObligationLoweringPath::RelationalInvariantBackstop,
            "milestone-one-reference-integrity-pack",
        ),
        TopologyOperatorGraphObligationCatalogRow::residue(
            "topology.local_guard.existing_entity_incoming_relation_count",
            "incoming relation-count local guard",
            "topology operator local rewrite admission guards",
            TopologyOperatorGraphObligationLoweringPath::RelationalInvariantBackstop,
            "existing-entity-incoming-relation-count-mismatch-guards",
        ),
    ]
}

fn topology_rewire_loop_successor_runtime_registration() -> ForgeQueryGraphObligationRegistration {
    topology_rewire_loop_successor_registration(
        ForgeQueryGraphObligationSupportLane::GraphComposition,
        ForgeQueryGraphObligationOperatingWorldSelector::any_operating_world(),
    )
}

fn topology_rewire_loop_successor_contribution_registration(
) -> ForgeQueryGraphObligationRegistration {
    topology_rewire_loop_successor_registration(
        ForgeQueryGraphObligationSupportLane::ContributionOrchestration,
        ForgeQueryGraphObligationOperatingWorldSelector::configured_domain_handle(),
    )
}

pub(crate) fn topology_rewire_loop_successor_registration(
    support_lane: ForgeQueryGraphObligationSupportLane,
    operating_world_selector: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::advisory_obligation(
        ForgeQueryGraphObligationRuleIdentity::new(
            "worth-topo.topology-operator",
            "topology.rewire_loop_successor_program.graph-obligation",
            "v1",
        )
        .expect("topology rewire successor graph obligation identity is static and non-empty"),
        ForgeQueryGraphTouchSelector::lifecycle_family(
            ForgeQueryGraphTouchLifecycleFamily::VerifiedExistingTargetRetarget,
        ),
        operating_world_selector,
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::diagnostic_only(
        support_lane,
    ))
}
