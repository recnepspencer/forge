use crate::grouped_authoring::{
    forge_query_grouped_declaration_checked_on_handle,
    forge_query_grouped_orchestration_checked_on_handle, ForgeQueryGroupedDeclarationChecked,
    ForgeQueryGroupedDeclarationInput, ForgeQueryGroupedOrchestrationChecked,
};

use super::support::{admitted_handle, GeometryInput};

#[test]
fn helper_grouped_declaration_matches_generic_grouped_path() {
    let handle = admitted_handle("main");
    let helper_input = handle
        .geometry_helpers()
        .local_neighborhood_for_active_face_selection(GeometryInput::new("face-a"))
        .with_member(GeometryInput::new("face-b"))
        .with_shared_rationale("split the local neighborhood");
    let generic_input =
        ForgeQueryGroupedDeclarationInput::local_neighborhood(GeometryInput::new("face-a"))
            .with_member(GeometryInput::new("face-b"))
            .with_shared_rationale("split the local neighborhood");

    let helper = handle
        .geometry_helpers()
        .declare_local_neighborhood_for_active_face_selection_checked(helper_input);
    let generic = forge_query_grouped_declaration_checked_on_handle(&handle, generic_input);

    match (helper, generic) {
        (
            ForgeQueryGroupedDeclarationChecked::Bound(left),
            ForgeQueryGroupedDeclarationChecked::Bound(right),
        ) => {
            assert_eq!(left.group_digest(), right.group_digest());
            assert_eq!(left.shared_rationale(), right.shared_rationale());
        }
        _ => panic!("expected grouped declaration parity"),
    }
}

#[test]
fn grouped_orchestration_matches_generic_checked_lowering() {
    let handle = admitted_handle("main");
    let declaration = handle
        .geometry_helpers()
        .declare_local_neighborhood_for_active_face_selection(
            handle
                .geometry_helpers()
                .local_neighborhood_for_active_face_selection(GeometryInput::new("face-a"))
                .with_member(GeometryInput::new("face-b")),
        )
        .unwrap();

    let helper = handle
        .geometry_helpers()
        .orchestrate_local_neighborhood_for_active_face_selection_checked(declaration.clone());
    let generic = forge_query_grouped_orchestration_checked_on_handle(&handle, declaration);

    match (helper, generic) {
        (
            ForgeQueryGroupedOrchestrationChecked::Bound(left),
            ForgeQueryGroupedOrchestrationChecked::Bound(right),
        ) => {
            assert_eq!(left.orchestration_digest(), right.orchestration_digest());
            assert_eq!(
                left.member_envelopes().len(),
                right.member_envelopes().len()
            );
            assert_eq!(
                left.member_envelopes()[0].envelope().declaration_digest(),
                right.member_envelopes()[0].envelope().declaration_digest()
            );
        }
        _ => panic!("expected grouped orchestration parity"),
    }
}
