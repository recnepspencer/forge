use worth_ui::facade::{
    app::WorthUi,
    declaration::{
        CommandId, ComponentId, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass,
        SurfaceStateClass, ViewBindingId,
    },
    diagnostics::{CapabilityDiagnosticCode, CapabilityRegistrationDiagnostic},
};

#[test]
fn surface_descriptor_reports_multiple_independent_violations() {
    let report = WorthUi::app()
        .register_surface(
            SurfaceDescriptor::new(
                surface_id("workspace.surface.invalid"),
                SurfaceKind::product_domain_name_for_diagnostics("project explorer"),
                component_id("workspace.component.missing"),
                SurfacePlacementClass::unsupported_for_diagnostics("floating-dock"),
                SurfaceStateClass::invalid_for_diagnostics("ambient-memory"),
            )
            .with_command_slot(command_id("workspace.command.missing"))
            .with_view_binding(view_binding_id("workspace.view_binding.missing")),
        )
        .freeze_with_registration_report();

    assert!(report.has_errors());
    assert!(report.accepted_snapshot().surfaces().is_empty());
    assert_exact_diagnostic_topology(
        report.registration_diagnostics(),
        &[
            DiagnosticTopology::descriptor(
                CapabilityDiagnosticCode::UnsupportedSurfacePlacementClass,
                "workspace.surface.invalid",
            ),
            DiagnosticTopology::descriptor(
                CapabilityDiagnosticCode::InvalidSurfaceStateClass,
                "workspace.surface.invalid",
            ),
            DiagnosticTopology::descriptor(
                CapabilityDiagnosticCode::ProductDomainSurfaceKind,
                "workspace.surface.invalid",
            ),
            DiagnosticTopology::dependency(
                "workspace.surface.invalid",
                "command",
                "workspace.command.missing",
            ),
            DiagnosticTopology::dependency(
                "workspace.surface.invalid",
                "component",
                "workspace.component.missing",
            ),
            DiagnosticTopology::dependency(
                "workspace.surface.invalid",
                "view_binding",
                "workspace.view_binding.missing",
            ),
        ],
    );
}

#[derive(Debug, Eq, PartialEq)]
struct DiagnosticTopology<'a> {
    code: CapabilityDiagnosticCode,
    identity_text: &'a str,
    related_family_name: Option<&'a str>,
    related_identity_text: Option<&'a str>,
}

impl<'a> DiagnosticTopology<'a> {
    fn descriptor(code: CapabilityDiagnosticCode, identity_text: &'a str) -> Self {
        Self {
            code,
            identity_text,
            related_family_name: None,
            related_identity_text: None,
        }
    }

    fn dependency(
        identity_text: &'a str,
        related_family_name: &'a str,
        related_identity_text: &'a str,
    ) -> Self {
        Self {
            code: CapabilityDiagnosticCode::MissingDependency,
            identity_text,
            related_family_name: Some(related_family_name),
            related_identity_text: Some(related_identity_text),
        }
    }
}

fn assert_exact_diagnostic_topology(
    diagnostics: &[CapabilityRegistrationDiagnostic],
    expected_topology: &[DiagnosticTopology],
) {
    let actual_topology = diagnostics
        .iter()
        .map(diagnostic_topology)
        .collect::<Vec<_>>();

    assert_eq!(actual_topology, expected_topology);
}

fn diagnostic_topology(diagnostic: &CapabilityRegistrationDiagnostic) -> DiagnosticTopology<'_> {
    DiagnosticTopology {
        code: diagnostic.code(),
        identity_text: diagnostic
            .identity_text()
            .expect("diagnostic should identify the invalid surface"),
        related_family_name: diagnostic.related_family_name(),
        related_identity_text: diagnostic.related_identity_text(),
    }
}

fn command_id(raw_text: &str) -> CommandId {
    CommandId::new(raw_text).expect("valid command id")
}

fn component_id(raw_text: &str) -> ComponentId {
    ComponentId::new(raw_text).expect("valid component id")
}

fn surface_id(raw_text: &str) -> SurfaceId {
    SurfaceId::new(raw_text).expect("valid surface id")
}

fn view_binding_id(raw_text: &str) -> ViewBindingId {
    ViewBindingId::new(raw_text).expect("valid view binding id")
}
