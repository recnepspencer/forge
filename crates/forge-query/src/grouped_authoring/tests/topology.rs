use crate::grouped_authoring::{
    forge_query_grouped_declaration_checked_on_handle,
    forge_query_grouped_orchestration_proof_on_handle,
    ordinary_outcome_from_grouped_orchestration_checked, ForgeQueryGroupedDeclarationChecked,
    ForgeQueryGroupedOrchestrationChecked,
};
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryBindingCheckedTopologyKind, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPostureKind,
};

use super::support::{admitted_handle, GeometryInput};

#[test]
fn grouped_wrong_world_projects_to_binding_topology() {
    let left = admitted_handle("main");
    let right = admitted_handle("other");
    let declaration = match forge_query_grouped_declaration_checked_on_handle(
        &left,
        left.geometry_helpers()
            .local_neighborhood_for_active_face_selection(GeometryInput::new("face-a"))
            .with_member(GeometryInput::new("face-b")),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("expected grouped declaration admission")
        }
    };

    let outcome = ordinary_outcome_from_grouped_orchestration_checked(
        right
            .geometry_helpers()
            .orchestrate_local_neighborhood_for_active_face_selection_checked(declaration),
    );

    match outcome {
        ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            assert_eq!(posture.kind(), ForgeQueryOrdinaryPostureKind::WrongWorld);
            assert_eq!(
                posture.next_step(),
                ForgeQueryOrdinaryNextStep::CorrectWorld
            );
            assert_eq!(
                posture.checked_topology().binding_kind(),
                Some(ForgeQueryOrdinaryBindingCheckedTopologyKind::WrongWorld)
            );
            assert!(
                posture.reason().contains("different operating context"),
                "expected grouped wrong-world reason, got {:?}",
                posture.reason()
            );
        }
        _ => panic!("expected grouped wrong-world ordinary outcome"),
    }
}

#[test]
fn grouped_proof_into_checked_preserves_alignment_stop() {
    let left = admitted_handle("main");
    let right = admitted_handle("other");
    let declaration = match forge_query_grouped_declaration_checked_on_handle(
        &left,
        left.geometry_helpers()
            .local_neighborhood_for_active_face_selection(GeometryInput::new("face-a")),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("expected grouped declaration admission")
        }
    };

    let checked =
        forge_query_grouped_orchestration_proof_on_handle(&right, declaration).into_checked();

    match checked {
        ForgeQueryGroupedOrchestrationChecked::WrongWorld(stop) => {
            assert!(
                stop.reason().contains("different operating context"),
                "expected grouped proof alignment stop, got {:?}",
                stop.reason()
            );
        }
        _ => panic!("expected grouped wrong-world checked stop"),
    }
}
