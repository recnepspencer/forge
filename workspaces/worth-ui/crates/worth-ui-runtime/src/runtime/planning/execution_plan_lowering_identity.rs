use std::rc::Rc;

#[derive(Debug)]
struct WorthUiExecutionPlanLoweringIdentityAuthority;

/// Exact lineage shared only by facts and plans from one admitted lowering.
///
/// Semantic digests may compare plans across sessions, but they cannot grant
/// access to provenance carried by another lowering authority.
#[derive(Clone, Debug)]
pub(crate) struct WorthUiExecutionPlanLoweringIdentity {
    authority: Rc<WorthUiExecutionPlanLoweringIdentityAuthority>,
}

impl WorthUiExecutionPlanLoweringIdentity {
    pub(crate) fn seal() -> Self {
        Self {
            authority: Rc::new(WorthUiExecutionPlanLoweringIdentityAuthority),
        }
    }

    pub(crate) fn shares_authority_with(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.authority, &other.authority)
    }
}

impl PartialEq for WorthUiExecutionPlanLoweringIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.shares_authority_with(other)
    }
}

impl Eq for WorthUiExecutionPlanLoweringIdentity {}
