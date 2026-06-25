use super::super::super::super::catalog::DerivedTopologyProductFamilyIdentity;
use super::super::super::super::execution::{
    DerivedInvalidationExecutionReceipt, PlannedDerivedInvalidationProductExecutor,
};
use super::super::super::super::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    loop_cycles_touched_closure,
};
use super::super::super::super::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
    DerivedInvalidationTouchedClosure,
};
use crate::topology_operators::application::TopologyDeclaredMutationArtifact;

pub(in super::super) fn selected_plan() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("phase-seven-operator-cutover"),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("phase seven selected plan")
}

pub(in super::super) fn selected_plan_from_operator_artifact(
    artifact: &TopologyDeclaredMutationArtifact,
) -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &DerivedInvalidationTouchedClosure::from_declared_touch(artifact.declared_touched_basis()),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("phase seven selected plan from real operator artifact")
}

pub(in super::super) fn execution_receipt(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> DerivedInvalidationExecutionReceipt {
    DerivedInvalidationExecutionReceipt::execute_selected_plan_with_executor(
        selected_plan,
        &PlannedDerivedInvalidationProductExecutor,
    )
    .expect("phase seven execution receipt")
}

pub(super) fn selected_family_identities(
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> impl Iterator<Item = DerivedTopologyProductFamilyIdentity> + '_ {
    selected_plan
        .selected_rows()
        .iter()
        .map(|row| row.family_identity())
}
