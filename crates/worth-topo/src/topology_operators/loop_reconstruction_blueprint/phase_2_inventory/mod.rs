mod operator_rows;
mod validator_rows;

use super::operator_row::PlanarBooleanLoopOperatorRow;
use super::validator_row::PlanarBooleanLoopValidatorRow;

pub(super) fn phase_2_operators() -> Vec<PlanarBooleanLoopOperatorRow> {
    operator_rows::phase_2_operators()
}

pub(super) fn phase_2_validators() -> Vec<PlanarBooleanLoopValidatorRow> {
    validator_rows::phase_2_validators()
}
