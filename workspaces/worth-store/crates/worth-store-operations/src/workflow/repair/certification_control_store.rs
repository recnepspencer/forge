use worth_store_authority::StoreCurrentAuthorityWitness;

use crate::{
    AuthorizationRevocationObservation, IndeterminateRepairRecoveryHandle, OperationalControlStore,
    OperationalTransitionId,
};

use super::{
    AuthorityAffectingRepairReadinessDenial, AuthorizedAuthorityAffectingRepairPlan,
    AuthorizedRepairPlan, ExecutionReadyAuthorityAffectingRepair, ExecutionReadyRepair,
    LoweredAuthorityAffectingRepairOwnerPlanDag, LoweredRepairOwnerPlanDag, RepairReadinessDenial,
};

impl AuthorizedRepairPlan {
    pub fn ready_with_certification_control_store<'a>(
        self,
        control: &'a OperationalControlStore,
        append: &'a dyn crate::OperationalControlStorePort,
        transition: OperationalTransitionId,
        current: &StoreCurrentAuthorityWitness,
        observed_at: u64,
        revocation: AuthorizationRevocationObservation,
    ) -> Result<ExecutionReadyRepair<'a>, RepairReadinessDenial> {
        self.ready_through(
            control,
            append,
            transition,
            current,
            observed_at,
            revocation,
        )
    }
}

impl LoweredRepairOwnerPlanDag {
    pub fn recover_ready_with_certification_control_store<'a>(
        self,
        handle: &IndeterminateRepairRecoveryHandle,
        control: &'a OperationalControlStore,
        append: &'a dyn crate::OperationalControlStorePort,
        current: &StoreCurrentAuthorityWitness,
    ) -> Result<ExecutionReadyRepair<'a>, RepairReadinessDenial> {
        self.recover_ready_through(handle, control, append, current)
    }
}

impl AuthorizedAuthorityAffectingRepairPlan {
    pub fn ready_with_certification_control_store<'a>(
        self,
        control: &'a OperationalControlStore,
        append: &'a dyn crate::OperationalControlStorePort,
        transition: OperationalTransitionId,
        current: &StoreCurrentAuthorityWitness,
        observed_at: u64,
        revocation: AuthorizationRevocationObservation,
    ) -> Result<ExecutionReadyAuthorityAffectingRepair<'a>, AuthorityAffectingRepairReadinessDenial>
    {
        self.ready_through(
            control,
            append,
            transition,
            current,
            observed_at,
            revocation,
        )
    }
}

impl LoweredAuthorityAffectingRepairOwnerPlanDag {
    pub fn recover_ready_with_certification_control_store<'a>(
        self,
        handle: &IndeterminateRepairRecoveryHandle,
        control: &'a OperationalControlStore,
        append: &'a dyn crate::OperationalControlStorePort,
        current: &StoreCurrentAuthorityWitness,
    ) -> Result<ExecutionReadyAuthorityAffectingRepair<'a>, AuthorityAffectingRepairReadinessDenial>
    {
        self.recover_ready_through(handle, control, append, current)
    }
}
