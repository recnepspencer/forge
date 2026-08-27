mod ancestry;
mod application;
mod state;
mod unique;

pub(crate) use ancestry::{AcceptedDelta, AncestryError, OracleAncestry, OracleBranch};
pub(crate) use application::{
    apply, apply_from_parent, reject_duplicate_relation, OracleApplicationError,
};
pub(crate) use state::OracleState;
pub(crate) use unique::{
    insert_vessel, next_vessel_key, vessel_call_signs, UniqueEntityFieldOracleError,
};

impl OracleBranch {
    pub(crate) fn apply(&self, delta: super::DeltaId) -> Result<Self, OracleApplicationError> {
        application::apply(self, delta)
    }
}
