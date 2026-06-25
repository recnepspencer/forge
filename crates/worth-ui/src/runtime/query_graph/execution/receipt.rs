mod composition_access;
mod row;

use forge_query::facade::consumer_kit::ForgeQueryGraphObligationExecutionProof;
use forge_query::facade::runtime::ForgeQueryGraphObligationKind;
use std::collections::BTreeMap;

use super::super::{
    WorthUiQueryGraphObligationSemantic, WorthUiQueryGraphOperatingWorld,
    WorthUiQueryGraphTouchDescriptor,
};
use super::adapter::WorthUiQueryGraphExecutionAdapter;
use super::digest::execution_digest;
use super::registrations::{
    composition_context_registrations, composition_participation_registrations,
    composition_topology_registrations, live_view_conditional_projection_registrations,
    live_view_control_projection_registrations, live_view_expression_projection_registrations,
    live_view_interaction_intent_registrations, live_view_payload_projection_registrations,
    live_view_readiness_projection_registrations, live_view_state_binding_registrations,
    mounted_interaction_registrations,
    primitive_construction_registrations, primitive_content_anatomy_registrations,
    primitive_event_dispatch_registrations, user_intent_target_binding_registrations,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryGraphExecutionReceipt {
    touch_descriptor: WorthUiQueryGraphTouchDescriptor,
    operating_world: WorthUiQueryGraphOperatingWorld,
    rows: Vec<WorthUiQueryGraphExecutionRow>,
    selected_obligation_count: usize,
    proof_digest: String,
    execution_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryGraphExecutionRow {
    semantic: WorthUiQueryGraphObligationSemantic,
    canonical_kind: ForgeQueryGraphObligationKind,
    support_lane: String,
    support_status: String,
    execution_status: String,
    rule_identity_digest: String,
    registration_digest: String,
    row_digest: String,
}

impl WorthUiQueryGraphExecutionReceipt {
    pub(crate) fn primitive_construction(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            primitive_construction_registrations(),
            "Worth primitive construction graph registrations are generated from validated constants",
        )
    }

    pub(crate) fn composition_topology(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            composition_topology_registrations(),
            "Worth composition topology registrations are generated from validated constants",
        )
    }

    pub(crate) fn composition_context_propagation(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            composition_context_registrations(),
            "Worth composition context registrations are generated from validated constants",
        )
    }

    pub(crate) fn composition_participation(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            composition_participation_registrations(),
            "Worth composition participation registrations are generated from validated constants",
        )
    }

    pub(crate) fn mounted_interaction_activation(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            mounted_interaction_registrations(),
            "Worth query graph registrations are generated from validated constants",
        )
    }

    pub(crate) fn primitive_event_dispatch(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            primitive_event_dispatch_registrations(),
            "Worth primitive event graph registrations are generated from validated constants",
        )
    }

    pub(crate) fn primitive_content_anatomy(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            primitive_content_anatomy_registrations(),
            "Worth primitive content graph registrations are generated from validated constants",
        )
    }

    pub(crate) fn user_intent_target_binding(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            user_intent_target_binding_registrations(),
            "Worth user intent target registrations are generated from validated constants",
        )
    }

    pub(crate) fn live_view_state_binding(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            live_view_state_binding_registrations(),
            "Worth live view registrations are generated from validated constants",
        )
    }

    pub(crate) fn live_view_control_projection(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            live_view_control_projection_registrations(),
            "Worth live view control registrations are generated from validated constants",
        )
    }

    pub(crate) fn live_view_conditional_projection(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            live_view_conditional_projection_registrations(),
            "Worth live view conditional registrations are generated from validated constants",
        )
    }

    pub(crate) fn live_view_readiness_projection(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            live_view_readiness_projection_registrations(),
            "Worth live view readiness registrations are generated from validated constants",
        )
    }

    pub(crate) fn live_view_expression_projection(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            live_view_expression_projection_registrations(),
            "Worth live view expression registrations are generated from validated constants",
        )
    }

    pub(crate) fn live_view_interaction_intent(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            live_view_interaction_intent_registrations(),
            "Worth live view interaction registrations are generated from validated constants",
        )
    }

    pub(crate) fn live_view_payload_projection(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
    ) -> Self {
        Self::from_registrations(
            touch_descriptor,
            operating_world,
            live_view_payload_projection_registrations(),
            "Worth live view payload registrations are generated from validated constants",
        )
    }

    fn from_registrations(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
        registrations: Vec<(
            WorthUiQueryGraphObligationSemantic,
            forge_query::facade::runtime::ForgeQueryGraphObligationRegistration,
        )>,
        expect_message: &str,
    ) -> Self {
        let semantic_by_registration = semantic_map(&registrations);
        let adapter = WorthUiQueryGraphExecutionAdapter::from_registrations(
            registrations.into_iter().map(|row| row.1),
        )
        .expect(expect_message);
        let proof = adapter.execute(&touch_descriptor, &operating_world);
        Self::from_execution_proof(
            touch_descriptor,
            operating_world,
            proof,
            &semantic_by_registration,
        )
    }

    fn from_execution_proof(
        touch_descriptor: WorthUiQueryGraphTouchDescriptor,
        operating_world: WorthUiQueryGraphOperatingWorld,
        proof: ForgeQueryGraphObligationExecutionProof,
        semantic_by_registration: &BTreeMap<String, WorthUiQueryGraphObligationSemantic>,
    ) -> Self {
        let selected = proof.selection_proof().selected_obligations();
        let rows = proof
            .rows()
            .iter()
            .zip(selected.iter())
            .map(|(row, selected)| {
                let semantic = semantic_by_registration
                    .get(selected.registration_digest())
                    .copied()
                    .expect("selected Query registration must come from Worth semantic map");
                WorthUiQueryGraphExecutionRow {
                    semantic,
                    canonical_kind: selected.obligation_kind(),
                    support_lane: selected.support_lane().as_str().to_owned(),
                    support_status: selected.support_status().as_str().to_owned(),
                    execution_status: row.status().as_str().to_owned(),
                    rule_identity_digest: selected.rule_identity_digest().to_owned(),
                    registration_digest: selected.registration_digest().to_owned(),
                    row_digest: row.row_digest().to_owned(),
                }
            })
            .collect::<Vec<_>>();
        let selected_obligation_count = proof.selected_obligation_count();
        let proof_digest = proof.proof_digest().to_owned();
        let execution_digest = execution_digest(
            touch_descriptor.descriptor_digest(),
            operating_world.descriptor_digest(),
            proof.proof_digest(),
            &rows,
        );
        Self {
            touch_descriptor,
            operating_world,
            rows,
            selected_obligation_count,
            proof_digest,
            execution_digest,
        }
    }

    pub fn touch_descriptor(&self) -> &WorthUiQueryGraphTouchDescriptor {
        &self.touch_descriptor
    }

    pub fn operating_world(&self) -> &WorthUiQueryGraphOperatingWorld {
        &self.operating_world
    }

    pub fn rows(&self) -> &[WorthUiQueryGraphExecutionRow] {
        &self.rows
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }

    pub fn execution_digest(&self) -> u64 {
        self.execution_digest
    }
}

fn semantic_map(
    registrations: &[(
        WorthUiQueryGraphObligationSemantic,
        forge_query::facade::runtime::ForgeQueryGraphObligationRegistration,
    )],
) -> BTreeMap<String, WorthUiQueryGraphObligationSemantic> {
    registrations
        .iter()
        .map(|(semantic, registration)| (registration.registration_digest().to_owned(), *semantic))
        .collect()
}
