use forge_store_physical_isolation::CompactionRewritePublication;

use super::{BaselineLsmExecutionAdmissionDenial, PreparedLsmCompaction};

#[derive(Debug, Clone)]
pub struct InterlockedLsmCompaction {
    pub(crate) prepared: PreparedLsmCompaction,
    pub(crate) physical: CompactionRewritePublication,
}

impl InterlockedLsmCompaction {
    pub fn prepare_membership_activation(
        &self,
    ) -> Result<
        forge_store_lsm_authority::LsmMembershipActivationDeclaration,
        BaselineLsmExecutionAdmissionDenial,
    > {
        forge_store_lsm_authority::prepare_lsm_membership_activation(
            &self.prepared.membership,
            self.prepared.output.clone(),
            &self.physical,
        )
        .map_err(super::execution::map_membership_denial)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LsmPhysicalCompactionRuntime;

pub const fn lsm_physical_compaction_runtime() -> LsmPhysicalCompactionRuntime {
    LsmPhysicalCompactionRuntime
}

impl LsmPhysicalCompactionRuntime {
    pub fn admit(
        self,
        prepared: PreparedLsmCompaction,
        physical: CompactionRewritePublication,
    ) -> Result<InterlockedLsmCompaction, BaselineLsmExecutionAdmissionDenial> {
        let old_root = physical.publication().old_root();
        let new_root = physical.publication().new_root();
        let expected = &prepared.physical_intent;
        if physical.delta().plan() != expected.plan()
            || old_root.scope() != expected.root_scope()
            || new_root.epoch().get() != expected.target_epoch()
            || new_root.manifest_epoch().get() != expected.manifest_epoch()
        {
            return Err(BaselineLsmExecutionAdmissionDenial::PhysicalPublicationBindingMismatch);
        }
        Ok(InterlockedLsmCompaction { prepared, physical })
    }
}
