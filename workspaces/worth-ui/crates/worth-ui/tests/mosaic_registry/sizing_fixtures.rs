use worth_ui::facade::registry::{
    MeasurementConstraint, MeasurementValue, MosaicMeasurementAuthority, MosaicOverflowBehavior,
    MosaicParentGrowthBehavior, MosaicResizePermission, MosaicSizingContractDescriptor,
    MosaicSizingContractId, MosaicSizingKind, MosaicSizingPersistence, MosaicViewportConstraint,
    NamedMeasurementDefinition, NamedMeasurementToken,
};

pub(crate) fn bounded_sidebar_contract(id: &str) -> MosaicSizingContractDescriptor {
    complete_sizing_contract(id, MosaicSizingKind::bounded())
        .with_named_measurement(sidebar_measurement())
}

pub(crate) fn fixed_toolbar_contract(id: &str) -> MosaicSizingContractDescriptor {
    complete_sizing_contract(id, MosaicSizingKind::fixed()).with_named_measurement(
        NamedMeasurementDefinition::new(
            measurement_token("workspace.measurement.toolbar.height"),
            MeasurementValue::logical_pixels(48),
            MeasurementConstraint::between(
                MeasurementValue::logical_pixels(40),
                MeasurementValue::logical_pixels(64),
            ),
        ),
    )
}

pub(crate) fn complete_sizing_contract(
    id: &str,
    kind: MosaicSizingKind,
) -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(sizing_contract_id(id), kind)
        .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
        .with_resize_permission(MosaicResizePermission::user_resizable())
        .with_persistence(MosaicSizingPersistence::restorable())
        .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
        .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
        .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
}

pub(crate) fn unitless_sizing_contract(id: &str) -> MosaicSizingContractDescriptor {
    complete_sizing_contract(id, MosaicSizingKind::bounded()).with_named_measurement(
        NamedMeasurementDefinition::new(
            measurement_token("workspace.measurement.unitless"),
            MeasurementValue::unitless_for_diagnostics(12),
            MeasurementConstraint::unconstrained(),
        ),
    )
}

pub(crate) fn unitless_constraint_sizing_contract(id: &str) -> MosaicSizingContractDescriptor {
    complete_sizing_contract(id, MosaicSizingKind::bounded()).with_named_measurement(
        NamedMeasurementDefinition::new(
            measurement_token("workspace.measurement.unitless_constraint"),
            MeasurementValue::logical_pixels(320),
            MeasurementConstraint::between(
                MeasurementValue::unitless_for_diagnostics(0),
                MeasurementValue::logical_pixels(520),
            ),
        ),
    )
}

pub(crate) fn inverted_constraint_sizing_contract(id: &str) -> MosaicSizingContractDescriptor {
    complete_sizing_contract(id, MosaicSizingKind::bounded()).with_named_measurement(
        NamedMeasurementDefinition::new(
            measurement_token("workspace.measurement.inverted_constraint"),
            MeasurementValue::logical_pixels(320),
            MeasurementConstraint::between(
                MeasurementValue::logical_pixels(520),
                MeasurementValue::logical_pixels(240),
            ),
        ),
    )
}

pub(crate) fn mixed_unit_constraint_sizing_contract(id: &str) -> MosaicSizingContractDescriptor {
    complete_sizing_contract(id, MosaicSizingKind::bounded()).with_named_measurement(
        NamedMeasurementDefinition::new(
            measurement_token("workspace.measurement.mixed_unit_constraint"),
            MeasurementValue::logical_pixels(320),
            MeasurementConstraint::between(
                MeasurementValue::logical_pixels(240),
                MeasurementValue::milliseconds(120),
            ),
        ),
    )
}

pub(crate) fn sizing_contract_id(raw_text: &str) -> MosaicSizingContractId {
    MosaicSizingContractId::new(raw_text).expect("valid mosaic sizing contract id")
}

fn sidebar_measurement() -> NamedMeasurementDefinition {
    NamedMeasurementDefinition::new(
        measurement_token("workspace.measurement.sidebar.width"),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(240),
            MeasurementValue::logical_pixels(520),
        ),
    )
}

fn measurement_token(raw_text: &str) -> NamedMeasurementToken {
    NamedMeasurementToken::new(raw_text).expect("valid named measurement token")
}
