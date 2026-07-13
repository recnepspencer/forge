use crate::access::execution::BTreeLookupReady;
use crate::keyspace::AdmittedPhysicalAccessIdentity;
use crate::planning::{AccessPlanIdentity, SelectedBTreeReplayRecovery};

use super::BaselineBTreeExecutionDenial;

#[derive(Debug, PartialEq, Eq)]
struct BaselineBTreeOperationAdmission {
    request_identity: AdmittedPhysicalAccessIdentity,
    plan_binding: AccessPlanIdentity,
}

impl BaselineBTreeOperationAdmission {
    const fn issue(
        request_identity: AdmittedPhysicalAccessIdentity,
        plan_binding: AccessPlanIdentity,
    ) -> Self {
        Self {
            request_identity,
            plan_binding,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BaselineBTreeLookupAdmission {
    operation: BaselineBTreeOperationAdmission,
    current_materialization: crate::CurrentLayoutMaterialization,
}

impl BaselineBTreeLookupAdmission {
    pub fn admit(ready: BTreeLookupReady) -> Self {
        let selected = ready.selected();
        Self {
            operation: BaselineBTreeOperationAdmission::issue(
                selected.request_identity(),
                selected.fingerprint().clone(),
            ),
            current_materialization: ready.current_materialization().clone(),
        }
    }
    pub(crate) fn plan_binding(&self) -> &AccessPlanIdentity {
        &self.operation.plan_binding
    }
    pub(crate) fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
        self.operation.request_identity
    }
    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BaselineBTreeReplayAdmission {
    operation: BaselineBTreeOperationAdmission,
    materialization: crate::AdmittedLayoutMaterialization,
}

impl BaselineBTreeReplayAdmission {
    pub fn admit(
        selected: SelectedBTreeReplayRecovery,
    ) -> Result<Self, BaselineBTreeExecutionDenial> {
        let materialization = selected
            .materialization()
            .ok_or(BaselineBTreeExecutionDenial::WrongSelectedOperation)?;
        Ok(Self {
            operation: BaselineBTreeOperationAdmission::issue(
                selected.request_identity(),
                selected.fingerprint().clone(),
            ),
            materialization: materialization.clone(),
        })
    }
    pub(crate) fn plan_binding(&self) -> &AccessPlanIdentity {
        &self.operation.plan_binding
    }
    pub(crate) fn request_identity(&self) -> AdmittedPhysicalAccessIdentity {
        self.operation.request_identity
    }
    pub(crate) const fn materialization(&self) -> &crate::AdmittedLayoutMaterialization {
        &self.materialization
    }
}
