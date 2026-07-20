use super::mutation_sensitivity::{locate_production_edge, MutationCase};
use super::{OperationalRecoveryAction, OperationalRecoveryActionKind as Action};

#[test]
fn every_mutation_family_selects_an_exact_same_operation_production_edge() {
    for case in MutationCase::all() {
        let actions = [
            action("foreign", "foreign-prerequisite", case.prerequisite),
            action("target", "prerequisite", case.prerequisite),
            action("target", "affected", case.affected[0]),
        ];
        let edge = locate_production_edge(case, &actions).expect("production edge");
        assert_eq!(edge.removed_index, 1);
        assert_eq!(edge.affected_index, 2);
    }
}

fn action(operation: &str, transition: &str, kind: Action) -> OperationalRecoveryAction {
    OperationalRecoveryAction::controlled_defect_probe(operation, transition, kind)
}
