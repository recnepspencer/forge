use crate::application::{
    ForgeQueryMilestoneClosureStatus, ForgeQueryMilestoneNineSevenDerivedClosure,
    ForgeQueryMilestoneNineSevenPhaseClosure,
};

#[test]
fn milestone_nine_seven_closes_only_from_required_phase_local_closures() {
    let closed = phase_local_closure(ForgeQueryMilestoneClosureStatus::Closed);

    assert_eq!(closed.status(), ForgeQueryMilestoneClosureStatus::Closed);
    assert_eq!(
        closed.phase_closures().len(),
        ForgeQueryMilestoneNineSevenDerivedClosure::required_phases().len()
    );
    assert!(!closed.closure_digest().is_empty());

    for index in 0..ForgeQueryMilestoneNineSevenDerivedClosure::required_phases().len() {
        let sabotaged = phase_local_closure_with_status_at(
            index,
            ForgeQueryMilestoneClosureStatus::Open,
            "phase-local-sabotage",
        );
        assert_ne!(
            sabotaged.status(),
            ForgeQueryMilestoneClosureStatus::Closed,
            "required phase {index} must reopen milestone 9.7 when its local proof opens"
        );
    }

    let empty_digest =
        phase_local_closure_with_status_at(2, ForgeQueryMilestoneClosureStatus::Closed, "");
    assert_ne!(
        empty_digest.status(),
        ForgeQueryMilestoneClosureStatus::Closed,
        "closed status without a phase-local evidence digest must not close milestone 9.7"
    );
}

#[test]
fn milestone_nine_seven_support_profile_contract_names_defended_exclusion() {
    let contract =
        ForgeQueryMilestoneNineSevenDerivedClosure::support_profile_publication_contract();

    assert_eq!(contract.status(), ForgeQueryMilestoneClosureStatus::Partial);
    assert_eq!(
        contract.defended_exclusions(),
        &["store-backed execution parity belongs to Milestone 10".to_string()]
    );
    assert!(contract
        .phase_closures()
        .iter()
        .all(|closure| closure.status() == ForgeQueryMilestoneClosureStatus::Partial));
}

#[test]
fn milestone_nine_seven_docs_and_test_requirements_publish_phase_eighteen() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let docs_root = crate_root
        .parent()
        .and_then(std::path::Path::parent)
        .expect("crate should live under workspace crates directory")
        .join("_docs/forge-query");
    let test_requirements = std::fs::read_to_string(docs_root.join("test-requirements.md"))
        .expect("test requirements should be readable");
    let closeout = std::fs::read_to_string(docs_root.join("milestone-9.7-closeout.md"))
        .expect("milestone 9.7 closeout should be readable");

    assert!(test_requirements.contains("Milestone 9.7 Phase 18 Required Suite"));
    assert!(test_requirements.contains("Milestone 9.7 Derived Closure Posture Test"));
    assert!(closeout.contains("Milestone 9.7 Closeout"));
    assert!(closeout.contains("Status: Closed"));
    for phase in ForgeQueryMilestoneNineSevenDerivedClosure::required_phases() {
        assert!(
            closeout.contains(phase),
            "closeout must name required phase-local proof {phase}"
        );
    }
}

fn phase_local_closure(
    status: ForgeQueryMilestoneClosureStatus,
) -> ForgeQueryMilestoneNineSevenDerivedClosure {
    ForgeQueryMilestoneNineSevenDerivedClosure::derive_from_phase_closures(
        ForgeQueryMilestoneNineSevenDerivedClosure::required_phases()
            .iter()
            .copied()
            .map(|phase| {
                ForgeQueryMilestoneNineSevenPhaseClosure::new(
                    phase,
                    status,
                    format!("{phase}:digest"),
                )
            }),
        ["store-backed execution parity belongs to Milestone 10"],
    )
}

fn phase_local_closure_with_status_at(
    target_index: usize,
    target_status: ForgeQueryMilestoneClosureStatus,
    target_digest: &str,
) -> ForgeQueryMilestoneNineSevenDerivedClosure {
    ForgeQueryMilestoneNineSevenDerivedClosure::derive_from_phase_closures(
        ForgeQueryMilestoneNineSevenDerivedClosure::required_phases()
            .iter()
            .copied()
            .enumerate()
            .map(|(index, phase)| {
                let status = if index == target_index {
                    target_status
                } else {
                    ForgeQueryMilestoneClosureStatus::Closed
                };
                let digest = if index == target_index {
                    target_digest.to_string()
                } else {
                    format!("{phase}:digest")
                };
                ForgeQueryMilestoneNineSevenPhaseClosure::new(phase, status, digest)
            }),
        ["store-backed execution parity belongs to Milestone 10"],
    )
}
