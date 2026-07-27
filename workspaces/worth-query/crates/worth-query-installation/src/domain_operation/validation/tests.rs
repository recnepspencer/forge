use std::num::NonZeroU32;

use super::validate_invariant_execution;
use crate::domain_operation::{
    WorthQueryInstalledInvariantExecutionRequirement, WorthQueryInvariantEnforcement,
    WorthQueryInvariantExecutionContract, WorthQueryOperationInvariantContract,
    WorthQueryOperationTouchContract,
};

#[test]
fn execution_requirements_need_declared_invariant_slots() {
    let execution =
        WorthQueryInvariantExecutionContract::declared([requirement("closed-loop")]).unwrap();
    assert_eq!(
        validate_invariant_execution(
            &WorthQueryOperationInvariantContract::NotRequired,
            &execution,
            &touches(),
        ),
        Err("invariant-execution-without-declared-slots")
    );
    assert_eq!(
        validate_invariant_execution(
            &WorthQueryOperationInvariantContract::Declared {
                invariant_slots: vec!["manifold".to_owned()],
            },
            &execution,
            &touches(),
        ),
        Err("invariant-execution-slot-set-mismatch")
    );
}

#[test]
fn every_execution_requirement_may_reference_an_installed_slot() {
    let execution =
        WorthQueryInvariantExecutionContract::declared([requirement("closed-loop")]).unwrap();
    assert_eq!(
        validate_invariant_execution(
            &WorthQueryOperationInvariantContract::Declared {
                invariant_slots: vec!["closed-loop".to_owned()],
            },
            &execution,
            &touches(),
        ),
        Ok(())
    );
}

#[test]
fn every_declared_invariant_needs_an_exact_requirement_and_declared_executor() {
    let slots = WorthQueryOperationInvariantContract::Declared {
        invariant_slots: vec!["closed-loop".to_owned(), "manifold".to_owned()],
    };
    let incomplete =
        WorthQueryInvariantExecutionContract::declared([requirement("closed-loop")]).unwrap();
    assert_eq!(
        validate_invariant_execution(&slots, &incomplete, &touches()),
        Err("invariant-execution-slot-set-mismatch")
    );
    let foreign = WorthQueryInvariantExecutionContract::declared([
        requirement("closed-loop"),
        WorthQueryInstalledInvariantExecutionRequirement::new(
            "manifold",
            "topology",
            NonZeroU32::new(1).unwrap(),
            WorthQueryInvariantEnforcement::Blocking,
            "foreign-graph",
            ["region"],
            4,
            8,
        )
        .unwrap(),
    ])
    .unwrap();
    assert_eq!(
        validate_invariant_execution(&slots, &foreign, &touches()),
        Err("invariant-executor-role-does-not-own-provisional-state")
    );
}

#[test]
fn declared_invariant_slots_cannot_omit_execution_requirements() {
    assert_eq!(
        validate_invariant_execution(
            &WorthQueryOperationInvariantContract::Declared {
                invariant_slots: vec!["closed-loop".to_owned()],
            },
            &WorthQueryInvariantExecutionContract::NotRequired,
            &touches(),
        ),
        Err("declared-invariant-without-execution-requirement")
    );
}

fn requirement(slot: &str) -> WorthQueryInstalledInvariantExecutionRequirement {
    WorthQueryInstalledInvariantExecutionRequirement::new(
        slot,
        "topology",
        NonZeroU32::new(1).unwrap(),
        WorthQueryInvariantEnforcement::Blocking,
        "graph",
        ["region"],
        4,
        8,
    )
    .unwrap()
}

fn touches() -> WorthQueryOperationTouchContract {
    WorthQueryOperationTouchContract::Declared {
        graph_roles: vec!["graph".to_owned()],
        scopes: vec!["region".to_owned()],
    }
}
