pub const NATIVE_EGUI_BOUNDARY_FILES: &[&str] = &[
    "app\\frame_update.rs",
    "app\\live_view\\control_rendering.rs",
    "app\\live_view\\evidence_rendering.rs",
    "app\\live_view\\host_observation\\native_collector.rs",
    "app\\live_view\\interaction_rendering.rs",
    "app\\live_view\\receipt_color_translation.rs",
    "app\\live_view\\rendering.rs",
    "header\\header_renderer.rs",
    "main.rs",
    "native_window.rs",
];

pub const HOST_MEASUREMENT_OBSERVATION_TOKENS: &[&str] = &[
    "available_rect_before_wrap",
    "available_width()",
    "available_height()",
    "available_size",
    "pixels_per_point",
    "cumulative_pass_nr",
    "screen_rect",
    "content_rect",
    "allocate_rect",
    "allocate_ui",
    "allocate_exact_size",
    "allocate_at_least",
    "max_rect",
    "min_rect",
];

pub struct NativeAdapterCapability {
    pub file: &'static str,
    pub tokens: &'static [&'static str],
    pub role: &'static str,
}

pub const HOST_MEASUREMENT_ADAPTER_CAPABILITIES: &[NativeAdapterCapability] = &[
    NativeAdapterCapability {
        file: "app\\live_view\\host_observation\\native_collector.rs",
        tokens: &[
            "available_rect_before_wrap",
            "pixels_per_point",
            "cumulative_pass_nr",
        ],
        role: "host measurement observation collector",
    },
    NativeAdapterCapability {
        file: "app\\live_view\\rendering.rs",
        tokens: &["max_rect"],
        role: "mechanical mounted-view allocation adapter",
    },
];

pub const FORBIDDEN_EGUI_MEANING_TOKENS: &[&str] = &[
    "CentralPanel",
    "TopBottomPanel",
    "SidePanel",
    "Frame::new",
    "Button::new",
    "TextEdit",
    "ComboBox",
    "Color32::from_rgb",
    "ui.horizontal",
    "ui.vertical",
    "add_space",
    "set_min_width",
    "desired_width",
    "with_layout",
    "rect_filled",
];

pub const PRIMITIVE_EVENT_TOPOLOGY_CONSTRUCTORS: &[&str] = &[
    "WorthUiPrimitiveEventRegionOrder::new",
    "WorthUiPrimitiveEventRegionReceipt::from_primitive_draw_plan",
    "WorthUiPrimitiveEventRegionReceipt::from_primitive_draw_plan_at",
    "WorthUiPrimitiveEventRegionReceipt::from_child_primitive_draw_plan_at",
];

pub const QUERY_RELOAD_PROOF_TYPES: &[&str] = &[
    "WorthUiQueryRuntimeFactLoweringInput",
    "WorthUiQueryProjectionFactReceipt",
    "WorthUiQueryStateSnapshotReceipt",
    "WorthUiQueryEffectPostureReceipt",
    "WorthUiQueryLiveRebindPlan",
    "WorthUiQueryLiveRebindCounters",
    "WorthUiQueryLiveRebindEntry",
    "WorthUiQueryLiveRebindOutcome",
    "WorthUiQueryBindingPreservation",
    "WorthUiQueryBindingRebind",
    "WorthUiQueryBindingRetirement",
    "WorthUiQueryBindingDriftDenial",
    "WorthUiAdmittedRuntimeChangeEvidence",
    "WorthUiAdmittedProjectionPlan",
    "WorthUiProjectionRebindPlan",
];

pub const THEME_RELOAD_AND_SOURCE_AUTHORITY_MARKERS: &[&str] = &[
    "WorthUiHeaderThemeRuntime",
    "WorthUiHeaderThemeRuntimeFrame",
    "WorthUiHeaderThemeRuntimeDenial",
    "WorthUiSourceParser",
    "WorthUiParsedSourceToArtifactInputLowerer",
    "WorthUiArtifactInputResolver",
    "WorthUiCanonicalArtifactAssembler",
    "HeaderThemeHotReload",
    "WorthUiSourceWatcher",
    "WorthUiWatchedCandidateSubmission",
    "WorthUiCandidateAdmission",
    "WorthUiReplacementCandidate",
    "from_snapshot_and_source_file",
    "WorthUiHeaderThemePlan::from_snapshot_and_source",
    "update_token_color",
    "query_delivery_count",
];

pub const ACCESSIBILITY_AND_FOCUS_MEANING_MARKERS: &[&str] = &[
    "tab_order",
    "focus_order",
    "focus_nodes",
    "accessibility_nodes",
    "label_for",
    "described_by",
    "role_for_node",
    "set_accessible_name",
    "accessible_name",
    "AccessibilityNodeParticipation",
    "FocusNodeParticipation",
];

pub const MOUNTED_CHILD_RENDER_GROUPING_MARKERS: &[&str] = &[
    "render_plan().controls()",
    "render_plan().interactions()",
    "WorthUiLiveViewProjectionRenderControl",
    "WorthUiLiveViewProjectionRenderInteraction",
];
