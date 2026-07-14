use worth_store_physical_isolation::{CompactionReadInterlockPlan, CompactionRewritePublication};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LsmPhysicalCompactionIntent {
    plan: CompactionReadInterlockPlan,
    target_manifest_epoch: u64,
}

impl LsmPhysicalCompactionIntent {
    pub fn from_interlock_plan(
        plan: CompactionReadInterlockPlan,
        target_manifest_epoch: u64,
    ) -> Option<Self> {
        if target_manifest_epoch <= plan.protected().root().manifest_epoch().get() {
            return None;
        }
        Some(Self {
            plan,
            target_manifest_epoch,
        })
    }

    pub const fn plan(&self) -> &CompactionReadInterlockPlan {
        &self.plan
    }

    pub fn root_scope(&self) -> u64 {
        self.plan.protected().root().scope()
    }

    pub const fn target_epoch(&self) -> u64 {
        self.plan.target_epoch().get()
    }

    pub const fn manifest_epoch(&self) -> u64 {
        self.target_manifest_epoch
    }

    pub(crate) fn binds(&self, publication: &CompactionRewritePublication) -> bool {
        let old_root = publication.publication().old_root();
        let new_root = publication.publication().new_root();
        publication.delta().plan() == &self.plan
            && old_root.scope() == self.root_scope()
            && new_root.epoch().get() == self.target_epoch()
            && new_root.manifest_epoch().get() == self.manifest_epoch()
    }
}
