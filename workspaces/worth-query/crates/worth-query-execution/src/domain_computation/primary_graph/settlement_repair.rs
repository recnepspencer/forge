#[derive(Debug)]
pub enum WorthQueryApplicationSettlementRecoveryError {
    Durability(worth_relational::facade::publication::DeferredPublicationSettlementError),
    Publication(&'static str),
    IdempotencyAbsent,
    IdempotencyDrift,
}

impl<Schema> super::WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    pub fn recover_deferred_application_settlement(
        &self,
        deferred: &super::WorthQueryApplicationSettlementDeferred,
    ) -> Result<
        worth_relational::facade::history::RelationalCommitReceipt,
        WorthQueryApplicationSettlementRecoveryError,
    > {
        self.primary_provider.recover_application_settlement(
            deferred.settlement(),
            deferred.branch(),
            deferred.idempotency_binding(),
        )
    }
}
