use worth_ui::facade::{
    app::WorthUi,
    registry::{MeasurementConstraint, MeasurementValue, MosaicMeasurementAuthority, MosaicOverflowBehavior, MosaicParentGrowthBehavior, MosaicResizePermission, MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind, MosaicSizingPersistence, MosaicViewportConstraint, NamedMeasurementDefinition, NamedMeasurementToken},
};

fn main() {
    let measurement = NamedMeasurementDefinition::new(
        NamedMeasurementToken::new("workspace.measurement.sidebar.width")
            .expect("valid named measurement token"),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(240),
            MeasurementValue::logical_pixels(520),
        ),
    );

    let _app = WorthUi::app()
        .register_mosaic_sizing_contract(
            MosaicSizingContractDescriptor::new(
                MosaicSizingContractId::new("workspace.sizing.sidebar")
                    .expect("valid mosaic sizing contract id"),
                MosaicSizingKind::bounded(),
            )
            .with_named_measurement(measurement)
            .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
            .with_resize_permission(MosaicResizePermission::user_resizable())
            .with_persistence(MosaicSizingPersistence::restorable())
            .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
            .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
            .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport()),
        )
        .freeze().expect("application preparation should succeed");
}
