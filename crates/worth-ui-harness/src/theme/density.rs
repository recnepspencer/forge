use worth_ui::facade::{
    MeasurementConstraint, MeasurementValue, MosaicMeasurementAuthority, MosaicOverflowBehavior,
    MosaicParentGrowthBehavior, MosaicResizePermission, MosaicSizingContractDescriptor,
    MosaicSizingContractId, MosaicSizingKind, MosaicSizingPersistence, MosaicViewportConstraint,
    NamedMeasurementDefinition, NamedMeasurementToken,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessDensity {
    CompactWorkbench,
    ComfortableWorkbench,
}

impl HarnessDensity {
    pub const DEFAULT: Self = Self::CompactWorkbench;
    pub const REQUIRED_SIZING_CONTRACT_IDS: [&'static str; 8] = [
        "harness.sizing.activity_bar_width",
        "harness.sizing.sidebar_width",
        "harness.sizing.panel_height",
        "harness.sizing.toolbar_height",
        "harness.sizing.status_bar_height",
        "harness.sizing.tab_row_height",
        "harness.sizing.command_palette_width",
        "harness.sizing.overlay_max_width",
    ];

    pub fn sizing_contracts(self) -> Vec<MosaicSizingContractDescriptor> {
        match self {
            Self::CompactWorkbench => compact_contracts(),
            Self::ComfortableWorkbench => comfortable_contracts(),
        }
    }
}

fn compact_contracts() -> Vec<MosaicSizingContractDescriptor> {
    vec![
        sizing_contract("activity_bar_width", "activity_bar.width", 48, 40, 56),
        sizing_contract("sidebar_width", "sidebar.width", 300, 240, 520),
        sizing_contract("panel_height", "panel.height", 260, 180, 520),
        sizing_contract("toolbar_height", "toolbar.height", 40, 36, 52),
        sizing_contract("status_bar_height", "status_bar.height", 24, 20, 32),
        sizing_contract("tab_row_height", "tab_row.height", 34, 28, 42),
        sizing_contract(
            "command_palette_width",
            "command_palette.width",
            640,
            480,
            820,
        ),
        sizing_contract("overlay_max_width", "overlay.max_width", 720, 480, 960),
    ]
}

fn comfortable_contracts() -> Vec<MosaicSizingContractDescriptor> {
    vec![
        sizing_contract("activity_bar_width", "activity_bar.width", 52, 44, 64),
        sizing_contract("sidebar_width", "sidebar.width", 340, 260, 560),
        sizing_contract("panel_height", "panel.height", 300, 220, 560),
        sizing_contract("toolbar_height", "toolbar.height", 44, 38, 56),
        sizing_contract("status_bar_height", "status_bar.height", 28, 22, 36),
        sizing_contract("tab_row_height", "tab_row.height", 38, 32, 48),
        sizing_contract(
            "command_palette_width",
            "command_palette.width",
            680,
            520,
            860,
        ),
        sizing_contract("overlay_max_width", "overlay.max_width", 760, 520, 1000),
    ]
}

fn sizing_contract(
    contract_suffix: &str,
    measurement_suffix: &str,
    value: u32,
    minimum: u32,
    maximum: u32,
) -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        sizing_contract_id(contract_suffix),
        MosaicSizingKind::bounded(),
    )
    .with_named_measurement(NamedMeasurementDefinition::new(
        measurement_token(measurement_suffix),
        MeasurementValue::logical_pixels(value),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(minimum),
            MeasurementValue::logical_pixels(maximum),
        ),
    ))
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_label(format!("Harness {contract_suffix}"))
}

fn sizing_contract_id(suffix: &str) -> MosaicSizingContractId {
    MosaicSizingContractId::new(format!("harness.sizing.{suffix}"))
        .expect("valid harness sizing contract id")
}

fn measurement_token(suffix: &str) -> NamedMeasurementToken {
    NamedMeasurementToken::new(format!("harness.measurement.{suffix}"))
        .expect("valid harness measurement token")
}
