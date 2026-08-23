use super::*;

#[test]
fn pending_admission_rejects_a_foreign_runtime_correspondence_capability() {
    let (mut left, mut left_correspondence) = installed_owner_parts();
    let (_right, mut right_correspondence) = installed_owner_parts();

    let foreign = right_correspondence.issue(super::tests::basis(41)).unwrap();
    assert!(matches!(
        left.admit_pending(foreign),
        Err(WorthUiPresentationPendingAdmissionDenial::ForeignCorrespondenceAuthority)
    ));

    let local = left_correspondence.issue(super::tests::basis(42)).unwrap();
    assert_eq!(
        left.admit_pending(local).unwrap().observation().posture(),
        WorthUiPresentationAsyncPosture::Pending
    );
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P5-TEXT-ASYNC-PRESENTATION-01\":\"bypass-query-or-stale-presentation-completion\"}}"
    );
}

#[test]
fn rejected_foreign_completion_preserves_the_exact_pending_recovery_receipt() {
    let (mut owner, mut correspondence) = installed_owner_parts();
    let (_foreign_owner, foreign_correspondence) = installed_owner_parts();
    let request = correspondence.issue(super::tests::basis(43)).unwrap();
    let receipt = owner.admit_pending(request).unwrap();

    let foreign_completion = foreign_correspondence.certify_presented(&receipt, 64);
    assert!(matches!(
        owner.admit_presented(&receipt, foreign_completion),
        Err(WorthUiPresentationSettlementDenial::ForeignCompletionAuthority)
    ));
    assert_eq!(
        owner.observation(&receipt).unwrap().posture(),
        WorthUiPresentationAsyncPosture::Pending
    );

    let local_completion = correspondence.certify_presented(&receipt, 64);
    assert_eq!(
        owner
            .admit_presented(&receipt, local_completion)
            .unwrap()
            .observation()
            .posture(),
        WorthUiPresentationAsyncPosture::Current
    );
}

#[test]
fn local_issuer_cannot_launder_a_foreign_colliding_pending_receipt() {
    let (mut left, mut left_correspondence) = installed_owner_parts();
    let (mut right, mut right_correspondence) = installed_owner_parts();
    let shared_basis = super::tests::basis(44);
    let left_receipt = left
        .admit_pending(left_correspondence.issue(shared_basis.clone()).unwrap())
        .unwrap();
    let right_receipt = right
        .admit_pending(right_correspondence.issue(shared_basis).unwrap())
        .unwrap();
    assert_eq!(left_receipt.nonce, right_receipt.nonce);

    let laundered = left_correspondence.certify_presented(&right_receipt, 64);
    assert!(matches!(
        left.admit_presented(&right_receipt, laundered),
        Err(WorthUiPresentationSettlementDenial::ForeignPendingReceiptAuthority)
    ));
    assert_eq!(
        left.observation(&left_receipt).unwrap().posture(),
        WorthUiPresentationAsyncPosture::Pending
    );
}

pub(super) fn installed_owner_parts() -> (
    WorthUiPresentationAsyncOwner,
    WorthUiPresentationCorrespondenceIssuer,
) {
    let plan = WorthUiPresentationAsyncHostPlan::prepare().unwrap();
    let (request, completion) = plan.into_parts();
    let installation =
        worth_query_host::facade::runtime::WorthQueryExecutionRuntimeInstaller::new()
            .install(request.generation(), request.into_packages())
            .unwrap();
    completion
        .complete(installation)
        .unwrap()
        .into_runtime_parts()
}
