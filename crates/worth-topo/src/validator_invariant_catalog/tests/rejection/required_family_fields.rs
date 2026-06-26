use crate::validator_invariant_catalog::source_catalog::{
    current_invariant_family_inputs, current_validator_family_inputs,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalog, WorthTopologyLegalityCatalogError,
};

#[derive(Clone, Copy)]
enum RequiredFamilyField {
    TouchedApplicability,
    RequiredAccessPosture,
    EnforcementPhase,
    WitnessPosture,
    DiagnosticProjection,
}

impl RequiredFamilyField {
    const ALL: [Self; 5] = [
        Self::TouchedApplicability,
        Self::RequiredAccessPosture,
        Self::EnforcementPhase,
        Self::WitnessPosture,
        Self::DiagnosticProjection,
    ];
}

#[test]
fn validator_family_missing_required_field_cannot_enter_catalog() {
    for missing_field in RequiredFamilyField::ALL {
        assert_validator_rejects_missing_field(missing_field);
    }
}

#[test]
fn invariant_family_missing_required_field_cannot_enter_catalog() {
    for missing_field in RequiredFamilyField::ALL {
        assert_invariant_rejects_missing_field(missing_field);
    }
}

fn assert_validator_rejects_missing_field(missing_field: RequiredFamilyField) {
    let mut row = current_validator_family_inputs("phase-eight-posture")
        .expect("validator inputs should derive from current rules")
        .into_iter()
        .next()
        .expect("validator row should exist");
    remove_required_field_from_validator_input(&mut row.input, missing_field);

    let error = WorthTopologyLegalityCatalog::validator_record_from_input_for_tests(row.input)
        .expect_err("missing validator family field must be rejected");

    assert_missing_field_error(error, missing_field);
}

fn assert_invariant_rejects_missing_field(missing_field: RequiredFamilyField) {
    let mut row = current_invariant_family_inputs("phase-eight-posture")
        .expect("invariant inputs should derive from current registrations")
        .into_iter()
        .next()
        .expect("invariant row should exist");
    remove_required_field_from_invariant_input(&mut row.input, missing_field);

    let error = WorthTopologyLegalityCatalog::invariant_record_from_input_for_tests(row.input)
        .expect_err("missing invariant family field must be rejected");

    assert_missing_field_error(error, missing_field);
}

fn remove_required_field_from_validator_input(
    input: &mut crate::validator_invariant_catalog::family_record::WorthTopologyLegalityFamilyRecordInput<
        crate::validator_invariant_catalog::WorthTopologyValidatorFamilyIdentity,
    >,
    missing_field: RequiredFamilyField,
) {
    match missing_field {
        RequiredFamilyField::TouchedApplicability => input.touched_applicability = None,
        RequiredFamilyField::RequiredAccessPosture => input.required_access_posture = None,
        RequiredFamilyField::EnforcementPhase => input.enforcement_phase = None,
        RequiredFamilyField::WitnessPosture => input.witness_posture = None,
        RequiredFamilyField::DiagnosticProjection => input.diagnostic_projection = None,
    }
}

fn remove_required_field_from_invariant_input(
    input: &mut crate::validator_invariant_catalog::family_record::WorthTopologyLegalityFamilyRecordInput<
        crate::validator_invariant_catalog::WorthTopologyInvariantFamilyIdentity,
    >,
    missing_field: RequiredFamilyField,
) {
    match missing_field {
        RequiredFamilyField::TouchedApplicability => input.touched_applicability = None,
        RequiredFamilyField::RequiredAccessPosture => input.required_access_posture = None,
        RequiredFamilyField::EnforcementPhase => input.enforcement_phase = None,
        RequiredFamilyField::WitnessPosture => input.witness_posture = None,
        RequiredFamilyField::DiagnosticProjection => input.diagnostic_projection = None,
    }
}

fn assert_missing_field_error(
    error: WorthTopologyLegalityCatalogError,
    missing_field: RequiredFamilyField,
) {
    match missing_field {
        RequiredFamilyField::TouchedApplicability => assert!(matches!(
            error,
            WorthTopologyLegalityCatalogError::MissingTouchedApplicability(_)
        )),
        RequiredFamilyField::RequiredAccessPosture => assert!(matches!(
            error,
            WorthTopologyLegalityCatalogError::MissingRequiredAccessPosture(_)
        )),
        RequiredFamilyField::EnforcementPhase => assert!(matches!(
            error,
            WorthTopologyLegalityCatalogError::MissingEnforcementPhase(_)
        )),
        RequiredFamilyField::WitnessPosture => assert!(matches!(
            error,
            WorthTopologyLegalityCatalogError::MissingWitnessPosture(_)
        )),
        RequiredFamilyField::DiagnosticProjection => assert!(matches!(
            error,
            WorthTopologyLegalityCatalogError::MissingDiagnosticProjection(_)
        )),
    }
}
