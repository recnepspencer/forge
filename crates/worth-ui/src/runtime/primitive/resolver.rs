use crate::capability::{CommandId, ComponentId};
use crate::runtime::{
    WorthUiPrimitiveConstructionFamily, WorthUiPrimitiveConstructionRequest,
    WorthUiPrimitiveInteractionReceipt, WorthUiPrimitiveMotionReceipt,
    WorthUiPrimitiveProofTargetBinding, WorthUiRuntimeFactId, WorthUiRuntimeHost,
};

use super::{
    prove_primitive_construction_graph,
    resolver_digest::primitive_receipt_digest,
    resolver_measurement::{resolve_measurement, resolve_measurement_receipt},
    WorthUiPrimitiveAppearanceReceipt, WorthUiPrimitiveContainerReceipt,
    WorthUiPrimitiveFamilyAdmissionDigests, WorthUiPrimitiveProofDenial,
    WorthUiPrimitiveProofReceipt,
};

const PRIMITIVE_PROOF_COMPONENT_ID: &str = "worth.component.primitive_proof";
const PRIMITIVE_ROW_PROOF_COMPONENT_ID: &str = "worth.component.primitive_row_proof";
const PRIMITIVE_CARD_PROOF_COMPONENT_ID: &str = "worth.component.primitive_card_proof";

impl WorthUiRuntimeHost {
    #[cfg(test)]
    pub(crate) fn resolve_primitive_proof(
        &self,
        surface_id: &crate::capability::SurfaceId,
    ) -> Result<WorthUiPrimitiveProofReceipt, WorthUiPrimitiveProofDenial> {
        let target = self
            .bind_authored_primitive_proof_target(surface_id)
            .map_err(primitive_target_denial_to_proof_denial)?;
        self.resolve_primitive_proof_for_target(&target)
    }

    pub fn resolve_primitive_proof_for_target(
        &self,
        target: &WorthUiPrimitiveProofTargetBinding,
    ) -> Result<WorthUiPrimitiveProofReceipt, WorthUiPrimitiveProofDenial> {
        let surface_id = target.surface_id();
        let component_id = self
            .inspect_active_authored_surface_component_id(surface_id)
            .or_else(|| {
                self.inspect_active_surface_descriptor(surface_id)
                    .map(|surface| surface.component_id().as_str())
            })
            .ok_or_else(|| WorthUiPrimitiveProofDenial::MissingSurface {
                surface_id: surface_id.as_str().to_owned(),
            })?;
        if !is_primitive_proof_component(component_id) {
            return Err(WorthUiPrimitiveProofDenial::ComponentMismatch {
                surface_id: surface_id.as_str().to_owned(),
                expected_component_id: PRIMITIVE_PROOF_COMPONENT_ID.to_owned(),
                actual_component_id: component_id.to_owned(),
            });
        }

        let construction_plan = self
            .graph_authority()
            .plan_primitive_construction(WorthUiPrimitiveConstructionRequest::for_surface(
                surface_id.clone(),
            ))
            .map_err(|_| WorthUiPrimitiveProofDenial::EmptyDependencyContract {
                surface_id: surface_id.as_str().to_owned(),
            })?;
        let family_selection = construction_plan.family_selection();
        debug_assert!(family_selection.requires(WorthUiPrimitiveConstructionFamily::BasePrimitive));
        debug_assert!(family_selection.requires(WorthUiPrimitiveConstructionFamily::FlowLayout));
        debug_assert!(family_selection.requires(WorthUiPrimitiveConstructionFamily::Content));
        debug_assert!(
            family_selection.requires(WorthUiPrimitiveConstructionFamily::AppearanceState)
        );
        debug_assert!(family_selection.requires(WorthUiPrimitiveConstructionFamily::Interaction));
        debug_assert!(family_selection.requires(WorthUiPrimitiveConstructionFamily::EventGeometry));

        let admission_report = self.admit_primitive_props(surface_id);
        let admitted = admission_report
            .status()
            .accepted_receipt()
            .ok_or_else(
                || WorthUiPrimitiveProofDenial::InvalidAuthoredPrimitiveValues {
                    report: admission_report.clone(),
                },
            )?;
        let props = admitted.prop_set();
        let flow_report = self.admit_flow_layout_props(surface_id);
        let admitted_flow = flow_report.status().accepted_receipt().ok_or_else(|| {
            WorthUiPrimitiveProofDenial::InvalidFlowLayoutValues {
                report: flow_report.clone(),
            }
        })?;
        let content_report = self.admit_primitive_content_props(surface_id);
        let admitted_content = content_report.status().accepted_receipt().ok_or_else(|| {
            WorthUiPrimitiveProofDenial::InvalidContentValues {
                report: content_report.clone(),
            }
        })?;
        let appearance_state_report = self.admit_appearance_state_props(surface_id);
        let admitted_appearance_state = appearance_state_report
            .status()
            .accepted_receipt()
            .ok_or_else(
                || WorthUiPrimitiveProofDenial::InvalidAppearanceStateValues {
                    report: appearance_state_report.clone(),
                },
            )?;
        let interaction_report = self.admit_interaction_props(surface_id);
        let admitted_interaction =
            interaction_report
                .status()
                .accepted_receipt()
                .ok_or_else(|| WorthUiPrimitiveProofDenial::InvalidInteractionValues {
                    report: interaction_report.clone(),
                })?;
        let event_geometry_report = self.admit_event_geometry_props(surface_id);
        let admitted_event_geometry = event_geometry_report
            .status()
            .accepted_receipt()
            .ok_or_else(|| WorthUiPrimitiveProofDenial::InvalidEventGeometryValues {
                report: event_geometry_report.clone(),
            })?;
        let dependency_contract = construction_plan.dependency_contract().clone();
        let measurement = resolve_measurement_receipt(self, props)?;
        let container = WorthUiPrimitiveContainerReceipt::new(
            props.align(),
            measurement.padding().edges(),
            measurement.radius().points(),
        );
        let appearance_state = admitted_appearance_state.resolved_receipt();
        let content = admitted_content.resolved_receipt(self);
        let appearance = WorthUiPrimitiveAppearanceReceipt::new(
            props.background_color(),
            props.foreground_color(),
        );
        let event_geometry = admitted_event_geometry.resolved_receipt();
        let component_handle =
            ComponentId::new(component_id).expect("primitive proof component id is valid");
        let interaction_dependency_facts =
            self.graph_authority().plan_mounted_interaction_operability(
                surface_id,
                admitted_interaction.prop_set().interaction_id(),
                interaction_target_dependency_facts(admitted_interaction.prop_set().target()),
            );
        let interaction_lane_receipt =
            admitted_interaction.emit_receipt(surface_id, &component_handle);
        let mounted_interaction_plan =
            crate::runtime::WorthUiMountedInteractionPlan::from_admitted_interaction(
                self.graph_authority(),
                crate::runtime::WorthUiMountedInteractionPlanRequest::primary_click(
                    surface_id.clone(),
                ),
                component_handle,
                &admitted_interaction,
                props.disabled(),
                props.focus(),
                interaction_dependency_facts,
            );
        let interaction = WorthUiPrimitiveInteractionReceipt::from_graph_operability(
            admitted_interaction.prop_set().kind(),
            props.cursor(),
            props.focus(),
            props.selected(),
            event_geometry.resolved_cursor(),
            interaction_lane_receipt,
            mounted_interaction_plan.operability(),
        );
        let motion = WorthUiPrimitiveMotionReceipt::new(
            props.motion_kind(),
            props.motion_target(),
            resolve_measurement(self, props.motion_duration_token())?,
            props.motion_easing(),
        );
        let flow_layout = admitted_flow.resolved_receipt();
        let construction_graph_proof = prove_primitive_construction_graph(
            surface_id.as_str(),
            component_id,
            construction_plan.dependency_contract().clone(),
            construction_plan.query_graph_execution().clone(),
            WorthUiPrimitiveFamilyAdmissionDigests {
                primitive: admitted.admission_digest(),
                flow: admitted_flow.admission_digest(),
                content: admitted_content.admission_digest(),
                appearance_state: admitted_appearance_state.admission_digest(),
                interaction: admitted_interaction.admission_digest(),
                event_geometry: admitted_event_geometry.admission_digest(),
            },
        );
        let receipt_digest = primitive_receipt_digest(
            construction_graph_proof.graph_proof_digest(),
            admitted.admission_digest(),
            admitted_flow.admission_digest(),
            admitted_content.admission_digest(),
            admitted_appearance_state.admission_digest(),
            admitted_interaction.admission_digest(),
            admitted_event_geometry.admission_digest(),
            dependency_contract.dependencies(),
            &container,
            &measurement,
            &content,
            &appearance,
            &appearance_state,
            &interaction,
            &event_geometry,
            &motion,
            &flow_layout,
        );

        Ok(WorthUiPrimitiveProofReceipt::new(
            surface_id.as_str(),
            component_id,
            container,
            measurement,
            content,
            appearance,
            appearance_state,
            interaction,
            event_geometry,
            motion,
            flow_layout,
            target.clone(),
            construction_graph_proof,
            receipt_digest,
        ))
    }
}

#[cfg(test)]
fn primitive_target_denial_to_proof_denial(
    denial: crate::runtime::WorthUiUserIntentTargetDenial,
) -> WorthUiPrimitiveProofDenial {
    match denial {
        crate::runtime::WorthUiUserIntentTargetDenial::MissingSlot {
            page_name,
            slot_name,
            ..
        } => WorthUiPrimitiveProofDenial::MissingSurface {
            surface_id: format!("{page_name}.{slot_name}"),
        },
        crate::runtime::WorthUiUserIntentTargetDenial::MissingSurface { surface_id, .. }
        | crate::runtime::WorthUiUserIntentTargetDenial::InvalidSurfaceId { surface_id, .. } => {
            WorthUiPrimitiveProofDenial::MissingSurface { surface_id }
        }
        crate::runtime::WorthUiUserIntentTargetDenial::InvalidComponentId {
            surface_id,
            component_id,
            ..
        } => WorthUiPrimitiveProofDenial::ComponentMismatch {
            surface_id,
            expected_component_id: PRIMITIVE_PROOF_COMPONENT_ID.to_owned(),
            actual_component_id: component_id,
        },
    }
}

fn interaction_target_dependency_facts(
    target: &crate::runtime::WorthUiInteractionTarget,
) -> Vec<WorthUiRuntimeFactId> {
    match target {
        crate::runtime::WorthUiInteractionTarget::Command(command_id) => CommandId::new(command_id)
            .ok()
            .map(|command_id| WorthUiRuntimeFactId::command(&command_id))
            .into_iter()
            .collect(),
        _ => Vec::new(),
    }
}

fn is_primitive_proof_component(component_id: &str) -> bool {
    matches!(
        component_id,
        PRIMITIVE_PROOF_COMPONENT_ID
            | PRIMITIVE_ROW_PROOF_COMPONENT_ID
            | PRIMITIVE_CARD_PROOF_COMPONENT_ID
    )
}
