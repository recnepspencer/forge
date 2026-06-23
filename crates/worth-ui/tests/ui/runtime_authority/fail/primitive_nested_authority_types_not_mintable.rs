use worth_ui::facade::{
    WorthUiAuthoredDeltaChangePosture, WorthUiBoxEdges, WorthUiFlowLayoutAdmissionCounters,
    WorthUiFlowLayoutAdmissionReceipt, WorthUiFlowLayoutAdmissionReport,
    WorthUiFlowLayoutAdmissionStatus, WorthUiFlowLayoutAlign, WorthUiFlowLayoutCrossAlign,
    WorthUiFlowLayoutFill, WorthUiFlowLayoutFit, WorthUiFlowLayoutKind, WorthUiFlowLayoutReceipt,
    WorthUiFlowLayoutValueDenialCode, WorthUiFlowLayoutValueDenialReceipt,
    WorthUiFlowLayoutValueDenialSet, WorthUiFlowLayoutValueKind, WorthUiPrimitiveAlign,
    WorthUiPrimitiveAppearanceReceipt, WorthUiPrimitiveAuthoredValueKind,
    WorthUiPrimitiveChangedFactEvidenceRow, WorthUiPrimitiveColor,
    WorthUiPrimitiveContainerReceipt, WorthUiPrimitiveContentReceipt,
    WorthUiPrimitiveCursorPosture, WorthUiPrimitiveDrawPlan, WorthUiPrimitiveFlowItemFrame,
    WorthUiPrimitiveFocusPosture, WorthUiPrimitiveFrame, WorthUiPrimitiveInteractionKind,
    WorthUiPrimitiveLayoutExecutionCounters, WorthUiPrimitiveMotionEasing,
    WorthUiPrimitiveMotionKind, WorthUiPrimitiveMotionTarget, WorthUiPrimitiveProjectionReceipt,
    WorthUiPrimitiveProjectionRebindPlan, WorthUiPrimitiveProjectionRebindStatus,
    WorthUiPrimitivePropAdmissionCounters, WorthUiPrimitivePropAdmissionReceipt,
    WorthUiPrimitivePropAdmissionReport, WorthUiPrimitivePropAdmissionStatus,
    WorthUiPrimitiveValueDenialCode, WorthUiPrimitiveValueDenialReceipt,
    WorthUiPrimitiveValueDenialSet, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
    WorthUiSemanticSliceId, WorthUiValidatedFlowLayoutPropSet, WorthUiValidatedPrimitivePropSet,
};

fn main() {
    let color = WorthUiPrimitiveColor {
        red: 0,
        green: 0,
        blue: 0,
    };
    let edges = WorthUiBoxEdges {
        top: 8.0,
        right: 24.0,
        bottom: 8.0,
        left: 24.0,
    };
    let content = WorthUiPrimitiveContentReceipt {
        items: Vec::new(),
        accessibility_name: None,
        dependency_fact: primitive_content_fact(),
        receipt_digest: 1,
    };
    let container = WorthUiPrimitiveContainerReceipt {
        align: WorthUiPrimitiveAlign::Center,
        padding_edges: edges,
        radius_points: 8.0,
    };
    let appearance = WorthUiPrimitiveAppearanceReceipt {
        background_color: color,
        foreground_color: color,
    };
    let frame = WorthUiPrimitiveFrame {
        x: 0.0,
        y: 0.0,
        width: 280.0,
        height: 64.0,
    };
    let draw_plan = WorthUiPrimitiveDrawPlan {
        frame,
        item_frames: flow_item_frames(),
        counters: layout_counters(),
        receipt: primitive_receipt(),
    };
    let layout_counters = WorthUiPrimitiveLayoutExecutionCounters {
        content_item_count: 1,
        layout_item_count: 1,
        source_parse_count: 0,
        artifact_scan_count: 0,
    };
    let changed_row = WorthUiPrimitiveChangedFactEvidenceRow {
        semantic_slice: WorthUiSemanticSliceId::PrimitiveContent,
        subject_surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        change_posture: WorthUiAuthoredDeltaChangePosture::Changed,
        changed_facts: Vec::new(),
    };
    let projection = WorthUiPrimitiveProjectionReceipt {
        primitive_receipt: primitive_receipt(),
        rebind_status: WorthUiPrimitiveProjectionRebindStatus::Rebound,
        rebind_plan: rebind_plan(),
        changed_rows: vec![changed_row],
    };
    let prop_set = WorthUiValidatedPrimitivePropSet {
        text: "Submit".to_owned(),
        align: WorthUiPrimitiveAlign::Center,
        padding_token: "validation.density.primitive.padding".to_owned(),
        radius_token: "validation.density.primitive.radius".to_owned(),
        background_color: color,
        foreground_color: color,
        interaction_kind: WorthUiPrimitiveInteractionKind::Submit,
        cursor: WorthUiPrimitiveCursorPosture::Pointer,
        focus: WorthUiPrimitiveFocusPosture::Focusable,
        interaction_id: "worth.interaction.primitive.submit".to_owned(),
        submit_payload: "submit.primary".to_owned(),
        motion_kind: WorthUiPrimitiveMotionKind::Transition,
        motion_target: WorthUiPrimitiveMotionTarget::Background,
        motion_duration_token: "validation.density.primitive.motion.fast".to_owned(),
        motion_easing: WorthUiPrimitiveMotionEasing::Standard,
    };
    let admission = WorthUiPrimitivePropAdmissionReceipt {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        prop_set,
        authored_digest: 1,
        admission_digest: 2,
    };
    let denial = WorthUiPrimitiveValueDenialReceipt {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        prop_key: "primitive_background".to_owned(),
        schema_id: "worth.primitive.prop.primitive_background",
        value_kind: WorthUiPrimitiveAuthoredValueKind::Color,
        raw_value: "blue".to_owned(),
        expected_shape: "a hex color like `#2f7de1`",
        examples: &["#2f7de1"],
        semantic_slice: WorthUiSemanticSliceId::PrimitiveAppearance,
        fact_family: WorthUiRuntimeFactFamily::PrimitiveAppearance,
        denial_code: WorthUiPrimitiveValueDenialCode::InvalidColorHex,
        source_span: None,
        denial_digest: 3,
    };
    let counters = WorthUiPrimitivePropAdmissionCounters {
        schema_count: 5,
        authored_props_seen: 5,
        defaults_applied: 0,
        values_validated: 5,
        denials_emitted: 1,
    };
    let denial_set = WorthUiPrimitiveValueDenialSet {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        denials: vec![denial],
        denial_set_digest: 4,
    };
    let report = WorthUiPrimitivePropAdmissionReport {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        status: WorthUiPrimitivePropAdmissionStatus::Rejected(denial_set),
        counters,
        schema_digest: 5,
        admission_digest: 6,
    };
    let flow_receipt = WorthUiFlowLayoutReceipt {
        kind: WorthUiFlowLayoutKind::Inline,
        gap_token: "validation.density.primitive.flow.gap.default".to_owned(),
        gap_points: 8.0,
        padding_token: "validation.density.primitive.flow.padding.default".to_owned(),
        padding_edges: edges,
        align: WorthUiFlowLayoutAlign::Center,
        cross_align: WorthUiFlowLayoutCrossAlign::Center,
        fit: WorthUiFlowLayoutFit::Hug,
        fill: WorthUiFlowLayoutFill::None,
        receipt_digest: 7,
    };
    let flow_prop_set = WorthUiValidatedFlowLayoutPropSet {
        kind: WorthUiFlowLayoutKind::Inline,
        gap_token: "validation.density.primitive.flow.gap.default".to_owned(),
        gap_points: 8.0,
        padding_token: "validation.density.primitive.flow.padding.default".to_owned(),
        padding_edges: edges,
        align: WorthUiFlowLayoutAlign::Center,
        cross_align: WorthUiFlowLayoutCrossAlign::Center,
        fit: WorthUiFlowLayoutFit::Hug,
        fill: WorthUiFlowLayoutFill::None,
    };
    let flow_admission = WorthUiFlowLayoutAdmissionReceipt {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        prop_set: flow_prop_set,
        authored_digest: 8,
        admission_digest: 9,
    };
    let flow_denial = WorthUiFlowLayoutValueDenialReceipt {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        prop_key: "flow_gap".to_owned(),
        schema_id: "worth.primitive.flow.prop.flow_gap",
        value_kind: WorthUiFlowLayoutValueKind::MeasurementToken,
        raw_value: "fat".to_owned(),
        expected_shape: "a density measurement token id",
        examples: &["validation.density.primitive.flow.gap.default"],
        semantic_slice: WorthUiSemanticSliceId::PrimitiveFlowLayout,
        fact_family: WorthUiRuntimeFactFamily::PrimitiveFlowLayout,
        denial_code: WorthUiFlowLayoutValueDenialCode::InvalidMeasurementToken,
        source_span: None,
        denial_digest: 10,
    };
    let flow_counters = WorthUiFlowLayoutAdmissionCounters {
        schema_count: 6,
        authored_props_seen: 6,
        defaults_applied: 0,
        values_validated: 6,
        denials_emitted: 1,
    };
    let flow_denial_set = WorthUiFlowLayoutValueDenialSet {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        denials: vec![flow_denial],
        denial_set_digest: 11,
    };
    let flow_report = WorthUiFlowLayoutAdmissionReport {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        status: WorthUiFlowLayoutAdmissionStatus::Rejected(flow_denial_set),
        counters: flow_counters,
        schema_digest: 12,
        admission_digest: 13,
    };
    let _ = (
        content, container, appearance, draw_plan, layout_counters, projection, admission, report,
        flow_receipt, flow_admission, flow_report,
    );
}

fn layout_counters() -> WorthUiPrimitiveLayoutExecutionCounters {
    panic!("test fixture never executes")
}

fn flow_item_frames() -> Vec<WorthUiPrimitiveFlowItemFrame> {
    panic!("test fixture never executes")
}

fn primitive_receipt() -> worth_ui::facade::WorthUiPrimitiveProofReceipt {
    panic!("test fixture never executes")
}

fn rebind_plan() -> WorthUiPrimitiveProjectionRebindPlan {
    panic!("test fixture never executes")
}

fn primitive_content_fact() -> WorthUiRuntimeFactId {
    panic!("test fixture never executes")
}
