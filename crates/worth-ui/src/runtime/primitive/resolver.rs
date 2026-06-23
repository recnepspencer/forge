use crate::capability::{DensityTokenId, SurfaceId, WorthUiDensityValue};
use crate::runtime::{
    WorthUiPrimitiveInteractionReceipt, WorthUiPrimitiveMeasurementReceipt,
    WorthUiPrimitiveMotionReceipt, WorthUiPrimitiveResolvedInsets,
    WorthUiPrimitiveResolvedMeasurement, WorthUiProjectionDependencySet, WorthUiRuntimeHost,
};

use super::{
    dependency::primitive_dependency_contract, WorthUiBoxEdges, WorthUiFlowLayoutReceipt,
    WorthUiPrimitiveAppearanceReceipt, WorthUiPrimitiveContainerReceipt,
    WorthUiPrimitiveContentReceipt, WorthUiPrimitiveEventGeometryReceipt,
    WorthUiPrimitiveProofDenial, WorthUiPrimitiveProofReceipt,
};

const PRIMITIVE_PROOF_COMPONENT_ID: &str = "worth.component.primitive_proof";
const PRIMITIVE_ROW_PROOF_COMPONENT_ID: &str = "worth.component.primitive_row_proof";
const PRIMITIVE_CARD_PROOF_COMPONENT_ID: &str = "worth.component.primitive_card_proof";

impl WorthUiRuntimeHost {
    pub fn resolve_primitive_proof(
        &self,
        surface_id: &SurfaceId,
    ) -> Result<WorthUiPrimitiveProofReceipt, WorthUiPrimitiveProofDenial> {
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
        let dependency_contract = primitive_dependency_contract(surface_id)?;
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
        let interaction = WorthUiPrimitiveInteractionReceipt::new(
            admitted_interaction.prop_set().kind(),
            props.cursor(),
            props.focus(),
            props.disabled(),
            props.selected(),
            event_geometry.resolved_cursor(),
            admitted_interaction.emit_receipt(
                surface_id,
                &crate::capability::ComponentId::new(component_id)
                    .expect("primitive proof component id is valid"),
            ),
        );
        let motion = WorthUiPrimitiveMotionReceipt::new(
            props.motion_kind(),
            props.motion_target(),
            resolve_measurement(self, props.motion_duration_token())?,
            props.motion_easing(),
        );
        let flow_layout = admitted_flow.resolved_receipt();
        let receipt_digest = primitive_receipt_digest(
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
            dependency_contract,
            receipt_digest,
        ))
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

fn primitive_receipt_digest(
    authored_digest: u64,
    flow_digest: u64,
    content_digest: u64,
    appearance_state_digest: u64,
    interaction_digest: u64,
    event_geometry_digest: u64,
    dependencies: &WorthUiProjectionDependencySet,
    container: &WorthUiPrimitiveContainerReceipt,
    measurement: &WorthUiPrimitiveMeasurementReceipt,
    content: &WorthUiPrimitiveContentReceipt,
    appearance: &WorthUiPrimitiveAppearanceReceipt,
    appearance_state: &crate::runtime::WorthUiStatefulAppearanceRecipeReceipt,
    interaction: &WorthUiPrimitiveInteractionReceipt,
    event_geometry: &WorthUiPrimitiveEventGeometryReceipt,
    motion: &WorthUiPrimitiveMotionReceipt,
    flow_layout: &WorthUiFlowLayoutReceipt,
) -> u64 {
    let content_item_count = content.items().len();
    let basis = format!(
        "primitive|authored:{authored_digest}|flow:{flow_digest}|content:{content_digest}|state:{appearance_state_digest}|interaction_admission:{interaction_digest}|event_geometry:{event_geometry_digest}|deps:{}|align:{:?}|padding:{}:{}|radius:{}:{}|text:{}|items:{}|content_receipt:{}|bg:{}|fg:{}|state_receipt:{}|interaction:{:?}:{:?}:{}:{}:{}:{}|operability:{:?}:{:?}|affordance:{:?}:{}|event_geometry_receipt:{}:{:?}:{:?}:{:?}:{:?}|motion:{:?}:{:?}:{}:{}:{:?}|flow_receipt:{}",
        dependencies.digest().value(),
        container.align(),
        measurement.padding().token(),
        measurement.padding().edges().digest_basis(),
        measurement.radius().token(),
        measurement.radius().points(),
        content.text(),
        content_item_count,
        content.receipt_digest(),
        appearance.background_color().hex_triplet(),
        appearance.foreground_color().hex_triplet(),
        appearance_state.receipt_digest(),
        interaction.kind(),
        interaction.focus(),
        interaction.disabled(),
        interaction.selected(),
        interaction.interaction_id(),
        interaction.submit_payload().digest(),
        interaction.operability().posture(),
        interaction.operability().basis(),
        interaction.affordance().cursor(),
        interaction.affordance().can_activate(),
        event_geometry.receipt_digest(),
        event_geometry.cursor(),
        event_geometry.hit_area(),
        event_geometry.containment(),
        event_geometry.capture(),
        motion.kind(),
        motion.target(),
        motion.duration().token(),
        motion.duration().points(),
        motion.easing(),
        flow_layout.receipt_digest()
    );
    basis.bytes().fold(0xcbf2_9ce4_8422_2325, |mut acc, byte| {
        acc ^= u64::from(byte);
        acc.wrapping_mul(0x0000_0100_0000_01b3)
    })
}

fn resolve_measurement_receipt(
    runtime: &WorthUiRuntimeHost,
    props: &crate::runtime::WorthUiValidatedPrimitivePropSet,
) -> Result<WorthUiPrimitiveMeasurementReceipt, WorthUiPrimitiveProofDenial> {
    Ok(WorthUiPrimitiveMeasurementReceipt::new(
        resolve_padding(runtime, props.padding_token())?,
        resolve_measurement(runtime, props.radius_token())?,
    ))
}

fn resolve_padding(
    runtime: &WorthUiRuntimeHost,
    token_text: &str,
) -> Result<WorthUiPrimitiveResolvedInsets, WorthUiPrimitiveProofDenial> {
    let token = DensityTokenId::new(token_text).map_err(|_| {
        WorthUiPrimitiveProofDenial::MissingPrimitiveMeasurementToken {
            token: token_text.to_owned(),
        }
    })?;
    let Some(descriptor) = runtime.inspect_active_density_token_descriptor(&token) else {
        return Err(
            WorthUiPrimitiveProofDenial::MissingPrimitiveMeasurementToken {
                token: token_text.to_owned(),
            },
        );
    };
    let edges = match descriptor.value() {
        WorthUiDensityValue::Padding(value) => WorthUiBoxEdges::new(
            value.top().points(),
            value.right().points(),
            value.bottom().points(),
            value.left().points(),
        ),
        WorthUiDensityValue::Spacing(value) => WorthUiBoxEdges::uniform(value.points()),
        WorthUiDensityValue::HitTargetMinimum(value) => WorthUiBoxEdges::uniform(value.points()),
        WorthUiDensityValue::Posture(_) => {
            return Err(WorthUiPrimitiveProofDenial::WrongPrimitiveMeasurementKind {
                token: token_text.to_owned(),
                expected: "padding, spacing, or length".to_owned(),
                actual: "posture".to_owned(),
            });
        }
    };
    Ok(WorthUiPrimitiveResolvedInsets::new(&token, edges))
}

fn resolve_measurement(
    runtime: &WorthUiRuntimeHost,
    token_text: &str,
) -> Result<WorthUiPrimitiveResolvedMeasurement, WorthUiPrimitiveProofDenial> {
    let token = DensityTokenId::new(token_text).map_err(|_| {
        WorthUiPrimitiveProofDenial::MissingPrimitiveMeasurementToken {
            token: token_text.to_owned(),
        }
    })?;
    let Some(descriptor) = runtime.inspect_active_density_token_descriptor(&token) else {
        return Err(
            WorthUiPrimitiveProofDenial::MissingPrimitiveMeasurementToken {
                token: token_text.to_owned(),
            },
        );
    };
    let points = match descriptor.value() {
        WorthUiDensityValue::Padding(value) => value.horizontal_points(),
        WorthUiDensityValue::Spacing(value) => value.points(),
        WorthUiDensityValue::HitTargetMinimum(value) => value.points(),
        WorthUiDensityValue::Posture(_) => {
            return Err(WorthUiPrimitiveProofDenial::WrongPrimitiveMeasurementKind {
                token: token_text.to_owned(),
                expected: "padding, spacing, or length".to_owned(),
                actual: "posture".to_owned(),
            });
        }
    };
    Ok(WorthUiPrimitiveResolvedMeasurement::new(&token, points))
}
