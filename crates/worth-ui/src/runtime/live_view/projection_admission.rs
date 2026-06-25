mod content_projection;
mod ordering;
mod receipt;
mod view_primitive_binding;

use crate::runtime::live_view::conditional_projection::admission::{
    conditional_projection_denials, lower_live_view_conditional_projection_receipts,
};
use crate::runtime::live_view::control_projection::admission::{
    control_has_denial, control_projection_denials, lower_live_view_control_projection_receipts,
};
use crate::runtime::live_view::interaction_intent::{
    interaction_intent_denials, lower_live_view_interaction_intents,
};
use crate::runtime::live_view::payload_projection::{
    lower_live_view_payload_projection_receipts_for_bindings, payload_denials,
};
use crate::runtime::live_view::readiness_projection::{
    lower_live_view_readiness_receipts, readiness_denials,
};
use crate::runtime::{
    admit_composition_context_propagation, composition_participation_denial_report_for_graph,
    WorthUiAdmittedCompositionGraphReceipt, WorthUiAuthoredLiveViewDeclaration,
    WorthUiCompositionChildSizing, WorthUiCompositionGraphDefinition,
    WorthUiCompositionNodeDefinition, WorthUiCompositionNodeKind, WorthUiCompositionRootDefinition,
    WorthUiCompositionRootKind, WorthUiLiveViewConditionalProjectionDeclaration,
    WorthUiLiveViewControlProjectionDeclaration, WorthUiLiveViewControlProjectionReceipt,
    WorthUiLiveViewDeclarationReceipt, WorthUiRuntimeHost,
};
use std::collections::BTreeSet;

pub use receipt::{
    WorthUiGraphBackedLiveViewProjectionReceipt, WorthUiLiveViewProjectionAdmissionCounters,
    WorthUiLiveViewProjectionAdmissionDenial, WorthUiLiveViewProjectionAdmissionReceipt,
    WorthUiLiveViewProjectionAdmissionReport,
};

use content_projection::{composition_content_denials, lower_composition_content_receipts};
use ordering::{
    authored_projection_denials, conditional_has_denial, projection_denials, readiness_has_denial,
};
use view_primitive_binding::{
    append_live_view_primitive_denials, lower_live_view_default_primitive_binding,
    lower_live_view_primitive_binding,
};

impl WorthUiRuntimeHost {
    pub fn admit_live_view_projections(
        &self,
        live_view: &WorthUiLiveViewDeclarationReceipt,
        control_declarations: &[WorthUiLiveViewControlProjectionDeclaration],
        conditional_declarations: &[WorthUiLiveViewConditionalProjectionDeclaration],
    ) -> Result<WorthUiLiveViewProjectionAdmissionReceipt, WorthUiLiveViewProjectionAdmissionReport>
    {
        let control_denials = control_projection_denials(self, live_view, control_declarations);
        let admissible_controls = lower_admissible_controls(self, live_view, control_declarations);
        let conditional_denials = conditional_projection_denials(
            live_view,
            &admissible_controls,
            conditional_declarations,
        );
        let denials = projection_denials(control_denials, conditional_denials);
        let counters = WorthUiLiveViewProjectionAdmissionCounters::new(
            control_declarations.len(),
            conditional_declarations.len(),
            0,
            0,
            0,
            denials.len(),
        );
        if !denials.is_empty() {
            return Err(WorthUiLiveViewProjectionAdmissionReport::denied(
                denials, counters,
            ));
        }
        let conditionals = lower_live_view_conditional_projection_receipts(
            self,
            live_view,
            &admissible_controls,
            conditional_declarations,
        );
        let view_primitives =
            lower_live_view_default_primitive_binding(self, live_view.live_view_id());
        Ok(WorthUiLiveViewProjectionAdmissionReceipt::new(
            live_view.live_view_id(),
            view_primitives.flow_layout,
            view_primitives.appearance,
            admissible_controls,
            conditionals,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            Vec::new(),
            counters,
        ))
    }

    pub fn admit_authored_live_view_projections(
        &self,
        live_view: &WorthUiLiveViewDeclarationReceipt,
        authored: &WorthUiAuthoredLiveViewDeclaration,
    ) -> Result<WorthUiLiveViewProjectionAdmissionReceipt, WorthUiLiveViewProjectionAdmissionReport>
    {
        self.admit_graph_backed_authored_live_view_projections(live_view, authored)
            .map(|receipt| receipt.projection().clone())
    }

    pub fn admit_graph_backed_authored_live_view_projections(
        &self,
        live_view: &WorthUiLiveViewDeclarationReceipt,
        authored: &WorthUiAuthoredLiveViewDeclaration,
    ) -> Result<WorthUiGraphBackedLiveViewProjectionReceipt, WorthUiLiveViewProjectionAdmissionReport>
    {
        let control_denials = control_projection_denials(self, live_view, authored.controls());
        let admissible_controls = lower_admissible_controls(self, live_view, authored.controls());
        let conditional_denials = conditional_projection_denials(
            live_view,
            &admissible_controls,
            authored.conditionals(),
        );
        let lowerable_conditionals = authored
            .conditionals()
            .iter()
            .filter(|conditional| {
                !conditional_has_denial(&conditional_denials, conditional.control_id())
            })
            .cloned()
            .collect::<Vec<_>>();
        let admissible_conditionals = lower_live_view_conditional_projection_receipts(
            self,
            live_view,
            &admissible_controls,
            &lowerable_conditionals,
        );
        let readiness_denials = readiness_denials(live_view, authored.readinesses());
        let lowerable_readinesses = authored
            .readinesses()
            .iter()
            .filter(|readiness| !readiness_has_denial(&readiness_denials, readiness.readiness_id()))
            .cloned()
            .collect::<Vec<_>>();
        let admissible_readinesses = lower_live_view_readiness_receipts(
            self,
            live_view,
            &admissible_conditionals,
            &lowerable_readinesses,
        );
        let payload_denials = payload_denials(authored.payloads());
        let payload_consumed_bindings = authored_payload_consumed_bindings(authored);
        let admissible_payloads = lower_live_view_payload_projection_receipts_for_bindings(
            self,
            live_view,
            authored.payloads(),
            payload_consumed_bindings.iter().map(String::as_str),
        );
        let interaction_denials = interaction_intent_denials(
            self,
            live_view,
            &admissible_readinesses,
            &admissible_payloads,
            authored.interactions(),
        );
        let composition_result = authored.composition().map(|composition| {
            composition.admit_source(
                &authored_control_ids(authored),
                &authored_interaction_ids(authored),
            )
        });
        let mut denials = authored_projection_denials(
            authored.projections(),
            control_denials,
            conditional_denials,
            readiness_denials,
            payload_denials,
            interaction_denials,
        );
        if let Some(Err(composition_report)) = &composition_result {
            denials.extend(
                composition_report
                    .denials()
                    .iter()
                    .cloned()
                    .map(WorthUiLiveViewProjectionAdmissionDenial::CompositionSource),
            );
        }
        denials.extend(
            composition_content_denials(self, authored.composition())
                .into_iter()
                .map(WorthUiLiveViewProjectionAdmissionDenial::PrimitiveContent),
        );
        append_live_view_primitive_denials(self, authored, &mut denials);
        let counters = WorthUiLiveViewProjectionAdmissionCounters::new(
            authored.controls().len(),
            authored.conditionals().len(),
            authored.readinesses().len(),
            authored.payloads().len(),
            authored.interactions().len(),
            denials.len(),
        );
        if !denials.is_empty() {
            return Err(WorthUiLiveViewProjectionAdmissionReport::denied(
                denials, counters,
            ));
        }
        let composition_graph = match composition_result {
            Some(Ok(composition_graph)) => composition_graph,
            Some(Err(_)) => {
                unreachable!("composition source denial would have returned before lowering")
            }
            None => default_live_view_composition_graph(authored),
        };
        let context_definitions = authored
            .composition()
            .map(|composition| composition.contexts())
            .unwrap_or_default();
        let participation_report = authored
            .composition()
            .map(|composition| {
                composition_participation_denial_report_for_graph(
                    &composition_graph,
                    composition.accessibility_associations(),
                )
            })
            .unwrap_or_default();
        if let Some(participation_report) = participation_report {
            denials.extend(
                participation_report
                    .denials()
                    .iter()
                    .cloned()
                    .map(WorthUiLiveViewProjectionAdmissionDenial::CompositionParticipation),
            );
            let denial_count = denials.len();
            let counters = WorthUiLiveViewProjectionAdmissionCounters::new(
                authored.controls().len(),
                authored.conditionals().len(),
                authored.readinesses().len(),
                authored.payloads().len(),
                authored.interactions().len(),
                denial_count,
            );
            return Err(WorthUiLiveViewProjectionAdmissionReport::denied(
                denials, counters,
            ));
        }
        let context_propagation =
            match admit_composition_context_propagation(&composition_graph, context_definitions) {
                Ok(receipt) => receipt,
                Err(report) => {
                    denials.extend(
                        report
                            .denials()
                            .iter()
                            .cloned()
                            .map(WorthUiLiveViewProjectionAdmissionDenial::CompositionContext),
                    );
                    let denial_count = denials.len();
                    let counters = WorthUiLiveViewProjectionAdmissionCounters::new(
                        authored.controls().len(),
                        authored.conditionals().len(),
                        authored.readinesses().len(),
                        authored.payloads().len(),
                        authored.interactions().len(),
                        denial_count,
                    );
                    return Err(WorthUiLiveViewProjectionAdmissionReport::denied(
                        denials, counters,
                    ));
                }
            };
        let admissible_interactions = lower_live_view_interaction_intents(
            self,
            live_view,
            &admissible_readinesses,
            &admissible_payloads,
            authored.interactions(),
        );
        let content_receipts = lower_composition_content_receipts(self, authored.composition());
        let view_primitives = lower_live_view_primitive_binding(self, authored);
        let projection = WorthUiLiveViewProjectionAdmissionReceipt::new(
            live_view.live_view_id(),
            view_primitives.flow_layout,
            view_primitives.appearance,
            admissible_controls,
            admissible_conditionals,
            admissible_readinesses,
            admissible_payloads,
            admissible_interactions,
            content_receipts,
            Some(composition_graph.clone()),
            authored
                .composition()
                .map(|composition| composition.accessibility_associations().to_vec())
                .unwrap_or_default(),
            counters,
        );
        Ok(WorthUiGraphBackedLiveViewProjectionReceipt::new(
            projection,
            composition_graph,
            context_propagation,
        ))
    }
}

fn lower_admissible_controls(
    runtime: &WorthUiRuntimeHost,
    live_view: &WorthUiLiveViewDeclarationReceipt,
    declarations: &[WorthUiLiveViewControlProjectionDeclaration],
) -> Vec<WorthUiLiveViewControlProjectionReceipt> {
    let admissible = declarations
        .iter()
        .filter(|declaration| {
            !control_has_denial(runtime, live_view, declarations, declaration.control_id())
        })
        .cloned()
        .collect::<Vec<_>>();
    lower_live_view_control_projection_receipts(runtime, live_view, &admissible)
}

fn authored_payload_consumed_bindings(
    authored: &WorthUiAuthoredLiveViewDeclaration,
) -> Vec<String> {
    let mut binding_ids = Vec::new();
    for interaction in authored.interactions() {
        let Some(readiness) = authored
            .readinesses()
            .iter()
            .find(|readiness| readiness.readiness_id() == interaction.readiness_id())
        else {
            continue;
        };
        binding_ids.extend(readiness.required_bindings().iter().cloned());
    }
    binding_ids.sort();
    binding_ids.dedup();
    binding_ids
}

fn authored_control_ids(authored: &WorthUiAuthoredLiveViewDeclaration) -> BTreeSet<String> {
    authored
        .controls()
        .iter()
        .map(|control| control.control_id().to_owned())
        .collect()
}

fn authored_interaction_ids(authored: &WorthUiAuthoredLiveViewDeclaration) -> BTreeSet<String> {
    authored
        .interactions()
        .iter()
        .map(|interaction| interaction.interaction_id().to_owned())
        .collect()
}

fn default_live_view_composition_graph(
    authored: &WorthUiAuthoredLiveViewDeclaration,
) -> WorthUiAdmittedCompositionGraphReceipt {
    let mut graph =
        WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::new(
            WorthUiCompositionRootKind::PageContentSlot,
            authored.target_slot(),
        ))
        .with_node(WorthUiCompositionNodeDefinition::new(
            WorthUiCompositionNodeKind::Surface,
            "live_view.form_card",
            "live_view.form_card",
        ))
        .with_root_child_at_with_sizing(
            "live_view.form_card",
            0,
            WorthUiCompositionChildSizing::Auto,
        );
    for (order, control) in authored.controls().iter().enumerate() {
        let node_id = format!("live_view.control.{}", control.control_id());
        graph = graph
            .with_node(WorthUiCompositionNodeDefinition::new(
                WorthUiCompositionNodeKind::Control,
                node_id.clone(),
                control.control_id(),
            ))
            .with_parent_at_with_sizing(
                "live_view.form_card",
                node_id,
                order as u32,
                WorthUiCompositionChildSizing::Auto,
            );
    }
    let interaction_order_offset = authored.controls().len() as u32;
    for (order, interaction) in authored.interactions().iter().enumerate() {
        let node_id = format!("live_view.interaction.{}", interaction.interaction_id());
        graph = graph
            .with_node(WorthUiCompositionNodeDefinition::new(
                WorthUiCompositionNodeKind::Interaction,
                node_id.clone(),
                interaction.interaction_id(),
            ))
            .with_parent_at_with_sizing(
                "live_view.form_card",
                node_id,
                interaction_order_offset + order as u32,
                WorthUiCompositionChildSizing::Auto,
            );
    }
    graph
        .admit()
        .expect("default live-view composition graph uses admitted runtime identities")
}
