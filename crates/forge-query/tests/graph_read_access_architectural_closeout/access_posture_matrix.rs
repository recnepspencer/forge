use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessDenialKind, ForgeQueryGraphReadAccessRequirementKind,
    ForgeQueryGraphReadRequiredCapabilityOwner,
};

use crate::graph_read_access_cost_model_support::{
    dense_traversal_family, projection_only_family, simple_traversal_family, workspace,
};
use crate::support::graph_index_inventory::runtime_profiles::{
    default_graph_support_workspace, profile_requiring_graph_access_capability_registration,
    profile_requiring_store_backed_graph_index, profile_with_ephemeral_graph_support,
    profile_with_graph_support_temporarily_unavailable, profile_without_graph_support,
    workspace_with_graph_support,
};
use crate::support::graph_read_access::read_surface_declarations::graph_access_family;

#[test]
fn closeout_matrix_certifies_representative_access_postures() {
    assert_posture(
        "closeout.matrix.inline",
        |workspace| projection_only_family(workspace, "closeout-inline"),
        ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed,
        None,
        None,
    );

    let mut traversal_workspace =
        default_graph_support_workspace("graph-read-access.closeout.matrix.traversal");
    let traversal = graph_access_family(&mut traversal_workspace, "closeout-traversal");
    assert_admission_posture(
        &admission_for_family(&mut traversal_workspace, &traversal),
        ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed,
        None,
        None,
    );

    let mut ephemeral_workspace = workspace_with_graph_support(
        "graph-read-access.closeout.matrix.ephemeral",
        profile_with_ephemeral_graph_support(
            ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        ),
    );
    let ephemeral = simple_traversal_family(&mut ephemeral_workspace, "closeout-ephemeral");
    assert_admission_posture(
        &admission_for_family(&mut ephemeral_workspace, &ephemeral),
        ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex,
        None,
        None,
    );

    assert_posture(
        "closeout.matrix.budget-denied",
        |workspace| dense_traversal_family(workspace, "closeout-budget-denied"),
        ForgeQueryGraphReadAccessAdmissionPosture::Denied,
        Some(ForgeQueryGraphReadAccessDenialKind::BudgetExceeded),
        Some(ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired),
    );

    assert_support_profile_posture(
        profile_without_graph_support(
            ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        ),
        ForgeQueryGraphReadAccessAdmissionPosture::Denied,
        Some(ForgeQueryGraphReadAccessDenialKind::UnsupportedGraphIndexSupport),
        Some(ForgeQueryGraphReadAccessAdmissionPosture::Denied),
    );

    assert_support_profile_posture(
        profile_with_graph_support_temporarily_unavailable(
            ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        ),
        ForgeQueryGraphReadAccessAdmissionPosture::Denied,
        Some(ForgeQueryGraphReadAccessDenialKind::RequiredAsyncMaterialization),
        Some(ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired),
    );

    assert_support_profile_posture(
        profile_requiring_store_backed_graph_index(
            ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        ),
        ForgeQueryGraphReadAccessAdmissionPosture::Denied,
        Some(ForgeQueryGraphReadAccessDenialKind::RequiredPersistentIndex),
        Some(ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired),
    );

    assert_support_profile_posture(
        profile_requiring_graph_access_capability_registration(
            ForgeQueryGraphReadAccessRequirementKind::DirectionalAdjacency,
        ),
        ForgeQueryGraphReadAccessAdmissionPosture::Denied,
        Some(ForgeQueryGraphReadAccessDenialKind::RequiredAccessCapabilityRegistration),
        Some(ForgeQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired),
    );
}

#[test]
fn closeout_matrix_every_requirement_has_case_and_support_owner() {
    let mut workspace = workspace("graph-read-access.closeout.matrix.requirements");
    let family = simple_traversal_family(&mut workspace, "closeout-requirements");
    let admission = admission_for_family(&mut workspace, &family);

    for requirement_row in admission.requirement_set().rows() {
        let case = admission
            .case_registry()
            .case_for_requirement_kind(requirement_row.kind())
            .expect("every requirement row must map to an access case");
        assert_eq!(case.requirement_kind(), requirement_row.kind());
    }

    assert!(admission
        .graph_index_inventory_match_report()
        .matches()
        .iter()
        .all(
            |inventory_match| inventory_match.required_capability_owner()
                == &ForgeQueryGraphReadRequiredCapabilityOwner::QueryRuntime
        ));
}

fn assert_posture(
    workspace_name: &str,
    family: impl FnOnce(
        &mut forge_query::facade::runtime::ForgeQueryWorkspace,
    ) -> forge_query::facade::runtime::ForgeQueryReadFamily,
    expected_posture: ForgeQueryGraphReadAccessAdmissionPosture,
    expected_denial: Option<ForgeQueryGraphReadAccessDenialKind>,
    expected_suggested_posture: Option<ForgeQueryGraphReadAccessAdmissionPosture>,
) {
    let mut workspace = workspace(workspace_name);
    let family = family(&mut workspace);
    assert_admission_posture(
        &admission_for_family(&mut workspace, &family),
        expected_posture,
        expected_denial,
        expected_suggested_posture,
    );
}

fn assert_support_profile_posture(
    support_profile: forge_query::facade::runtime::ForgeQueryRuntimeSupportProfile,
    expected_posture: ForgeQueryGraphReadAccessAdmissionPosture,
    expected_denial: Option<ForgeQueryGraphReadAccessDenialKind>,
    expected_suggested_posture: Option<ForgeQueryGraphReadAccessAdmissionPosture>,
) {
    let mut workspace =
        workspace_with_graph_support("graph-read-access.closeout.matrix.support", support_profile);
    let family = simple_traversal_family(&mut workspace, "closeout-support-posture");
    assert_admission_posture(
        &admission_for_family(&mut workspace, &family),
        expected_posture,
        expected_denial,
        expected_suggested_posture,
    );
}

fn admission_for_family(
    workspace: &mut forge_query::facade::runtime::ForgeQueryWorkspace,
    family: &forge_query::facade::runtime::ForgeQueryReadFamily,
) -> ForgeQueryGraphReadAccessAdmission {
    workspace
        .read_family_intent(family)
        .review()
        .expect("closeout read should review")
        .graph_read_access_admission()
        .expect("closeout read should produce access admission")
}

fn assert_admission_posture(
    admission: &ForgeQueryGraphReadAccessAdmission,
    expected_posture: ForgeQueryGraphReadAccessAdmissionPosture,
    expected_denial: Option<ForgeQueryGraphReadAccessDenialKind>,
    expected_suggested_posture: Option<ForgeQueryGraphReadAccessAdmissionPosture>,
) {
    assert_eq!(admission.posture(), &expected_posture);
    assert_eq!(
        admission.denial().map(|denial| denial.kind().clone()),
        expected_denial
    );
    assert_eq!(
        admission
            .denial()
            .map(|denial| denial.suggested_posture().clone()),
        expected_suggested_posture
    );
}
