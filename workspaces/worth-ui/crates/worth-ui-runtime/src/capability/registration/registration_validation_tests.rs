use crate::capability::{
    CapabilityDiagnosticCode, CapabilityDiagnosticRichness, CapabilityRegistrationDiagnostic,
    CapabilitySnapshot, CapabilitySupportKind,
};

use super::registration_candidate::{RegistrationCandidate, RegistrationDependency};
use super::registration_validation::validate_registration_candidates;
use super::registration_validation_report::RegistrationValidationReport;

const COMMAND: &str = "command";
const COMPONENT: &str = "component";
const SURFACE: &str = "surface";

#[test]
fn invalid_registration_replay_produces_identical_diagnostics() {
    let first = validate_registration_candidates(&invalid_candidates(), rich());
    let second = validate_registration_candidates(&invalid_candidates(), rich());

    assert_eq!(diagnostic_codes(&first), diagnostic_codes(&second));
    assert_eq!(first, second);
}

#[test]
fn shuffled_invalid_registration_diagnostics_are_canonical() {
    let ordered = validate_registration_candidates(&invalid_candidates(), rich());
    let shuffled = validate_registration_candidates(&shuffled_invalid_candidates(), rich());

    assert_eq!(
        diagnostic_fingerprints(&ordered),
        diagnostic_fingerprints(&shuffled)
    );
}

#[test]
fn diagnostic_codes_distinguish_failure_topology() {
    let report = validate_registration_candidates(&invalid_candidates(), rich());

    assert_eq!(
        diagnostic_codes(&report),
        [
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::UnsupportedPostureReference,
            CapabilityDiagnosticCode::MissingDependency,
            CapabilityDiagnosticCode::FamilyMismatch,
        ]
    );
}

#[test]
fn diagnostics_do_not_change_accepted_snapshot_digest() {
    let minimal = validate_registration_candidates(&accepted_candidates(), minimal());
    let rich = validate_registration_candidates(&accepted_candidates(), rich());

    let minimal_snapshot =
        CapabilitySnapshot::from_registered_capabilities(minimal.accepted_capabilities().clone());
    let rich_snapshot =
        CapabilitySnapshot::from_registered_capabilities(rich.accepted_capabilities().clone());

    assert_eq!(minimal_snapshot.digest(), rich_snapshot.digest());
}

#[test]
fn valid_dependency_reference_remains_accepted_snapshot_meaning() {
    let report = validate_registration_candidates(&candidates_with_valid_dependency(), rich());
    let snapshot =
        CapabilitySnapshot::from_registered_capabilities(report.accepted_capabilities().clone());

    assert!(report.diagnostics().is_empty());
    assert_eq!(snapshot.metrics().registered_family_count(), 2);
    assert_eq!(snapshot.metrics().total_width(), 2);
}

#[test]
fn invalid_diagnostic_richness_does_not_change_accepted_snapshot_digest() {
    let minimal = validate_registration_candidates(&invalid_candidates(), minimal());
    let rich = validate_registration_candidates(&invalid_candidates(), rich());

    let minimal_snapshot =
        CapabilitySnapshot::from_registered_capabilities(minimal.accepted_capabilities().clone());
    let rich_snapshot =
        CapabilitySnapshot::from_registered_capabilities(rich.accepted_capabilities().clone());

    assert_eq!(diagnostic_codes(&minimal), diagnostic_codes(&rich));
    assert!(minimal
        .diagnostics()
        .iter()
        .all(|diagnostic| diagnostic.detail().is_none()));
    assert!(rich
        .diagnostics()
        .iter()
        .any(|diagnostic| diagnostic.detail().is_some()));
    assert_eq!(minimal_snapshot.digest(), rich_snapshot.digest());
}

#[test]
fn dependency_reference_to_duplicate_target_is_not_accepted_as_resolved() {
    let report = validate_registration_candidates(&duplicate_target_dependency(), rich());
    let snapshot =
        CapabilitySnapshot::from_registered_capabilities(report.accepted_capabilities().clone());

    assert_eq!(
        diagnostic_codes(&report),
        [
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::DuplicateCapabilityId,
            CapabilityDiagnosticCode::MissingDependency,
        ]
    );
    assert_eq!(snapshot.metrics().total_width(), 0);
}

fn invalid_candidates() -> Vec<RegistrationCandidate> {
    vec![
        command("app.command.save"),
        command("app.command.save"),
        RegistrationCandidate::with_support(
            COMMAND,
            "app.command.retired",
            CapabilitySupportKind::Unsupported,
        ),
        command("app.command.preview").with_dependency(RegistrationDependency::new(
            COMPONENT,
            COMPONENT,
            "app.component.missing",
        )),
        command("app.command.surface").with_dependency(RegistrationDependency::new(
            COMPONENT,
            SURFACE,
            "app.surface.main",
        )),
    ]
}

fn shuffled_invalid_candidates() -> Vec<RegistrationCandidate> {
    vec![
        command("app.command.surface").with_dependency(RegistrationDependency::new(
            COMPONENT,
            SURFACE,
            "app.surface.main",
        )),
        RegistrationCandidate::with_support(
            COMMAND,
            "app.command.retired",
            CapabilitySupportKind::Unsupported,
        ),
        command("app.command.save"),
        command("app.command.preview").with_dependency(RegistrationDependency::new(
            COMPONENT,
            COMPONENT,
            "app.component.missing",
        )),
        command("app.command.save"),
    ]
}

fn accepted_candidates() -> Vec<RegistrationCandidate> {
    vec![
        command("app.command.save"),
        RegistrationCandidate::admitted(COMPONENT, "app.component.editor"),
    ]
}

fn candidates_with_valid_dependency() -> Vec<RegistrationCandidate> {
    vec![
        command("app.command.save").with_dependency(RegistrationDependency::new(
            COMPONENT,
            COMPONENT,
            "app.component.editor",
        )),
        RegistrationCandidate::admitted(COMPONENT, "app.component.editor"),
    ]
}

fn duplicate_target_dependency() -> Vec<RegistrationCandidate> {
    vec![
        RegistrationCandidate::admitted(COMPONENT, "app.component.editor"),
        RegistrationCandidate::admitted(COMPONENT, "app.component.editor"),
        command("app.command.save").with_dependency(RegistrationDependency::new(
            COMPONENT,
            COMPONENT,
            "app.component.editor",
        )),
    ]
}

fn command(identity_text: &str) -> RegistrationCandidate {
    RegistrationCandidate::admitted(COMMAND, identity_text)
}

fn diagnostic_codes(report: &RegistrationValidationReport) -> Vec<CapabilityDiagnosticCode> {
    report
        .diagnostics()
        .iter()
        .map(CapabilityRegistrationDiagnostic::code)
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct DiagnosticFingerprint {
    code: CapabilityDiagnosticCode,
    family_name: Option<String>,
    identity_text: Option<String>,
    related_family_name: Option<String>,
    related_identity_text: Option<String>,
}

fn diagnostic_fingerprints(report: &RegistrationValidationReport) -> Vec<DiagnosticFingerprint> {
    report
        .diagnostics()
        .iter()
        .map(|diagnostic| DiagnosticFingerprint {
            code: diagnostic.code(),
            family_name: diagnostic.family_name().map(str::to_owned),
            identity_text: diagnostic.identity_text().map(str::to_owned),
            related_family_name: diagnostic.related_family_name().map(str::to_owned),
            related_identity_text: diagnostic.related_identity_text().map(str::to_owned),
        })
        .collect()
}

fn minimal() -> CapabilityDiagnosticRichness {
    CapabilityDiagnosticRichness::Minimal
}

fn rich() -> CapabilityDiagnosticRichness {
    CapabilityDiagnosticRichness::Rich
}
