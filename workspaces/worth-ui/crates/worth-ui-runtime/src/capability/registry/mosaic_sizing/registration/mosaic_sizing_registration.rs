use crate::capability::{
    CapabilityDiagnosticCode, CapabilitySupportKind, RegistrationCandidate,
    RegistrationCandidateDiagnostic, MOSAIC_SIZING_CONTRACT_FAMILY_NAME,
};

use super::super::{
    MosaicSizingContractDescriptor, RawLayoutMeasurementForDiagnostics, RawLayoutMeasurementKind,
};

impl MosaicSizingContractDescriptor {
    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        let candidate = RegistrationCandidate::new(
            MOSAIC_SIZING_CONTRACT_FAMILY_NAME,
            self.id().as_str(),
            CapabilitySupportKind::Admitted,
        );
        add_mosaic_sizing_descriptor_diagnostics(candidate, self)
    }
}

fn add_mosaic_sizing_descriptor_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    candidate = add_missing_authority_diagnostic(candidate, descriptor);
    candidate = add_missing_resize_diagnostic(candidate, descriptor);
    candidate = add_missing_persistence_diagnostic(candidate, descriptor);
    candidate = add_missing_overflow_diagnostic(candidate, descriptor);
    candidate = add_missing_parent_growth_diagnostic(candidate, descriptor);
    candidate = add_missing_viewport_diagnostic(candidate, descriptor);
    candidate = add_unitless_measurement_diagnostic(candidate, descriptor);
    candidate = add_invalid_measurement_constraint_diagnostic(candidate, descriptor);
    add_raw_measurement_diagnostics(candidate, descriptor)
}

fn add_missing_authority_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .measurement_authority()
        .is_none_or(|authority| authority.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicSizingMeasurementAuthority,
            "mosaic sizing contract must declare measurement authority",
        );
    }
    candidate
}

fn add_missing_resize_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .resize_permission()
        .is_none_or(|permission| permission.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicSizingResizePermission,
            "mosaic sizing contract must declare resize permission",
        );
    }
    candidate
}

fn add_missing_persistence_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .persistence()
        .is_none_or(|persistence| persistence.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicSizingPersistence,
            "mosaic sizing contract must declare persistence behavior",
        );
    }
    candidate
}

fn add_missing_overflow_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .overflow_behavior()
        .is_none_or(|overflow| overflow.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicSizingOverflowBehavior,
            "mosaic sizing contract must declare overflow behavior",
        );
    }
    candidate
}

fn add_missing_parent_growth_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .parent_growth_behavior()
        .is_none_or(|growth| growth.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicSizingParentGrowthBehavior,
            "mosaic sizing contract must declare parent growth behavior",
        );
    }
    candidate
}

fn add_missing_viewport_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .viewport_constraint()
        .is_none_or(|viewport| viewport.is_missing())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::MissingMosaicSizingViewportConstraint,
            "mosaic sizing contract must declare viewport constraint behavior",
        );
    }
    candidate
}

fn add_unitless_measurement_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .named_measurement()
        .is_some_and(|measurement| measurement.has_unitless_value_or_constraint())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::UnitlessMosaicSizingMeasurementDefinition,
            "named mosaic sizing measurements must carry unit metadata",
        );
    }
    candidate
}

fn add_invalid_measurement_constraint_diagnostic(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    if descriptor
        .named_measurement()
        .is_some_and(|measurement| measurement.has_invalid_constraint_bounds())
    {
        candidate = with_descriptor_diagnostic(
            candidate,
            CapabilityDiagnosticCode::InvalidMosaicSizingMeasurementConstraint,
            "named mosaic sizing measurement constraints must use comparable ordered bounds",
        );
    }
    candidate
}

fn add_raw_measurement_diagnostics(
    mut candidate: RegistrationCandidate,
    descriptor: &MosaicSizingContractDescriptor,
) -> RegistrationCandidate {
    for measurement in descriptor.raw_measurements_for_diagnostics() {
        candidate = add_raw_measurement_diagnostic(candidate, measurement);
    }
    candidate
}

fn add_raw_measurement_diagnostic(
    candidate: RegistrationCandidate,
    measurement: &RawLayoutMeasurementForDiagnostics,
) -> RegistrationCandidate {
    let (code, message) = raw_measurement_diagnostic(measurement.kind());
    with_descriptor_diagnostic(candidate, code, message)
}

fn raw_measurement_diagnostic(
    kind: &RawLayoutMeasurementKind,
) -> (CapabilityDiagnosticCode, &'static str) {
    match kind {
        RawLayoutMeasurementKind::Width | RawLayoutMeasurementKind::Height => (
            CapabilityDiagnosticCode::RawMosaicWidthMeasurementOutsideNamedMeasurement,
            "raw mosaic width or height values must be named measurements",
        ),
        RawLayoutMeasurementKind::Gap | RawLayoutMeasurementKind::Padding => (
            CapabilityDiagnosticCode::RawMosaicGapMeasurementOutsideNamedMeasurement,
            "raw mosaic gap or padding values must be named measurements",
        ),
        RawLayoutMeasurementKind::ZOrder => (
            CapabilityDiagnosticCode::RawMosaicZOrderMeasurementOutsideNamedMeasurement,
            "raw mosaic z-order values must be named measurements",
        ),
        RawLayoutMeasurementKind::Timing => (
            CapabilityDiagnosticCode::RawMosaicTimingMeasurementOutsideNamedMeasurement,
            "raw mosaic timing values must be named measurements",
        ),
        RawLayoutMeasurementKind::Breakpoint => (
            CapabilityDiagnosticCode::RawMosaicBreakpointMeasurementOutsideNamedMeasurement,
            "raw mosaic breakpoint values must be named measurements",
        ),
    }
}

fn with_descriptor_diagnostic(
    candidate: RegistrationCandidate,
    code: CapabilityDiagnosticCode,
    message: &'static str,
) -> RegistrationCandidate {
    candidate.with_descriptor_diagnostic(RegistrationCandidateDiagnostic::new(code, message))
}
