use super::{S10ScenarioExecutionMatrix, S10ScenarioExecutionMatrixDenial};

#[test]
fn execution_matrix_rejects_an_empty_schedule_claim() {
    assert_eq!(
        S10ScenarioExecutionMatrix::join([], []).unwrap_err(),
        S10ScenarioExecutionMatrixDenial::Empty
    );
}
