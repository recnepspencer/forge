use crate::grouped_authoring::{
    forge_query_grouped_declaration_checked_on_handle,
    forge_query_grouped_orchestration_proof_on_handle,
    ordinary_outcome_from_grouped_orchestration_checked, ForgeQueryGroupedAtomicity,
    ForgeQueryGroupedContinuityAssumption, ForgeQueryGroupedDeclarationChecked,
    ForgeQueryGroupedDeclarationInput, ForgeQueryGroupedOrchestrationChecked,
    ForgeQueryGroupedSharedPostureClaim, ForgeQueryGroupedSupportFeature,
    ForgeQueryGroupedSupportStatus,
};
use crate::ordinary_outcome::{
    ForgeQueryOrdinaryBindingCheckedTopologyKind, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPostureKind,
};

use super::support::{admitted_handle, GeometryInput, RequiredIntentGeometryInput};

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

    let proof = forge_query_grouped_orchestration_proof_on_handle(&right, declaration);
    assert!(
        proof.member_transcripts().is_empty(),
        "wrong-world proof should short-circuit before member lowering"
    );
    let checked = proof.into_checked();

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

#[test]
fn grouped_proof_member_stop_short_circuits_after_first_failing_member() {
    let handle = admitted_handle("main");
    let declaration = match forge_query_grouped_declaration_checked_on_handle(
        &handle,
        ForgeQueryGroupedDeclarationInput::local_neighborhood(RequiredIntentGeometryInput::new(
            "edge-a",
        ))
        .with_member(RequiredIntentGeometryInput::new("edge-b")),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("expected grouped declaration admission")
        }
    };

    let proof = forge_query_grouped_orchestration_proof_on_handle(&handle, declaration);
    assert_eq!(
        proof.member_transcripts().len(),
        1,
        "proof should stop after the first failing member instead of lowering later members"
    );

    match proof.outcome() {
        ForgeQueryGroupedOrchestrationChecked::MemberStopped(stop) => {
            assert_eq!(stop.member_index(), 0);
            assert_eq!(stop.member_role().as_str(), "seed");
        }
        _ => panic!("expected grouped member-stop proof outcome"),
    }
}

#[test]
fn grouped_support_report_marks_unsupported_shared_claims() {
    let handle = admitted_handle("main");
    let declaration = match forge_query_grouped_declaration_checked_on_handle(
        &handle,
        ForgeQueryGroupedDeclarationInput::local_neighborhood(RequiredIntentGeometryInput::new(
            "edge-a",
        ))
        .with_member(RequiredIntentGeometryInput::new("edge-b"))
        .with_atomicity(ForgeQueryGroupedAtomicity::Atomic)
        .with_continuity_assumption(ForgeQueryGroupedContinuityAssumption::PreserveNeighborhood)
        .with_shared_posture_claim(ForgeQueryGroupedSharedPostureClaim::SharedMaterialPreview)
        .with_shared_posture_claim(ForgeQueryGroupedSharedPostureClaim::SharedContinuity),
    ) {
        ForgeQueryGroupedDeclarationChecked::Bound(value) => value,
        ForgeQueryGroupedDeclarationChecked::MemberStopped(_) => {
            panic!("expected grouped declaration admission")
        }
    };

    let report = handle.grouped_support_report(&declaration);
    assert_eq!(
        report.status_for(ForgeQueryGroupedSupportFeature::SharedPostureClaims),
        ForgeQueryGroupedSupportStatus::Unsupported
    );
    assert!(report
        .unsupported_claims()
        .contains(&ForgeQueryGroupedSharedPostureClaim::SharedMaterialPreview));
    assert!(!report
        .unsupported_claims()
        .contains(&ForgeQueryGroupedSharedPostureClaim::SharedContinuity));
}
