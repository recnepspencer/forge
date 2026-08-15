use super::*;

#[test]
fn scalar_and_collection_requirements_preserve_distinct_shape_contracts() {
    let status = UiProjectionFieldRequirement::declared("status").unwrap();
    let scalar =
        UiScalarSchemaRequirement::text(status.clone(), UiProjectionLifecycleRequirement::Live);
    let collection = UiCollectionSchemaRequirement::text(
        UiProjectionFieldRequirement::declared("identity").unwrap(),
        [status],
        UiProjectionLifecycleRequirement::Live,
        false,
        true,
    )
    .unwrap();

    assert_eq!(scalar.shape(), UiProjectionShape::Scalar);
    assert_eq!(scalar.selected_field().declared_name(), "status");
    assert_eq!(scalar.native_family(), UiProjectionNativeFamily::Text);
    assert_eq!(collection.shape(), UiProjectionShape::Collection);
    assert_eq!(collection.row_identity_field().declared_name(), "identity");
    assert_eq!(collection.selected_fields().len(), 1);
    assert!(collection.permits_continuation());
}

#[test]
fn collection_requirement_rejects_noncanonical_selected_fields() {
    let row = UiProjectionFieldRequirement::declared("identity").unwrap();
    let status = UiProjectionFieldRequirement::declared("status").unwrap();

    assert_eq!(
        UiCollectionSchemaRequirement::text(
            row.clone(),
            [],
            UiProjectionLifecycleRequirement::Live,
            false,
            true,
        ),
        Err(UiCollectionSchemaRequirementError::NoSelectedFields)
    );
    assert_eq!(
        UiCollectionSchemaRequirement::text(
            row,
            [status.clone(), status],
            UiProjectionLifecycleRequirement::Live,
            false,
            true,
        ),
        Err(UiCollectionSchemaRequirementError::DuplicateSelectedField)
    );
}

#[test]
fn field_requirements_reject_ambiguous_source_names() {
    assert_eq!(
        UiProjectionFieldRequirement::declared(""),
        Err(UiProjectionFieldRequirementError::Empty)
    );
    assert_eq!(
        UiProjectionFieldRequirement::declared(" status"),
        Err(UiProjectionFieldRequirementError::SurroundingWhitespace)
    );
}

#[test]
fn typed_measurement_and_size_fields_cannot_alias_through_the_native_value_key() {
    let measurement = UiProjectionFieldRequirement::measurement_value();
    let size = UiProjectionFieldRequirement::size_value();

    assert_eq!(measurement.declared_name(), "value");
    assert_eq!(size.declared_name(), "value");
    assert_eq!(
        measurement.typed_field(),
        Some(WorthUiProjectionField::MeasurementValue)
    );
    assert_eq!(size.typed_field(), Some(WorthUiProjectionField::SizeValue));
    assert_ne!(measurement, size);
    assert!(UiCollectionSchemaRequirement::text(
        UiProjectionFieldRequirement::identity_id(),
        [measurement, size],
        UiProjectionLifecycleRequirement::Snapshot,
        true,
        false,
    )
    .is_ok());
}

#[test]
fn platform_pulse_budget_is_exact_and_independently_dimensioned() {
    let budget = UiProjectionConsumptionBudget::platform_pulse();

    assert_eq!(budget.bindings_admitted(), 1);
    assert_eq!(budget.scalar_fields_accessed(), 1);
    assert_eq!(budget.collection_rows(), 0);
    assert_eq!(budget.collection_change_operations(), 0);
    assert_eq!(budget.continuation_operations(), 0);
    assert_eq!(budget.native_bytes_retained(), 65_536);
    assert_eq!(budget.diagnostic_summary_bytes(), 4_096);
    assert_eq!(budget.rich_diagnostic_bytes(), 262_144);
}

#[test]
fn supporting_scenarios_can_declare_bounded_collection_work() {
    let limits = UiProjectionConsumptionLimits::new(2, 3, 8_192)
        .with_collection(1_024, 64, 1)
        .with_diagnostics(512, 4_096);
    let budget = UiProjectionConsumptionBudget::bounded(limits).unwrap();

    assert_eq!(budget.bindings_admitted(), 2);
    assert_eq!(budget.collection_rows(), 1_024);
    assert_eq!(budget.collection_change_operations(), 64);
    assert_eq!(budget.continuation_operations(), 1);
    assert_eq!(
        UiProjectionConsumptionBudget::bounded(UiProjectionConsumptionLimits::new(0, 1, 1)),
        Err(UiProjectionConsumptionBudgetError::ZeroBindings)
    );
}

#[test]
fn result_posture_vocabulary_keeps_lifecycle_meaning_distinct() {
    let unavailable = [
        UiProjectionUnavailableKind::Pending,
        UiProjectionUnavailableKind::Failed,
        UiProjectionUnavailableKind::Cancelled,
        UiProjectionUnavailableKind::Retried,
        UiProjectionUnavailableKind::Superseded,
        UiProjectionUnavailableKind::Denied,
        UiProjectionUnavailableKind::Unsupported,
        UiProjectionUnavailableKind::Remasked,
        UiProjectionUnavailableKind::BasisDrift,
        UiProjectionUnavailableKind::GenerationDrift,
    ];
    let stops = [
        UiProjectionFactStopKind::SchemaMismatch,
        UiProjectionFactStopKind::PayloadShapeMismatch,
        UiProjectionFactStopKind::NativeFamilyMismatch,
        UiProjectionFactStopKind::WrongWorld,
        UiProjectionFactStopKind::StaleBindingGeneration,
        UiProjectionFactStopKind::StaleResultGeneration,
        UiProjectionFactStopKind::BasisMismatch,
        UiProjectionFactStopKind::Unsupported,
        UiProjectionFactStopKind::Remasked,
        UiProjectionFactStopKind::BudgetExceeded,
        UiProjectionFactStopKind::ResetRequired,
    ];

    assert_eq!(unavailable.len(), 10);
    assert_eq!(stops.len(), 11);
    assert_ne!(
        UiProjectionRetainedActivityKind::Idle,
        UiProjectionRetainedActivityKind::Revalidating
    );
}
