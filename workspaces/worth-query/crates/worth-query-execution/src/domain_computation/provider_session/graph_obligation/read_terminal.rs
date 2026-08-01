use super::{
    basis_release::WorthQueryGraphWorkBasisRelease, WorthQueryCompleteGraphWorkDecisionReadSet,
    WorthQueryGraphWorkSessionReleaseReceipt, WorthQueryReadGraphWorkLane,
};

impl<Basis> WorthQueryCompleteGraphWorkDecisionReadSet<WorthQueryReadGraphWorkLane, Basis>
where
    Basis: WorthQueryGraphWorkBasisRelease,
{
    pub(in crate::domain_computation) fn finish_read(
        self,
    ) -> WorthQueryGraphWorkSessionReleaseReceipt {
        self.session.release()
    }
}

impl<Basis> super::WorthQueryManagedGraphWorkSession<WorthQueryReadGraphWorkLane, Basis>
where
    Basis: WorthQueryGraphWorkBasisRelease,
{
    pub(in crate::domain_computation) fn abort_read(
        self,
    ) -> WorthQueryGraphWorkSessionReleaseReceipt {
        self.release()
    }
}
