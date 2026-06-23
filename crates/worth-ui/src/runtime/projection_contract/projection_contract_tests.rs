use crate::facade::{
    AppearanceTokenId, CommandDescriptor, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface,
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, DensityTokenId, ThemeColorValue, ThemeTokenDescriptor,
    ThemeTokenFamily, ThemeTokenId, ThemeTokenSource, ThemeTokenValue, WorthUi, WorthUiApp,
    WorthUiAppearanceFamily, WorthUiAppearanceTokenDescriptor, WorthUiAppearanceTokenSource,
    WorthUiAppearanceValue, WorthUiDensityFamily, WorthUiDensityTokenDescriptor,
    WorthUiDensityValue, WorthUiLengthValue, WorthUiPaddingValue, WorthUiSpacingValue,
};
use crate::runtime::{
    WorthUiChangedRuntimeFacts, WorthUiDropdownAppearanceRequest, WorthUiHeaderMenuPlan,
    WorthUiHeaderMenuProjectionRequest, WorthUiProjectionDependencyAdmissionDenial,
    WorthUiProjectionDependencyDeclaration, WorthUiProjectionDependencySet,
    WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily, WorthUiProjectionIdentity,
    WorthUiProjectionPlanAdmissionDenial, WorthUiProjectionPlanContract, WorthUiRuntimeFactId,
    WorthUiRuntimeFactSet, WorthUiRuntimeInstanceWitness,
};

use super::{plan_contract::private::Sealed, WorthUiAdmittedProjectionPlan};

#[derive(Clone, Debug, Eq, PartialEq)]
struct EmptyDependencyPlan;

impl Sealed for EmptyDependencyPlan {}

impl WorthUiProjectionPlanContract for EmptyDependencyPlan {
    fn projection_identity(&self) -> WorthUiProjectionIdentity {
        WorthUiProjectionIdentity::runtime("test.empty")
    }

    fn projection_family(&self) -> WorthUiProjectionFamily {
        WorthUiProjectionFamily::HeaderMenu
    }

    fn projection_dependency_declaration(&self) -> WorthUiProjectionDependencyDeclaration {
        WorthUiProjectionDependencyDeclaration::from_set(WorthUiProjectionDependencySet::empty())
    }

    fn projection_equivalence_digest(&self) -> u64 {
        0
    }

    fn projection_equivalence_basis_kind(&self) -> WorthUiProjectionEquivalenceBasisKind {
        WorthUiProjectionEquivalenceBasisKind::ProjectionDigest
    }
}

#[test]
fn projection_plan_admission_rejects_plans_without_declared_dependencies() {
    let denial = WorthUiAdmittedProjectionPlan::admit(EmptyDependencyPlan, runtime_witness())
        .expect_err("empty dependency declaration should not be admissible");

    assert_eq!(
        denial,
        WorthUiProjectionPlanAdmissionDenial::Dependency(
            WorthUiProjectionDependencyAdmissionDenial::EmptyDependencies
        )
    );
}

#[test]
fn admitted_header_menu_plan_preserves_identity_dependencies_and_equivalence_basis() {
    let projection_id = CommandProjectionId::new("workspace.header.file").unwrap();
    let plan = header_menu_plan(&projection_id);
    let admitted = WorthUiAdmittedProjectionPlan::admit(plan, runtime_witness())
        .expect("header menu plan has dependencies");

    assert_eq!(admitted.runtime_instance(), runtime_witness());
    assert_eq!(admitted.proof().runtime_instance(), runtime_witness());
    assert_eq!(
        admitted.dependencies().identity().as_str(),
        "worth-ui.header.menu"
    );
    assert_eq!(
        admitted.dependencies().family(),
        WorthUiProjectionFamily::HeaderMenu
    );
    assert!(admitted
        .dependencies()
        .dependencies()
        .contains_exact(&WorthUiRuntimeFactId::command_projection(&projection_id)));
    assert_eq!(
        admitted.equivalence_basis().kind(),
        WorthUiProjectionEquivalenceBasisKind::ProjectionDigest
    );
    assert_eq!(
        admitted.proof().dependency_digest(),
        admitted
            .dependencies()
            .validation_proof()
            .dependency_digest()
    );
}

#[test]
fn validated_dependency_contract_requires_changed_fact_proof_for_intersection() {
    let projection_id = CommandProjectionId::new("workspace.header.file").unwrap();
    let admitted =
        WorthUiAdmittedProjectionPlan::admit(header_menu_plan(&projection_id), runtime_witness())
            .expect("header menu plan has dependencies");
    let intersecting = WorthUiChangedRuntimeFacts::from_runtime(WorthUiRuntimeFactSet::single(
        WorthUiRuntimeFactId::command_projection(&projection_id),
    ));
    let unrelated = WorthUiChangedRuntimeFacts::from_runtime(WorthUiRuntimeFactSet::single(
        WorthUiRuntimeFactId::theme_token(&ThemeTokenId::new("theme.header.bg").unwrap()),
    ));

    assert!(admitted
        .dependencies()
        .intersects_changed_facts(&intersecting));
    assert!(!admitted.dependencies().intersects_changed_facts(&unrelated));
}

#[test]
fn projection_equivalence_digest_changes_when_frame_digest_changes() {
    let projection_id = CommandProjectionId::new("workspace.header.file").unwrap();
    let first =
        WorthUiAdmittedProjectionPlan::admit(header_menu_plan(&projection_id), runtime_witness())
            .unwrap();
    let second = WorthUiAdmittedProjectionPlan::admit(
        header_menu_plan_with_label(&projection_id, "Save As"),
        runtime_witness(),
    )
    .unwrap();

    assert_eq!(
        first.proof().dependency_digest(),
        second.proof().dependency_digest()
    );
    assert_ne!(
        first.proof().equivalence_digest(),
        second.proof().equivalence_digest()
    );
}

fn runtime_witness() -> WorthUiRuntimeInstanceWitness {
    WorthUiRuntimeInstanceWitness::from_raw(7)
}

fn header_menu_plan(projection_id: &CommandProjectionId) -> WorthUiHeaderMenuPlan {
    header_menu_plan_with_label(projection_id, "Save")
}

fn header_menu_plan_with_label(
    projection_id: &CommandProjectionId,
    label: &str,
) -> WorthUiHeaderMenuPlan {
    let app = header_capability_app(label);

    WorthUiHeaderMenuPlan::from_snapshot(
        app.capabilities(),
        [WorthUiHeaderMenuProjectionRequest::new(
            "File",
            projection_id.clone(),
            ComponentId::new("validation.component.sample").unwrap(),
            ComponentId::new("validation.component.sample").unwrap(),
        )],
        dropdown_appearance_request(),
    )
    .expect("projection references registered command")
}

fn dropdown_appearance_request() -> WorthUiDropdownAppearanceRequest {
    WorthUiDropdownAppearanceRequest::new(
        AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
        DensityTokenId::new("density.header.row_padding").unwrap(),
        DensityTokenId::new("density.header.control_spacing").unwrap(),
    )
}

fn header_capability_app(label: &str) -> WorthUiApp {
    let command_id = CommandId::new("workspace.command.save").unwrap();
    WorthUi::app()
        .register_command(CommandDescriptor::new(command_id.clone(), label))
        .register_command_projection(
            CommandProjectionDescriptor::new(
                CommandProjectionId::new("workspace.header.file").unwrap(),
                CommandProjectionSurface::menu_bar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(command_id)),
        )
        .register_theme_token(ThemeTokenDescriptor::define(
            ThemeTokenId::new("theme.header.bg").unwrap(),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(ThemeColorValue::hex("#1e1e1e").unwrap()),
        ))
        .register_appearance_token(WorthUiAppearanceTokenDescriptor::define(
            AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
            WorthUiAppearanceFamily::Layout,
            WorthUiAppearanceTokenSource::Application,
            WorthUiAppearanceValue::Length(WorthUiLengthValue::from_px("220px").unwrap()),
        ))
        .register_density_token(WorthUiDensityTokenDescriptor::define(
            DensityTokenId::new("density.header.row_padding").unwrap(),
            WorthUiDensityFamily::RowPadding,
            WorthUiDensityValue::Padding(
                WorthUiPaddingValue::from_shorthand_px("1px 6px").unwrap(),
            ),
        ))
        .register_density_token(WorthUiDensityTokenDescriptor::define(
            DensityTokenId::new("density.header.control_spacing").unwrap(),
            WorthUiDensityFamily::ControlSpacing,
            WorthUiDensityValue::Spacing(WorthUiSpacingValue::from_px("8px").unwrap()),
        ))
        .register_component(ComponentDescriptor::new(
            ComponentId::new("validation.component.sample").unwrap(),
            ComponentPropSchema::named("validation.sample.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .freeze()
}
